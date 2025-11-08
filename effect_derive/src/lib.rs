use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input,
    parse_quote,
    Data,
    DeriveInput,
    Fields,
    FieldsNamed,
    FieldsUnnamed,
    GenericParam,
    Generics,
    Variant,
};

#[proc_macro_derive(Effect)]
pub fn derive_effect(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    // Add a bound `T: Effect` to every type parameter T.
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut_param = effect_tuple_for_data(&input.data);
    let affect_calls = affect_calls_for_data(&input.data);

    let expanded = quote! {
        // The generated impl.
        impl #impl_generics bevy_pipe_affect::Effect for #name #ty_generics #where_clause {
            type MutParam = <#mut_param as bevy_pipe_affect::Effect>::MutParam;

            fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
                #affect_calls
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

fn destructure_named_fields(fields: &FieldsNamed) -> TokenStream {
    let names = fields.named.iter().map(|f| &f.ident);

    quote! {
        { #(#names,)* }
    }
}

fn destructure_unnamed_fields(fields: &FieldsUnnamed) -> TokenStream {
    let names = (0..fields.unnamed.len()).map(|i| format_ident!("f{i}"));

    quote! {
        ( #(#names,)* )
    }
}

fn destructure_fields(fields: &Fields) -> TokenStream {
    match &fields {
        Fields::Named(fields) => destructure_named_fields(&fields),
        Fields::Unnamed(fields) => destructure_unnamed_fields(&fields),
        Fields::Unit => quote! {},
    }
}

// Add a bound `T: Effect` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
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

fn effect_tuple_for_fields(fields: &Fields) -> TokenStream {
    match fields {
        Fields::Named(fields) => {
            let recurse = fields.named.iter().map(|f| {
                let ty = &f.ty;
                quote_spanned! {f.span()=>
                    #ty
                }
            });
            quote! {
                (#(#recurse,)*)
            }
        }
        Fields::Unnamed(fields) => {
            let recurse = fields.unnamed.iter().map(|f| {
                let ty = &f.ty;
                quote_spanned! {f.span()=>
                    #ty
                }
            });
            quote! {
                (#(#recurse,)*)
            }
        }
        Fields::Unit => {
            quote! {
                ()
            }
        }
    }
}

// Generate an expression to sum up the heap size of each field.
fn affect_calls_for_data(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            let destructure = destructure_fields(&data.fields);
            let affect_calls = affect_calls_for_fields(&data.fields);
            quote! {
                let Self #destructure = self;
                #affect_calls
            }
        }
        Data::Enum(ref data) => affect_calls_for_variants(&data.variants),
        Data::Union(_) => unimplemented!(),
    }
}

fn affect_calls_for_variants<'a>(variants: impl IntoIterator<Item = &'a Variant>) -> TokenStream {
    let recurse = variants.into_iter().enumerate().map(|(i, v)| {
        let name = &v.ident;
        let param_method = format_ident!("p{}", i);
        let affect_calls = affect_calls_for_fields(&v.fields);

        let destructure = destructure_fields(&v.fields);

        quote_spanned! {v.span()=>
            Self::#name #destructure => {
                #[allow(unused_variables, unused_mut)]
                let mut param = param.#param_method();
                #affect_calls
            }
        }
    });

    quote! {
        match self {
            #(#recurse)*
        }
    }
}

fn affect_calls_for_fields(fields: &Fields) -> TokenStream {
    match fields {
        Fields::Named(fields) => {
            let recurse = fields.named.iter().enumerate().map(|(i, f)| {
                let name = &f.ident;
                let param_method = format_ident!("p{}", i);
                quote_spanned! {f.span()=>
                    bevy_pipe_affect::Effect::affect(#name, &mut param.#param_method());
                }
            });
            quote! {
                #(#recurse)*
            }
        }
        Fields::Unnamed(fields) => {
            let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                let name = format_ident!("f{i}");
                let param_method = format_ident!("p{}", i);
                quote_spanned! {f.span()=>
                    bevy_pipe_affect::Effect::affect(#name, &mut param.#param_method());
                }
            });
            quote! {
                #(#recurse)*
            }
        }
        Fields::Unit => {
            quote!()
        }
    }
}
