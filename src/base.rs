/// Administrative trait used to simulate a HKT. Needs to be implemented for
/// all types that want to use the functional traits. Use it together with
/// the Plug trait.
///
/// Allows to take the inner type A out of an embellished type F<A>.
pub trait Generic1 {
    type Inner;
}

/// The same as Generic1, but for two inner types.
pub trait Generic2 {
    type First;
    type Second;
}

/// Administrative trait used to simulate a HKT. Needs to be implemented for
/// all types that want to use the functional traits. Use it together with
/// the Generic1 trait.
///
/// Allows to replace the inner type A of an embellished type F<A> with B.
///
/// When implementing Plug for F<A>, A should implement Clone because the functor
/// is supposed to return a copy without modifying the original or having any
/// relation to it. If it returned a reference, it would break this requirement.
pub trait Plug<B> {
    type Return;
}

/// The same as Plug, but for two inner types.
pub trait Plug2<B, D> {
    type Return;
}
