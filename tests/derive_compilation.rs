//! Some basic `Effect`-deriving types that should compile.
#![cfg(feature = "derive")]
#![allow(dead_code, clippy::enum_variant_names)]

use bevy::ecs::component::Mutable;
use bevy::prelude::*;
use bevy_pipe_affect::prelude::*;

/// Fails to compile if a non-effect is used as `E`.
fn type_implements_effect<E: Effect>() -> bool {
    true
}

/// Fails to compile if a non-effect is passed in.
fn value_implements_effect<E: Effect>(_effect: E) -> bool {
    true
}

#[derive(Message)]
struct MessageN<const N: usize>;

#[derive(Effect)]
struct MyUnitStruct;

#[derive(Effect)]
struct MyZeroTupleStruct();

#[derive(Effect)]
struct MyOneTupleStruct(MessageWrite<MessageN<0>>);

#[derive(Effect)]
struct MyThreeTupleStruct(
    MessageWrite<MessageN<1>>,
    MessageWrite<MessageN<2>>,
    MessageWrite<MessageN<3>>,
);

#[derive(Effect)]
struct MyZeroFieldStruct {}

#[derive(Effect)]
struct MyOneFieldStruct {
    message_4: MessageWrite<MessageN<4>>,
}

#[derive(Effect)]
struct MyThreeFieldStruct {
    message_5: MessageWrite<MessageN<5>>,
    message_6: MessageWrite<MessageN<6>>,
    message_7: MessageWrite<MessageN<7>>,
}

#[derive(Effect)]
enum MyZeroEnum {}

#[derive(Effect)]
enum MyOneEnum {
    MyUnitVariant,
}

#[derive(Effect)]
enum MyComplexEnum {
    MyUnitVariant,
    MyZeroTupleVariant(),
    MyOneTupleVariant(MessageWrite<MessageN<8>>),
    MyTupleVariant(MessageWrite<MessageN<9>>, MessageWrite<MessageN<10>>),
    MyZeroStructVariant {},
    MyOneStructVariant {
        message_11: MessageWrite<MessageN<11>>,
    },
    MyStructVariant {
        message_12: MessageWrite<MessageN<12>>,
        message_13: MessageWrite<MessageN<13>>,
    },
}

#[derive(Effect)]
struct MyGenericTupleStruct<T: Effect, U: Effect>(T, U);

#[derive(Effect)]
struct MyGenericStruct<T: Effect, U: Effect> {
    generic_effect_1: T,
    generic_effect_2: U,
}

#[derive(Effect)]
enum MyGenericEnum<T: Effect, U: Effect, V: Effect> {
    MyUnitVariant,
    MyOneStructVariant(T),
    MyTwoStructVariant {
        generic_effect_1: U,
        generic_effect_2: V,
    },
}

#[test]
fn effect_with_effect_parameter_implements_effect() {
    assert!(type_implements_effect::<
        MyGenericTupleStruct<MessageWrite<MessageN<0>>, MessageWrite<MessageN<1>>>,
    >());
    assert!(type_implements_effect::<
        MyGenericStruct<MessageWrite<MessageN<2>>, MessageWrite<MessageN<3>>>,
    >());
    assert!(type_implements_effect::<
        MyGenericEnum<
            MessageWrite<MessageN<4>>,
            MessageWrite<MessageN<5>>,
            MessageWrite<MessageN<6>>,
        >,
    >());
}

#[derive(Clone, Component, Resource)]
struct MyComponentResource;

#[derive(Effect)]
struct SetResAndComponent<C: Clone + Resource + Component<Mutability = Mutable>> {
    resource: ResSet<C>,
    components: ComponentsSet<(C,)>,
}

#[test]
fn effect_with_non_effect_parameter_can_implement_effect() {
    assert!(type_implements_effect::<
        SetResAndComponent<MyComponentResource>,
    >());
}

#[derive(Effect)]
struct MyEffectWithFunction {
    effect_with_fn: CommandSpawnAnd<MyComponentResource, MessageWrite<MessageN<100>>>,
}

#[test]
fn effect_with_fn_can_implement_effect() {
    assert!(value_implements_effect(command_spawn_and(
        MyComponentResource,
        |_| message_write(MessageN::<100>)
    )));
}
