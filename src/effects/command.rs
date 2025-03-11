use std::marker::PhantomData;

use bevy::prelude::*;

use crate::Effect;

/// [`Effect`] that pushes a generic command to the command queue.
#[doc = include_str!("defer_command_note.md")]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct CommandQueue<C>(pub C)
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

/// [`Effect`] that queues a command for inserting the provided `Resource` in the `World`.
#[doc = include_str!("defer_command_note.md")]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct CommandInsertResource<R>(pub R)
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

/// [`Effect`] that queues a command for removing a `Resource` from the `World`.
#[doc = include_str!("defer_command_note.md")]
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
    /// Construct a new [`CommandRemoveResource`]
    pub fn new() -> Self {
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
