# mt-kahypar-rs

Unofficial Rust bindings for [Mt-KaHyPar](https://github.com/kahypar/mt-kahypar),
the multi-level hypergraph partitioner. The build script compiles the bundled
`mt-kahypar-upstream` submodule into a static library.

This is a fork of [gzz2000/mt-kahypar-rs](https://github.com/gzz2000/mt-kahypar-rs)
by Zizheng Guo, modified by Verion GmbH for use in
[gem-harness](https://github.com/verion-eda/gem-harness). Neither Mt-KaHyPar nor
the original bindings are our work.

## License

The Rust bindings are licensed under either of [Apache License 2.0](LICENSE-APACHE)
or [MIT](LICENSE-MIT), at your option, matching the upstream `Cargo.toml`.

Mt-KaHyPar itself (`mt-kahypar-upstream/`) is MIT-licensed by its authors
(Copyright (C) 2022 Lars Gottesbüren, Tobias Heuer, Nikolai Maas), see
`mt-kahypar-upstream/LICENSE`. It is statically linked into anything built with
this crate, so its license notice must be shipped with such binaries.
