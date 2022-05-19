use crate::{monoid::Monoid, segment_tree::SegmentTree};

impl<M, Id> std::ops::Index<usize> for SegmentTree<M, Id>
where
    M: Monoid<Id>,
{
    type Output = M::S;

    fn index(&self, i: usize) -> &Self::Output {
        assert!(i < self.size());
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
        impl BinaryOperation<Additive> for Mon {
            type Codomain = usize;
            type Lhs = usize;
            type Rhs = usize;

            fn map(x: usize, y: usize) -> usize { x + y }
        }
        impl AssociativeProperty<Additive> for Mon {}
        impl IdentityElement<Additive> for Mon {
            type X = usize;

            fn identity() -> usize { 0 }
        }
        let mut seg = super::SegmentTree::<Mon, _>::new(10, || 0);
        seg.set(5, 10);
        assert_eq!(seg[5], 10);
    }
}
