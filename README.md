# ACore

**Shared foundation for the A Launcher ecosystem.**

ACore is the small, dependency-light contract layer used by the launcher, JVM/runtime, native bridge, graphics, audio, and Android components.

## Current status

**Version:** 0.1.0  
**Language:** Rust  
**Stage:** Foundation / API bootstrap

### Included

- Core API versioning
- Platform and CPU architecture types
- Launch request/result contracts
- Shared error type
- Unit tests
- GitHub Actions CI
- Architecture documentation

### Deliberately not included

ACore does not implement the JVM, JNI, LWJGL, GLFW, graphics, audio, Minecraft metadata, mod loading, or Android UI. Those systems will live in their own repositories and depend on ACore only for shared contracts.

## Build

~~~bash
cargo test
cargo check
~~~

## Architecture

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).

## Ecosystem

ACore is intended to be the common foundation for the A Launcher repositories. Higher layers should depend on stable ACore contracts instead of importing implementation details from sibling repositories.
