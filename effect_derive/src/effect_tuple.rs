use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Data, DataEnum, Field, Fields};

fn type_for_field(field: &Field) -> TokenStream {
    let ty = &field.ty;
    quote_spanned! { field.span() =>
        #ty
    }
}

fn effect_tuple_for_field_iter<'a>(fields: impl IntoIterator<Item = &'a Field>) -> TokenStream {
    let fields = fields.into_iter().map(type_for_field);

    quote! {
        ( #(#fields,)* )
    }
}

fn effect_tuple_for_fields(fields: &Fields) -> TokenStream {
    match fields {
        Fields::Named(fields) => effect_tuple_for_field_iter(&fields.named),
        Fields::Unnamed(fields) => effect_tuple_for_field_iter(&fields.unnamed),
        Fields::Unit => quote! { () },
    }
}

fn effect_tuple_for_enum(data_enum: &DataEnum) -> TokenStream {
    let variants = data_enum
        .variants
        .iter()
        .map(|v| effect_tuple_for_fields(&v.fields));

    quote! {
        ( #(#variants,)* )
    }
}

/// Returns the tokens for a tuple type with the same elements as the provided type's fields.
///
/// For enums, a nested tuple with an element per variant is returned.
pub fn effect_tuple_for_data(data: &Data) -> TokenStream {
    match data {
        Data::Struct(data) => effect_tuple_for_fields(&data.fields),
        Data::Enum(data) => effect_tuple_for_enum(data),
        Data::Union(_) => unimplemented!(),
    }
}
