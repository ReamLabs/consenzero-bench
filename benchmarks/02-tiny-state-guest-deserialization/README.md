# 02-tiny-state-guest-deserialization

## Main features

- Use `TinyBeaconState` i.e. reduced 2<sup>40</sup>-size list down to 2<sup>29</sup> to be able to deserialize and merkleize under 32-bit systems.
- Pass and deserialize SSZ bytes in the guest zkVM, instead of deserializing in the host and passing in native Rust structs.
