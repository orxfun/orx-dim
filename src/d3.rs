use crate::d2::D2;
use crate::dim::Dim;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Marker type for three-dimensional collections.
pub struct D3;

impl Dim for D3 {
    const D: usize = 3;

    type ChildDim = D2;

    type Idx = [usize; Self::D];

    type ChildIdx = usize;
}
