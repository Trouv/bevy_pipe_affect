use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod generics;

mod mut_param_type;

mod destructure;

mod affect_fn;

#[proc_macro_derive(Effect)]
pub fn derive_effect(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let generics = generics::generics_with_effect_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut_param_type = mut_param_type::mut_param_type_for_data(&input.data);

    let affect_fn = affect_fn::affect_fn_for_data(&input.data, &name);

    let expanded = quote! {
        impl #impl_generics bevy_pipe_affect::Effect for #name #ty_generics #where_clause {
            #mut_param_type

            #affect_fn
        }
    };

    proc_macro::TokenStream::from(expanded)
}
