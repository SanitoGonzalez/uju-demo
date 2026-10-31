# uju-demo

A game built on uju, laid out as one binary per role.

| Crate | |
|---|---|
| `crates/model` | Shared by every role: the protocol generated from `schema/`, and the components and keys more than one binary needs |
| `crates/sim` | Sim node: systems, inputs |
| `crates/hub` | Hub node: parties, guilds, presence |
| `crates/edge` | Edge node: client sessions, reads from replicas |
| `crates/entry` | Entry: login, character select |

Atlas has no game logic, so the demo runs `uju-atlas` from uju itself.

## Run

```sh
cargo run --release --bin uju-demo-sim
```

## Develop against a local uju

`uju` comes from its git repository, pinned in `Cargo.lock`, so a change to
uju reaches the demo only once it is pushed and `cargo update -p uju` runs.
To build against a local checkout instead, clone uju next to this repository
and add to `.cargo/config.toml`:

```toml
[patch."ssh://git@github.com/SanitoGonzalez/uju.git"]
uju = { path = "../uju/crates/uju" }
```

Don't commit it: it would point everyone else at a path they don't have.
While it is in place, Cargo also drops uju's git source from `Cargo.lock`,
so don't commit that change either. Removing the patch brings it back.

## Regenerate the protocol

From uju's workspace:

```sh
cargo run -p uju-protoc --bin ujuc -- protocol rust -o ../uju-demo/crates/model/src/proto ../uju-demo/schema
```
