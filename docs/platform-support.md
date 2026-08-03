# Platform Support

The library architecture is platform-neutral and `no_std` from day one.

| Platform | Initial evidence | Notes |
| --- | --- | --- |
| Linux | native CI build and tests | Primary development host |
| 32-bit Linux | `i686-unknown-linux-gnu` core-only build and test compilation | Exercises target-width rejection paths without foreign linking |
| WebAssembly | `wasm32-unknown-unknown` core-only build | No browser, WASI, or JavaScript dependency |
| Windows | native MSVC CI build | No POSIX assumptions in libraries |
| FreeBSD/BSD | cross-target core-only build | Expand to native BSD CI when maintained infrastructure is available |
| macOS | native CI build | Apple desktop target |
| Android | `aarch64-linux-android` core-only build | No Android SDK required for library check |
| iOS | `aarch64-apple-ios` core-only build on macOS | No UIKit assumptions |
| Aesynx | architecture review only | Not currently buildable; core-only/caller-buffer design preserves the path |

Codecs may not use filesystem, environment, network, clock, dynamic loading,
threads, or OS allocation APIs. `std` convenience adapters remain separate
features and do not alter parser validation.
