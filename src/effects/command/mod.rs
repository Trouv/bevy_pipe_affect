//! [`Effect`]s that queue `Commands`.
//!
//! [`Effect`]: crate::Effect

mod command_queue;
pub use command_queue::{CommandQueue, command_queue};

mod command_insert_resource;
pub use command_insert_resource::{CommandInsertResource, command_insert_resource};

mod command_remove_resource;
pub use command_remove_resource::{CommandRemoveResource, command_remove_resource};

mod command_spawn_and;
pub use command_spawn_and::{CommandSpawn, CommandSpawnAnd, command_spawn, command_spawn_and};

mod command_trigger;
pub use command_trigger::{CommandTrigger, command_trigger};
