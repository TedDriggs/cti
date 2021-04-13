cargo publish --manifest-path stix_derive/Cargo.toml &&
sleep 10 &&
cargo publish --manifest-path stix/Cargo.toml &&
sleep 10 &&
cargo publish --manifest-path attck/Cargo.toml;