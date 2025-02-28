//! # foundry-test-utils
//!
//! Internal Foundry testing utilities.

#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![allow(clippy::disallowed_macros)]

#[macro_use]
extern crate foundry_common;

#[macro_use]
extern crate tracing;

// Macros useful for testing.
mod macros;

pub mod rpc;

pub mod fd_lock;

mod filter;
pub use filter::Filter;

// Utilities for making it easier to handle tests.
pub mod util;
pub use util::{TestCommand, TestProject};

mod script;
pub use script::{ScriptOutcome, ScriptTester};

// TODO: remove once anvil supports zksync node
mod zksync;
pub use zksync::ZkSyncNode;

// re-exports for convenience
pub use foundry_compilers;

pub use snapbox::{self, assert_data_eq, file, str};

/// Initializes tracing for tests.
pub fn init_tracing() {
    let _ = tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init();
}
