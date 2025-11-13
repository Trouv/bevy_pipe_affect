use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, Fields, FieldsNamed, FieldsUnnamed, Ident};

fn destructure_named_fields(
    fields: &FieldsNamed,
    field_ident_fn: impl Fn((usize, &Field)) -> Ident,
) -> TokenStream {
    let names = fields.named.iter().enumerate().map(field_ident_fn);

    quote! {
        { #(#names,)* }
    }
}

fn destructure_unnamed_fields(
    fields: &FieldsUnnamed,
    field_ident_fn: impl Fn((usize, &Field)) -> Ident,
) -> TokenStream {
    let names = fields.unnamed.iter().enumerate().map(field_ident_fn);

    quote! {
        ( #(#names,)* )
    }
}

/// Returns the tokens that can destructure the provided fields into the idents from the ident fns.
///
/// Result does not include the type/variant name, just the braces:
/// - `{ field, field, field }` for named fields
/// - `( field, field, field )` for unnamed fields
pub fn destructure_fields(
    fields: &Fields,
    field_ident_fn: impl Fn((usize, &Field)) -> Ident,
) -> TokenStream {
    match &fields {
        Fields::Named(fields) => destructure_named_fields(fields, field_ident_fn),
        Fields::Unnamed(fields) => destructure_unnamed_fields(fields, field_ident_fn),
        Fields::Unit => quote! {},
    }
}
