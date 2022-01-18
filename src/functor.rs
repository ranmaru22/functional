/// The Functor trait provides a structure preserving mapping operation `fmap`
/// that morphs any F<A> into an F<B> when provided with a function A -> B.
///
/// Any A should implement the Clone trait so that the return value is a new
/// object and the origial remains untouched.
pub trait Functor {
    type Inner;
    type Output<B>;

    fn fmap<B>(&self, f: fn(Self::Inner) -> B) -> Self::Output<B>;
}

impl<A: Clone> Functor for Option<A> {
    type Inner = A;
    type Output<B> = Option<B>;

    fn fmap<B>(&self, f: fn(A) -> B) -> Option<B> {
        self.clone().map(f)
    }
}

impl<A: Clone> Functor for Vec<A> {
    type Inner = A;
    type Output<B> = Vec<B>;

    fn fmap<B>(&self, f: fn(A) -> B) -> Vec<B> {
        self.iter().cloned().map(f).collect::<Vec<_>>()
    }
}


#[cfg(test)]
mod tests {
    use std::convert::identity;
    use super::Functor;

    #[test]
    fn functor_works_for_option() {
        let opt = Some(10);

        // Preserves identity
        let ret = opt.fmap(identity);
        assert_eq!(ret, identity(opt));

        // Preserves composition, excuse the ugly syntax but Rust doesn't have
        // a proper compose operator.
        assert_eq!(
            opt.fmap(|x| x + 1).fmap(|x| x * 10),
            opt.fmap(|x| { |x| x + 1 }(x) * 10)
        );
    }

    #[test]
    fn functor_works_for_vec() {
        let vec = vec![1, 2, 3];

        // Preserves identity
        let ret = vec.fmap(identity);
        // Need to clone here because Vec does not implement Copy.
        assert_eq!(ret, identity(vec.clone()));

        // Preserves composition
        assert_eq!(
            vec.fmap(|x| x + 1).fmap(|x| x * 10),
            vec.fmap(|x| { |x| x + 1 }(x) * 10)
        );
    }
}
