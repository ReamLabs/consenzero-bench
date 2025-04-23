use ssz::DecodeError;

pub fn to_ssz<T: ssz::Decode>(ssz_bytes: &[u8]) -> Result<T, DecodeError> {
    T::from_ssz_bytes(ssz_bytes)
}
