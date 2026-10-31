//! What every role shares: the protocol generated from `schema/`, and the
//! components and keys more than one binary needs.
//!
//! Regenerate `proto` from the workspace root with
//! `ujuc protocol rust -o crates/model/src/proto schema`.

pub mod proto;
