pub trait Bifunctor<A, B, C, D, F, G> where
    F: Fn(&A) -> B,
    G: Fn(&C) -> D {
    type Return;
    fn bimap(&self, f: F, g: G) -> Self::Return;
}

impl<A, B, C, D, F, G> Bifunctor<A, B, C, D, F, G> for (A, C) where
    F: Fn(&A) -> B,
    G: Fn(&C) -> D {
    type Return = (B, D);
    fn bimap(&self, f: F, g: G) -> Self::Return {
        (f(&self.0), g(&self.1))
    }
}

impl<A, B, C, D, F, G> Bifunctor<A, B, C, D, F, G> for Result<A, C> where
    F: Fn(&A) -> B,
    G: Fn(&C) -> D {
    type Return = Result<B, D>;
    fn bimap(&self, f: F, g: G) -> Self::Return {
        match self {
            Ok(a) => Ok(f(a)),
            Err(c) => Err(g(c)),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Bifunctor;

    #[test]
    fn bifunctor_works_for_pair() {
        let pair = (1, true);
        let ret = pair.bimap(|x| x * 2, |b| !b);

        assert_eq!(ret, (2, false));
    }

    #[test]
    fn bifunctor_works_for_result() {
        let left_int: Result<i32, bool> = Ok(10);
        let right_bool: Result<i32, bool> = Err(true);

        let ret_left = left_int.bimap(|x| x * 2, |b| !b);
        let ret_right = right_bool.bimap(|x| x * 2, |b| !b);

        assert_eq!(ret_left, Ok(20));
        assert_eq!(ret_right, Err(false));
    }
}
