use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Data, DataEnum, DataStruct, Field, Fields, FieldsNamed, FieldsUnnamed, Ident};

use crate::destructure::destructure_fields;

fn effect_ident_for_named_field(field: &Field) -> &Option<Ident> {
    &field.ident
}

fn effect_ident_for_unnamed_field(field_index: usize) -> Ident {
    format_ident!("f{field_index}")
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
            {
                let mut #param_ident = #params_ident.#param_method;
                #affect_call
            }
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
                {
                    let mut #param_ident = #params_ident.#param_method;
                    #affect_call
                }
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

fn affect_calls_for_data(
    data: &Data,
    value_ident: &Ident,
    type_ident: &Ident,
    params_ident: &Ident,
) -> TokenStream {
    match data {
        Data::Struct(data) => affect_calls_for_struct(data, value_ident, type_ident, params_ident),
        Data::Enum(data) => affect_calls_for_enum(data, value_ident, type_ident, params_ident),
        Data::Union(_) => unimplemented!(),
    }
}

/// Returns the tokens for an `Effect::affect` function for the given type.
pub fn affect_fn_for_data(data: &Data, type_ident: &Ident) -> TokenStream {
    let self_ident = format_ident!("self");
    let params_ident = format_ident!("params");

    let affect_calls = affect_calls_for_data(data, &self_ident, type_ident, &params_ident);

    quote! {
        fn affect(#self_ident, #params_ident: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
            #affect_calls
        }
    }
}
