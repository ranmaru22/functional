/// The Contravariant trait implements contravariant functors, which are
/// functors from the opposite category, i.e. their morphisms are reversed. The
/// `contramap` function takes a function B -> A and a type F<A> and then
/// returns an F<B>.
pub trait Contravariant  {
    type Inner;
    type Output<B>;

    fn contramap<B>(&self, f: fn(B) -> Self::Inner) -> Self::Output<B>;
}
