use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod generics;

mod effect_tuple;

mod destructure;

mod affect_fn;

#[proc_macro_derive(Effect)]
pub fn derive_effect(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    // Add a bound `T: Effect` to every type parameter T.
    let generics = generics::generics_with_effect_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut_param = effect_tuple::effect_tuple_for_data(&input.data);

    let affect_fn = affect_fn::affect_fn_for_data(&input.data, &name);

    let expanded = quote! {
        // The generated impl.
        impl #impl_generics bevy_pipe_affect::Effect for #name #ty_generics #where_clause {
            type MutParam = <#mut_param as bevy_pipe_affect::Effect>::MutParam;

            #affect_fn
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}
