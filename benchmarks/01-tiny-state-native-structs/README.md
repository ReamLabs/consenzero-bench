# 01-tiny-state-native-structs

## Main features

- Use `TinyBeaconState` i.e. reduced 2<sup>40</sup>-size list down to 2<sup>29</sup> to be able to deserialize and merkleize under 32-bit systems.
- The beacon state is deserialized from SSZ bytes before passing into the zkVM as Rust structs (less overhead on guest-deserialization, more overhead(?) than passing bytes by passing native structs)
