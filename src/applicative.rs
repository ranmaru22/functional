use crate::functor::Functor;

/// Applicative functors are lax monoidal functors. Given a function inside a
/// functorial context F (A -> B) and an F<A>, it produces an F<B>.
pub trait Applicative: Functor {
    type Inner;
    type Wrapper<B>;

    fn pure<B>(b: B) -> Self::Output<B>;

    fn apply<B>(
        &self,
        f: Self::Wrapper<B>,
    ) -> Self::Output<B>;
}

impl<A: Copy> Applicative for Option<A> {
    type Inner = A;
    type Wrapper<B> = Option<fn(A) -> B>;

    fn pure<B>(b: B) -> Option<B> {
        Some(b)
    }

    fn apply<B>(&self, f: Option<fn(A) -> B>) -> Option<B> {
        match self {
            None => None,
            Some(a) => f.map(|f| f(*a)),
        }
    }
}

// This implementation is bad because you really never want a vector of function
// pointers. You also can't easily call them and you can't use this with closures
// either.
// It's just a proof of concept.
impl<A: Copy> Applicative for Vec<A> {
    type Inner = A;
    type Wrapper<B> = Vec<fn(A) -> B>;

    fn pure<B>(b: B) -> Vec<B> {
        vec![b]
    }

    fn apply<B>(&self, f: Vec<fn(A) -> B>) -> Vec<B> {
        let mut ret = vec![];

        for func in f {
            ret.extend(self.fmap(func))
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use std::convert::identity;
    use super::Applicative;
    use super::Functor;

    #[test]
    fn applicative_works_for_option() {
        let opt = Some(10);

        // Preserves identity
        let ret = opt.apply(Some(identity));
        assert_eq!(ret, identity(opt));

        // Homomorphism
        assert_eq!(
            opt.apply(Some(|x| x + 2)),
            opt.fmap(|x| x + 2)
        );

        // Preserves composition
        assert_eq!(
            opt.apply(Some(|x| x + 1)).apply(Some(|x| x * 2)),
            opt.apply(Some(|x| { |x| x + 1 }(x) * 2))
        );
    }

    #[test]
    fn applicative_works_for_vec() {
        let vec = vec![1,2,3];

        // Preserves identity
        let ret = vec.apply(vec![identity]);
        assert_eq!(ret, identity(vec.clone()));

        // Homomorphism
        assert_eq!(
            vec.apply(vec![|x| x + 2]),
            vec.fmap(|x| x + 2)
        );

        // Composition
        assert_eq!(
            vec.apply(vec![|x| x + 1]).apply(vec![|x| x * 2]),
            vec.apply(vec![|x| { |x| x + 1 }(x) * 2])
        );
    }
}
