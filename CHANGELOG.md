# Changelog

## [0.2.0](https://github.com/Trouv/bevy_pipe_affect/compare/v0.1.0...v0.2.0) (2026-01-15)


### ⚠ BREAKING CHANGES

* split ComponentsSetWith into 4 with different generics ([#111](https://github.com/Trouv/bevy_pipe_affect/issues/111))
* upgrade to bevy 0.18 ([#110](https://github.com/Trouv/bevy_pipe_affect/issues/110))
* split ComponentsSet into two with and without filter ([#109](https://github.com/Trouv/bevy_pipe_affect/issues/109))
* split EntityComponentsSetWith into two with and without query data ([#108](https://github.com/Trouv/bevy_pipe_affect/issues/108))
* return simpler CommandSpawn effect from command_spawn constructor ([#107](https://github.com/Trouv/bevy_pipe_affect/issues/107))

### Features

* return simpler CommandSpawn effect from command_spawn constructor ([#107](https://github.com/Trouv/bevy_pipe_affect/issues/107)) ([4b58dea](https://github.com/Trouv/bevy_pipe_affect/commit/4b58dea13b82c632bedcb56241ea36c1b0206b92))
* split ComponentsSet into two with and without filter ([#109](https://github.com/Trouv/bevy_pipe_affect/issues/109)) ([b15c1a0](https://github.com/Trouv/bevy_pipe_affect/commit/b15c1a08ced2f5ce3a3e7e9285ed55868e68d1f6))
* split ComponentsSetWith into 4 with different generics ([#111](https://github.com/Trouv/bevy_pipe_affect/issues/111)) ([4efe59b](https://github.com/Trouv/bevy_pipe_affect/commit/4efe59b216ed8134efef03c5732677416494cec6))
* split EntityComponentsSetWith into two with and without query data ([#108](https://github.com/Trouv/bevy_pipe_affect/issues/108)) ([b362f45](https://github.com/Trouv/bevy_pipe_affect/commit/b362f456b68758fd127af81c3b1c2c65b79230c3))
* upgrade to bevy 0.18 ([#110](https://github.com/Trouv/bevy_pipe_affect/issues/110)) ([f12948b](https://github.com/Trouv/bevy_pipe_affect/commit/f12948bed1df3ad9990cb91549fc9adacdd23a8f))


### Documentation Changes

* add 0.2 to compatibility chart in README.md ([#114](https://github.com/Trouv/bevy_pipe_affect/issues/114)) ([8a95fac](https://github.com/Trouv/bevy_pipe_affect/commit/8a95fac4f68506ec9935689495a7cabc2dd33823))
* enable minimal features in mdbook-keeper ([#113](https://github.com/Trouv/bevy_pipe_affect/issues/113)) ([ceb0982](https://github.com/Trouv/bevy_pipe_affect/commit/ceb09820b8f186bc83c88e240b84e3ca980873d4))


### Example Changes

* add unit test to rainbow-clear-color example ([#99](https://github.com/Trouv/bevy_pipe_affect/issues/99)) ([f837c0f](https://github.com/Trouv/bevy_pipe_affect/commit/f837c0f4d4a1026622837b0f2a3328613211a666))


### CI Changes

* exclude component in release-please tag ([#105](https://github.com/Trouv/bevy_pipe_affect/issues/105)) ([2d3e16c](https://github.com/Trouv/bevy_pipe_affect/commit/2d3e16c69674d5f7129dcf7a2214221e052642bb)), closes [#94](https://github.com/Trouv/bevy_pipe_affect/issues/94)
* rename book changelog section to docs ([#106](https://github.com/Trouv/bevy_pipe_affect/issues/106)) ([413b4bc](https://github.com/Trouv/bevy_pipe_affect/commit/413b4bc5d6200e320410a52e37ff97c9c09e90b3))

## 0.1.0 (2025-12-26)


### Features

* add `-Resource` related `Command` effects ([#14](https://github.com/Trouv/bevy_pipe_affect/issues/14)) ([a5700d6](https://github.com/Trouv/bevy_pipe_affect/commit/a5700d66de411b796b7bd866b4c833405ee9988a))
* add `AffectOrHandle` effect and `Result` implementation ([#25](https://github.com/Trouv/bevy_pipe_affect/issues/25)) ([d94dad2](https://github.com/Trouv/bevy_pipe_affect/commit/d94dad29137f5c066ed18298ec679b6c8a3bdc46))
* add `AssetServerLoadAnd` effect ([#39](https://github.com/Trouv/bevy_pipe_affect/issues/39)) ([dea9772](https://github.com/Trouv/bevy_pipe_affect/commit/dea9772610eec05720bd1313b096a25b2fc582be))
* add `CommandQueue` effect ([#13](https://github.com/Trouv/bevy_pipe_affect/issues/13)) ([1362950](https://github.com/Trouv/bevy_pipe_affect/commit/13629504a71d8e2ebb2e23877539ca0b13f1aa01))
* add `CommandSpawnAnd` effect ([#20](https://github.com/Trouv/bevy_pipe_affect/issues/20)) ([5f2c04b](https://github.com/Trouv/bevy_pipe_affect/commit/5f2c04b069a2de1536dd1e1b3175bddcade4e0c1))
* add `EntityCommandDespawn` effect ([#21](https://github.com/Trouv/bevy_pipe_affect/issues/21)) ([bec00cf](https://github.com/Trouv/bevy_pipe_affect/commit/bec00cfdff5f78aa8914db7cc1568798906e175b))
* add `EntityCommandInsert` and `EntityCommandRemove` effects ([#18](https://github.com/Trouv/bevy_pipe_affect/issues/18)) ([dfd2e02](https://github.com/Trouv/bevy_pipe_affect/commit/dfd2e02f0b85d9d2ea011fa5385e5d0f98d10c21))
* add `EntityCommandQueue` effect ([#17](https://github.com/Trouv/bevy_pipe_affect/issues/17)) ([db6bdea](https://github.com/Trouv/bevy_pipe_affect/commit/db6bdea5f0821bb7f670f929e0bdbee982c0aa33))
* add `EntityComponentsPut` and `EntityComponentsWith` effects ([#11](https://github.com/Trouv/bevy_pipe_affect/issues/11)) ([417a71c](https://github.com/Trouv/bevy_pipe_affect/commit/417a71ccff5cfd8e116f676ba155bf59dbe5a529))
* add `EventSend` effect for writing to `EventWriter` ([#4](https://github.com/Trouv/bevy_pipe_affect/issues/4)) ([2b6a543](https://github.com/Trouv/bevy_pipe_affect/commit/2b6a5437cdd25cee954b5c54ef5476948c9b200b))
* add `IterEffect` effect and add `Vec` and `Option` implementations ([#24](https://github.com/Trouv/bevy_pipe_affect/issues/24)) ([de3e6a2](https://github.com/Trouv/bevy_pipe_affect/commit/de3e6a2d84850f2d0234c164ae8e8333bc07da4c))
* add `ResWith` effect for transforming a resource with a function ([#2](https://github.com/Trouv/bevy_pipe_affect/issues/2)) ([a5d7de2](https://github.com/Trouv/bevy_pipe_affect/commit/a5d7de27b004caa17dd020617dbc9f603e8e36da))
* add and_combine system combinator ([#36](https://github.com/Trouv/bevy_pipe_affect/issues/36)) ([49b935b](https://github.com/Trouv/bevy_pipe_affect/commit/49b935b2f9eb2bcc753cc53277899407f1afc449))
* add and_compose pipe combinator for piping into other systems and composing effects ([814f9af](https://github.com/Trouv/bevy_pipe_affect/commit/814f9af87063cd4bcf47fce669a82d80e56efa6d))
* add basic EffectOut composition methods ([#43](https://github.com/Trouv/bevy_pipe_affect/issues/43)) ([b9ae369](https://github.com/Trouv/bevy_pipe_affect/commit/b9ae369d24d7bbb05b73034adc0abebd96830e51))
* add book with outline ([#53](https://github.com/Trouv/bevy_pipe_affect/issues/53)) ([5b74ba2](https://github.com/Trouv/bevy_pipe_affect/commit/5b74ba2e8d77c244e78998f86197f07e31264c6e))
* add CommandTrigger Effect ([#70](https://github.com/Trouv/bevy_pipe_affect/issues/70)) ([0fc84f6](https://github.com/Trouv/bevy_pipe_affect/commit/0fc84f6d9d6d22babb77b0d8472b95c839e751dc))
* add common effect composition functions for use with and_compose" ([#33](https://github.com/Trouv/bevy_pipe_affect/issues/33)) ([c21d204](https://github.com/Trouv/bevy_pipe_affect/commit/c21d204ee1290d8133ecb85bfcab7f7e9cda454a))
* add ComponentsPut and ComponentsWith effects ([#7](https://github.com/Trouv/bevy_pipe_affect/issues/7)) ([3474927](https://github.com/Trouv/bevy_pipe_affect/commit/34749275bfea6f05bf36a597f4ff8969c5087166))
* add effect_out constructor ([#34](https://github.com/Trouv/bevy_pipe_affect/issues/34)) ([660a74d](https://github.com/Trouv/bevy_pipe_affect/commit/660a74de58e555a119a49416a29912ce9d43b802))
* add EffectOut to allow simultaneous piping of normal output ([df0639f](https://github.com/Trouv/bevy_pipe_affect/commit/df0639f7eb77f26509b49d6adf178774ab1948df))
* add EffectOut::and_extend method ([#68](https://github.com/Trouv/bevy_pipe_affect/issues/68)) ([243d118](https://github.com/Trouv/bevy_pipe_affect/commit/243d1181bbc42df4d631dbe5c711568b3b4f7022))
* add EffectOut::from_out ([#89](https://github.com/Trouv/bevy_pipe_affect/issues/89)) ([95671e0](https://github.com/Trouv/bevy_pipe_affect/commit/95671e06a3efc32f21aa2b4c21d36af9a584ab07))
* add EffectOut::map_effect ([#88](https://github.com/Trouv/bevy_pipe_affect/issues/88)) ([800515f](https://github.com/Trouv/bevy_pipe_affect/commit/800515f49872f281beaf86ec54cfb5f8208ff746))
* add extend effect composition function ([#66](https://github.com/Trouv/bevy_pipe_affect/issues/66)) ([c61e4ee](https://github.com/Trouv/bevy_pipe_affect/commit/c61e4ee41054c3ff1f7ee966a57fd100d9225181))
* add in_and_extend system combinator ([#67](https://github.com/Trouv/bevy_pipe_affect/issues/67)) ([6dbf5f1](https://github.com/Trouv/bevy_pipe_affect/commit/6dbf5f14d8bf29b73c785e8d287d2538e98ad916))
* add in_and_extend to prelude ([#72](https://github.com/Trouv/bevy_pipe_affect/issues/72)) ([2200014](https://github.com/Trouv/bevy_pipe_affect/commit/22000141955a61680eab9c21f814507936ff62c6))
* add mapping methods for AffectOrHandle ([#31](https://github.com/Trouv/bevy_pipe_affect/issues/31)) ([0c89d0f](https://github.com/Trouv/bevy_pipe_affect/commit/0c89d0f243843cf8e73691e56c830788bae1de08))
* add module functions for constructing all effects ([#30](https://github.com/Trouv/bevy_pipe_affect/issues/30)) ([869b70f](https://github.com/Trouv/bevy_pipe_affect/commit/869b70fbeb1c4823b0ba7a372ed4119613977895))
* add prelude module for common imports ([fe1c06e](https://github.com/Trouv/bevy_pipe_affect/commit/fe1c06ebbe74f0e235bbb6111b4ce757f08d082f))
* add pure system combinator opt-in compile-time read-only system checks ([#32](https://github.com/Trouv/bevy_pipe_affect/issues/32)) ([1d9fe6a](https://github.com/Trouv/bevy_pipe_affect/commit/1d9fe6a281c222d98cb9251122402d57a592ce1a))
* **ci:** add mdbook tests to CI ([#54](https://github.com/Trouv/bevy_pipe_affect/issues/54)) ([a9cfb06](https://github.com/Trouv/bevy_pipe_affect/commit/a9cfb065d0f5aa81d293e2a73c8c1d3cb667b4d0))
* **ci:** publish book in CI ([#56](https://github.com/Trouv/bevy_pipe_affect/issues/56)) ([113af44](https://github.com/Trouv/bevy_pipe_affect/commit/113af4444c9e67bddae520451e5d1028281ed3ee))
* convert crate to lib and move sample code to example ([725ca3c](https://github.com/Trouv/bevy_pipe_affect/commit/725ca3cbeaf28801eabe484c0d1371be1034f2cd))
* define packages with more detail ([#85](https://github.com/Trouv/bevy_pipe_affect/issues/85)) ([b658b1e](https://github.com/Trouv/bevy_pipe_affect/commit/b658b1e42eb2f442703c0d6be593e2b7c360efc0))
* derive common traits for `ResPut` ([#3](https://github.com/Trouv/bevy_pipe_affect/issues/3)) ([4b23951](https://github.com/Trouv/bevy_pipe_affect/commit/4b239513e43d107c2a000a6b860945afe8d649d4))
* derive common traits for components effects ([#10](https://github.com/Trouv/bevy_pipe_affect/issues/10)) ([9aa3d69](https://github.com/Trouv/bevy_pipe_affect/commit/9aa3d699883cd8fac8f61f0c01e7d1f9ead51b28))
* derive common traits for EffectOut ([#42](https://github.com/Trouv/bevy_pipe_affect/issues/42)) ([5dea2ac](https://github.com/Trouv/bevy_pipe_affect/commit/5dea2ac0b1bd3f9dc7a1938c7ce81d6e4074009e))
* impl Effect for tuples ([e37e15d](https://github.com/Trouv/bevy_pipe_affect/commit/e37e15d7f730c1aabd41d5a51f600c94d23234f5))
* implement `Effect` for `Either` ([#23](https://github.com/Trouv/bevy_pipe_affect/issues/23)) ([4c03d35](https://github.com/Trouv/bevy_pipe_affect/commit/4c03d3583c58dbdc1fe7a6a57a4dd5f4b8005b31))
* implement `Effect` for unit type `()` ([#19](https://github.com/Trouv/bevy_pipe_affect/issues/19)) ([40c12ce](https://github.com/Trouv/bevy_pipe_affect/commit/40c12ce9eb0490954a118f6b3a83af7b0481fc87))
* implement EffectOut conversion from generic Effects ([2cbdee9](https://github.com/Trouv/bevy_pipe_affect/commit/2cbdee9739b9bec60eb91885a9d6cd04dfafac51))
* implement Extend for AffectMany ([#69](https://github.com/Trouv/bevy_pipe_affect/issues/69)) ([708313a](https://github.com/Trouv/bevy_pipe_affect/commit/708313aa0fc350c0cdf9b886e8e6a8fce6112276))
* implement FromIterator for EffectOut ([#65](https://github.com/Trouv/bevy_pipe_affect/issues/65)) ([7aa2904](https://github.com/Trouv/bevy_pipe_affect/commit/7aa2904492df517b524af1f8f224e8c1dfd88c5d))
* implement initial failed prototype of SideEffects ([033f0be](https://github.com/Trouv/bevy_pipe_affect/commit/033f0be44cbf8f338f3dd86222f12f3385a871f9))
* implement IntoIterator for AffectMany ([#46](https://github.com/Trouv/bevy_pipe_affect/issues/46)) ([0d8bb0a](https://github.com/Trouv/bevy_pipe_affect/commit/0d8bb0afe3abfab8009406a1bee11070d2a70ea5))
* improve naming and documentation of system combinators ([#44](https://github.com/Trouv/bevy_pipe_affect/issues/44)) ([7aa73d1](https://github.com/Trouv/bevy_pipe_affect/commit/7aa73d1549b238f4f79f7018bddede4136e2272f))
* license repository with either mit apache according to rust ecosystem standard ([7c2bb6d](https://github.com/Trouv/bevy_pipe_affect/commit/7c2bb6dd60db28d01b00078aa6146a4e04c0b148))
* make Effect derivable ([#37](https://github.com/Trouv/bevy_pipe_affect/issues/37)) ([ea8d1a8](https://github.com/Trouv/bevy_pipe_affect/commit/ea8d1a802cc6d0389a789b47a2f959f45d742ab3))
* make Effect::affect mutably borrow the system param ([e654606](https://github.com/Trouv/bevy_pipe_affect/commit/e65460658afddd08949e7eaf59ae0a8290b4162f))
* minimize bevy dependencies ([#55](https://github.com/Trouv/bevy_pipe_affect/issues/55)) ([6ea787f](https://github.com/Trouv/bevy_pipe_affect/commit/6ea787f10ea041f24f63096eb1e2d97d3a060c42))
* parameterize system combinators with Into&lt;EffectOut&gt; piping ([39c0fce](https://github.com/Trouv/bevy_pipe_affect/commit/39c0fce3aec7dd894d6d7dcdfb56b6c8aa031f36))
* prototype effect systems with piping ([5a83ea6](https://github.com/Trouv/bevy_pipe_affect/commit/5a83ea603c0599e1a6d29a7e85db159266090e97))
* rename crate to bevy_pipe_affect ([bdc434c](https://github.com/Trouv/bevy_pipe_affect/commit/bdc434cad8549dd2b95aefdddc097a319b65cb4f))
* reuse Result effect for EntityComponentsSet- error handling ([#82](https://github.com/Trouv/bevy_pipe_affect/issues/82)) ([cff44f9](https://github.com/Trouv/bevy_pipe_affect/commit/cff44f966911fa66bf62b37ca731e0f1c53d12c5))
* upgrade dependencies, including bevy 0.16 ([#12](https://github.com/Trouv/bevy_pipe_affect/issues/12)) ([4a034ec](https://github.com/Trouv/bevy_pipe_affect/commit/4a034ec183d8cdd4ffba1a2d37d630dad8ddbfa1))
* upgrade to bevy 0.17 ([#29](https://github.com/Trouv/bevy_pipe_affect/issues/29)) ([f71a384](https://github.com/Trouv/bevy_pipe_affect/commit/f71a384ed46e220c0f268db9583badaf58155423))
* upgrade to rust edition 2024 ([#38](https://github.com/Trouv/bevy_pipe_affect/issues/38)) ([6c5b32d](https://github.com/Trouv/bevy_pipe_affect/commit/6c5b32d1f324c0ec93afbd0f53b829d8d439e722))
* use no bevy features by default ([#83](https://github.com/Trouv/bevy_pipe_affect/issues/83)) ([00e8b0a](https://github.com/Trouv/bevy_pipe_affect/commit/00e8b0a88eb53d573794578eb8d435768ad2f7c4))


### Bug Fixes

* add bevy compatibility chart to README.md ([#81](https://github.com/Trouv/bevy_pipe_affect/issues/81)) ([e909ce6](https://github.com/Trouv/bevy_pipe_affect/commit/e909ce65fa614d05f9ad1cd5073a2db587204d63))
* **book:** add dummy README.md ([#58](https://github.com/Trouv/bevy_pipe_affect/issues/58)) ([18c99b6](https://github.com/Trouv/bevy_pipe_affect/commit/18c99b6e0170703b0989b9469128fe191160714b))
* **ci:** give publish-book workflow write permissions ([#57](https://github.com/Trouv/bevy_pipe_affect/issues/57)) ([bcb5e17](https://github.com/Trouv/bevy_pipe_affect/commit/bcb5e1797f74d12a5435e0d3292dd0e90f613325))
* copy rainbow_clear_color example into README.md ([#92](https://github.com/Trouv/bevy_pipe_affect/issues/92)) ([cbeb18d](https://github.com/Trouv/bevy_pipe_affect/commit/cbeb18d05ec7dcfa76db4cf9fbd16065840928e2))
* describe documentation and add links in api reference ([#79](https://github.com/Trouv/bevy_pipe_affect/issues/79)) ([aba2aea](https://github.com/Trouv/bevy_pipe_affect/commit/aba2aeaf2ddef44281c1dc1d01d313bec795ee39))
* document feature flags ([#90](https://github.com/Trouv/bevy_pipe_affect/issues/90)) ([3f0aba7](https://github.com/Trouv/bevy_pipe_affect/commit/3f0aba7fb8eba7356ef4b6aa989c1a41e69b2ade))
* feature-flag derive tests ([#40](https://github.com/Trouv/bevy_pipe_affect/issues/40)) ([8f9209c](https://github.com/Trouv/bevy_pipe_affect/commit/8f9209c3e48e6235850e83e847f5e73b8f1c6569))
* move release-please config to config file ([#52](https://github.com/Trouv/bevy_pipe_affect/issues/52)) ([7de56fc](https://github.com/Trouv/bevy_pipe_affect/commit/7de56fc86de62c6cc2df41e6aca44b35c1d6235c))
* move release-please configuration from job to config ([0591eb3](https://github.com/Trouv/bevy_pipe_affect/commit/0591eb3928c118a877d45a1ba617f45df17c6bbd))
* note that variadics_please and either are already included by bevy ([#84](https://github.com/Trouv/bevy_pipe_affect/issues/84)) ([85d6181](https://github.com/Trouv/bevy_pipe_affect/commit/85d61818ae6c203545cd303e1da584ed158c4baf))
* prepare README.md for release ([#93](https://github.com/Trouv/bevy_pipe_affect/issues/93)) ([761ac26](https://github.com/Trouv/bevy_pipe_affect/commit/761ac26f87f76f33e65959798e2b04e5e1502348))
* test no-default-features in CI test job ([#41](https://github.com/Trouv/bevy_pipe_affect/issues/41)) ([f27da2f](https://github.com/Trouv/bevy_pipe_affect/commit/f27da2f402831e1113cdb03f9778a6e77790f995))


### Book Changes

* add chapter that redirects to api reference ([#78](https://github.com/Trouv/bevy_pipe_affect/issues/78)) ([043ddcd](https://github.com/Trouv/bevy_pipe_affect/commit/043ddcd3e07dd2669d6ccca4928bede997397d96))
* complete introduction ([#77](https://github.com/Trouv/bevy_pipe_affect/issues/77)) ([03f8835](https://github.com/Trouv/bevy_pipe_affect/commit/03f8835d8abafa10864695887fe816f6c07bfae5))
* describe documentation and add links in README.md ([#80](https://github.com/Trouv/bevy_pipe_affect/issues/80)) ([c2371aa](https://github.com/Trouv/bevy_pipe_affect/commit/c2371aaba037f9379e9812e56ecf96c279310b86))
* share blurb between book and api ref ([#76](https://github.com/Trouv/bevy_pipe_affect/issues/76)) ([e9ddec4](https://github.com/Trouv/bevy_pipe_affect/commit/e9ddec45a09665ff01df7d4d355eafedc081cdbc))
* write 'Output and Effect Composition' chapter ([#74](https://github.com/Trouv/bevy_pipe_affect/issues/74)) ([e330185](https://github.com/Trouv/bevy_pipe_affect/commit/e330185b2bc8ace5da5ae534385550a944cc50e5))
* write 'Spawn a Relationship Synchronously' chapter ([#64](https://github.com/Trouv/bevy_pipe_affect/issues/64)) ([ee1f8ab](https://github.com/Trouv/bevy_pipe_affect/commit/ee1f8ab736d12592c792d00c10ddbe8a15323367))
* write 'Spawn and Trigger an Observer' chapter ([#87](https://github.com/Trouv/bevy_pipe_affect/issues/87)) ([48dc336](https://github.com/Trouv/bevy_pipe_affect/commit/48dc336b1c6be81fc12dea45bd3a41f13c052177))
* write motivations chapter ([#61](https://github.com/Trouv/bevy_pipe_affect/issues/61)) ([29d9d8e](https://github.com/Trouv/bevy_pipe_affect/commit/29d9d8ef2e31eda30939065b0d75f61f99fecb94))


### Example Changes

* add observer example ([#86](https://github.com/Trouv/bevy_pipe_affect/issues/86)) ([c4b8862](https://github.com/Trouv/bevy_pipe_affect/commit/c4b886204064d8629ed878e0249b8f1367748407))
* add relationship example ([#62](https://github.com/Trouv/bevy_pipe_affect/issues/62)) ([abfe3af](https://github.com/Trouv/bevy_pipe_affect/commit/abfe3af13665d9d288ab0e90481e488fc5e28e42))
* add sokoban example ([#71](https://github.com/Trouv/bevy_pipe_affect/issues/71)) ([5bb73b6](https://github.com/Trouv/bevy_pipe_affect/commit/5bb73b6a879f67c3768bdd047b54ce1235318b60))
* require bevy default features for examples ([#63](https://github.com/Trouv/bevy_pipe_affect/issues/63)) ([27c4e91](https://github.com/Trouv/bevy_pipe_affect/commit/27c4e9196c13245d3176ccaddc95f9458bb6d35b))


### CI Changes

* add basic CI for test/clippy/fmt based off bevy template ([e041d28](https://github.com/Trouv/bevy_pipe_affect/commit/e041d288d65a83e575c8ff5b5724942b699df808))
* add basic release-please workflow ([#49](https://github.com/Trouv/bevy_pipe_affect/issues/49)) ([b8f88e1](https://github.com/Trouv/bevy_pipe_affect/commit/b8f88e1e04c5596e6c294979e2250c3c9d2d9f69))
* add more changelog-sections for representing non-src changes in changelog ([#59](https://github.com/Trouv/bevy_pipe_affect/issues/59)) ([98f5719](https://github.com/Trouv/bevy_pipe_affect/commit/98f5719a5ae03423b3c0a2f47baace2054d47fc2))
* add rustfmt.toml to enforce some custom formatting rules in eventual CI ([5e12c38](https://github.com/Trouv/bevy_pipe_affect/commit/5e12c386e1af95083c502aa7261c8f6a6ed05e36))
* add test section to release-please-config ([#60](https://github.com/Trouv/bevy_pipe_affect/issues/60)) ([2b1e0c3](https://github.com/Trouv/bevy_pipe_affect/commit/2b1e0c3f997bf8c225da02a90bcfddfcb4529559))
* deny cargo doc warnings via new doc_check CI job ([#1](https://github.com/Trouv/bevy_pipe_affect/issues/1)) ([ffeb0b7](https://github.com/Trouv/bevy_pipe_affect/commit/ffeb0b72942d9b969fbdb0e8ffdd84b2929b805d))
* enable all-features in doc_check job ([#91](https://github.com/Trouv/bevy_pipe_affect/issues/91)) ([b379d25](https://github.com/Trouv/bevy_pipe_affect/commit/b379d255999f0c9bccbe8f7a47031c5bd96cd65e))
* format code in doc comments ([8b314d0](https://github.com/Trouv/bevy_pipe_affect/commit/8b314d06f5b381b2d7f358b3a45c7adf30c70b49))
* install bevy system dependencies for book jobs ([#75](https://github.com/Trouv/bevy_pipe_affect/issues/75)) ([2522ea9](https://github.com/Trouv/bevy_pipe_affect/commit/2522ea9319c5499fbedd6e530cb8fdeb10be8c3c))
* invert and correct error check in doc_check CI job ([ccb64c6](https://github.com/Trouv/bevy_pipe_affect/commit/ccb64c6eeb00ed19bf4ef1ad8f19a50f26a38f23))
* just deny rustdoc lints instead of attempting bash solution ([849b0a0](https://github.com/Trouv/bevy_pipe_affect/commit/849b0a05acb9b87abf89550b6cfa410b82c25658))
* separate cargo doc and warning check into separate steps in doc_check job ([241c10a](https://github.com/Trouv/bevy_pipe_affect/commit/241c10a0cbc870d37a516b011d36b90c2567fb06))
* upgrade mdbook to 0.4.52 and mdbook-keeper to 0.5.0 ([#73](https://github.com/Trouv/bevy_pipe_affect/issues/73)) ([29a1d8c](https://github.com/Trouv/bevy_pipe_affect/commit/29a1d8c0661053215e0e7aeea96f1aad30855349))


### Test Changes

* add basic proptest for ResPut effect ([0d9f6f8](https://github.com/Trouv/bevy_pipe_affect/commit/0d9f6f8861d532c7ce596ca697c410aff7a316ef))
* add proptest dependency ([b48eda1](https://github.com/Trouv/bevy_pipe_affect/commit/b48eda14b46770afe9c0c004665b7342ef32e08d))
* create `OneWayFn` test utility for function generation with `proptest` ([#6](https://github.com/Trouv/bevy_pipe_affect/issues/6)) ([1d72444](https://github.com/Trouv/bevy_pipe_affect/commit/1d72444407ce5d5f6d25debaecae67f99dea33bf))
* extract common `ComponentsWith` test functions ([#9](https://github.com/Trouv/bevy_pipe_affect/issues/9)) ([6b0ebb5](https://github.com/Trouv/bevy_pipe_affect/commit/6b0ebb5034a962da64df0e019e6a6aaa144d65f9))
* move `Number`- test types to their own module ([#8](https://github.com/Trouv/bevy_pipe_affect/issues/8)) ([2a144aa](https://github.com/Trouv/bevy_pipe_affect/commit/2a144aa1947922eadbcfb0358c8d5ba2a2db70bf))
* use bigger number type for test `NumberResource` ([#5](https://github.com/Trouv/bevy_pipe_affect/issues/5)) ([f4b8868](https://github.com/Trouv/bevy_pipe_affect/commit/f4b8868cd4f52078b9ce7446df7cbd6b99a89143))
* use EffectOut and and_compose in main sample ([042ee8c](https://github.com/Trouv/bevy_pipe_affect/commit/042ee8c6141c3c823fd64f50acd41e7b377c5828))
