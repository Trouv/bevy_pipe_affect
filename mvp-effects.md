# Resource Effects
- [x] `ResSet`
- [x] `ResSetWith`

# Local?
- [ ] `LocalSet`?
- [ ] `LocalSetWith`?

*Need to research.. Could lead to confusing experience about which system the parameter is local to.*

# Event Effects
- [x] `EventWrite`

# Components Effects
- [x] `ComponentsSet`
- [x] `ComponentsSetWith`

# EntityComponents Effects
- [x] `EntityComponentsSet`
- [x] `EntityComponentsSetWith`

# Command Effects
- [x] `CommandQueue<C>`

- [x] `CommandInsertResource`
- [x] `CommandRemoveResource`

- [x] `CommandSpawnAnd`

## Nice to have
- [ ] `CommandSpawnBatch`
- [ ] `CommandInsertBatch`

- [ ] `CommandRunSystem`
- [ ] `CommandRunSystemWith`
- [ ] `CommandRegisterSystemAnd`
- [ ] `CommandUnregisterSystemAnd`

- [ ] `CommandTrigger`
- [ ] `CommandTriggerTargets`
- [ ] `CommandAddObserverAnd`

- [ ] `CommandRunSchedule`

# Entity command effects:
- [x] `EntityCommandQueue<C>`

- [x] `EntityCommandInsert`
- [x] `EntityCommandRemove`
- [x] `EntityCommandDespawn`

## Nice to have
- [ ] `EntityCommandRetain`
- [ ] `EntityCommandClear`

- [ ] `EntityCommandObserve`
- [ ] `EntityCommandTrigger`

- [ ] `EntityCommandSetParentInPlace`
- [ ] `EntityCommandRemoveParentInPlace`

- [ ] `EntityCommandInsertRecursive`
- [ ] `EntityCommandRemoveRecursive`

*Most relationship/target commands have feature parity with plain insertions
and removals on the relationship entity, so they don't have effects for mvp*

# Algebra effects
- [x] `(E0, E1, ... En)`
- [x] `()`
- [x] `Either<E0, E1>`

# Iterator Effects
- [x] `AffectMany`
- [x] `Vec<E>`
- [x] `Option<E>`

# Result Effects
- [x] `AffectOrHandle(Result<E, Er>, Fn(BevyError, ErrorContext))`
- [x] `Result<E, Er>` *uses global error handler, or panics if unavailable, just like vanilla bevy*
