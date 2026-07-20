# Crate Version Matrix

The facade is the integration train. Support crates publish only when their
code or published dependency requirements change.

| Crate | Previous | Planned | Change | Publish | Reason |
| --- | --- | --- | --- | --- | --- |
| `mynd-core` | `0.0.0` | `0.1.0` | code | Yes | Establish the documented no_std core package boundary. |
| `mynd` | `0.0.0` | `0.1.0` | code | Yes | Establish the facade and repository foundation. |

Change kinds follow the adapted `eth` release tool: facade code uses the
milestone version; support-crate code gets its next independent minor; bugfixes
increment patch; dependency-only changes increment patch; unchanged crates are
not published.
