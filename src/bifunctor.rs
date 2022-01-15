use crate::base::{Generic2, Plug2};

/// The Bifunctor trait is similar to Functor, but has two type arguments that
/// are both functorial. The `bimap` function takes two functions, A -> B and
/// C -> D and morphs any F<A, C> into an F<B, D>.
///
/// The Generic2 and Plug2 traits need to be implemented for this to work.
pub trait Bifunctor {
    fn bimap<B, D>(
        &self,
        f: fn(<Self as Generic2>::First) -> B,
        g: fn(<Self as Generic2>::Second) -> D
    ) -> <Self as Plug2<B, D>>::Return
    where
        Self: Plug2<B, D> + Generic2;
}


impl<A, C> Generic2 for Result<A, C> {
    type First = A;
    type Second = C;
}

impl<A: Clone, C: Clone, B, D> Plug2<B, D> for Result<A, C> {
    type Return = Result<B, D>;
}

impl<A: Clone, C: Clone> Bifunctor for Result<A, C> {
    fn bimap<B, D>(
        &self,
        f: fn(<Self as Generic2>::First) -> B,
        g: fn(<Self as Generic2>::Second) -> D
    ) -> <Self as Plug2<B, D>>::Return {
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
