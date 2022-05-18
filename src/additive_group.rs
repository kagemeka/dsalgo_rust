use crate::{abelian_group::AbelianGroup, group_theory_id::Additive};

pub trait AdditiveGroup<S>: AbelianGroup<S, Additive> {}

impl<S, T> AdditiveGroup<S> for T where T: AbelianGroup<S, Additive> {}

use crate::abelian_group::AbelianGroup2;

pub trait AdditiveGroup2: AbelianGroup2<Additive> {}

impl<T> AdditiveGroup2 for T where T: AbelianGroup2<Additive> {}
