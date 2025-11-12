use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, Fields, FieldsNamed, FieldsUnnamed, Ident};

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

/// Returns the tokens that can destructure the provided fields into the idents from the ident fns.
///
/// Result does not include the type/variant name, just the braces:
/// - `{ field, field, field }` for named fields
/// - `( field, field, field )` for unnamed fields
pub fn destructure_fields(
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
