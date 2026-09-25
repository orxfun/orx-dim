use crate::dim::Dim;

pub enum IdxNever {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DNever {}

impl Dim for DNever {
    const D: usize = 0;

    type ChildDim = Self;

    type Idx = IdxNever;

    type ChildIdx = IdxNever;
}
