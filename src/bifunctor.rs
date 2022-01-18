/// The Bifunctor trait is similar to Functor, but has two type arguments that
/// are both functorial. The `bimap` function takes two functions, A -> B and
/// C -> D and morphs any F<A, C> into an F<B, D>.
pub trait Bifunctor {
    type First;
    type Second;
    type Output<B, D>;

    fn bimap<B, D>(
        &self,
        f: fn(Self::First) -> B,
        g: fn(Self::Second) -> D
    ) -> Self::Output<B, D>;
}

impl<A: Clone, C: Clone> Bifunctor for (A, C) {
    type First = A;
    type Second = C;
    type Output<B, D> = (B, D);

    fn bimap<B, D>(
        &self,
        f: fn(A) -> B,
        g: fn(C) -> D
    ) -> (B, D) {
        (f(self.0.clone()), g(self.1.clone()))
    }
}

impl<A: Clone, C: Clone> Bifunctor for Result<A, C> {
    type First = A;
    type Second = C;
    type Output<B, D> = Result<B, D>;

    fn bimap<B, D>(
        &self,
        f: fn(A) -> B,
        g: fn(C) -> D
    ) -> Result<B, D> {
        match self {
            Ok(a) => Ok(f(a.clone())),
            Err(c) => Err(g(c.clone()))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::identity;
    use super::Bifunctor;

    #[test]
    fn bifunctor_works_for_pair() {
        let pair = (10, 10);

        // Preserves identity
        let ret = pair.bimap(identity, identity);
        assert_eq!(ret, identity(pair));

        // Preserves composition
        assert_eq!(
            pair.bimap(|x| { |x| x + 1 }(x) * 10, identity),
            pair.bimap(|x| x + 1, identity).bimap(|x| x * 10, identity),
        );
    }

    #[test]
    fn bifunctor_works_for_result() {
        let left: Result<_, ()> = Ok(10);
        let right: Result<(), _> = Err(10);

        // Preserves identity
        let l_ret = left.bimap(identity, identity);
        let r_ret = right.bimap(identity, identity);
        assert_eq!(l_ret, identity(left));
        assert_eq!(r_ret, identity(right));

        // Preserves composition
        assert_eq!(
            left.bimap(|x| { |x| x + 1 }(x) * 10, identity),
            left.bimap(|x| x + 1, identity).bimap(|x| x * 10, identity),
        );
        assert_eq!(
            right.bimap(identity, |x| { |x| x + 1 }(x) * 10),
            right.bimap(identity, |x| x + 1).bimap(identity, |x| x * 10),
        );
    }
}
