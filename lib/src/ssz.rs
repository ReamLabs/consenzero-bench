use ssz::DecodeError;

pub fn from_ssz_bytes<T: ssz::Decode>(ssz_bytes: &[u8]) -> Result<T, DecodeError> {
    T::from_ssz_bytes(ssz_bytes)
}
