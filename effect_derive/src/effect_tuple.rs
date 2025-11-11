use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Data, Field, Fields, Variant};

pub fn effect_tuple_for_data(data: &Data) -> TokenStream {
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
