pub trait Functor<A, B, F> where
    F: Fn(&A) -> B {
    type Return;
    fn fmap(&self, f: F) -> Self::Return;
}

impl<A, B, F> Functor<A, B, F> for Option<A> where
    F: Fn(&A) -> B {
    type Return = Option<B>;
    fn fmap(&self, f: F) -> Self::Return {
        match self {
            Some(a) => Some(f(a)),
            None => None,
        }
    }
}

impl<A, B, F> Functor<A, B, F> for Vec<A> where
    F: Fn(&A) -> B {
    type Return = Vec<B>;
    fn fmap (&self, f: F) -> Self::Return {
        let mut ret = Vec::with_capacity(self.len());
        for item in self.iter() {
            ret.push(f(item));
        }
        ret
    }
}


#[cfg(test)]
mod tests {
    use super::Functor;

    #[test]
    fn functor_works_for_option() {
        let opt = Some(10);
        let ret = opt.fmap(|x| x * 2);

        assert_eq!(ret, Some(20));
    }

    #[test]
    fn functor_works_for_vec() {
        let vec = vec![1,2,3,4,5];
        let ret = vec.fmap(|x| x * 2);

        assert_eq!(ret, vec![2,4,6,8,10]);
    }
}
