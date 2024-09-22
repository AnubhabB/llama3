/// The different types of elements allowed in tensors.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DType {
    // Brain floating-point using half precision (16 bits).
    BF16,
    // Floating-point using half precision (16 bits).
    F16,
    // Floating-point using single precision (32 bits).
    F32,
    // Unsigned 32 bits unsigned integer - max 4294967295 which will happily encode our tokenizer with vocab size of 128,000
    U32,
}

impl DType {
    /// The size used by each element in bytes, i.e. 1 for `U8`, 4 for `F32`.
    pub fn size_bytes(&self) -> usize {
        match self {
            Self::BF16 => 2,
            Self::F16 => 2,
            Self::F32 => 4,
            Self::U32 => 4,
        }
    }
}
