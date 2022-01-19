/// A profuctor is a bifunctor where the first argument is contravariant and the
/// second argument is covariant. So for a pair of functions B -> A and C -> D
/// it maps an F<A, C> into an F<B, D>.
pub trait Profunctor {
    type Left;
    type Right;
    type Output<B, D>;

    fn dimap<B, D>(
        &self,
        f: fn(B) -> Self::Left,
        g: fn(Self::Right) -> D
    ) -> Self::Output<B, D>;
}
