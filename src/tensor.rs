use tinyvec::ArrayVec;

use crate::dtype::DType;

struct Tensor_ {
    dtype: DType,
    shape: ArrayVec<[usize; 4]>,
}
