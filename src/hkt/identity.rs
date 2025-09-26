pub struct Id;

impl naan::HKT1 for Id {
    type T<A> = A;
}
