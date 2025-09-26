mod archery_adapt;
mod associative;
pub mod traversable;
mod validation;
pub mod identity;


pub use archery_adapt::{
    ArcHKT, BoxHKT, HKT1Unsized, K1, RcHKT, RefHKT,
    SendHKT, SharedPointerHKT, SyncHKT, newtype,
};
pub use associative::Debug;
pub use validation::{Validation, ValidationHKT};

// Requirements:
// Convert A(unbound) => F/F<A>(bound) not dependant on A.
// Convert F to A if above constraint is true
// F<A> implement bound, delegate calls to A
// Keeping type information is required by default
// Generic Function Object Use cases:
// 1. Type Parameter must be unchanged for type binding AND constraints required: Not supported (A != F<A>), use macros or manually match enums for every generic call
// 2*. Type Parameter must be the same / Call associated method => Use this
// 3. Objects share the same interface & type is not important => Use enums/dyn
// 4*. Type Parameter must be unchanged for type binding => Use old trait
