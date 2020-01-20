
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct H256([u8;32]);

pub type Address = H256;
pub type StorageKey = H256;

impl From<[u8; 32]> for H256 {
    /// Constructs a hash type from the given bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(bytes: [u8; 32]) -> Self {
        H256(bytes)
    }
}

impl<'a> From<&'a [u8; 32]> for H256 {
    /// Constructs a hash type from the given reference
    /// to the bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(bytes: &'a [u8; 32]) -> Self {
        H256(*bytes)
    }
}

impl<'a> From<&'a mut [u8; 32]> for H256 {
    /// Constructs a hash type from the given reference
    /// to the mutable bytes array of fixed length.
    ///
    /// # Note
    ///
    /// The given bytes are interpreted in big endian order.
    #[inline]
    fn from(bytes: &'a mut [u8; 32]) -> Self {
        H256(*bytes)
    }
}

impl From<H256> for [u8; 32] {
    #[inline]
    fn from(s: H256) -> Self {
        s.0
    }
}

impl AsRef<[u8]> for H256 {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsMut<[u8]> for H256 {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_bytes_mut()
    }
}


impl H256 {
    /// Returns a new fixed hash where all bits are set to the given byte.
    #[inline]
    pub fn repeat_byte(byte: u8) -> H256 {
        H256([byte; 32])
    }

    /// Returns a new zero-initialized fixed hash.
    #[inline]
    pub fn zero() -> H256 {
        H256::repeat_byte(0u8)
    }

    /// Returns the size of this hash in bytes.
    #[inline]
    pub fn len_bytes() -> usize {
        32
    }

    /// Extracts a byte slice containing the entire fixed hash.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Extracts a mutable byte slice containing the entire fixed hash.
    #[inline]
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }

    /// Extracts a reference to the byte array containing the entire fixed hash.
    #[inline]
    pub fn as_fixed_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    #[inline]
    pub fn copy_to_array(&self, target: &mut [u8; 32]) {
        target.copy_from_slice(&self.0[..]);
    }
}