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
impl Vector for nalgebra::Vector2<f32> {
    const DIM: usize = 2;

    fn to_le_bytes(&self) -> Vec<u8> {
        [self.x, self.y]
            .iter()
            .flat_map(|f| f.to_le_bytes())
            .collect()
    }

    fn from_le_bytes(bytes: &[u8]) -> Self {
        let mut iter = (0..2)
            .map(|i| {
                [
                    bytes[i * 4],
                    bytes[i * 4 + 1],
                    bytes[i * 4 + 2],
                    bytes[i * 4 + 3],
                ]
            })
            .map(|b| f32::from_le_bytes(b));
        nalgebra::Vector2::new(iter.next().unwrap(), iter.next().unwrap())
    }
}
