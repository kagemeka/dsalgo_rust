use crate::{abelian_group::AbelianGroup, group_theory_id::Additive};

pub trait AdditiveGroup {}

impl<T> AdditiveGroup for T where T: AbelianGroup<Additive> {}
