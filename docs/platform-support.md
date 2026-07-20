# Platform Support

The library architecture is platform-neutral and `no_std` from day one.

| Platform | Initial evidence | Notes |
| --- | --- | --- |
| Linux | native CI build and tests | Primary development host |
| Windows | native MSVC CI build | No POSIX assumptions in libraries |
| FreeBSD/BSD | cross-target core-only build | Expand to native BSD CI when maintained infrastructure is available |
| macOS | native CI build | Apple desktop target |
| Android | `aarch64-linux-android` core-only build | No Android SDK required for library check |
| iOS | `aarch64-apple-ios` core-only build on macOS | No UIKit assumptions |
| Aesynx | architecture review only | Not currently buildable; core-only/caller-buffer design preserves the path |

Codecs may not use filesystem, environment, network, clock, dynamic loading,
threads, or OS allocation APIs. `std` convenience adapters remain separate
features and do not alter parser validation.
