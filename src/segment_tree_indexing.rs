use crate::{monoid::Monoid, segment_tree::SegmentTree};

impl<S, Id, M> std::ops::Index<usize> for SegmentTree<S, Id, M>
where
    M: Monoid<S, Id>,
{
    type Output = S;

    fn index(&self, i: usize) -> &Self::Output {
        assert!(i < self.size);
        &self.data[i + self.n()]
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_as_monoid() {
        // use crate::monoid::Monoid;
        use crate::{
            associative_property::AssociativeProperty,
            binary_operation::BinaryOperation,
            group_theory_id::Additive,
            identity_element::IdentityElement,
        };
        struct Mon;
        impl BinaryOperation<usize, usize, usize, Additive> for Mon {
            fn operate(x: usize, y: usize) -> usize { x + y }
        }
        impl AssociativeProperty<usize, Additive> for Mon {}
        impl IdentityElement<usize, Additive> for Mon {
            fn identity() -> usize { 0 }
        }
        let mut seg = super::SegmentTree::<_, _, Mon>::new(10, || 0);
        seg.set(5, 10);
        assert_eq!(seg[5], 10);
    }
}
