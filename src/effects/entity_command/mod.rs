//! [`Effect`]s that queue entity-specific `Commands`.

mod entity_command_queue;
pub use entity_command_queue::{EntityCommandQueue, entity_command_queue};

mod entity_command_insert;
pub use entity_command_insert::{EntityCommandInsert, entity_command_insert};

mod entity_command_remove;
pub use entity_command_remove::{EntityCommandRemove, entity_command_remove};

mod entity_command_despawn;
pub use entity_command_despawn::{EntityCommandDespawn, entity_command_despawn};

mod entity_command_insert_recursive;
pub use entity_command_insert_recursive::{
    EntityCommandInsertRecursive,
    entity_command_insert_recursive,
};

mod entity_command_remove_recursive;
pub use entity_command_remove_recursive::{
    EntityCommandRemoveRecursive,
    entity_command_remove_recursive,
};
