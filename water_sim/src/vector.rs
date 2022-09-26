/// Vector that is writable
pub trait Vector: Clone {
    const DIM: usize;
    fn to_le_bytes(&self) -> Vec<u8>;
    fn from_le_bytes(bytes: &[u8]) -> Self;
}
impl Vector for f32 {
    const DIM: usize = 1;

    fn to_le_bytes(&self) -> Vec<u8> {
        f32::to_le_bytes(*self).to_vec()
    }

    fn from_le_bytes(bytes: &[u8]) -> Self {
        f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
}
