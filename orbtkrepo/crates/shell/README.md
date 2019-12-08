# orbtk-shell

Cross platform window shell library. It's part of [OrbTk](https://gitlab.redox-os.org/redox-os/orbtk) - The Rust UI-Toolkit.

[![Build and test](https://github.com/redox-os/orbtk/workflows/build/badge.svg)](https://github.com/redox-os/orbtk/actions)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](../../LICENSE)

## Platforms

* Redox OS
* Linux
* macOS
* Windows
* openBSD (not tested, but should work)
* Web
* Android (planned)
* iOS (planned)
* Ubuntu Touch (planned)

## Dependencies

* [stdweb](https://github.com/koute/stdweb) (Apache 2.0, MIT): web window and events
* [minifb](https://github.com/emoon/rust_minifb) (Apache 2.0, MIT): window and events for desktop platforms
* [image](https://github.com/image-rs/image)(MIT): load pixel images e.g. png
* [spin_sleep](https://github.com/alexheretic/spin-sleep)(Apache 2.0): Frame rate limiter and fps counter

## License

Licensed under MIT license ([LICENSE](../../LICENSE)).