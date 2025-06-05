#![allow(clippy::toplevel_ref_arg)]
pub mod configuration;
pub mod routes;
pub mod startup;
pub mod telemetry;

#[macro_export]
macro_rules! my_name_of {
    ($n: path) => {{ $stringify($n) }};
}
