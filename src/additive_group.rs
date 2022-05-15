use crate::{abelian_group::AbelianGroup, group_theory_id::Additive};

pub trait AdditiveGroup<S>: AbelianGroup<S, Additive> {}
impl<S, T> AdditiveGroup<S> for T where T: AbelianGroup<S, Additive> {}
