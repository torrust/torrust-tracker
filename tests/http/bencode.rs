pub type ByteArray20 = [u8; 20];

pub struct InfoHash(ByteArray20);

impl InfoHash {
    pub fn new(vec: &[u8]) -> Self {
        let mut byte_array_20: ByteArray20 = Default::default();
        byte_array_20.clone_from_slice(vec);
        Self(byte_array_20)
    }

    pub fn bytes(&self) -> ByteArray20 {
        self.0
    }
}
