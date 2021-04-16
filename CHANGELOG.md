## v0.3.0 (2021-04-16)

- Allow `Ref` objects to outlive the `Node` method that created them
- Make `Ref` always implement `Clone`
- Add `impl Display for Ref`
- Add `mitre_id()` methods to `attck` objects
- Add `CollectionBuilder::insert`
- Add `impl Extend<impl Into<Declaration>> for CollectionBuilder`
- Add `From<Bundle<Declaration>> for CollectionBuilder`
- Improve doc comments for generated structs

## v0.2.0 (2021-04-15)

- Add open-vocabulary support
- Add `stix::custom_properties` macro to support [Custom Properties](https://docs.oasis-open.org/cti/stix/v2.1/cs01/stix-v2.1-cs01.html#_8072zpptza86)
