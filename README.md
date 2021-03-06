# kg-display

[![Build Status](https://travis-ci.org/Kodegenix/kg-display.svg?branch=master)](https://travis-ci.org/Kodegenix/kg-display)
[![codecov](https://codecov.io/gh/kodegenix/kg-display/branch/master/graph/badge.svg)](https://codecov.io/gh/kodegenix/kg-display)

Set of crates for ergonomic implementation of `std::fmt::Display` trait.

* crate [`kg-display`](kg-display) contains format string parser implemented in [nom](https://github.com/Geal/nom), as well 
as other utility types.
* crate [`kg-display-derive`](kg-display-derive) implements macro for `#[derive(Display)]`

## Builds statuses for Rust channels

| stable            | beta              | nightly           |
|-------------------|-------------------|-------------------|
| [![Build1][3]][4] | [![Build2][2]][4] | [![Build3][1]][4] |

[1]: https://travis-matrix-badges.herokuapp.com/repos/kodegenix/kg-display/branches/master/1
[2]: https://travis-matrix-badges.herokuapp.com/repos/kodegenix/kg-display/branches/master/2
[3]: https://travis-matrix-badges.herokuapp.com/repos/kodegenix/kg-display/branches/master/3
[4]: https://travis-ci.org/kodegenix/kg-display

## License

Licensed under either of
* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

## Copyright

Copyright (c) 2018 Kodegenix Sp. z o.o. [http://www.kodegenix.pl](http://www.kodegenix.pl)
