//! Some basic `Effect`-deriving types that should compile.
#![allow(dead_code, clippy::enum_variant_names)]

use bevy::prelude::*;
use bevy_pipe_affect::prelude::*;

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
struct MyGenericTupleStruct<T, U>(T, U);

#[derive(Effect)]
struct MyGenericStruct<T, U> {
    generic_effect_1: T,
    generic_effect_2: U,
}

#[derive(Effect)]
enum MyGenericEnum<T, U, V> {
    MyUnitVariant,
    MyOneStructVariant(T),
    MyTwoStructVariant {
        generic_effect_1: U,
        generic_effect_2: V,
    },
}
