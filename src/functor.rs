use crate::base::{Generic1, Plug};

/// The Functor trait provides a structure preserving mapping operation `fmap`
/// that morphs any F<A> into an F<B> when provided with a function A -> B.
///
/// The Generic1 and Plug traits need to be implemented for the functor for this
/// to work properly.
pub trait Functor {
    fn fmap<B>(
        &self,
        f: fn(<Self as Generic1>::Inner) -> B
    ) -> <Self as Plug<B>>::Return
    where
        Self: Plug<B> + Generic1;
}


impl<A> Generic1 for Option<A> {
    type Inner = A;
}

impl<A: Clone, B> Plug<B> for Option<A> {
    type Return = Option<B>;
}

impl<A: Clone> Functor for Option<A> {
    fn fmap<B>(
        &self,
        f: fn(<Self as Generic1>::Inner) -> B
    ) -> <Self as Plug<B>>::Return {
        self.as_ref().map(|a| f(a.clone()))
    }
}


impl<A> Generic1 for Vec<A> {
    type Inner = A;
}

impl<A: Clone, B> Plug<B> for Vec<A> {
    type Return = Vec<B>;
}

impl<A: Clone> Functor for Vec<A> {
    fn fmap<B>(
        &self,
        f: fn(<Self as Generic1>::Inner) -> B
    ) -> <Self as Plug<B>>::Return {
        self.iter().map(|a| f(a.clone())).collect()
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
