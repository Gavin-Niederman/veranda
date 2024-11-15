# Veranda

A [`rand`](https://crates.io/crates/rand) RNG source for [vexide](https://vexide.dev) programs.

Veranda uses system metrics like brain uptime, program uptime, battery current and voltage, and, optionally, empty ADI ports.

> [NOTE]
> Veranda is not upheld to the same update schedule as vexide, as it is not an official vexide crate.

# Usage

You can choose between two RNG sources:
- `SystemRng`: Uses all supported sources of entropy excluding empty ADI ports.
- `AdiRng`: Uses all of the sources of entropy used by `SystemRng`, but can also take advantage of the noise on **empty** ADI ports.
