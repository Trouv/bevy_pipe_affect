{{#include blurb.md}}

## This book
This book aims to provide a place for the following pieces of documentation:
- tutorials: lessons detailing the creation of simple games from start to finish
- explanation: clarification of concepts and strategies employed by `bevy_pipe_affect`, including details about how it works and why
- how-to guides: recommended solutions to common problems, as well as migration guides

This book is not an API reference.
For that, please refer to `bevy_pipe_affect`'s documentation on [docs.rs](https://docs.rs/bevy_pipe_affect/0.1.0/bevy_pipe_affect). <!-- x-release-please-version -->

While this book aims to be comprehensive, it should also be easy to maintain and up-to-date.
This is why, in consort with the API reference, documentation for `bevy_pipe_affect` aims to satisfy [The Grand Unified Theory of Documentation](https://documentation.divio.com/).
Furthermore, code snippets in this book are automatically tested by `bevy_pipe_affect`'s CI wherever possible with the help of [mdBook-Keeper](https://github.com/tfpk/mdbook-keeper/).
This should help inform maintainers when changes to the library have made documentation out-of-date.
Deployment of this book to github pages is also performed by `bevy_pipe_affect`'s CI automatically on new releases.

Splitting the documentation up this way means that docs are not necessarily meant to be read in order.
Some chapters are intended to be read while working on your own project, while others are meant to be more like studying material.
The following are good jumping-off points for beginners:
- [*Motivations* explanation](explanation/motivations.md)
- [*effects* module api reference](https://docs.rs/bevy_pipe_affect/0.1.0/bevy_pipe_affect/effects/index.html) (a list of effects and constructors provided by the library) <!-- x-release-please-version -->

## Other resources
This book is not suitable documentation for Bevy.
Some resources for learning Bevy include those listed on the [Bevy website](https://bevyengine.org/learn), as well as the unofficial [Bevy Cheat Book](https://bevy-cheatbook.github.io/).

`bevy_pipe_affect`'s [source code](https://github.com/Trouv/bevy_pipe_affect/tree/v0.1.0) is available on github. <!-- x-release-please-version -->
This repository also contains [cargo examples](https://github.com/Trouv/bevy_pipe_affect/tree/v0.1.0/examples), which can be run after cloning the repository using `$ cargo run --example example-name --features bevy/default`. <!-- x-release-please-version -->
These examples may be difficult to follow on their own, and many of their strategies are described in this book.
When viewing these examples, be careful to checkout the correct git tag for the version of the library you are using.
Some changes may have been made to the library or to the examples on the `main` branch that are not released yet, and trying to apply these to the version of the library you are using can lead to errors.

## License
The pages of this book fall under the same license as the rest of the `bevy_pipe_affect` repository.
I.e., this book is dual-licensed under [MIT](http://opensource.org/licenses/MIT) and [Apache 2.0](http://www.apache.org/licenses/LICENSE-2.0) at your option.
