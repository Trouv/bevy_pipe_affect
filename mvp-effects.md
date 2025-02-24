# Resource Effects
- [x] `ResPut`
- [x] `ResWith`

# Local?
- [ ] `LocalPut`?
- [ ] `LocalWith`?

*Need to research.. Could lead to confusing experience about which system the parameter is local to.*

# Event Effects
- [x] `EventWrite`

# Components Effects
- [x] `ComponentsPut`
- [x] `ComponentsWith`

# EntityComponents Effects
- [x] `EntityComponentsPut`
- [x] `EntityComponentsWith`

# Command Effects
- [x] `CommandQueue<C>`
- [x] `CommandInsertResource`
- [x] `CommandRemoveResource`
- [x] `CommandSpawnEmptyAnd`

# Entity command effects:
- [ ] `CommandEntityInsert`
- [ ] `CommandEntityRemove`
- [ ] `CommandEntityDespawnRecursive`

*For MVP, `CommandEffect<C>` enables hierarchy commands*

# Tuple effects
- [x] `(E0, E1, ... En)`

# Iterator Effects
- [ ] `IterEffect`
- [ ] `Vec<E>`
- [ ] `Option<E>`

# Result Effects
- [ ] `OrLog(Result<E, Error>, LogLevel)`
- [ ] `Result<E, Error> = OrLog(..., LogLevel::Error)`
