use crate::d1::D1;
use crate::dim::Dim;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Marker type for two-dimensional collections.
pub struct D2;

impl Dim for D2 {
    const D: usize = 2;

    type ChildDim = D1;

    type Idx = [usize; Self::D];

    type ChildIdx = usize;
}
