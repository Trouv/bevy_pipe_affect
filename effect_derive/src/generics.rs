use syn::{parse_quote, GenericParam, Generics};

pub fn generics_with_effect_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(parse_quote!(bevy_pipe_affect::Effect));
        }
    }
    generics
}
