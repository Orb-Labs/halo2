# halo2 [![Crates.io](https://img.shields.io/crates/v/halo2.svg)](https://crates.io/crates/halo2) #
[![Hercules-ci][Herc badge]][Herc link]
[![Cachix Cache][Cachix badge]][Cachix link]

[Herc badge]: https://img.shields.io/badge/ci--by--hercules-green.svg
[Herc link]: https://hercules-ci.com/github/Orbis-Tertius/halo2
[Cachix badge]: https://img.shields.io/badge/cachix-private_Orbis--Tertius-blue.svg
[Cachix link]: https://private-Orbis-Tertius.cachix.org

**IMPORTANT**: This library is in beta, and should not be used in production software.

## [Documentation](https://docs.rs/halo2)

## Minimum Supported Rust Version

Requires Rust **1.51** or higher.

Minimum supported Rust version can be changed in the future, but it will be done with a
minor version bump.

## Controlling parallelism

`halo2` currently uses [rayon](https://github.com/rayon-rs/rayon) for parallel computation.
The `RAYON_NUM_THREADS` environment variable can be used to set the number of threads.

## License

Copyright 2020-2021 The Electric Coin Company.

You may use this package under the Bootstrap Open Source Licence, version 1.0,
or at your option, any later version. See the file [`COPYING`](COPYING) for
more details, and [`LICENSE-BOSL`](LICENSE-BOSL) for the terms of the Bootstrap
Open Source Licence, version 1.0.

The purpose of the BOSL is to allow commercial improvements to the package
while ensuring that all improvements are open source. See
[here](https://electriccoin.co/blog/introducing-tgppl-a-radically-new-type-of-open-source-license/)
for why the BOSL exists.
