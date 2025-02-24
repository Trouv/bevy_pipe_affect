use std::marker::PhantomData;

use bevy::prelude::*;

use crate::Effect;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct CommandQueue<C>(C)
where
    C: Command;

impl<C> Effect for CommandQueue<C>
where
    C: Command,
{
    type MutParam = Commands<'static, 'static>;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        param.queue(self.0)
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct CommandInsertResource<R>(R)
where
    R: Resource;

impl<R> Effect for CommandInsertResource<R>
where
    R: Resource,
{
    type MutParam = Commands<'static, 'static>;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        param.insert_resource(self.0);
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct CommandRemoveResource<R>
where
    R: Resource,
{
    resource: PhantomData<R>,
}

impl<R> CommandRemoveResource<R>
where
    R: Resource,
{
    fn new() -> Self {
        CommandRemoveResource {
            resource: PhantomData,
        }
    }
}

impl<R> Effect for CommandRemoveResource<R>
where
    R: Resource,
{
    type MutParam = Commands<'static, 'static>;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        param.remove_resource::<R>();
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct CommandSpawnEmptyAnd<F, E>(F)
where
    F: FnOnce(Entity) -> E,
    E: Effect;

impl<F, E> Effect for CommandSpawnEmptyAnd<F, E>
where
    F: FnOnce(Entity) -> E,
    E: Effect,
{
    type MutParam = ParamSet<'static, 'static, (Commands<'static, 'static>, E::MutParam)>;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        let entity = param.p0().spawn_empty().id();
        (self.0)(entity).affect(&mut param.p1());
    }
}
