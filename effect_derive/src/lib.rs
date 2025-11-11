use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

mod generics;

mod effect_tuple;

mod destructure;

mod affect_calls;

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

    let params_ident = format_ident!("params");
    let affect_calls = affect_calls::affect_calls_for_data(
        &input.data,
        &format_ident!("self"),
        &name,
        &params_ident,
    );

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
