#![allow(clippy::toplevel_ref_arg)]
#[warn(clippy::pedantic)]
#[warn(clippy::correctness)]
#[warn(clippy::suspicious)]
#[warn(clippy::perf)]
#[warn(clippy::complexity)]
#[warn(clippy::style)]
#[warn(clippy::cargo)]
#[allow(clippy::uninlined_format_args)]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::missing_errors_doc)]
pub mod authentication;
pub mod configuration;
pub mod domain;
pub mod email_client;
pub mod hkt;
pub mod routes;
pub mod startup;
pub mod telemetry;
#[macro_use]
pub mod utils;
pub mod idempotency;
pub mod issue_delivery_worker;
pub mod session_state;
