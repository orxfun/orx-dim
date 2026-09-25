use crate::d3::D3;
use crate::dim::Dim;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Marker type for four-dimensional collections.
pub struct D4;

impl Dim for D4 {
    const D: usize = 4;

    type ChildDim = D3;

    type Idx = [usize; Self::D];

    type ChildIdx = usize;
}
