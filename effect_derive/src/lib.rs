use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input,
    parse_quote,
    Data,
    DataEnum,
    DataStruct,
    DeriveInput,
    Field,
    Fields,
    FieldsNamed,
    FieldsUnnamed,
    GenericParam,
    Generics,
    Ident,
    Variant,
};

#[proc_macro_derive(Effect)]
pub fn derive_effect(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    // Add a bound `T: Effect` to every type parameter T.
    let generics = generics_with_effect_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut_param = effect_tuple_for_data(&input.data);

    let params_ident = format_ident!("params");
    let affect_calls =
        affect_calls_for_data(&input.data, &format_ident!("self"), &name, &params_ident);

    let expanded = quote! {
        // The generated impl.
        impl #impl_generics bevy_pipe_affect::Effect for #name #ty_generics #where_clause {
            type MutParam = <#mut_param as bevy_pipe_affect::Effect>::MutParam;

            fn affect(self, #params_ident: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
                #affect_calls
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

// Add a bound `T: Effect` to every type parameter T.
fn generics_with_effect_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(parse_quote!(bevy_pipe_affect::Effect));
        }
    }
    generics
}

fn effect_tuple_for_data(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => effect_tuple_for_fields(&data.fields),
        Data::Enum(ref data) => effect_tuple_for_variants(&data.variants),
        Data::Union(_) => unimplemented!(),
    }
}

fn effect_tuple_for_variants<'a>(variants: impl IntoIterator<Item = &'a Variant>) -> TokenStream {
    let recurse = variants
        .into_iter()
        .map(|v| effect_tuple_for_fields(&v.fields));

    quote! {
        (#(#recurse,)*)
    }
}

fn type_for_field(field: &Field) -> TokenStream {
    let ty = &field.ty;
    quote_spanned! { field.span() =>
        #ty
    }
}

fn effect_tuple_for_fields(fields: &Fields) -> TokenStream {
    let fields = match fields {
        Fields::Named(fields) => fields.named.iter().map(type_for_field).collect::<Vec<_>>(),
        Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .map(type_for_field)
            .collect::<Vec<_>>(),
        Fields::Unit => Vec::new(),
    };

    quote! {
        (#(#fields,)*)
    }
}

fn effect_ident_for_named_field(field: &Field) -> &Option<Ident> {
    &field.ident
}

fn effect_ident_for_unnamed_field(field_index: usize) -> Ident {
    format_ident!("f{field_index}")
}

fn destructure_named_fields(
    fields: &FieldsNamed,
    ident_fn: impl Fn(&Field) -> &Option<Ident>,
) -> TokenStream {
    let names = fields.named.iter().map(ident_fn);

    quote! {
        { #(#names,)* }
    }
}

fn destructure_unnamed_fields(
    fields: &FieldsUnnamed,
    ident_fn: impl Fn(usize) -> Ident,
) -> TokenStream {
    let names = (0..fields.unnamed.len()).map(ident_fn);

    quote! {
        ( #(#names,)* )
    }
}

fn destructure_fields(
    fields: &Fields,
    named_ident_fn: impl Fn(&Field) -> &Option<Ident>,
    unnamed_ident_fn: impl Fn(usize) -> Ident,
) -> TokenStream {
    match &fields {
        Fields::Named(fields) => destructure_named_fields(fields, named_ident_fn),
        Fields::Unnamed(fields) => destructure_unnamed_fields(fields, unnamed_ident_fn),
        Fields::Unit => quote! {},
    }
}

fn param_method(param_index: usize) -> TokenStream {
    let ident = format_ident!("p{param_index}");

    quote! {
        #ident()
    }
}

fn affect_call(effect_ident: &Ident, param_ident: &Ident) -> TokenStream {
    quote_spanned! { effect_ident.span() =>
        bevy_pipe_affect::Effect::affect(#effect_ident, &mut #param_ident);
    }
}

fn affect_calls_for_named_fields(
    fields: &FieldsNamed,
    field_ident_fn: impl Fn(&Field) -> &Option<Ident>,
    params_ident: &Ident,
) -> TokenStream {
    let affect_calls = fields.named.iter().enumerate().map(|(field_index, field)| {
        let field_ident = field_ident_fn(field)
            .as_ref()
            .expect("named fields should have idents");
        let param_ident = format_ident!("param");
        let param_method = param_method(field_index);
        let affect_call = affect_call(field_ident, &param_ident);

        quote_spanned! { field.span() =>
            let mut #param_ident = #params_ident.#param_method;
            #affect_call
        }
    });

    quote! {
        #(#affect_calls)*
    }
}

fn affect_calls_for_unnamed_fields(
    fields: &FieldsUnnamed,
    field_ident_fn: impl Fn(usize) -> Ident,
    params_ident: &Ident,
) -> TokenStream {
    let affect_calls = fields
        .unnamed
        .iter()
        .enumerate()
        .map(|(field_index, field)| {
            let field_ident = field_ident_fn(field_index);
            let param_ident = format_ident!("param");
            let param_method = param_method(field_index);
            let affect_call = affect_call(&field_ident, &param_ident);

            quote_spanned! { field.span() =>
                let mut #param_ident = #params_ident.#param_method;
                #affect_call
            }
        });

    quote! {
        #(#affect_calls)*
    }
}

fn affect_calls_for_fields(
    fields: &Fields,
    named_field_ident_fn: impl Fn(&Field) -> &Option<Ident>,
    unnamed_field_ident_fn: impl Fn(usize) -> Ident,
    params_ident: &Ident,
) -> TokenStream {
    match fields {
        Fields::Named(fields) => {
            affect_calls_for_named_fields(fields, named_field_ident_fn, params_ident)
        }
        Fields::Unnamed(fields) => {
            affect_calls_for_unnamed_fields(fields, unnamed_field_ident_fn, params_ident)
        }
        Fields::Unit => {
            quote! {}
        }
    }
}

fn affect_calls_for_struct(
    data_struct: &DataStruct,
    struct_ident: &Ident,
    struct_ty: &Ident,
    params_ident: &Ident,
) -> TokenStream {
    let destructure = destructure_fields(
        &data_struct.fields,
        effect_ident_for_named_field,
        effect_ident_for_unnamed_field,
    );
    let affect_calls = affect_calls_for_fields(
        &data_struct.fields,
        effect_ident_for_named_field,
        effect_ident_for_unnamed_field,
        params_ident,
    );
    quote! {
        let #struct_ty #destructure = #struct_ident;
        #affect_calls
    }
}

fn affect_calls_for_enum(
    data_enum: &DataEnum,
    enum_ident: &Ident,
    enum_ty: &Ident,
    params_ident: &Ident,
) -> TokenStream {
    let affect_calls = data_enum
        .variants
        .iter()
        .enumerate()
        .map(|(variant_index, variant)| {
            let name = &variant.ident;
            let destructure = destructure_fields(
                &variant.fields,
                effect_ident_for_named_field,
                effect_ident_for_unnamed_field,
            );
            let variant_params_ident = format_ident!("variant_params");
            let param_method = param_method(variant_index);
            let affect_calls = affect_calls_for_fields(
                &variant.fields,
                effect_ident_for_named_field,
                effect_ident_for_unnamed_field,
                &variant_params_ident,
            );

            quote_spanned! { variant.span()=>
                #enum_ty::#name #destructure => {
                    #[allow(unused_variables, unused_mut)]
                    let mut #variant_params_ident = #params_ident.#param_method;
                    #affect_calls
                }
            }
        });

    quote! {
        match #enum_ident {
            #(#affect_calls)*
        }
    }
}

// Generate an expression to sum up the heap size of each field.
fn affect_calls_for_data(
    data: &Data,
    type_ident: &Ident,
    type_ty: &Ident,
    params_ident: &Ident,
) -> TokenStream {
    match *data {
        Data::Struct(ref data) => affect_calls_for_struct(data, type_ident, type_ty, params_ident),
        Data::Enum(ref data) => affect_calls_for_enum(data, type_ident, type_ty, params_ident),
        Data::Union(_) => unimplemented!(),
    }
}
