//! Functional traits for Rust
//!
//! This crate is really just a pet project and not really meant to be used
//! in production. Be aware of that.
//!
//! Because Rust lacks higher-kinded types (HKTs) as of now, working with these
//! traits requires some arguably ugly hacks to make implementing them possible.

pub mod base;
pub mod functor;
pub mod bifunctor;
pub mod contravariant;
