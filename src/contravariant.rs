pub trait Contravariant<A, B, F> where
    F: Fn(&B) -> A {
    type Return;
    fn contramap(&self, f: F) -> Self::Return;
}

// impl<A, B, F> Contravariant<A, B, F> for fn(A) -> bool where
//     F: Fn(&B) -> A {
//     type Return = fn(B) -> bool;
//     fn contramap(&self, f: F) -> Self::Return {
//         move |x| self(f(&x))
//     }
// }


#[cfg(test)]
mod tests {
    use super::Contravariant;

    // #[test]
    // fn contravariant_works_for_predicate() {
    //     let predicate = |x| x < 20;
    //     let vec_to_int = |v| v[0];
    //     let vec = vec![10, 20, 30];
    //     let composed = predicate.contramap(vec_to_int);

    //     assert_eq!(composed(vec), true);
    // }
}
