use syn::{parse_quote, GenericParam, Generics, TypeParam};

fn type_param_with_effect_bound(type_param: TypeParam) -> TypeParam {
    let bounds = type_param
        .bounds
        .into_iter()
        .chain(core::iter::once(parse_quote!(bevy_pipe_affect::Effect)))
        .collect();

    TypeParam {
        bounds,
        ..type_param
    }
}

fn generic_param_with_effect_bound(generic_param: GenericParam) -> GenericParam {
    match generic_param {
        GenericParam::Type(type_param) => {
            GenericParam::Type(type_param_with_effect_bound(type_param))
        }
        p => p,
    }
}

/// Returns the provided generics with the bound `: Effect` applied to the type params.
pub fn generics_with_effect_bounds(generics: Generics) -> Generics {
    let params = generics
        .params
        .into_iter()
        .map(generic_param_with_effect_bound)
        .collect();

    Generics { params, ..generics }
}
