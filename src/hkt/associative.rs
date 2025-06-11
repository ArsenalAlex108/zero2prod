use crate::hkt::HKT1Unsized;

pub trait Debug: HKT1Unsized {
    fn fmt_k<A: ?Sized + std::fmt::Debug>(
        value: &Self::T<A>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result;
}
