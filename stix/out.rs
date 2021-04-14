#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
mod attack_pattern {
    use serde::Deserialize;
    use crate::CommonProperties;
    #[typed_object(core)]
    pub struct AttackPattern {
        #[serde(flatten)]
        base: CommonProperties,
        pub name: String,
        #[serde(default)]
        pub description: Option<String>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for AttackPattern {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field<'de> {
                    __field1,
                    __field2,
                    __other(_serde::__private::de::Content<'de>),
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field<'de>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_bool<__E>(
                        self,
                        __value: bool,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Bool(__value),
                        ))
                    }
                    fn visit_i8<__E>(
                        self,
                        __value: i8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(_serde::__private::de::Content::I8(
                            __value,
                        )))
                    }
                    fn visit_i16<__E>(
                        self,
                        __value: i16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I16(__value),
                        ))
                    }
                    fn visit_i32<__E>(
                        self,
                        __value: i32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I32(__value),
                        ))
                    }
                    fn visit_i64<__E>(
                        self,
                        __value: i64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I64(__value),
                        ))
                    }
                    fn visit_u8<__E>(
                        self,
                        __value: u8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(_serde::__private::de::Content::U8(
                            __value,
                        )))
                    }
                    fn visit_u16<__E>(
                        self,
                        __value: u16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U16(__value),
                        ))
                    }
                    fn visit_u32<__E>(
                        self,
                        __value: u32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U32(__value),
                        ))
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U64(__value),
                        ))
                    }
                    fn visit_f32<__E>(
                        self,
                        __value: f32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::F32(__value),
                        ))
                    }
                    fn visit_f64<__E>(
                        self,
                        __value: f64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::F64(__value),
                        ))
                    }
                    fn visit_char<__E>(
                        self,
                        __value: char,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Char(__value),
                        ))
                    }
                    fn visit_unit<__E>(self) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Unit,
                        ))
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field1),
                            "description" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = _serde::__private::de::Content::String(
                                    _serde::__private::ToString::to_string(__value),
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field1),
                            b"description" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value =
                                    _serde::__private::de::Content::ByteBuf(__value.to_vec());
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_str<__E>(
                        self,
                        __value: &'de str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field1),
                            "description" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = _serde::__private::de::Content::Str(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_bytes<__E>(
                        self,
                        __value: &'de [u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field1),
                            b"description" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = _serde::__private::de::Content::Bytes(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field<'de> {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<AttackPattern>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = AttackPattern;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct AttackPattern")
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Option<String>> =
                            _serde::__private::None;
                        let mut __collect = _serde::__private::Vec::<
                            _serde::__private::Option<(
                                _serde::__private::de::Content,
                                _serde::__private::de::Content,
                            )>,
                        >::new();
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "description",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__other(__name) => {
                                    __collect.push(_serde::__private::Some((
                                        __name,
                                        match _serde::de::MapAccess::next_value(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    )));
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field0: CommonProperties = match _serde::de::Deserialize::deserialize(
                            _serde::__private::de::FlatMapDeserializer(
                                &mut __collect,
                                _serde::__private::PhantomData,
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(AttackPattern {
                            base: __field0,
                            name: __field1,
                            description: __field2,
                        })
                    }
                }
                _serde::Deserializer::deserialize_map(
                    __deserializer,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<AttackPattern>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl crate::TypedObject for AttackPattern {
        const TYPE: &'static str = "attack-pattern";
    }
    impl AttackPattern {
        pub fn name(&self) -> &str {
            &self.name
        }
        pub fn description(&self) -> Option<&str> {
            self.description.as_ref().map(|s| s.as_str())
        }
    }
    impl AsRef<CommonProperties> for AttackPattern {
        fn as_ref(&self) -> &CommonProperties {
            &self.base
        }
    }
}
mod bundle {
    use serde::Deserialize;
    use crate::{Id, TypedObject};
    pub struct Bundle<T> {
        pub id: Id,
        pub spec_version: String,
        pub objects: Vec<T>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de, T> _serde::Deserialize<'de> for Bundle<T>
        where
            T: _serde::Deserialize<'de>,
        {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "id" => _serde::__private::Ok(__Field::__field0),
                            "spec_version" => _serde::__private::Ok(__Field::__field1),
                            "objects" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"id" => _serde::__private::Ok(__Field::__field0),
                            b"spec_version" => _serde::__private::Ok(__Field::__field1),
                            b"objects" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de, T>
                where
                    T: _serde::Deserialize<'de>,
                {
                    marker: _serde::__private::PhantomData<Bundle<T>>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de, T> _serde::de::Visitor<'de> for __Visitor<'de, T>
                where
                    T: _serde::Deserialize<'de>,
                {
                    type Value = Bundle<T>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct Bundle")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<Id>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Bundle with 3 elements",
                                        ),
                                    );
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Bundle with 3 elements",
                                        ),
                                    );
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<Vec<T>>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Bundle with 3 elements",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(Bundle {
                            id: __field0,
                            spec_version: __field1,
                            objects: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Id> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Vec<T>> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "id",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Id>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "spec_version",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "objects",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<T>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("id") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("spec_version") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("objects") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(Bundle {
                            id: __field0,
                            spec_version: __field1,
                            objects: __field2,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["id", "spec_version", "objects"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Bundle",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Bundle<T>>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl<T> TypedObject for Bundle<T> {
        const TYPE: &'static str = "bundle";
    }
}
mod campaign {
    use std::collections::BTreeSet;
    use chrono::{DateTime, Utc};
    use serde::Deserialize;
    use crate::CommonProperties;
    #[typed_object(core)]
    pub struct Campaign {
        #[serde(flatten)]
        common: CommonProperties,
        pub name: String,
        #[serde(default)]
        pub description: Option<String>,
        #[serde(default)]
        pub aliases: BTreeSet<String>,
        #[serde(default)]
        pub first_seen: Option<DateTime<Utc>>,
        #[serde(default)]
        pub last_seen: Option<DateTime<Utc>>,
        #[serde(default)]
        pub objective: Option<String>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Campaign {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field<'de> {
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __other(_serde::__private::de::Content<'de>),
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field<'de>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_bool<__E>(
                        self,
                        __value: bool,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Bool(__value),
                        ))
                    }
                    fn visit_i8<__E>(
                        self,
                        __value: i8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(_serde::__private::de::Content::I8(
                            __value,
                        )))
                    }
                    fn visit_i16<__E>(
                        self,
                        __value: i16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I16(__value),
                        ))
                    }
                    fn visit_i32<__E>(
                        self,
                        __value: i32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I32(__value),
                        ))
                    }
                    fn visit_i64<__E>(
                        self,
                        __value: i64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I64(__value),
                        ))
                    }
                    fn visit_u8<__E>(
                        self,
                        __value: u8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(_serde::__private::de::Content::U8(
                            __value,
                        )))
                    }
                    fn visit_u16<__E>(
                        self,
                        __value: u16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U16(__value),
                        ))
                    }
                    fn visit_u32<__E>(
                        self,
                        __value: u32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U32(__value),
                        ))
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U64(__value),
                        ))
                    }
                    fn visit_f32<__E>(
                        self,
                        __value: f32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::F32(__value),
                        ))
                    }
                    fn visit_f64<__E>(
                        self,
                        __value: f64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::F64(__value),
                        ))
                    }
                    fn visit_char<__E>(
                        self,
                        __value: char,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Char(__value),
                        ))
                    }
                    fn visit_unit<__E>(self) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Unit,
                        ))
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field1),
                            "description" => _serde::__private::Ok(__Field::__field2),
                            "aliases" => _serde::__private::Ok(__Field::__field3),
                            "first_seen" => _serde::__private::Ok(__Field::__field4),
                            "last_seen" => _serde::__private::Ok(__Field::__field5),
                            "objective" => _serde::__private::Ok(__Field::__field6),
                            _ => {
                                let __value = _serde::__private::de::Content::String(
                                    _serde::__private::ToString::to_string(__value),
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field1),
                            b"description" => _serde::__private::Ok(__Field::__field2),
                            b"aliases" => _serde::__private::Ok(__Field::__field3),
                            b"first_seen" => _serde::__private::Ok(__Field::__field4),
                            b"last_seen" => _serde::__private::Ok(__Field::__field5),
                            b"objective" => _serde::__private::Ok(__Field::__field6),
                            _ => {
                                let __value =
                                    _serde::__private::de::Content::ByteBuf(__value.to_vec());
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_str<__E>(
                        self,
                        __value: &'de str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field1),
                            "description" => _serde::__private::Ok(__Field::__field2),
                            "aliases" => _serde::__private::Ok(__Field::__field3),
                            "first_seen" => _serde::__private::Ok(__Field::__field4),
                            "last_seen" => _serde::__private::Ok(__Field::__field5),
                            "objective" => _serde::__private::Ok(__Field::__field6),
                            _ => {
                                let __value = _serde::__private::de::Content::Str(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_bytes<__E>(
                        self,
                        __value: &'de [u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field1),
                            b"description" => _serde::__private::Ok(__Field::__field2),
                            b"aliases" => _serde::__private::Ok(__Field::__field3),
                            b"first_seen" => _serde::__private::Ok(__Field::__field4),
                            b"last_seen" => _serde::__private::Ok(__Field::__field5),
                            b"objective" => _serde::__private::Ok(__Field::__field6),
                            _ => {
                                let __value = _serde::__private::de::Content::Bytes(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field<'de> {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Campaign>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Campaign;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct Campaign")
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Option<String>> =
                            _serde::__private::None;
                        let mut __field3: _serde::__private::Option<BTreeSet<String>> =
                            _serde::__private::None;
                        let mut __field4: _serde::__private::Option<Option<DateTime<Utc>>> =
                            _serde::__private::None;
                        let mut __field5: _serde::__private::Option<Option<DateTime<Utc>>> =
                            _serde::__private::None;
                        let mut __field6: _serde::__private::Option<Option<String>> =
                            _serde::__private::None;
                        let mut __collect = _serde::__private::Vec::<
                            _serde::__private::Option<(
                                _serde::__private::de::Content,
                                _serde::__private::de::Content,
                            )>,
                        >::new();
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "description",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "aliases",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<BTreeSet<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "first_seen",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<DateTime<Utc>>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "last_seen",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<DateTime<Utc>>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::__private::Option::is_some(&__field6) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "objective",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__other(__name) => {
                                    __collect.push(_serde::__private::Some((
                                        __name,
                                        match _serde::de::MapAccess::next_value(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    )));
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field6 = match __field6 {
                            _serde::__private::Some(__field6) => __field6,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field0: CommonProperties = match _serde::de::Deserialize::deserialize(
                            _serde::__private::de::FlatMapDeserializer(
                                &mut __collect,
                                _serde::__private::PhantomData,
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(Campaign {
                            common: __field0,
                            name: __field1,
                            description: __field2,
                            aliases: __field3,
                            first_seen: __field4,
                            last_seen: __field5,
                            objective: __field6,
                        })
                    }
                }
                _serde::Deserializer::deserialize_map(
                    __deserializer,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Campaign>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl crate::TypedObject for Campaign {
        const TYPE: &'static str = "campaign";
    }
    impl AsRef<CommonProperties> for Campaign {
        fn as_ref(&self) -> &CommonProperties {
            &self.common
        }
    }
}
mod course_of_action {
    use serde::Deserialize;
    use crate::CommonProperties;
    #[typed_object(core)]
    pub struct CourseOfAction {
        #[serde(flatten)]
        base: CommonProperties,
        pub name: String,
        #[serde(default)]
        pub description: Option<String>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for CourseOfAction {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field<'de> {
                    __field1,
                    __field2,
                    __other(_serde::__private::de::Content<'de>),
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field<'de>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_bool<__E>(
                        self,
                        __value: bool,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Bool(__value),
                        ))
                    }
                    fn visit_i8<__E>(
                        self,
                        __value: i8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(_serde::__private::de::Content::I8(
                            __value,
                        )))
                    }
                    fn visit_i16<__E>(
                        self,
                        __value: i16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I16(__value),
                        ))
                    }
                    fn visit_i32<__E>(
                        self,
                        __value: i32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I32(__value),
                        ))
                    }
                    fn visit_i64<__E>(
                        self,
                        __value: i64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I64(__value),
                        ))
                    }
                    fn visit_u8<__E>(
                        self,
                        __value: u8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(_serde::__private::de::Content::U8(
                            __value,
                        )))
                    }
                    fn visit_u16<__E>(
                        self,
                        __value: u16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U16(__value),
                        ))
                    }
                    fn visit_u32<__E>(
                        self,
                        __value: u32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U32(__value),
                        ))
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U64(__value),
                        ))
                    }
                    fn visit_f32<__E>(
                        self,
                        __value: f32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::F32(__value),
                        ))
                    }
                    fn visit_f64<__E>(
                        self,
                        __value: f64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::F64(__value),
                        ))
                    }
                    fn visit_char<__E>(
                        self,
                        __value: char,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Char(__value),
                        ))
                    }
                    fn visit_unit<__E>(self) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Unit,
                        ))
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field1),
                            "description" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = _serde::__private::de::Content::String(
                                    _serde::__private::ToString::to_string(__value),
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field1),
                            b"description" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value =
                                    _serde::__private::de::Content::ByteBuf(__value.to_vec());
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_str<__E>(
                        self,
                        __value: &'de str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field1),
                            "description" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = _serde::__private::de::Content::Str(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_bytes<__E>(
                        self,
                        __value: &'de [u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field1),
                            b"description" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = _serde::__private::de::Content::Bytes(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field<'de> {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<CourseOfAction>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = CourseOfAction;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct CourseOfAction",
                        )
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Option<String>> =
                            _serde::__private::None;
                        let mut __collect = _serde::__private::Vec::<
                            _serde::__private::Option<(
                                _serde::__private::de::Content,
                                _serde::__private::de::Content,
                            )>,
                        >::new();
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "description",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__other(__name) => {
                                    __collect.push(_serde::__private::Some((
                                        __name,
                                        match _serde::de::MapAccess::next_value(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    )));
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field0: CommonProperties = match _serde::de::Deserialize::deserialize(
                            _serde::__private::de::FlatMapDeserializer(
                                &mut __collect,
                                _serde::__private::PhantomData,
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(CourseOfAction {
                            base: __field0,
                            name: __field1,
                            description: __field2,
                        })
                    }
                }
                _serde::Deserializer::deserialize_map(
                    __deserializer,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<CourseOfAction>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl crate::TypedObject for CourseOfAction {
        const TYPE: &'static str = "course-of-action";
    }
    impl CourseOfAction {
        pub fn name(&self) -> &str {
            &self.name
        }
        pub fn description(&self) -> Option<&str> {
            self.description.as_ref().map(|s| s.as_str())
        }
    }
    impl AsRef<CommonProperties> for CourseOfAction {
        fn as_ref(&self) -> &CommonProperties {
            &self.base
        }
    }
}
mod id {
    use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
    use std::{borrow::Cow, fmt, str::FromStr};
    use thiserror::Error;
    use uuid::Uuid;
    use crate::TypedObject;
    pub struct Id {
        namespace: Cow<'static, str>,
        id: Uuid,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Id {
        #[inline]
        fn clone(&self) -> Id {
            match *self {
                Id {
                    namespace: ref __self_0_0,
                    id: ref __self_0_1,
                } => Id {
                    namespace: ::core::clone::Clone::clone(&(*__self_0_0)),
                    id: ::core::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for Id {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Id {
        #[inline]
        fn eq(&self, other: &Id) -> bool {
            match *other {
                Id {
                    namespace: ref __self_1_0,
                    id: ref __self_1_1,
                } => match *self {
                    Id {
                        namespace: ref __self_0_0,
                        id: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Id) -> bool {
            match *other {
                Id {
                    namespace: ref __self_1_0,
                    id: ref __self_1_1,
                } => match *self {
                    Id {
                        namespace: ref __self_0_0,
                        id: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    impl ::core::marker::StructuralEq for Id {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for Id {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<Cow<'static, str>>;
                let _: ::core::cmp::AssertParamIsEq<Uuid>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialOrd for Id {
        #[inline]
        fn partial_cmp(&self, other: &Id) -> ::core::option::Option<::core::cmp::Ordering> {
            match *other {
                Id {
                    namespace: ref __self_1_0,
                    id: ref __self_1_1,
                } => match *self {
                    Id {
                        namespace: ref __self_0_0,
                        id: ref __self_0_1,
                    } => match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0))
                    {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            match ::core::cmp::PartialOrd::partial_cmp(
                                &(*__self_0_1),
                                &(*__self_1_1),
                            ) {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                    ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                                }
                                cmp => cmp,
                            }
                        }
                        cmp => cmp,
                    },
                },
            }
        }
        #[inline]
        fn lt(&self, other: &Id) -> bool {
            match *other {
                Id {
                    namespace: ref __self_1_0,
                    id: ref __self_1_1,
                } => match *self {
                    Id {
                        namespace: ref __self_0_0,
                        id: ref __self_0_1,
                    } => {
                        ::core::cmp::Ordering::then_with(
                            ::core::option::Option::unwrap_or(
                                ::core::cmp::PartialOrd::partial_cmp(
                                    &(*__self_0_0),
                                    &(*__self_1_0),
                                ),
                                ::core::cmp::Ordering::Equal,
                            ),
                            || {
                                ::core::option::Option::unwrap_or(
                                    ::core::cmp::PartialOrd::partial_cmp(
                                        &(*__self_0_1),
                                        &(*__self_1_1),
                                    ),
                                    ::core::cmp::Ordering::Greater,
                                )
                            },
                        ) == ::core::cmp::Ordering::Less
                    }
                },
            }
        }
        #[inline]
        fn le(&self, other: &Id) -> bool {
            match *other {
                Id {
                    namespace: ref __self_1_0,
                    id: ref __self_1_1,
                } => match *self {
                    Id {
                        namespace: ref __self_0_0,
                        id: ref __self_0_1,
                    } => {
                        ::core::cmp::Ordering::then_with(
                            ::core::option::Option::unwrap_or(
                                ::core::cmp::PartialOrd::partial_cmp(
                                    &(*__self_0_0),
                                    &(*__self_1_0),
                                ),
                                ::core::cmp::Ordering::Equal,
                            ),
                            || {
                                ::core::option::Option::unwrap_or(
                                    ::core::cmp::PartialOrd::partial_cmp(
                                        &(*__self_0_1),
                                        &(*__self_1_1),
                                    ),
                                    ::core::cmp::Ordering::Greater,
                                )
                            },
                        ) != ::core::cmp::Ordering::Greater
                    }
                },
            }
        }
        #[inline]
        fn gt(&self, other: &Id) -> bool {
            match *other {
                Id {
                    namespace: ref __self_1_0,
                    id: ref __self_1_1,
                } => match *self {
                    Id {
                        namespace: ref __self_0_0,
                        id: ref __self_0_1,
                    } => {
                        ::core::cmp::Ordering::then_with(
                            ::core::option::Option::unwrap_or(
                                ::core::cmp::PartialOrd::partial_cmp(
                                    &(*__self_0_0),
                                    &(*__self_1_0),
                                ),
                                ::core::cmp::Ordering::Equal,
                            ),
                            || {
                                ::core::option::Option::unwrap_or(
                                    ::core::cmp::PartialOrd::partial_cmp(
                                        &(*__self_0_1),
                                        &(*__self_1_1),
                                    ),
                                    ::core::cmp::Ordering::Less,
                                )
                            },
                        ) == ::core::cmp::Ordering::Greater
                    }
                },
            }
        }
        #[inline]
        fn ge(&self, other: &Id) -> bool {
            match *other {
                Id {
                    namespace: ref __self_1_0,
                    id: ref __self_1_1,
                } => match *self {
                    Id {
                        namespace: ref __self_0_0,
                        id: ref __self_0_1,
                    } => {
                        ::core::cmp::Ordering::then_with(
                            ::core::option::Option::unwrap_or(
                                ::core::cmp::PartialOrd::partial_cmp(
                                    &(*__self_0_0),
                                    &(*__self_1_0),
                                ),
                                ::core::cmp::Ordering::Equal,
                            ),
                            || {
                                ::core::option::Option::unwrap_or(
                                    ::core::cmp::PartialOrd::partial_cmp(
                                        &(*__self_0_1),
                                        &(*__self_1_1),
                                    ),
                                    ::core::cmp::Ordering::Less,
                                )
                            },
                        ) != ::core::cmp::Ordering::Less
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Ord for Id {
        #[inline]
        fn cmp(&self, other: &Id) -> ::core::cmp::Ordering {
            match *other {
                Id {
                    namespace: ref __self_1_0,
                    id: ref __self_1_1,
                } => match *self {
                    Id {
                        namespace: ref __self_0_0,
                        id: ref __self_0_1,
                    } => match ::core::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                        ::core::cmp::Ordering::Equal => {
                            match ::core::cmp::Ord::cmp(&(*__self_0_1), &(*__self_1_1)) {
                                ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                                cmp => cmp,
                            }
                        }
                        cmp => cmp,
                    },
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::hash::Hash for Id {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                Id {
                    namespace: ref __self_0_0,
                    id: ref __self_0_1,
                } => {
                    ::core::hash::Hash::hash(&(*__self_0_0), state);
                    ::core::hash::Hash::hash(&(*__self_0_1), state)
                }
            }
        }
    }
    impl Id {
        #[doc = " Create a new ID for the specified object type."]
        pub fn new<T: TypedObject>(uuid: Uuid) -> Self {
            Self {
                namespace: Cow::Borrowed(T::TYPE),
                id: uuid,
            }
        }
        pub fn object_type(&self) -> &str {
            &self.namespace
        }
        pub fn uuid(&self) -> &Uuid {
            &self.id
        }
    }
    impl fmt::Debug for Id {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &[""],
                &match (&self,) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            ))
        }
    }
    impl fmt::Display for Id {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", "--"],
                &match (&self.namespace, &self.id.to_hyphenated()) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                    ],
                },
            ))
        }
    }
    impl FromStr for Id {
        type Err = IdParseError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut parts = s.splitn(2, "--");
            let raw_ns = parts.next().ok_or(IdParseError::TooFewParts)?;
            if raw_ns.chars().any(|c| c.is_uppercase()) {
                return Err(IdParseError::ObjectType(raw_ns.to_string()));
            }
            let namespace = Cow::Owned(raw_ns.to_string());
            let id = parts.next().ok_or(IdParseError::TooFewParts)?.parse()?;
            Ok(Id { namespace, id })
        }
    }
    impl Serialize for Id {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(&{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &[""],
                    &match (&self,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ));
                res
            })
        }
    }
    impl<'de> Deserialize<'de> for Id {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            s.parse().map_err(D::Error::custom)
        }
    }
    #[non_exhaustive]
    pub enum IdParseError {
        #[error("Not enough parts. An ID is a namespace and UUID joined by '--'")]
        TooFewParts,
        # [error ("Invalid object-type string `{}`" , . 0)]
        ObjectType(String),
        #[error("Unable to parse UUID")]
        Uuid(#[from] uuid::Error),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for IdParseError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&IdParseError::TooFewParts,) => {
                    let mut debug_trait_builder = f.debug_tuple("TooFewParts");
                    debug_trait_builder.finish()
                }
                (&IdParseError::ObjectType(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("ObjectType");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&IdParseError::Uuid(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Uuid");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(unused_qualifications)]
    impl std::error::Error for IdParseError {
        fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
            use thiserror::private::AsDynError;
            #[allow(deprecated)]
            match self {
                IdParseError::TooFewParts { .. } => std::option::Option::None,
                IdParseError::ObjectType { .. } => std::option::Option::None,
                IdParseError::Uuid { 0: source, .. } => {
                    std::option::Option::Some(source.as_dyn_error())
                }
            }
        }
    }
    #[allow(unused_qualifications)]
    impl std::fmt::Display for IdParseError {
        fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
            match self {
                IdParseError::TooFewParts {} => {
                    __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                        &["Not enough parts. An ID is a namespace and UUID joined by \'--\'"],
                        &match () {
                            () => [],
                        },
                    ))
                }
                IdParseError::ObjectType(_0) => {
                    __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                        &["Invalid object-type string `", "`"],
                        &match (&_0,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ))
                }
                IdParseError::Uuid(_0) => __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Unable to parse UUID"],
                    &match () {
                        () => [],
                    },
                )),
            }
        }
    }
    #[allow(unused_qualifications)]
    impl std::convert::From<uuid::Error> for IdParseError {
        #[allow(deprecated)]
        fn from(source: uuid::Error) -> Self {
            IdParseError::Uuid { 0: source }
        }
    }
}
pub mod identity {
    use std::fmt;
    use serde::Deserialize;
    use crate::CommonProperties;
    #[typed_object(core)]
    pub struct Identity {
        #[serde(flatten)]
        common: CommonProperties,
        name: String,
        #[serde(default)]
        description: Option<String>,
        #[serde(default)]
        contact_information: Option<ContactInformation>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Identity {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field<'de> {
                    __field1,
                    __field2,
                    __field3,
                    __other(_serde::__private::de::Content<'de>),
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field<'de>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_bool<__E>(
                        self,
                        __value: bool,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Bool(__value),
                        ))
                    }
                    fn visit_i8<__E>(
                        self,
                        __value: i8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(_serde::__private::de::Content::I8(
                            __value,
                        )))
                    }
                    fn visit_i16<__E>(
                        self,
                        __value: i16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I16(__value),
                        ))
                    }
                    fn visit_i32<__E>(
                        self,
                        __value: i32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I32(__value),
                        ))
                    }
                    fn visit_i64<__E>(
                        self,
                        __value: i64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I64(__value),
                        ))
                    }
                    fn visit_u8<__E>(
                        self,
                        __value: u8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(_serde::__private::de::Content::U8(
                            __value,
                        )))
                    }
                    fn visit_u16<__E>(
                        self,
                        __value: u16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U16(__value),
                        ))
                    }
                    fn visit_u32<__E>(
                        self,
                        __value: u32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U32(__value),
                        ))
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U64(__value),
                        ))
                    }
                    fn visit_f32<__E>(
                        self,
                        __value: f32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::F32(__value),
                        ))
                    }
                    fn visit_f64<__E>(
                        self,
                        __value: f64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::F64(__value),
                        ))
                    }
                    fn visit_char<__E>(
                        self,
                        __value: char,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Char(__value),
                        ))
                    }
                    fn visit_unit<__E>(self) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Unit,
                        ))
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field1),
                            "description" => _serde::__private::Ok(__Field::__field2),
                            "contact_information" => _serde::__private::Ok(__Field::__field3),
                            _ => {
                                let __value = _serde::__private::de::Content::String(
                                    _serde::__private::ToString::to_string(__value),
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field1),
                            b"description" => _serde::__private::Ok(__Field::__field2),
                            b"contact_information" => _serde::__private::Ok(__Field::__field3),
                            _ => {
                                let __value =
                                    _serde::__private::de::Content::ByteBuf(__value.to_vec());
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_str<__E>(
                        self,
                        __value: &'de str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field1),
                            "description" => _serde::__private::Ok(__Field::__field2),
                            "contact_information" => _serde::__private::Ok(__Field::__field3),
                            _ => {
                                let __value = _serde::__private::de::Content::Str(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_bytes<__E>(
                        self,
                        __value: &'de [u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field1),
                            b"description" => _serde::__private::Ok(__Field::__field2),
                            b"contact_information" => _serde::__private::Ok(__Field::__field3),
                            _ => {
                                let __value = _serde::__private::de::Content::Bytes(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field<'de> {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Identity>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Identity;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct Identity")
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Option<String>> =
                            _serde::__private::None;
                        let mut __field3: _serde::__private::Option<Option<ContactInformation>> =
                            _serde::__private::None;
                        let mut __collect = _serde::__private::Vec::<
                            _serde::__private::Option<(
                                _serde::__private::de::Content,
                                _serde::__private::de::Content,
                            )>,
                        >::new();
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "description",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "contact_information",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<ContactInformation>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__other(__name) => {
                                    __collect.push(_serde::__private::Some((
                                        __name,
                                        match _serde::de::MapAccess::next_value(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    )));
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field0: CommonProperties = match _serde::de::Deserialize::deserialize(
                            _serde::__private::de::FlatMapDeserializer(
                                &mut __collect,
                                _serde::__private::PhantomData,
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(Identity {
                            common: __field0,
                            name: __field1,
                            description: __field2,
                            contact_information: __field3,
                        })
                    }
                }
                _serde::Deserializer::deserialize_map(
                    __deserializer,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Identity>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl crate::TypedObject for Identity {
        const TYPE: &'static str = "identity";
    }
    impl Identity {
        pub fn name(&self) -> &str {
            &self.name
        }
        pub fn description(&self) -> Option<&str> {
            self.description.as_ref().map(|s| s.as_str())
        }
        pub fn contact_information(&self) -> Option<&ContactInformation> {
            self.contact_information.as_ref()
        }
    }
    impl AsRef<CommonProperties> for Identity {
        fn as_ref(&self) -> &CommonProperties {
            &self.common
        }
    }
    #[doc = " Contact information for an [`Identity`]."]
    pub struct ContactInformation(String);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ContactInformation {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                ContactInformation(ref __self_0_0) => {
                    let mut debug_trait_builder = f.debug_tuple("ContactInformation");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ContactInformation {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ContactInformation>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ContactInformation;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "tuple struct ContactInformation",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: String =
                            match <String as _serde::Deserialize>::deserialize(__e) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                        _serde::__private::Ok(ContactInformation(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"tuple struct ContactInformation with 1 element",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(ContactInformation(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "ContactInformation",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ContactInformation>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for ContactInformation {
        #[inline]
        fn clone(&self) -> ContactInformation {
            match *self {
                ContactInformation(ref __self_0_0) => {
                    ContactInformation(::core::clone::Clone::clone(&(*__self_0_0)))
                }
            }
        }
    }
    impl fmt::Display for ContactInformation {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.0.fmt(f)
        }
    }
}
mod infrastructure {
    use std::collections::BTreeSet;
    use serde::Deserialize;
    use crate::{CommonProperties, KillChainPhase};
    #[typed_object(core)]
    pub struct Infrastructure {
        #[serde(flatten)]
        common: CommonProperties,
        pub name: String,
        #[serde(default)]
        pub description: Option<String>,
        #[serde(default)]
        pub aliases: BTreeSet<String>,
        #[serde(default)]
        pub kill_chain_phases: Vec<KillChainPhase>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Infrastructure {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field<'de> {
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __other(_serde::__private::de::Content<'de>),
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field<'de>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_bool<__E>(
                        self,
                        __value: bool,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Bool(__value),
                        ))
                    }
                    fn visit_i8<__E>(
                        self,
                        __value: i8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(_serde::__private::de::Content::I8(
                            __value,
                        )))
                    }
                    fn visit_i16<__E>(
                        self,
                        __value: i16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I16(__value),
                        ))
                    }
                    fn visit_i32<__E>(
                        self,
                        __value: i32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I32(__value),
                        ))
                    }
                    fn visit_i64<__E>(
                        self,
                        __value: i64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I64(__value),
                        ))
                    }
                    fn visit_u8<__E>(
                        self,
                        __value: u8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(_serde::__private::de::Content::U8(
                            __value,
                        )))
                    }
                    fn visit_u16<__E>(
                        self,
                        __value: u16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U16(__value),
                        ))
                    }
                    fn visit_u32<__E>(
                        self,
                        __value: u32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U32(__value),
                        ))
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U64(__value),
                        ))
                    }
                    fn visit_f32<__E>(
                        self,
                        __value: f32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::F32(__value),
                        ))
                    }
                    fn visit_f64<__E>(
                        self,
                        __value: f64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::F64(__value),
                        ))
                    }
                    fn visit_char<__E>(
                        self,
                        __value: char,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Char(__value),
                        ))
                    }
                    fn visit_unit<__E>(self) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Unit,
                        ))
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field1),
                            "description" => _serde::__private::Ok(__Field::__field2),
                            "aliases" => _serde::__private::Ok(__Field::__field3),
                            "kill_chain_phases" => _serde::__private::Ok(__Field::__field4),
                            _ => {
                                let __value = _serde::__private::de::Content::String(
                                    _serde::__private::ToString::to_string(__value),
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field1),
                            b"description" => _serde::__private::Ok(__Field::__field2),
                            b"aliases" => _serde::__private::Ok(__Field::__field3),
                            b"kill_chain_phases" => _serde::__private::Ok(__Field::__field4),
                            _ => {
                                let __value =
                                    _serde::__private::de::Content::ByteBuf(__value.to_vec());
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_str<__E>(
                        self,
                        __value: &'de str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field1),
                            "description" => _serde::__private::Ok(__Field::__field2),
                            "aliases" => _serde::__private::Ok(__Field::__field3),
                            "kill_chain_phases" => _serde::__private::Ok(__Field::__field4),
                            _ => {
                                let __value = _serde::__private::de::Content::Str(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_bytes<__E>(
                        self,
                        __value: &'de [u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field1),
                            b"description" => _serde::__private::Ok(__Field::__field2),
                            b"aliases" => _serde::__private::Ok(__Field::__field3),
                            b"kill_chain_phases" => _serde::__private::Ok(__Field::__field4),
                            _ => {
                                let __value = _serde::__private::de::Content::Bytes(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field<'de> {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Infrastructure>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Infrastructure;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Infrastructure",
                        )
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Option<String>> =
                            _serde::__private::None;
                        let mut __field3: _serde::__private::Option<BTreeSet<String>> =
                            _serde::__private::None;
                        let mut __field4: _serde::__private::Option<Vec<KillChainPhase>> =
                            _serde::__private::None;
                        let mut __collect = _serde::__private::Vec::<
                            _serde::__private::Option<(
                                _serde::__private::de::Content,
                                _serde::__private::de::Content,
                            )>,
                        >::new();
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "description",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "aliases",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<BTreeSet<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "kill_chain_phases",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<KillChainPhase>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__other(__name) => {
                                    __collect.push(_serde::__private::Some((
                                        __name,
                                        match _serde::de::MapAccess::next_value(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    )));
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field0: CommonProperties = match _serde::de::Deserialize::deserialize(
                            _serde::__private::de::FlatMapDeserializer(
                                &mut __collect,
                                _serde::__private::PhantomData,
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(Infrastructure {
                            common: __field0,
                            name: __field1,
                            description: __field2,
                            aliases: __field3,
                            kill_chain_phases: __field4,
                        })
                    }
                }
                _serde::Deserializer::deserialize_map(
                    __deserializer,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Infrastructure>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl crate::TypedObject for Infrastructure {
        const TYPE: &'static str = "infrastructure";
    }
    impl AsRef<CommonProperties> for Infrastructure {
        fn as_ref(&self) -> &CommonProperties {
            &self.common
        }
    }
}
mod intrusion_set {
    use std::collections::BTreeSet;
    use chrono::{DateTime, Utc};
    use serde::Deserialize;
    use crate::{
        vocab::{AttackMotivation, AttackResourceLevel},
        CommonProperties,
    };
    #[typed_object(core)]
    pub struct IntrusionSet {
        #[serde(flatten)]
        common: CommonProperties,
        name: String,
        #[serde(default)]
        description: Option<String>,
        #[serde(default)]
        aliases: BTreeSet<String>,
        #[serde(default)]
        pub first_seen: Option<DateTime<Utc>>,
        #[serde(default)]
        pub last_seen: Option<DateTime<Utc>>,
        #[serde(default)]
        pub goals: Vec<String>,
        #[serde(default)]
        pub resource_level: Option<AttackResourceLevel>,
        #[serde(default)]
        pub primary_motivation: Option<AttackMotivation>,
        #[serde(default)]
        pub secondary_motivations: BTreeSet<AttackMotivation>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for IntrusionSet {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field<'de> {
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                    __field9,
                    __other(_serde::__private::de::Content<'de>),
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field<'de>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_bool<__E>(
                        self,
                        __value: bool,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Bool(__value),
                        ))
                    }
                    fn visit_i8<__E>(
                        self,
                        __value: i8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(_serde::__private::de::Content::I8(
                            __value,
                        )))
                    }
                    fn visit_i16<__E>(
                        self,
                        __value: i16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I16(__value),
                        ))
                    }
                    fn visit_i32<__E>(
                        self,
                        __value: i32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I32(__value),
                        ))
                    }
                    fn visit_i64<__E>(
                        self,
                        __value: i64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I64(__value),
                        ))
                    }
                    fn visit_u8<__E>(
                        self,
                        __value: u8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(_serde::__private::de::Content::U8(
                            __value,
                        )))
                    }
                    fn visit_u16<__E>(
                        self,
                        __value: u16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U16(__value),
                        ))
                    }
                    fn visit_u32<__E>(
                        self,
                        __value: u32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U32(__value),
                        ))
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U64(__value),
                        ))
                    }
                    fn visit_f32<__E>(
                        self,
                        __value: f32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::F32(__value),
                        ))
                    }
                    fn visit_f64<__E>(
                        self,
                        __value: f64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::F64(__value),
                        ))
                    }
                    fn visit_char<__E>(
                        self,
                        __value: char,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Char(__value),
                        ))
                    }
                    fn visit_unit<__E>(self) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Unit,
                        ))
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field1),
                            "description" => _serde::__private::Ok(__Field::__field2),
                            "aliases" => _serde::__private::Ok(__Field::__field3),
                            "first_seen" => _serde::__private::Ok(__Field::__field4),
                            "last_seen" => _serde::__private::Ok(__Field::__field5),
                            "goals" => _serde::__private::Ok(__Field::__field6),
                            "resource_level" => _serde::__private::Ok(__Field::__field7),
                            "primary_motivation" => _serde::__private::Ok(__Field::__field8),
                            "secondary_motivations" => _serde::__private::Ok(__Field::__field9),
                            _ => {
                                let __value = _serde::__private::de::Content::String(
                                    _serde::__private::ToString::to_string(__value),
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field1),
                            b"description" => _serde::__private::Ok(__Field::__field2),
                            b"aliases" => _serde::__private::Ok(__Field::__field3),
                            b"first_seen" => _serde::__private::Ok(__Field::__field4),
                            b"last_seen" => _serde::__private::Ok(__Field::__field5),
                            b"goals" => _serde::__private::Ok(__Field::__field6),
                            b"resource_level" => _serde::__private::Ok(__Field::__field7),
                            b"primary_motivation" => _serde::__private::Ok(__Field::__field8),
                            b"secondary_motivations" => _serde::__private::Ok(__Field::__field9),
                            _ => {
                                let __value =
                                    _serde::__private::de::Content::ByteBuf(__value.to_vec());
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_str<__E>(
                        self,
                        __value: &'de str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field1),
                            "description" => _serde::__private::Ok(__Field::__field2),
                            "aliases" => _serde::__private::Ok(__Field::__field3),
                            "first_seen" => _serde::__private::Ok(__Field::__field4),
                            "last_seen" => _serde::__private::Ok(__Field::__field5),
                            "goals" => _serde::__private::Ok(__Field::__field6),
                            "resource_level" => _serde::__private::Ok(__Field::__field7),
                            "primary_motivation" => _serde::__private::Ok(__Field::__field8),
                            "secondary_motivations" => _serde::__private::Ok(__Field::__field9),
                            _ => {
                                let __value = _serde::__private::de::Content::Str(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_bytes<__E>(
                        self,
                        __value: &'de [u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field1),
                            b"description" => _serde::__private::Ok(__Field::__field2),
                            b"aliases" => _serde::__private::Ok(__Field::__field3),
                            b"first_seen" => _serde::__private::Ok(__Field::__field4),
                            b"last_seen" => _serde::__private::Ok(__Field::__field5),
                            b"goals" => _serde::__private::Ok(__Field::__field6),
                            b"resource_level" => _serde::__private::Ok(__Field::__field7),
                            b"primary_motivation" => _serde::__private::Ok(__Field::__field8),
                            b"secondary_motivations" => _serde::__private::Ok(__Field::__field9),
                            _ => {
                                let __value = _serde::__private::de::Content::Bytes(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field<'de> {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<IntrusionSet>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = IntrusionSet;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct IntrusionSet")
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Option<String>> =
                            _serde::__private::None;
                        let mut __field3: _serde::__private::Option<BTreeSet<String>> =
                            _serde::__private::None;
                        let mut __field4: _serde::__private::Option<Option<DateTime<Utc>>> =
                            _serde::__private::None;
                        let mut __field5: _serde::__private::Option<Option<DateTime<Utc>>> =
                            _serde::__private::None;
                        let mut __field6: _serde::__private::Option<Vec<String>> =
                            _serde::__private::None;
                        let mut __field7: _serde::__private::Option<Option<AttackResourceLevel>> =
                            _serde::__private::None;
                        let mut __field8: _serde::__private::Option<Option<AttackMotivation>> =
                            _serde::__private::None;
                        let mut __field9: _serde::__private::Option<BTreeSet<AttackMotivation>> =
                            _serde::__private::None;
                        let mut __collect = _serde::__private::Vec::<
                            _serde::__private::Option<(
                                _serde::__private::de::Content,
                                _serde::__private::de::Content,
                            )>,
                        >::new();
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "description",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "aliases",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<BTreeSet<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "first_seen",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<DateTime<Utc>>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "last_seen",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<DateTime<Utc>>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::__private::Option::is_some(&__field6) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "goals",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field7 => {
                                    if _serde::__private::Option::is_some(&__field7) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "resource_level",
                                            ),
                                        );
                                    }
                                    __field7 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<AttackResourceLevel>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field8 => {
                                    if _serde::__private::Option::is_some(&__field8) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "primary_motivation",
                                            ),
                                        );
                                    }
                                    __field8 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<AttackMotivation>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field9 => {
                                    if _serde::__private::Option::is_some(&__field9) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "secondary_motivations",
                                            ),
                                        );
                                    }
                                    __field9 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            BTreeSet<AttackMotivation>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__other(__name) => {
                                    __collect.push(_serde::__private::Some((
                                        __name,
                                        match _serde::de::MapAccess::next_value(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    )));
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field6 = match __field6 {
                            _serde::__private::Some(__field6) => __field6,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field7 = match __field7 {
                            _serde::__private::Some(__field7) => __field7,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field8 = match __field8 {
                            _serde::__private::Some(__field8) => __field8,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field9 = match __field9 {
                            _serde::__private::Some(__field9) => __field9,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field0: CommonProperties = match _serde::de::Deserialize::deserialize(
                            _serde::__private::de::FlatMapDeserializer(
                                &mut __collect,
                                _serde::__private::PhantomData,
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(IntrusionSet {
                            common: __field0,
                            name: __field1,
                            description: __field2,
                            aliases: __field3,
                            first_seen: __field4,
                            last_seen: __field5,
                            goals: __field6,
                            resource_level: __field7,
                            primary_motivation: __field8,
                            secondary_motivations: __field9,
                        })
                    }
                }
                _serde::Deserializer::deserialize_map(
                    __deserializer,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<IntrusionSet>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl crate::TypedObject for IntrusionSet {
        const TYPE: &'static str = "intrusion-set";
    }
    impl IntrusionSet {
        pub fn name(&self) -> &str {
            &self.name
        }
        pub fn description(&self) -> Option<&str> {
            self.description.as_ref().map(|s| s.as_str())
        }
        pub fn aliases(&self) -> &BTreeSet<String> {
            &self.aliases
        }
    }
    impl AsRef<CommonProperties> for IntrusionSet {
        fn as_ref(&self) -> &CommonProperties {
            &self.common
        }
    }
}
pub mod location {
    use serde::Deserialize;
    use crate::CommonProperties;
    pub struct Coordinates {
        pub latitude: f64,
        pub longitude: f64,
        #[serde(default)]
        pub precision: Option<f64>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Coordinates {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "latitude" => _serde::__private::Ok(__Field::__field0),
                            "longitude" => _serde::__private::Ok(__Field::__field1),
                            "precision" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"latitude" => _serde::__private::Ok(__Field::__field0),
                            b"longitude" => _serde::__private::Ok(__Field::__field1),
                            b"precision" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Coordinates>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Coordinates;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct Coordinates")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<f64>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Coordinates with 3 elements",
                                        ),
                                    );
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<f64>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Coordinates with 3 elements",
                                        ),
                                    );
                                }
                            };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<Option<f64>>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        _serde::__private::Ok(Coordinates {
                            latitude: __field0,
                            longitude: __field1,
                            precision: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<f64> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<f64> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Option<f64>> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "latitude",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<f64>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "longitude",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<f64>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "precision",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<f64>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("latitude") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("longitude") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        _serde::__private::Ok(Coordinates {
                            latitude: __field0,
                            longitude: __field1,
                            precision: __field2,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["latitude", "longitude", "precision"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Coordinates",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Coordinates>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub struct Region(String);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Region {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Region(ref __self_0_0) => {
                    let mut debug_trait_builder = f.debug_tuple("Region");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Region {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Region>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Region;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "tuple struct Region")
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: String =
                            match <String as _serde::Deserialize>::deserialize(__e) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                        _serde::__private::Ok(Region(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"tuple struct Region with 1 element",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(Region(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "Region",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Region>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Region {
        #[inline]
        fn clone(&self) -> Region {
            match *self {
                Region(ref __self_0_0) => Region(::core::clone::Clone::clone(&(*__self_0_0))),
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for Region {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Region {
        #[inline]
        fn eq(&self, other: &Region) -> bool {
            match *other {
                Region(ref __self_1_0) => match *self {
                    Region(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Region) -> bool {
            match *other {
                Region(ref __self_1_0) => match *self {
                    Region(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl ::core::marker::StructuralEq for Region {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for Region {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<String>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialOrd for Region {
        #[inline]
        fn partial_cmp(&self, other: &Region) -> ::core::option::Option<::core::cmp::Ordering> {
            match *other {
                Region(ref __self_1_0) => match *self {
                    Region(ref __self_0_0) => {
                        match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)) {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                            }
                            cmp => cmp,
                        }
                    }
                },
            }
        }
        #[inline]
        fn lt(&self, other: &Region) -> bool {
            match *other {
                Region(ref __self_1_0) => match *self {
                    Region(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Greater,
                        ) == ::core::cmp::Ordering::Less
                    }
                },
            }
        }
        #[inline]
        fn le(&self, other: &Region) -> bool {
            match *other {
                Region(ref __self_1_0) => match *self {
                    Region(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Greater,
                        ) != ::core::cmp::Ordering::Greater
                    }
                },
            }
        }
        #[inline]
        fn gt(&self, other: &Region) -> bool {
            match *other {
                Region(ref __self_1_0) => match *self {
                    Region(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Less,
                        ) == ::core::cmp::Ordering::Greater
                    }
                },
            }
        }
        #[inline]
        fn ge(&self, other: &Region) -> bool {
            match *other {
                Region(ref __self_1_0) => match *self {
                    Region(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Less,
                        ) != ::core::cmp::Ordering::Less
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Ord for Region {
        #[inline]
        fn cmp(&self, other: &Region) -> ::core::cmp::Ordering {
            match *other {
                Region(ref __self_1_0) => match *self {
                    Region(ref __self_0_0) => {
                        match ::core::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                            ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                            cmp => cmp,
                        }
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::hash::Hash for Region {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                Region(ref __self_0_0) => ::core::hash::Hash::hash(&(*__self_0_0), state),
            }
        }
    }
    pub struct Country(String);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Country {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Country(ref __self_0_0) => {
                    let mut debug_trait_builder = f.debug_tuple("Country");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Country {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Country>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Country;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "tuple struct Country")
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: String =
                            match <String as _serde::Deserialize>::deserialize(__e) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                        _serde::__private::Ok(Country(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"tuple struct Country with 1 element",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(Country(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "Country",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Country>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Country {
        #[inline]
        fn clone(&self) -> Country {
            match *self {
                Country(ref __self_0_0) => Country(::core::clone::Clone::clone(&(*__self_0_0))),
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for Country {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Country {
        #[inline]
        fn eq(&self, other: &Country) -> bool {
            match *other {
                Country(ref __self_1_0) => match *self {
                    Country(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Country) -> bool {
            match *other {
                Country(ref __self_1_0) => match *self {
                    Country(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl ::core::marker::StructuralEq for Country {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for Country {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<String>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialOrd for Country {
        #[inline]
        fn partial_cmp(&self, other: &Country) -> ::core::option::Option<::core::cmp::Ordering> {
            match *other {
                Country(ref __self_1_0) => match *self {
                    Country(ref __self_0_0) => {
                        match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)) {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                            }
                            cmp => cmp,
                        }
                    }
                },
            }
        }
        #[inline]
        fn lt(&self, other: &Country) -> bool {
            match *other {
                Country(ref __self_1_0) => match *self {
                    Country(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Greater,
                        ) == ::core::cmp::Ordering::Less
                    }
                },
            }
        }
        #[inline]
        fn le(&self, other: &Country) -> bool {
            match *other {
                Country(ref __self_1_0) => match *self {
                    Country(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Greater,
                        ) != ::core::cmp::Ordering::Greater
                    }
                },
            }
        }
        #[inline]
        fn gt(&self, other: &Country) -> bool {
            match *other {
                Country(ref __self_1_0) => match *self {
                    Country(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Less,
                        ) == ::core::cmp::Ordering::Greater
                    }
                },
            }
        }
        #[inline]
        fn ge(&self, other: &Country) -> bool {
            match *other {
                Country(ref __self_1_0) => match *self {
                    Country(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Less,
                        ) != ::core::cmp::Ordering::Less
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Ord for Country {
        #[inline]
        fn cmp(&self, other: &Country) -> ::core::cmp::Ordering {
            match *other {
                Country(ref __self_1_0) => match *self {
                    Country(ref __self_0_0) => {
                        match ::core::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                            ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                            cmp => cmp,
                        }
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::hash::Hash for Country {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                Country(ref __self_0_0) => ::core::hash::Hash::hash(&(*__self_0_0), state),
            }
        }
    }
    #[typed_object(core)]
    pub struct Location {
        #[serde(flatten)]
        base: CommonProperties,
        pub name: String,
        #[serde(default)]
        pub description: Option<String>,
        #[serde(default)]
        pub region: Option<Region>,
        #[serde(default)]
        pub country: Option<Country>,
        #[serde(default)]
        pub administrative_area: Option<String>,
        #[serde(default)]
        pub city: Option<String>,
        #[serde(default)]
        pub street_address: Option<String>,
        #[serde(default)]
        pub postal_code: Option<String>,
        #[serde(default, flatten)]
        coordinates: Option<Coordinates>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Location {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field<'de> {
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                    __other(_serde::__private::de::Content<'de>),
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field<'de>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_bool<__E>(
                        self,
                        __value: bool,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Bool(__value),
                        ))
                    }
                    fn visit_i8<__E>(
                        self,
                        __value: i8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(_serde::__private::de::Content::I8(
                            __value,
                        )))
                    }
                    fn visit_i16<__E>(
                        self,
                        __value: i16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I16(__value),
                        ))
                    }
                    fn visit_i32<__E>(
                        self,
                        __value: i32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I32(__value),
                        ))
                    }
                    fn visit_i64<__E>(
                        self,
                        __value: i64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I64(__value),
                        ))
                    }
                    fn visit_u8<__E>(
                        self,
                        __value: u8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(_serde::__private::de::Content::U8(
                            __value,
                        )))
                    }
                    fn visit_u16<__E>(
                        self,
                        __value: u16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U16(__value),
                        ))
                    }
                    fn visit_u32<__E>(
                        self,
                        __value: u32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U32(__value),
                        ))
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U64(__value),
                        ))
                    }
                    fn visit_f32<__E>(
                        self,
                        __value: f32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::F32(__value),
                        ))
                    }
                    fn visit_f64<__E>(
                        self,
                        __value: f64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::F64(__value),
                        ))
                    }
                    fn visit_char<__E>(
                        self,
                        __value: char,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Char(__value),
                        ))
                    }
                    fn visit_unit<__E>(self) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Unit,
                        ))
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field1),
                            "description" => _serde::__private::Ok(__Field::__field2),
                            "region" => _serde::__private::Ok(__Field::__field3),
                            "country" => _serde::__private::Ok(__Field::__field4),
                            "administrative_area" => _serde::__private::Ok(__Field::__field5),
                            "city" => _serde::__private::Ok(__Field::__field6),
                            "street_address" => _serde::__private::Ok(__Field::__field7),
                            "postal_code" => _serde::__private::Ok(__Field::__field8),
                            _ => {
                                let __value = _serde::__private::de::Content::String(
                                    _serde::__private::ToString::to_string(__value),
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field1),
                            b"description" => _serde::__private::Ok(__Field::__field2),
                            b"region" => _serde::__private::Ok(__Field::__field3),
                            b"country" => _serde::__private::Ok(__Field::__field4),
                            b"administrative_area" => _serde::__private::Ok(__Field::__field5),
                            b"city" => _serde::__private::Ok(__Field::__field6),
                            b"street_address" => _serde::__private::Ok(__Field::__field7),
                            b"postal_code" => _serde::__private::Ok(__Field::__field8),
                            _ => {
                                let __value =
                                    _serde::__private::de::Content::ByteBuf(__value.to_vec());
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_str<__E>(
                        self,
                        __value: &'de str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field1),
                            "description" => _serde::__private::Ok(__Field::__field2),
                            "region" => _serde::__private::Ok(__Field::__field3),
                            "country" => _serde::__private::Ok(__Field::__field4),
                            "administrative_area" => _serde::__private::Ok(__Field::__field5),
                            "city" => _serde::__private::Ok(__Field::__field6),
                            "street_address" => _serde::__private::Ok(__Field::__field7),
                            "postal_code" => _serde::__private::Ok(__Field::__field8),
                            _ => {
                                let __value = _serde::__private::de::Content::Str(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_bytes<__E>(
                        self,
                        __value: &'de [u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field1),
                            b"description" => _serde::__private::Ok(__Field::__field2),
                            b"region" => _serde::__private::Ok(__Field::__field3),
                            b"country" => _serde::__private::Ok(__Field::__field4),
                            b"administrative_area" => _serde::__private::Ok(__Field::__field5),
                            b"city" => _serde::__private::Ok(__Field::__field6),
                            b"street_address" => _serde::__private::Ok(__Field::__field7),
                            b"postal_code" => _serde::__private::Ok(__Field::__field8),
                            _ => {
                                let __value = _serde::__private::de::Content::Bytes(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field<'de> {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Location>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Location;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct Location")
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Option<String>> =
                            _serde::__private::None;
                        let mut __field3: _serde::__private::Option<Option<Region>> =
                            _serde::__private::None;
                        let mut __field4: _serde::__private::Option<Option<Country>> =
                            _serde::__private::None;
                        let mut __field5: _serde::__private::Option<Option<String>> =
                            _serde::__private::None;
                        let mut __field6: _serde::__private::Option<Option<String>> =
                            _serde::__private::None;
                        let mut __field7: _serde::__private::Option<Option<String>> =
                            _serde::__private::None;
                        let mut __field8: _serde::__private::Option<Option<String>> =
                            _serde::__private::None;
                        let mut __collect = _serde::__private::Vec::<
                            _serde::__private::Option<(
                                _serde::__private::de::Content,
                                _serde::__private::de::Content,
                            )>,
                        >::new();
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "description",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "region",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<Region>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "country",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<Country>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "administrative_area",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::__private::Option::is_some(&__field6) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "city",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field7 => {
                                    if _serde::__private::Option::is_some(&__field7) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "street_address",
                                            ),
                                        );
                                    }
                                    __field7 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field8 => {
                                    if _serde::__private::Option::is_some(&__field8) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "postal_code",
                                            ),
                                        );
                                    }
                                    __field8 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__other(__name) => {
                                    __collect.push(_serde::__private::Some((
                                        __name,
                                        match _serde::de::MapAccess::next_value(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    )));
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field6 = match __field6 {
                            _serde::__private::Some(__field6) => __field6,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field7 = match __field7 {
                            _serde::__private::Some(__field7) => __field7,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field8 = match __field8 {
                            _serde::__private::Some(__field8) => __field8,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field0: CommonProperties = match _serde::de::Deserialize::deserialize(
                            _serde::__private::de::FlatMapDeserializer(
                                &mut __collect,
                                _serde::__private::PhantomData,
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        let __field9: Option<Coordinates> =
                            match _serde::de::Deserialize::deserialize(
                                _serde::__private::de::FlatMapDeserializer(
                                    &mut __collect,
                                    _serde::__private::PhantomData,
                                ),
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                        _serde::__private::Ok(Location {
                            base: __field0,
                            name: __field1,
                            description: __field2,
                            region: __field3,
                            country: __field4,
                            administrative_area: __field5,
                            city: __field6,
                            street_address: __field7,
                            postal_code: __field8,
                            coordinates: __field9,
                        })
                    }
                }
                _serde::Deserializer::deserialize_map(
                    __deserializer,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Location>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl crate::TypedObject for Location {
        const TYPE: &'static str = "location";
    }
    impl Location {
        pub fn name(&self) -> &str {
            &self.name
        }
        pub fn description(&self) -> Option<&str> {
            self.description.as_ref().map(|s| s.as_str())
        }
        pub fn coordinates(&self) -> Option<&Coordinates> {
            self.coordinates.as_ref()
        }
    }
    impl AsRef<CommonProperties> for Location {
        fn as_ref(&self) -> &CommonProperties {
            &self.base
        }
    }
}
mod malware {
    use std::collections::BTreeSet;
    use chrono::{DateTime, Utc};
    use serde::Deserialize;
    use crate::{
        vocab::{ImplementationLanguage, MalwareCapabilities, MalwareType, ProcessorArchitecture},
        CommonProperties, Id, KillChainPhase,
    };
    #[typed_object(core)]
    pub struct Malware {
        #[serde(flatten)]
        base: CommonProperties,
        pub name: String,
        #[serde(default)]
        pub description: Option<String>,
        #[serde(default)]
        pub malware_types: BTreeSet<MalwareType>,
        #[serde(default)]
        pub is_family: Option<bool>,
        #[serde(default)]
        pub kill_chain_phases: Vec<KillChainPhase>,
        #[serde(default)]
        pub first_seen: Option<DateTime<Utc>>,
        #[serde(default)]
        pub last_seen: Option<DateTime<Utc>>,
        #[serde(default)]
        pub implementation_languages: BTreeSet<ImplementationLanguage>,
        #[serde(default)]
        pub operating_system_refs: BTreeSet<Id>,
        #[serde(default)]
        pub architecture_execution_envs: BTreeSet<ProcessorArchitecture>,
        #[serde(default)]
        pub capabilities: BTreeSet<MalwareCapabilities>,
        #[serde(default)]
        pub sample_refs: BTreeSet<Id>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Malware {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field<'de> {
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                    __field9,
                    __field10,
                    __field11,
                    __field12,
                    __other(_serde::__private::de::Content<'de>),
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field<'de>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_bool<__E>(
                        self,
                        __value: bool,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Bool(__value),
                        ))
                    }
                    fn visit_i8<__E>(
                        self,
                        __value: i8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(_serde::__private::de::Content::I8(
                            __value,
                        )))
                    }
                    fn visit_i16<__E>(
                        self,
                        __value: i16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I16(__value),
                        ))
                    }
                    fn visit_i32<__E>(
                        self,
                        __value: i32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I32(__value),
                        ))
                    }
                    fn visit_i64<__E>(
                        self,
                        __value: i64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I64(__value),
                        ))
                    }
                    fn visit_u8<__E>(
                        self,
                        __value: u8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(_serde::__private::de::Content::U8(
                            __value,
                        )))
                    }
                    fn visit_u16<__E>(
                        self,
                        __value: u16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U16(__value),
                        ))
                    }
                    fn visit_u32<__E>(
                        self,
                        __value: u32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U32(__value),
                        ))
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U64(__value),
                        ))
                    }
                    fn visit_f32<__E>(
                        self,
                        __value: f32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::F32(__value),
                        ))
                    }
                    fn visit_f64<__E>(
                        self,
                        __value: f64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::F64(__value),
                        ))
                    }
                    fn visit_char<__E>(
                        self,
                        __value: char,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Char(__value),
                        ))
                    }
                    fn visit_unit<__E>(self) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Unit,
                        ))
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field1),
                            "description" => _serde::__private::Ok(__Field::__field2),
                            "malware_types" => _serde::__private::Ok(__Field::__field3),
                            "is_family" => _serde::__private::Ok(__Field::__field4),
                            "kill_chain_phases" => _serde::__private::Ok(__Field::__field5),
                            "first_seen" => _serde::__private::Ok(__Field::__field6),
                            "last_seen" => _serde::__private::Ok(__Field::__field7),
                            "implementation_languages" => _serde::__private::Ok(__Field::__field8),
                            "operating_system_refs" => _serde::__private::Ok(__Field::__field9),
                            "architecture_execution_envs" => {
                                _serde::__private::Ok(__Field::__field10)
                            }
                            "capabilities" => _serde::__private::Ok(__Field::__field11),
                            "sample_refs" => _serde::__private::Ok(__Field::__field12),
                            _ => {
                                let __value = _serde::__private::de::Content::String(
                                    _serde::__private::ToString::to_string(__value),
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field1),
                            b"description" => _serde::__private::Ok(__Field::__field2),
                            b"malware_types" => _serde::__private::Ok(__Field::__field3),
                            b"is_family" => _serde::__private::Ok(__Field::__field4),
                            b"kill_chain_phases" => _serde::__private::Ok(__Field::__field5),
                            b"first_seen" => _serde::__private::Ok(__Field::__field6),
                            b"last_seen" => _serde::__private::Ok(__Field::__field7),
                            b"implementation_languages" => _serde::__private::Ok(__Field::__field8),
                            b"operating_system_refs" => _serde::__private::Ok(__Field::__field9),
                            b"architecture_execution_envs" => {
                                _serde::__private::Ok(__Field::__field10)
                            }
                            b"capabilities" => _serde::__private::Ok(__Field::__field11),
                            b"sample_refs" => _serde::__private::Ok(__Field::__field12),
                            _ => {
                                let __value =
                                    _serde::__private::de::Content::ByteBuf(__value.to_vec());
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_str<__E>(
                        self,
                        __value: &'de str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field1),
                            "description" => _serde::__private::Ok(__Field::__field2),
                            "malware_types" => _serde::__private::Ok(__Field::__field3),
                            "is_family" => _serde::__private::Ok(__Field::__field4),
                            "kill_chain_phases" => _serde::__private::Ok(__Field::__field5),
                            "first_seen" => _serde::__private::Ok(__Field::__field6),
                            "last_seen" => _serde::__private::Ok(__Field::__field7),
                            "implementation_languages" => _serde::__private::Ok(__Field::__field8),
                            "operating_system_refs" => _serde::__private::Ok(__Field::__field9),
                            "architecture_execution_envs" => {
                                _serde::__private::Ok(__Field::__field10)
                            }
                            "capabilities" => _serde::__private::Ok(__Field::__field11),
                            "sample_refs" => _serde::__private::Ok(__Field::__field12),
                            _ => {
                                let __value = _serde::__private::de::Content::Str(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_bytes<__E>(
                        self,
                        __value: &'de [u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field1),
                            b"description" => _serde::__private::Ok(__Field::__field2),
                            b"malware_types" => _serde::__private::Ok(__Field::__field3),
                            b"is_family" => _serde::__private::Ok(__Field::__field4),
                            b"kill_chain_phases" => _serde::__private::Ok(__Field::__field5),
                            b"first_seen" => _serde::__private::Ok(__Field::__field6),
                            b"last_seen" => _serde::__private::Ok(__Field::__field7),
                            b"implementation_languages" => _serde::__private::Ok(__Field::__field8),
                            b"operating_system_refs" => _serde::__private::Ok(__Field::__field9),
                            b"architecture_execution_envs" => {
                                _serde::__private::Ok(__Field::__field10)
                            }
                            b"capabilities" => _serde::__private::Ok(__Field::__field11),
                            b"sample_refs" => _serde::__private::Ok(__Field::__field12),
                            _ => {
                                let __value = _serde::__private::de::Content::Bytes(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field<'de> {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Malware>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Malware;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct Malware")
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Option<String>> =
                            _serde::__private::None;
                        let mut __field3: _serde::__private::Option<BTreeSet<MalwareType>> =
                            _serde::__private::None;
                        let mut __field4: _serde::__private::Option<Option<bool>> =
                            _serde::__private::None;
                        let mut __field5: _serde::__private::Option<Vec<KillChainPhase>> =
                            _serde::__private::None;
                        let mut __field6: _serde::__private::Option<Option<DateTime<Utc>>> =
                            _serde::__private::None;
                        let mut __field7: _serde::__private::Option<Option<DateTime<Utc>>> =
                            _serde::__private::None;
                        let mut __field8: _serde::__private::Option<
                            BTreeSet<ImplementationLanguage>,
                        > = _serde::__private::None;
                        let mut __field9: _serde::__private::Option<BTreeSet<Id>> =
                            _serde::__private::None;
                        let mut __field10: _serde::__private::Option<
                            BTreeSet<ProcessorArchitecture>,
                        > = _serde::__private::None;
                        let mut __field11: _serde::__private::Option<
                            BTreeSet<MalwareCapabilities>,
                        > = _serde::__private::None;
                        let mut __field12: _serde::__private::Option<BTreeSet<Id>> =
                            _serde::__private::None;
                        let mut __collect = _serde::__private::Vec::<
                            _serde::__private::Option<(
                                _serde::__private::de::Content,
                                _serde::__private::de::Content,
                            )>,
                        >::new();
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "description",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "malware_types",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            BTreeSet<MalwareType>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "is_family",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<bool>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "kill_chain_phases",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<KillChainPhase>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::__private::Option::is_some(&__field6) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "first_seen",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<DateTime<Utc>>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field7 => {
                                    if _serde::__private::Option::is_some(&__field7) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "last_seen",
                                            ),
                                        );
                                    }
                                    __field7 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<DateTime<Utc>>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field8 => {
                                    if _serde::__private::Option::is_some(&__field8) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "implementation_languages",
                                            ),
                                        );
                                    }
                                    __field8 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            BTreeSet<ImplementationLanguage>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field9 => {
                                    if _serde::__private::Option::is_some(&__field9) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "operating_system_refs",
                                            ),
                                        );
                                    }
                                    __field9 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<BTreeSet<Id>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field10 => {
                                    if _serde::__private::Option::is_some(&__field10) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "architecture_execution_envs",
                                            ),
                                        );
                                    }
                                    __field10 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            BTreeSet<ProcessorArchitecture>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field11 => {
                                    if _serde::__private::Option::is_some(&__field11) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "capabilities",
                                            ),
                                        );
                                    }
                                    __field11 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            BTreeSet<MalwareCapabilities>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field12 => {
                                    if _serde::__private::Option::is_some(&__field12) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "sample_refs",
                                            ),
                                        );
                                    }
                                    __field12 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<BTreeSet<Id>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__other(__name) => {
                                    __collect.push(_serde::__private::Some((
                                        __name,
                                        match _serde::de::MapAccess::next_value(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    )));
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field6 = match __field6 {
                            _serde::__private::Some(__field6) => __field6,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field7 = match __field7 {
                            _serde::__private::Some(__field7) => __field7,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field8 = match __field8 {
                            _serde::__private::Some(__field8) => __field8,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field9 = match __field9 {
                            _serde::__private::Some(__field9) => __field9,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field10 = match __field10 {
                            _serde::__private::Some(__field10) => __field10,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field11 = match __field11 {
                            _serde::__private::Some(__field11) => __field11,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field12 = match __field12 {
                            _serde::__private::Some(__field12) => __field12,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field0: CommonProperties = match _serde::de::Deserialize::deserialize(
                            _serde::__private::de::FlatMapDeserializer(
                                &mut __collect,
                                _serde::__private::PhantomData,
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(Malware {
                            base: __field0,
                            name: __field1,
                            description: __field2,
                            malware_types: __field3,
                            is_family: __field4,
                            kill_chain_phases: __field5,
                            first_seen: __field6,
                            last_seen: __field7,
                            implementation_languages: __field8,
                            operating_system_refs: __field9,
                            architecture_execution_envs: __field10,
                            capabilities: __field11,
                            sample_refs: __field12,
                        })
                    }
                }
                _serde::Deserializer::deserialize_map(
                    __deserializer,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Malware>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl crate::TypedObject for Malware {
        const TYPE: &'static str = "malware";
    }
    impl Malware {
        pub fn name(&self) -> &str {
            &self.name
        }
        pub fn description(&self) -> Option<&str> {
            self.description.as_ref().map(|s| s.as_str())
        }
    }
    impl AsRef<CommonProperties> for Malware {
        fn as_ref(&self) -> &CommonProperties {
            &self.base
        }
    }
}
mod marking_definition {
    use serde::Deserialize;
    use crate::CommonProperties;
    #[typed_object(core)]
    pub struct MarkingDefinition {
        #[serde(flatten)]
        common: CommonProperties,
        pub name: Option<String>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for MarkingDefinition {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field<'de> {
                    __field1,
                    __other(_serde::__private::de::Content<'de>),
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field<'de>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_bool<__E>(
                        self,
                        __value: bool,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Bool(__value),
                        ))
                    }
                    fn visit_i8<__E>(
                        self,
                        __value: i8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(_serde::__private::de::Content::I8(
                            __value,
                        )))
                    }
                    fn visit_i16<__E>(
                        self,
                        __value: i16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I16(__value),
                        ))
                    }
                    fn visit_i32<__E>(
                        self,
                        __value: i32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I32(__value),
                        ))
                    }
                    fn visit_i64<__E>(
                        self,
                        __value: i64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I64(__value),
                        ))
                    }
                    fn visit_u8<__E>(
                        self,
                        __value: u8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(_serde::__private::de::Content::U8(
                            __value,
                        )))
                    }
                    fn visit_u16<__E>(
                        self,
                        __value: u16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U16(__value),
                        ))
                    }
                    fn visit_u32<__E>(
                        self,
                        __value: u32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U32(__value),
                        ))
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U64(__value),
                        ))
                    }
                    fn visit_f32<__E>(
                        self,
                        __value: f32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::F32(__value),
                        ))
                    }
                    fn visit_f64<__E>(
                        self,
                        __value: f64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::F64(__value),
                        ))
                    }
                    fn visit_char<__E>(
                        self,
                        __value: char,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Char(__value),
                        ))
                    }
                    fn visit_unit<__E>(self) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Unit,
                        ))
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                let __value = _serde::__private::de::Content::String(
                                    _serde::__private::ToString::to_string(__value),
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                let __value =
                                    _serde::__private::de::Content::ByteBuf(__value.to_vec());
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_str<__E>(
                        self,
                        __value: &'de str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                let __value = _serde::__private::de::Content::Str(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_bytes<__E>(
                        self,
                        __value: &'de [u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                let __value = _serde::__private::de::Content::Bytes(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field<'de> {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<MarkingDefinition>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = MarkingDefinition;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct MarkingDefinition",
                        )
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<Option<String>> =
                            _serde::__private::None;
                        let mut __collect = _serde::__private::Vec::<
                            _serde::__private::Option<(
                                _serde::__private::de::Content,
                                _serde::__private::de::Content,
                            )>,
                        >::new();
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__other(__name) => {
                                    __collect.push(_serde::__private::Some((
                                        __name,
                                        match _serde::de::MapAccess::next_value(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    )));
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field0: CommonProperties = match _serde::de::Deserialize::deserialize(
                            _serde::__private::de::FlatMapDeserializer(
                                &mut __collect,
                                _serde::__private::PhantomData,
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(MarkingDefinition {
                            common: __field0,
                            name: __field1,
                        })
                    }
                }
                _serde::Deserializer::deserialize_map(
                    __deserializer,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<MarkingDefinition>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl crate::TypedObject for MarkingDefinition {
        const TYPE: &'static str = "marking-definition";
    }
    impl AsRef<CommonProperties> for MarkingDefinition {
        fn as_ref(&self) -> &CommonProperties {
            &self.common
        }
    }
}
mod object {
    use std::collections::BTreeSet;
    use chrono::{DateTime, Utc};
    use serde::Deserialize;
    use crate::{reference::ExternalReference, Id};
    pub struct CommonProperties {
        id: Id,
        created_by_ref: Option<Id>,
        #[serde(default)]
        revoked: bool,
        #[serde(default)]
        labels: BTreeSet<String>,
        #[serde(default)]
        object_marking_refs: BTreeSet<Id>,
        #[serde(default)]
        external_references: Vec<ExternalReference>,
        #[serde(default)]
        created: Option<DateTime<Utc>>,
        #[serde(default)]
        modified: Option<DateTime<Utc>>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for CommonProperties {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            6u64 => _serde::__private::Ok(__Field::__field6),
                            7u64 => _serde::__private::Ok(__Field::__field7),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "id" => _serde::__private::Ok(__Field::__field0),
                            "created_by_ref" => _serde::__private::Ok(__Field::__field1),
                            "revoked" => _serde::__private::Ok(__Field::__field2),
                            "labels" => _serde::__private::Ok(__Field::__field3),
                            "object_marking_refs" => _serde::__private::Ok(__Field::__field4),
                            "external_references" => _serde::__private::Ok(__Field::__field5),
                            "created" => _serde::__private::Ok(__Field::__field6),
                            "modified" => _serde::__private::Ok(__Field::__field7),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"id" => _serde::__private::Ok(__Field::__field0),
                            b"created_by_ref" => _serde::__private::Ok(__Field::__field1),
                            b"revoked" => _serde::__private::Ok(__Field::__field2),
                            b"labels" => _serde::__private::Ok(__Field::__field3),
                            b"object_marking_refs" => _serde::__private::Ok(__Field::__field4),
                            b"external_references" => _serde::__private::Ok(__Field::__field5),
                            b"created" => _serde::__private::Ok(__Field::__field6),
                            b"modified" => _serde::__private::Ok(__Field::__field7),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<CommonProperties>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = CommonProperties;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct CommonProperties",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<Id>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct CommonProperties with 8 elements",
                                        ),
                                    );
                                }
                            };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<Option<Id>>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct CommonProperties with 8 elements",
                                ));
                            }
                        };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => _serde::__private::Default::default(),
                            };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            BTreeSet<String>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field4 = match match _serde::de::SeqAccess::next_element::<BTreeSet<Id>>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field5 = match match _serde::de::SeqAccess::next_element::<
                            Vec<ExternalReference>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field6 = match match _serde::de::SeqAccess::next_element::<
                            Option<DateTime<Utc>>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field7 = match match _serde::de::SeqAccess::next_element::<
                            Option<DateTime<Utc>>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        _serde::__private::Ok(CommonProperties {
                            id: __field0,
                            created_by_ref: __field1,
                            revoked: __field2,
                            labels: __field3,
                            object_marking_refs: __field4,
                            external_references: __field5,
                            created: __field6,
                            modified: __field7,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Id> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Option<Id>> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<BTreeSet<String>> =
                            _serde::__private::None;
                        let mut __field4: _serde::__private::Option<BTreeSet<Id>> =
                            _serde::__private::None;
                        let mut __field5: _serde::__private::Option<Vec<ExternalReference>> =
                            _serde::__private::None;
                        let mut __field6: _serde::__private::Option<Option<DateTime<Utc>>> =
                            _serde::__private::None;
                        let mut __field7: _serde::__private::Option<Option<DateTime<Utc>>> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "id",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Id>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "created_by_ref",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<Id>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "revoked",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "labels",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<BTreeSet<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "object_marking_refs",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<BTreeSet<Id>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "external_references",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Vec<ExternalReference>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::__private::Option::is_some(&__field6) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "created",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<DateTime<Utc>>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field7 => {
                                    if _serde::__private::Option::is_some(&__field7) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "modified",
                                            ),
                                        );
                                    }
                                    __field7 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<DateTime<Utc>>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("id") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("created_by_ref") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field6 = match __field6 {
                            _serde::__private::Some(__field6) => __field6,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field7 = match __field7 {
                            _serde::__private::Some(__field7) => __field7,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        _serde::__private::Ok(CommonProperties {
                            id: __field0,
                            created_by_ref: __field1,
                            revoked: __field2,
                            labels: __field3,
                            object_marking_refs: __field4,
                            external_references: __field5,
                            created: __field6,
                            modified: __field7,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &[
                    "id",
                    "created_by_ref",
                    "revoked",
                    "labels",
                    "object_marking_refs",
                    "external_references",
                    "created",
                    "modified",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "CommonProperties",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<CommonProperties>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc = " Common properties for a STIX Domain Object."]
    pub trait Object {
        fn id(&self) -> &Id;
        fn created_by_ref(&self) -> Option<&Id>;
        fn revoked(&self) -> bool;
        fn labels(&self) -> &BTreeSet<String>;
        fn object_marking_refs(&self) -> &BTreeSet<Id>;
        fn external_references(&self) -> &[ExternalReference];
        fn created(&self) -> Option<&DateTime<Utc>>;
        fn modified(&self) -> Option<&DateTime<Utc>>;
    }
    impl Object for CommonProperties {
        fn id(&self) -> &Id {
            &self.id
        }
        fn created_by_ref(&self) -> Option<&Id> {
            self.created_by_ref.as_ref()
        }
        fn external_references(&self) -> &[ExternalReference] {
            &self.external_references
        }
        fn created(&self) -> Option<&DateTime<Utc>> {
            self.created.as_ref()
        }
        fn modified(&self) -> Option<&DateTime<Utc>> {
            self.modified.as_ref()
        }
        fn revoked(&self) -> bool {
            self.revoked
        }
        fn labels(&self) -> &BTreeSet<String> {
            &self.labels
        }
        fn object_marking_refs(&self) -> &BTreeSet<Id> {
            &self.object_marking_refs
        }
    }
    impl<T: AsRef<CommonProperties>> Object for T {
        fn id(&self) -> &Id {
            self.as_ref().id()
        }
        fn created_by_ref(&self) -> Option<&Id> {
            self.as_ref().created_by_ref()
        }
        fn revoked(&self) -> bool {
            self.as_ref().revoked()
        }
        fn labels(&self) -> &BTreeSet<String> {
            self.as_ref().labels()
        }
        fn object_marking_refs(&self) -> &BTreeSet<Id> {
            self.as_ref().object_marking_refs()
        }
        fn external_references(&self) -> &[ExternalReference] {
            self.as_ref().external_references()
        }
        fn created(&self) -> Option<&DateTime<Utc>> {
            self.as_ref().created()
        }
        fn modified(&self) -> Option<&DateTime<Utc>> {
            self.as_ref().modified()
        }
    }
    #[doc = " A STIX object associated with a specific ID type."]
    #[doc = ""]
    #[doc = " All instances of the struct should have this as their ID."]
    pub trait TypedObject {
        #[doc = " The kebab-case type used as the object's ID prefix and in the `type` field"]
        #[doc = " for declarations of the object."]
        #[doc = ""]
        #[doc = " # Examples"]
        #[doc = " * `course-of-action`"]
        #[doc = " * `intrusion-set`"]
        const TYPE: &'static str;
    }
}
mod reference {
    use serde::{Deserialize, Serialize};
    use url::Url;
    pub struct ExternalReference {
        source_name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        external_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        url: Option<Url>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ExternalReference {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                ExternalReference {
                    source_name: ref __self_0_0,
                    external_id: ref __self_0_1,
                    url: ref __self_0_2,
                    description: ref __self_0_3,
                } => {
                    let mut debug_trait_builder = f.debug_struct("ExternalReference");
                    let _ = debug_trait_builder.field("source_name", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("external_id", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("url", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("description", &&(*__self_0_3));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for ExternalReference {
        #[inline]
        fn clone(&self) -> ExternalReference {
            match *self {
                ExternalReference {
                    source_name: ref __self_0_0,
                    external_id: ref __self_0_1,
                    url: ref __self_0_2,
                    description: ref __self_0_3,
                } => ExternalReference {
                    source_name: ::core::clone::Clone::clone(&(*__self_0_0)),
                    external_id: ::core::clone::Clone::clone(&(*__self_0_1)),
                    url: ::core::clone::Clone::clone(&(*__self_0_2)),
                    description: ::core::clone::Clone::clone(&(*__self_0_3)),
                },
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ExternalReference {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "source_name" => _serde::__private::Ok(__Field::__field0),
                            "external_id" => _serde::__private::Ok(__Field::__field1),
                            "url" => _serde::__private::Ok(__Field::__field2),
                            "description" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"source_name" => _serde::__private::Ok(__Field::__field0),
                            b"external_id" => _serde::__private::Ok(__Field::__field1),
                            b"url" => _serde::__private::Ok(__Field::__field2),
                            b"description" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ExternalReference>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ExternalReference;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct ExternalReference",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct ExternalReference with 4 elements",
                                        ),
                                    );
                                }
                            };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<Option<Url>>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        _serde::__private::Ok(ExternalReference {
                            source_name: __field0,
                            external_id: __field1,
                            url: __field2,
                            description: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Option<String>> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Option<Url>> =
                            _serde::__private::None;
                        let mut __field3: _serde::__private::Option<Option<String>> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "source_name",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "external_id",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "url",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<Url>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "description",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("source_name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        _serde::__private::Ok(ExternalReference {
                            source_name: __field0,
                            external_id: __field1,
                            url: __field2,
                            description: __field3,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["source_name", "external_id", "url", "description"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ExternalReference",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ExternalReference>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ExternalReference {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "ExternalReference",
                    false as usize
                        + 1
                        + if Option::is_none(&self.external_id) {
                            0
                        } else {
                            1
                        }
                        + if Option::is_none(&self.url) { 0 } else { 1 }
                        + if Option::is_none(&self.description) {
                            0
                        } else {
                            1
                        },
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "source_name",
                    &self.source_name,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                if !Option::is_none(&self.external_id) {
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "external_id",
                        &self.external_id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                } else {
                    match _serde::ser::SerializeStruct::skip_field(
                        &mut __serde_state,
                        "external_id",
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                }
                if !Option::is_none(&self.url) {
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "url",
                        &self.url,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                } else {
                    match _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "url") {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                }
                if !Option::is_none(&self.description) {
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "description",
                        &self.description,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                } else {
                    match _serde::ser::SerializeStruct::skip_field(
                        &mut __serde_state,
                        "description",
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                }
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    pub struct KillChainPhase {
        kill_chain_name: String,
        phase_name: String,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for KillChainPhase {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                KillChainPhase {
                    kill_chain_name: ref __self_0_0,
                    phase_name: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("KillChainPhase");
                    let _ = debug_trait_builder.field("kill_chain_name", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("phase_name", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for KillChainPhase {
        #[inline]
        fn clone(&self) -> KillChainPhase {
            match *self {
                KillChainPhase {
                    kill_chain_name: ref __self_0_0,
                    phase_name: ref __self_0_1,
                } => KillChainPhase {
                    kill_chain_name: ::core::clone::Clone::clone(&(*__self_0_0)),
                    phase_name: ::core::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for KillChainPhase {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for KillChainPhase {
        #[inline]
        fn eq(&self, other: &KillChainPhase) -> bool {
            match *other {
                KillChainPhase {
                    kill_chain_name: ref __self_1_0,
                    phase_name: ref __self_1_1,
                } => match *self {
                    KillChainPhase {
                        kill_chain_name: ref __self_0_0,
                        phase_name: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &KillChainPhase) -> bool {
            match *other {
                KillChainPhase {
                    kill_chain_name: ref __self_1_0,
                    phase_name: ref __self_1_1,
                } => match *self {
                    KillChainPhase {
                        kill_chain_name: ref __self_0_0,
                        phase_name: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    impl ::core::marker::StructuralEq for KillChainPhase {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for KillChainPhase {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<String>;
                let _: ::core::cmp::AssertParamIsEq<String>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::hash::Hash for KillChainPhase {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                KillChainPhase {
                    kill_chain_name: ref __self_0_0,
                    phase_name: ref __self_0_1,
                } => {
                    ::core::hash::Hash::hash(&(*__self_0_0), state);
                    ::core::hash::Hash::hash(&(*__self_0_1), state)
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for KillChainPhase {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "kill_chain_name" => _serde::__private::Ok(__Field::__field0),
                            "phase_name" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"kill_chain_name" => _serde::__private::Ok(__Field::__field0),
                            b"phase_name" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<KillChainPhase>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = KillChainPhase;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct KillChainPhase",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct KillChainPhase with 2 elements",
                                        ),
                                    );
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct KillChainPhase with 2 elements",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(KillChainPhase {
                            kill_chain_name: __field0,
                            phase_name: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<String> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "kill_chain_name",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "phase_name",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("kill_chain_name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("phase_name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(KillChainPhase {
                            kill_chain_name: __field0,
                            phase_name: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["kill_chain_name", "phase_name"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "KillChainPhase",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<KillChainPhase>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for KillChainPhase {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "KillChainPhase",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "kill_chain_name",
                    &self.kill_chain_name,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "phase_name",
                    &self.phase_name,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    impl KillChainPhase {
        #[doc = " Create a new phase reference to the `mitre-attack` kill chain."]
        pub fn mitre(phase_name: impl Into<String>) -> Self {
            Self {
                kill_chain_name: "mitre-attack".into(),
                phase_name: phase_name.into(),
            }
        }
    }
}
pub mod relationship {
    use std::borrow::Cow;
    use petgraph::EdgeDirection;
    use serde::Deserialize;
    use crate::{CommonProperties, Id, TypedObject};
    #[non_exhaustive]
    #[strum(serialize_all = "kebab-case")]
    #[serde(rename_all = "kebab-case")]
    pub enum RelationshipType {
        AttributedTo,
        AuthoredBy,
        BeaconsTo,
        Compromises,
        Controls,
        Delivers,
        DerivedFrom,
        Downloads,
        Drops,
        DuplicateOf,
        ExfiltratesTo,
        Has,
        Hosts,
        Impersonates,
        Indicates,
        LocatedAt,
        Mitigates,
        OriginatesFrom,
        Owns,
        Remediates,
        RevokedBy,
        SubtechniqueOf,
        Targets,
        Uses,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for RelationshipType {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&RelationshipType::AttributedTo,) => {
                    let mut debug_trait_builder = f.debug_tuple("AttributedTo");
                    debug_trait_builder.finish()
                }
                (&RelationshipType::AuthoredBy,) => {
                    let mut debug_trait_builder = f.debug_tuple("AuthoredBy");
                    debug_trait_builder.finish()
                }
                (&RelationshipType::BeaconsTo,) => {
                    let mut debug_trait_builder = f.debug_tuple("BeaconsTo");
                    debug_trait_builder.finish()
                }
                (&RelationshipType::Compromises,) => {
                    let mut debug_trait_builder = f.debug_tuple("Compromises");
                    debug_trait_builder.finish()
                }
                (&RelationshipType::Controls,) => {
                    let mut debug_trait_builder = f.debug_tuple("Controls");
                    debug_trait_builder.finish()
                }
                (&RelationshipType::Delivers,) => {
                    let mut debug_trait_builder = f.debug_tuple("Delivers");
                    debug_trait_builder.finish()
                }
                (&RelationshipType::DerivedFrom,) => {
                    let mut debug_trait_builder = f.debug_tuple("DerivedFrom");
                    debug_trait_builder.finish()
                }
                (&RelationshipType::Downloads,) => {
                    let mut debug_trait_builder = f.debug_tuple("Downloads");
                    debug_trait_builder.finish()
                }
                (&RelationshipType::Drops,) => {
                    let mut debug_trait_builder = f.debug_tuple("Drops");
                    debug_trait_builder.finish()
                }
                (&RelationshipType::DuplicateOf,) => {
                    let mut debug_trait_builder = f.debug_tuple("DuplicateOf");
                    debug_trait_builder.finish()
                }
                (&RelationshipType::ExfiltratesTo,) => {
                    let mut debug_trait_builder = f.debug_tuple("ExfiltratesTo");
                    debug_trait_builder.finish()
                }
                (&RelationshipType::Has,) => {
                    let mut debug_trait_builder = f.debug_tuple("Has");
                    debug_trait_builder.finish()
                }
                (&RelationshipType::Hosts,) => {
                    let mut debug_trait_builder = f.debug_tuple("Hosts");
                    debug_trait_builder.finish()
                }
                (&RelationshipType::Impersonates,) => {
                    let mut debug_trait_builder = f.debug_tuple("Impersonates");
                    debug_trait_builder.finish()
                }
                (&RelationshipType::Indicates,) => {
                    let mut debug_trait_builder = f.debug_tuple("Indicates");
                    debug_trait_builder.finish()
                }
                (&RelationshipType::LocatedAt,) => {
                    let mut debug_trait_builder = f.debug_tuple("LocatedAt");
                    debug_trait_builder.finish()
                }
                (&RelationshipType::Mitigates,) => {
                    let mut debug_trait_builder = f.debug_tuple("Mitigates");
                    debug_trait_builder.finish()
                }
                (&RelationshipType::OriginatesFrom,) => {
                    let mut debug_trait_builder = f.debug_tuple("OriginatesFrom");
                    debug_trait_builder.finish()
                }
                (&RelationshipType::Owns,) => {
                    let mut debug_trait_builder = f.debug_tuple("Owns");
                    debug_trait_builder.finish()
                }
                (&RelationshipType::Remediates,) => {
                    let mut debug_trait_builder = f.debug_tuple("Remediates");
                    debug_trait_builder.finish()
                }
                (&RelationshipType::RevokedBy,) => {
                    let mut debug_trait_builder = f.debug_tuple("RevokedBy");
                    debug_trait_builder.finish()
                }
                (&RelationshipType::SubtechniqueOf,) => {
                    let mut debug_trait_builder = f.debug_tuple("SubtechniqueOf");
                    debug_trait_builder.finish()
                }
                (&RelationshipType::Targets,) => {
                    let mut debug_trait_builder = f.debug_tuple("Targets");
                    debug_trait_builder.finish()
                }
                (&RelationshipType::Uses,) => {
                    let mut debug_trait_builder = f.debug_tuple("Uses");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for RelationshipType {
        #[inline]
        fn clone(&self) -> RelationshipType {
            match (&*self,) {
                (&RelationshipType::AttributedTo,) => RelationshipType::AttributedTo,
                (&RelationshipType::AuthoredBy,) => RelationshipType::AuthoredBy,
                (&RelationshipType::BeaconsTo,) => RelationshipType::BeaconsTo,
                (&RelationshipType::Compromises,) => RelationshipType::Compromises,
                (&RelationshipType::Controls,) => RelationshipType::Controls,
                (&RelationshipType::Delivers,) => RelationshipType::Delivers,
                (&RelationshipType::DerivedFrom,) => RelationshipType::DerivedFrom,
                (&RelationshipType::Downloads,) => RelationshipType::Downloads,
                (&RelationshipType::Drops,) => RelationshipType::Drops,
                (&RelationshipType::DuplicateOf,) => RelationshipType::DuplicateOf,
                (&RelationshipType::ExfiltratesTo,) => RelationshipType::ExfiltratesTo,
                (&RelationshipType::Has,) => RelationshipType::Has,
                (&RelationshipType::Hosts,) => RelationshipType::Hosts,
                (&RelationshipType::Impersonates,) => RelationshipType::Impersonates,
                (&RelationshipType::Indicates,) => RelationshipType::Indicates,
                (&RelationshipType::LocatedAt,) => RelationshipType::LocatedAt,
                (&RelationshipType::Mitigates,) => RelationshipType::Mitigates,
                (&RelationshipType::OriginatesFrom,) => RelationshipType::OriginatesFrom,
                (&RelationshipType::Owns,) => RelationshipType::Owns,
                (&RelationshipType::Remediates,) => RelationshipType::Remediates,
                (&RelationshipType::RevokedBy,) => RelationshipType::RevokedBy,
                (&RelationshipType::SubtechniqueOf,) => RelationshipType::SubtechniqueOf,
                (&RelationshipType::Targets,) => RelationshipType::Targets,
                (&RelationshipType::Uses,) => RelationshipType::Uses,
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for RelationshipType {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for RelationshipType {
        #[inline]
        fn eq(&self, other: &RelationshipType) -> bool {
            {
                let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) };
                let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) };
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
    impl ::core::marker::StructuralEq for RelationshipType {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for RelationshipType {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::hash::Hash for RelationshipType {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match (&*self,) {
                _ => ::core::hash::Hash::hash(
                    &unsafe { ::core::intrinsics::discriminant_value(self) },
                    state,
                ),
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for RelationshipType {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                    __field9,
                    __field10,
                    __field11,
                    __field12,
                    __field13,
                    __field14,
                    __field15,
                    __field16,
                    __field17,
                    __field18,
                    __field19,
                    __field20,
                    __field21,
                    __field22,
                    __field23,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "variant identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            6u64 => _serde::__private::Ok(__Field::__field6),
                            7u64 => _serde::__private::Ok(__Field::__field7),
                            8u64 => _serde::__private::Ok(__Field::__field8),
                            9u64 => _serde::__private::Ok(__Field::__field9),
                            10u64 => _serde::__private::Ok(__Field::__field10),
                            11u64 => _serde::__private::Ok(__Field::__field11),
                            12u64 => _serde::__private::Ok(__Field::__field12),
                            13u64 => _serde::__private::Ok(__Field::__field13),
                            14u64 => _serde::__private::Ok(__Field::__field14),
                            15u64 => _serde::__private::Ok(__Field::__field15),
                            16u64 => _serde::__private::Ok(__Field::__field16),
                            17u64 => _serde::__private::Ok(__Field::__field17),
                            18u64 => _serde::__private::Ok(__Field::__field18),
                            19u64 => _serde::__private::Ok(__Field::__field19),
                            20u64 => _serde::__private::Ok(__Field::__field20),
                            21u64 => _serde::__private::Ok(__Field::__field21),
                            22u64 => _serde::__private::Ok(__Field::__field22),
                            23u64 => _serde::__private::Ok(__Field::__field23),
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"variant index 0 <= i < 24",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "attributed-to" => _serde::__private::Ok(__Field::__field0),
                            "authored-by" => _serde::__private::Ok(__Field::__field1),
                            "beacons-to" => _serde::__private::Ok(__Field::__field2),
                            "compromises" => _serde::__private::Ok(__Field::__field3),
                            "controls" => _serde::__private::Ok(__Field::__field4),
                            "delivers" => _serde::__private::Ok(__Field::__field5),
                            "derived-from" => _serde::__private::Ok(__Field::__field6),
                            "downloads" => _serde::__private::Ok(__Field::__field7),
                            "drops" => _serde::__private::Ok(__Field::__field8),
                            "duplicate-of" => _serde::__private::Ok(__Field::__field9),
                            "exfiltrates-to" => _serde::__private::Ok(__Field::__field10),
                            "has" => _serde::__private::Ok(__Field::__field11),
                            "hosts" => _serde::__private::Ok(__Field::__field12),
                            "impersonates" => _serde::__private::Ok(__Field::__field13),
                            "indicates" => _serde::__private::Ok(__Field::__field14),
                            "located-at" => _serde::__private::Ok(__Field::__field15),
                            "mitigates" => _serde::__private::Ok(__Field::__field16),
                            "originates-from" => _serde::__private::Ok(__Field::__field17),
                            "owns" => _serde::__private::Ok(__Field::__field18),
                            "remediates" => _serde::__private::Ok(__Field::__field19),
                            "revoked-by" => _serde::__private::Ok(__Field::__field20),
                            "subtechnique-of" => _serde::__private::Ok(__Field::__field21),
                            "targets" => _serde::__private::Ok(__Field::__field22),
                            "uses" => _serde::__private::Ok(__Field::__field23),
                            _ => _serde::__private::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"attributed-to" => _serde::__private::Ok(__Field::__field0),
                            b"authored-by" => _serde::__private::Ok(__Field::__field1),
                            b"beacons-to" => _serde::__private::Ok(__Field::__field2),
                            b"compromises" => _serde::__private::Ok(__Field::__field3),
                            b"controls" => _serde::__private::Ok(__Field::__field4),
                            b"delivers" => _serde::__private::Ok(__Field::__field5),
                            b"derived-from" => _serde::__private::Ok(__Field::__field6),
                            b"downloads" => _serde::__private::Ok(__Field::__field7),
                            b"drops" => _serde::__private::Ok(__Field::__field8),
                            b"duplicate-of" => _serde::__private::Ok(__Field::__field9),
                            b"exfiltrates-to" => _serde::__private::Ok(__Field::__field10),
                            b"has" => _serde::__private::Ok(__Field::__field11),
                            b"hosts" => _serde::__private::Ok(__Field::__field12),
                            b"impersonates" => _serde::__private::Ok(__Field::__field13),
                            b"indicates" => _serde::__private::Ok(__Field::__field14),
                            b"located-at" => _serde::__private::Ok(__Field::__field15),
                            b"mitigates" => _serde::__private::Ok(__Field::__field16),
                            b"originates-from" => _serde::__private::Ok(__Field::__field17),
                            b"owns" => _serde::__private::Ok(__Field::__field18),
                            b"remediates" => _serde::__private::Ok(__Field::__field19),
                            b"revoked-by" => _serde::__private::Ok(__Field::__field20),
                            b"subtechnique-of" => _serde::__private::Ok(__Field::__field21),
                            b"targets" => _serde::__private::Ok(__Field::__field22),
                            b"uses" => _serde::__private::Ok(__Field::__field23),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<RelationshipType>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = RelationshipType;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum RelationshipType",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match match _serde::de::EnumAccess::variant(__data) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            (__Field::__field0, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(RelationshipType::AttributedTo)
                            }
                            (__Field::__field1, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(RelationshipType::AuthoredBy)
                            }
                            (__Field::__field2, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(RelationshipType::BeaconsTo)
                            }
                            (__Field::__field3, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(RelationshipType::Compromises)
                            }
                            (__Field::__field4, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(RelationshipType::Controls)
                            }
                            (__Field::__field5, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(RelationshipType::Delivers)
                            }
                            (__Field::__field6, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(RelationshipType::DerivedFrom)
                            }
                            (__Field::__field7, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(RelationshipType::Downloads)
                            }
                            (__Field::__field8, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(RelationshipType::Drops)
                            }
                            (__Field::__field9, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(RelationshipType::DuplicateOf)
                            }
                            (__Field::__field10, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(RelationshipType::ExfiltratesTo)
                            }
                            (__Field::__field11, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(RelationshipType::Has)
                            }
                            (__Field::__field12, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(RelationshipType::Hosts)
                            }
                            (__Field::__field13, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(RelationshipType::Impersonates)
                            }
                            (__Field::__field14, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(RelationshipType::Indicates)
                            }
                            (__Field::__field15, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(RelationshipType::LocatedAt)
                            }
                            (__Field::__field16, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(RelationshipType::Mitigates)
                            }
                            (__Field::__field17, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(RelationshipType::OriginatesFrom)
                            }
                            (__Field::__field18, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(RelationshipType::Owns)
                            }
                            (__Field::__field19, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(RelationshipType::Remediates)
                            }
                            (__Field::__field20, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(RelationshipType::RevokedBy)
                            }
                            (__Field::__field21, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(RelationshipType::SubtechniqueOf)
                            }
                            (__Field::__field22, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(RelationshipType::Targets)
                            }
                            (__Field::__field23, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(RelationshipType::Uses)
                            }
                        }
                    }
                }
                const VARIANTS: &'static [&'static str] = &[
                    "attributed-to",
                    "authored-by",
                    "beacons-to",
                    "compromises",
                    "controls",
                    "delivers",
                    "derived-from",
                    "downloads",
                    "drops",
                    "duplicate-of",
                    "exfiltrates-to",
                    "has",
                    "hosts",
                    "impersonates",
                    "indicates",
                    "located-at",
                    "mitigates",
                    "originates-from",
                    "owns",
                    "remediates",
                    "revoked-by",
                    "subtechnique-of",
                    "targets",
                    "uses",
                ];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "RelationshipType",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<RelationshipType>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl ::std::fmt::Display for RelationshipType {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter,
        ) -> ::std::result::Result<(), ::std::fmt::Error> {
            match *self {
                RelationshipType::AttributedTo => f.pad("attributed-to"),
                RelationshipType::AuthoredBy => f.pad("authored-by"),
                RelationshipType::BeaconsTo => f.pad("beacons-to"),
                RelationshipType::Compromises => f.pad("compromises"),
                RelationshipType::Controls => f.pad("controls"),
                RelationshipType::Delivers => f.pad("delivers"),
                RelationshipType::DerivedFrom => f.pad("derived-from"),
                RelationshipType::Downloads => f.pad("downloads"),
                RelationshipType::Drops => f.pad("drops"),
                RelationshipType::DuplicateOf => f.pad("duplicate-of"),
                RelationshipType::ExfiltratesTo => f.pad("exfiltrates-to"),
                RelationshipType::Has => f.pad("has"),
                RelationshipType::Hosts => f.pad("hosts"),
                RelationshipType::Impersonates => f.pad("impersonates"),
                RelationshipType::Indicates => f.pad("indicates"),
                RelationshipType::LocatedAt => f.pad("located-at"),
                RelationshipType::Mitigates => f.pad("mitigates"),
                RelationshipType::OriginatesFrom => f.pad("originates-from"),
                RelationshipType::Owns => f.pad("owns"),
                RelationshipType::Remediates => f.pad("remediates"),
                RelationshipType::RevokedBy => f.pad("revoked-by"),
                RelationshipType::SubtechniqueOf => f.pad("subtechnique-of"),
                RelationshipType::Targets => f.pad("targets"),
                RelationshipType::Uses => f.pad("uses"),
            }
        }
    }
    #[typed_object(core)]
    pub struct Relationship {
        #[serde(flatten)]
        base: CommonProperties,
        pub source_ref: Id,
        pub target_ref: Id,
        pub relationship_type: RelationshipType,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Relationship {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field<'de> {
                    __field1,
                    __field2,
                    __field3,
                    __other(_serde::__private::de::Content<'de>),
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field<'de>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_bool<__E>(
                        self,
                        __value: bool,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Bool(__value),
                        ))
                    }
                    fn visit_i8<__E>(
                        self,
                        __value: i8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(_serde::__private::de::Content::I8(
                            __value,
                        )))
                    }
                    fn visit_i16<__E>(
                        self,
                        __value: i16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I16(__value),
                        ))
                    }
                    fn visit_i32<__E>(
                        self,
                        __value: i32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I32(__value),
                        ))
                    }
                    fn visit_i64<__E>(
                        self,
                        __value: i64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I64(__value),
                        ))
                    }
                    fn visit_u8<__E>(
                        self,
                        __value: u8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(_serde::__private::de::Content::U8(
                            __value,
                        )))
                    }
                    fn visit_u16<__E>(
                        self,
                        __value: u16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U16(__value),
                        ))
                    }
                    fn visit_u32<__E>(
                        self,
                        __value: u32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U32(__value),
                        ))
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U64(__value),
                        ))
                    }
                    fn visit_f32<__E>(
                        self,
                        __value: f32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::F32(__value),
                        ))
                    }
                    fn visit_f64<__E>(
                        self,
                        __value: f64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::F64(__value),
                        ))
                    }
                    fn visit_char<__E>(
                        self,
                        __value: char,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Char(__value),
                        ))
                    }
                    fn visit_unit<__E>(self) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Unit,
                        ))
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "source_ref" => _serde::__private::Ok(__Field::__field1),
                            "target_ref" => _serde::__private::Ok(__Field::__field2),
                            "relationship_type" => _serde::__private::Ok(__Field::__field3),
                            _ => {
                                let __value = _serde::__private::de::Content::String(
                                    _serde::__private::ToString::to_string(__value),
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"source_ref" => _serde::__private::Ok(__Field::__field1),
                            b"target_ref" => _serde::__private::Ok(__Field::__field2),
                            b"relationship_type" => _serde::__private::Ok(__Field::__field3),
                            _ => {
                                let __value =
                                    _serde::__private::de::Content::ByteBuf(__value.to_vec());
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_str<__E>(
                        self,
                        __value: &'de str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "source_ref" => _serde::__private::Ok(__Field::__field1),
                            "target_ref" => _serde::__private::Ok(__Field::__field2),
                            "relationship_type" => _serde::__private::Ok(__Field::__field3),
                            _ => {
                                let __value = _serde::__private::de::Content::Str(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_bytes<__E>(
                        self,
                        __value: &'de [u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"source_ref" => _serde::__private::Ok(__Field::__field1),
                            b"target_ref" => _serde::__private::Ok(__Field::__field2),
                            b"relationship_type" => _serde::__private::Ok(__Field::__field3),
                            _ => {
                                let __value = _serde::__private::de::Content::Bytes(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field<'de> {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Relationship>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Relationship;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct Relationship")
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<Id> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Id> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<RelationshipType> =
                            _serde::__private::None;
                        let mut __collect = _serde::__private::Vec::<
                            _serde::__private::Option<(
                                _serde::__private::de::Content,
                                _serde::__private::de::Content,
                            )>,
                        >::new();
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "source_ref",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Id>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "target_ref",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Id>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "relationship_type",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<RelationshipType>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__other(__name) => {
                                    __collect.push(_serde::__private::Some((
                                        __name,
                                        match _serde::de::MapAccess::next_value(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    )));
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("source_ref") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("target_ref") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("relationship_type") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field0: CommonProperties = match _serde::de::Deserialize::deserialize(
                            _serde::__private::de::FlatMapDeserializer(
                                &mut __collect,
                                _serde::__private::PhantomData,
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(Relationship {
                            base: __field0,
                            source_ref: __field1,
                            target_ref: __field2,
                            relationship_type: __field3,
                        })
                    }
                }
                _serde::Deserializer::deserialize_map(
                    __deserializer,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Relationship>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl crate::TypedObject for Relationship {
        const TYPE: &'static str = "relationship";
    }
    impl AsRef<CommonProperties> for Relationship {
        fn as_ref(&self) -> &CommonProperties {
            &self.base
        }
    }
    pub struct Filter {
        pub(crate) direction: EdgeDirection,
        pub(crate) relationship_type: RelationshipType,
        pub(crate) peer_type: Cow<'static, str>,
    }
    impl Filter {
        pub fn outgoing<Peer: TypedObject>(relationship_type: RelationshipType) -> Self {
            Self {
                direction: EdgeDirection::Outgoing,
                relationship_type,
                peer_type: Cow::Borrowed(Peer::TYPE),
            }
        }
        pub fn incoming<Peer: TypedObject>(relationship_type: RelationshipType) -> Self {
            Filter {
                direction: EdgeDirection::Incoming,
                relationship_type,
                peer_type: Cow::Borrowed(Peer::TYPE),
            }
        }
    }
    impl Filter {
        pub fn matches(&self, rel: &Relationship) -> bool {
            let peer = match self.direction {
                EdgeDirection::Outgoing => &rel.target_ref,
                EdgeDirection::Incoming => &rel.source_ref,
            };
            rel.relationship_type == self.relationship_type && peer.object_type() == self.peer_type
        }
    }
}
mod relationship_graph {
    use std::{collections::HashMap, iter::FromIterator};
    use petgraph::{
        graph::{EdgeReference, NodeIndex},
        EdgeDirection, Graph,
    };
    use crate::{relationship, Id, Relationship};
    pub struct RelationshipGraph<'a> {
        object_indices: HashMap<&'a Id, NodeIndex>,
        graph: Graph<&'a Id, &'a Relationship>,
    }
    impl<'a> RelationshipGraph<'a> {
        fn node_index(&self, id: &Id) -> Option<NodeIndex> {
            self.object_indices.get(id).copied()
        }
        fn edges_directed(
            &'a self,
            id: &Id,
            dir: EdgeDirection,
        ) -> impl Iterator<Item = &'a Relationship> {
            match self.node_index(id) {
                None => EdgeIter::Empty,
                Some(idx) => EdgeIter::Edges(self.graph.edges_directed(idx, dir)),
            }
        }
        pub fn peers_matching(
            &'a self,
            id: &Id,
            filter: relationship::Filter,
        ) -> impl Iterator<Item = &'a Id> {
            self.edges_directed(id, filter.direction)
                .filter_map(move |d| {
                    if filter.matches(d) {
                        match filter.direction {
                            EdgeDirection::Incoming => Some(&d.source_ref),
                            EdgeDirection::Outgoing => Some(&d.target_ref),
                        }
                    } else {
                        None
                    }
                })
        }
    }
    impl<'a> FromIterator<&'a Relationship> for RelationshipGraph<'a> {
        fn from_iter<T: IntoIterator<Item = &'a Relationship>>(iter: T) -> Self {
            let mut object_indices = HashMap::new();
            let mut graph = Graph::new();
            for relationship in iter {
                let source_idx = *object_indices
                    .entry(&relationship.source_ref)
                    .or_insert_with(|| graph.add_node(&relationship.source_ref));
                let target_idx = *object_indices
                    .entry(&relationship.target_ref)
                    .or_insert_with(|| graph.add_node(&relationship.target_ref));
                graph.add_edge(source_idx, target_idx, relationship);
            }
            Self {
                object_indices,
                graph,
            }
        }
    }
    enum EdgeIter<E> {
        Empty,
        Edges(E),
    }
    impl<'a, E: Iterator<Item = EdgeReference<'a, &'a Relationship>>> Iterator for EdgeIter<E> {
        type Item = &'a Relationship;
        fn next(&mut self) -> Option<Self::Item> {
            match self {
                EdgeIter::Empty => None,
                EdgeIter::Edges(ref mut edges) => edges.next().map(|e| *e.weight()),
            }
        }
    }
}
pub mod standard {
    #![doc = " Types for build a collection of STIX 2.1 objects with no user-defined object types,"]
    #![doc = " fields, or relationships."]
    use serde::Deserialize;
    #[non_exhaustive]
    #[serde(tag = "type", rename_all = "kebab-case")]
    pub enum Declaration {
        AttackPattern(crate::AttackPattern),
        Campaign(crate::Campaign),
        CourseOfAction(crate::CourseOfAction),
        Identity(crate::Identity),
        IntrusionSet(crate::IntrusionSet),
        Infrastructure(crate::Infrastructure),
        Location(crate::Location),
        Malware(crate::Malware),
        MarkingDefinition(crate::MarkingDefinition),
        Relationship(crate::Relationship),
        ThreatActor(crate::ThreatActor),
        Tool(crate::Tool),
        Vulnerability(crate::Vulnerability),
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Declaration {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                    __field9,
                    __field10,
                    __field11,
                    __field12,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "variant identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            6u64 => _serde::__private::Ok(__Field::__field6),
                            7u64 => _serde::__private::Ok(__Field::__field7),
                            8u64 => _serde::__private::Ok(__Field::__field8),
                            9u64 => _serde::__private::Ok(__Field::__field9),
                            10u64 => _serde::__private::Ok(__Field::__field10),
                            11u64 => _serde::__private::Ok(__Field::__field11),
                            12u64 => _serde::__private::Ok(__Field::__field12),
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"variant index 0 <= i < 13",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "attack-pattern" => _serde::__private::Ok(__Field::__field0),
                            "campaign" => _serde::__private::Ok(__Field::__field1),
                            "course-of-action" => _serde::__private::Ok(__Field::__field2),
                            "identity" => _serde::__private::Ok(__Field::__field3),
                            "intrusion-set" => _serde::__private::Ok(__Field::__field4),
                            "infrastructure" => _serde::__private::Ok(__Field::__field5),
                            "location" => _serde::__private::Ok(__Field::__field6),
                            "malware" => _serde::__private::Ok(__Field::__field7),
                            "marking-definition" => _serde::__private::Ok(__Field::__field8),
                            "relationship" => _serde::__private::Ok(__Field::__field9),
                            "threat-actor" => _serde::__private::Ok(__Field::__field10),
                            "tool" => _serde::__private::Ok(__Field::__field11),
                            "vulnerability" => _serde::__private::Ok(__Field::__field12),
                            _ => _serde::__private::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"attack-pattern" => _serde::__private::Ok(__Field::__field0),
                            b"campaign" => _serde::__private::Ok(__Field::__field1),
                            b"course-of-action" => _serde::__private::Ok(__Field::__field2),
                            b"identity" => _serde::__private::Ok(__Field::__field3),
                            b"intrusion-set" => _serde::__private::Ok(__Field::__field4),
                            b"infrastructure" => _serde::__private::Ok(__Field::__field5),
                            b"location" => _serde::__private::Ok(__Field::__field6),
                            b"malware" => _serde::__private::Ok(__Field::__field7),
                            b"marking-definition" => _serde::__private::Ok(__Field::__field8),
                            b"relationship" => _serde::__private::Ok(__Field::__field9),
                            b"threat-actor" => _serde::__private::Ok(__Field::__field10),
                            b"tool" => _serde::__private::Ok(__Field::__field11),
                            b"vulnerability" => _serde::__private::Ok(__Field::__field12),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                const VARIANTS: &'static [&'static str] = &[
                    "attack-pattern",
                    "campaign",
                    "course-of-action",
                    "identity",
                    "intrusion-set",
                    "infrastructure",
                    "location",
                    "malware",
                    "marking-definition",
                    "relationship",
                    "threat-actor",
                    "tool",
                    "vulnerability",
                ];
                let __tagged = match _serde::Deserializer::deserialize_any(
                    __deserializer,
                    _serde::__private::de::TaggedContentVisitor::<__Field>::new(
                        "type",
                        "internally tagged enum Declaration",
                    ),
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match __tagged.tag {
                    __Field::__field0 => _serde::__private::Result::map(
                        <crate::AttackPattern as _serde::Deserialize>::deserialize(
                            _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Declaration::AttackPattern,
                    ),
                    __Field::__field1 => _serde::__private::Result::map(
                        <crate::Campaign as _serde::Deserialize>::deserialize(
                            _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Declaration::Campaign,
                    ),
                    __Field::__field2 => _serde::__private::Result::map(
                        <crate::CourseOfAction as _serde::Deserialize>::deserialize(
                            _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Declaration::CourseOfAction,
                    ),
                    __Field::__field3 => _serde::__private::Result::map(
                        <crate::Identity as _serde::Deserialize>::deserialize(
                            _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Declaration::Identity,
                    ),
                    __Field::__field4 => _serde::__private::Result::map(
                        <crate::IntrusionSet as _serde::Deserialize>::deserialize(
                            _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Declaration::IntrusionSet,
                    ),
                    __Field::__field5 => _serde::__private::Result::map(
                        <crate::Infrastructure as _serde::Deserialize>::deserialize(
                            _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Declaration::Infrastructure,
                    ),
                    __Field::__field6 => _serde::__private::Result::map(
                        <crate::Location as _serde::Deserialize>::deserialize(
                            _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Declaration::Location,
                    ),
                    __Field::__field7 => _serde::__private::Result::map(
                        <crate::Malware as _serde::Deserialize>::deserialize(
                            _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Declaration::Malware,
                    ),
                    __Field::__field8 => _serde::__private::Result::map(
                        <crate::MarkingDefinition as _serde::Deserialize>::deserialize(
                            _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Declaration::MarkingDefinition,
                    ),
                    __Field::__field9 => _serde::__private::Result::map(
                        <crate::Relationship as _serde::Deserialize>::deserialize(
                            _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Declaration::Relationship,
                    ),
                    __Field::__field10 => _serde::__private::Result::map(
                        <crate::ThreatActor as _serde::Deserialize>::deserialize(
                            _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Declaration::ThreatActor,
                    ),
                    __Field::__field11 => _serde::__private::Result::map(
                        <crate::Tool as _serde::Deserialize>::deserialize(
                            _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Declaration::Tool,
                    ),
                    __Field::__field12 => _serde::__private::Result::map(
                        <crate::Vulnerability as _serde::Deserialize>::deserialize(
                            _serde::__private::de::ContentDeserializer::<__D::Error>::new(
                                __tagged.content,
                            ),
                        ),
                        Declaration::Vulnerability,
                    ),
                }
            }
        }
    };
    pub struct CollectionBuilder {
        attack_patterns: crate::export::IndexMap<crate::Id, crate::AttackPattern>,
        campaigns: crate::export::IndexMap<crate::Id, crate::Campaign>,
        courses_of_action: crate::export::IndexMap<crate::Id, crate::CourseOfAction>,
        identities: crate::export::IndexMap<crate::Id, crate::Identity>,
        intrusion_sets: crate::export::IndexMap<crate::Id, crate::IntrusionSet>,
        infrastructure: crate::export::IndexMap<crate::Id, crate::Infrastructure>,
        locations: crate::export::IndexMap<crate::Id, crate::Location>,
        malware: crate::export::IndexMap<crate::Id, crate::Malware>,
        marking_definitions: crate::export::IndexMap<crate::Id, crate::MarkingDefinition>,
        relationships: crate::export::IndexMap<crate::Id, crate::Relationship>,
        threat_actors: crate::export::IndexMap<crate::Id, crate::ThreatActor>,
        tools: crate::export::IndexMap<crate::Id, crate::Tool>,
        vulnerabilities: crate::export::IndexMap<crate::Id, crate::Vulnerability>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for CollectionBuilder {
        #[inline]
        fn default() -> CollectionBuilder {
            CollectionBuilder {
                attack_patterns: ::core::default::Default::default(),
                campaigns: ::core::default::Default::default(),
                courses_of_action: ::core::default::Default::default(),
                identities: ::core::default::Default::default(),
                intrusion_sets: ::core::default::Default::default(),
                infrastructure: ::core::default::Default::default(),
                locations: ::core::default::Default::default(),
                malware: ::core::default::Default::default(),
                marking_definitions: ::core::default::Default::default(),
                relationships: ::core::default::Default::default(),
                threat_actors: ::core::default::Default::default(),
                tools: ::core::default::Default::default(),
                vulnerabilities: ::core::default::Default::default(),
            }
        }
    }
    impl CollectionBuilder {
        #[doc = r" Add a bundle to the collection."]
        #[doc = r""]
        #[doc = r" # ID Collisions"]
        #[doc = r" If any of the IDs for objects in `bundle` were already in `self`, they will"]
        #[doc = r" be replaced with the new values."]
        pub fn add_bundle(&mut self, bundle: crate::Bundle<Declaration>) {
            for declaration in bundle.objects {
                match declaration {
                    Declaration::AttackPattern(v) => {
                        self.attack_patterns
                            .insert(crate::Object::id(&v).clone(), v);
                    }
                    Declaration::Campaign(v) => {
                        self.campaigns.insert(crate::Object::id(&v).clone(), v);
                    }
                    Declaration::CourseOfAction(v) => {
                        self.courses_of_action
                            .insert(crate::Object::id(&v).clone(), v);
                    }
                    Declaration::Identity(v) => {
                        self.identities.insert(crate::Object::id(&v).clone(), v);
                    }
                    Declaration::IntrusionSet(v) => {
                        self.intrusion_sets.insert(crate::Object::id(&v).clone(), v);
                    }
                    Declaration::Infrastructure(v) => {
                        self.infrastructure.insert(crate::Object::id(&v).clone(), v);
                    }
                    Declaration::Location(v) => {
                        self.locations.insert(crate::Object::id(&v).clone(), v);
                    }
                    Declaration::Malware(v) => {
                        self.malware.insert(crate::Object::id(&v).clone(), v);
                    }
                    Declaration::MarkingDefinition(v) => {
                        self.marking_definitions
                            .insert(crate::Object::id(&v).clone(), v);
                    }
                    Declaration::Relationship(v) => {
                        self.relationships.insert(crate::Object::id(&v).clone(), v);
                    }
                    Declaration::ThreatActor(v) => {
                        self.threat_actors.insert(crate::Object::id(&v).clone(), v);
                    }
                    Declaration::Tool(v) => {
                        self.tools.insert(crate::Object::id(&v).clone(), v);
                    }
                    Declaration::Vulnerability(v) => {
                        self.vulnerabilities
                            .insert(crate::Object::id(&v).clone(), v);
                    }
                }
            }
        }
        #[doc = r" Finish adding items to the collection and index it for querying."]
        pub fn build(self) -> Collection {
            Collection {
                data: CollectionData::new(self),
            }
        }
    }
    impl<'a> Into<crate::RelationshipGraph<'a>> for &'a CollectionBuilder {
        fn into(self) -> crate::RelationshipGraph<'a> {
            self.relationships.values().collect()
        }
    }
    struct CollectionData {
        unsafe_self_cell: ::once_self_cell::unsafe_once_self_cell::UnsafeOnceSelfCell<
            CollectionBuilder,
            ::once_self_cell::SyncOnceCell,
        >,
    }
    impl CollectionData {
        pub fn new(owner: CollectionBuilder) -> Self {
            Self {
                unsafe_self_cell: unsafe {
                    ::once_self_cell::unsafe_once_self_cell::UnsafeOnceSelfCell::new(owner)
                },
            }
        }
        pub fn get_owner<'a>(&'a self) -> &'a CollectionBuilder {
            unsafe { self.unsafe_self_cell.get_owner() }
        }
        pub fn get_or_init_dependent<'a>(&'a self) -> &'a crate::RelationshipGraph<'_> {
            unsafe {
                self.unsafe_self_cell
                    .get_or_init_dependent(|owner_ref| owner_ref.into())
            }
        }
        pub fn dependent_is_none(&self) -> bool {
            self.unsafe_self_cell.dependent_is_none()
        }
    }
    impl Drop for CollectionData {
        fn drop(&mut self) {
            unsafe {
                self.unsafe_self_cell
                    .drop_dependent::<crate::RelationshipGraph<'_>>();
            }
        }
    }
    #[doc = r" An immutable collection of STIX objects. "]
    #[doc = r""]
    #[doc = r" A collection has no special meaning beyond being a set of STIX objects all "]
    #[doc = r" loaded in memory at once."]
    #[doc = r""]
    #[doc = r" # Usage"]
    #[doc = r" Create a `Collection` by deserializing JSON into [`Bundle`](stix::Bundle) instances"]
    #[doc = r" and then adding those to a `CollectionBuilder`. Once all bundles are loaded, call"]
    #[doc = r" [`CollectionBuilder::build`] to finish the collection, which will index the objects"]
    #[doc = r" and prepare the collection for querying."]
    pub struct Collection {
        data: CollectionData,
    }
    impl Collection {
        #[doc = r" Create a new [`CollectionBuilder`]."]
        pub fn builder() -> CollectionBuilder {
            CollectionBuilder::default()
        }
        #[doc = r" Get the object identified by `id` if it is present in the collection. This function returns a"]
        #[doc = r" [`Node`] which provides access to the object's data and its relationships within the collection."]
        pub fn get<'id, 'a: 'id, D>(&'a self, id: &'id crate::Id) -> Option<Node<'a, D>>
        where
            Ref<'id, 'a, D>: crate::Resolve<Output = Node<'a, D>>,
        {
            crate::Resolve::resolve(Ref::<'id, 'a, D> {
                id,
                collection: self,
                object_type: ::std::marker::PhantomData::<D>,
            })
        }
    }
    #[doc = r" Accessors for STIX objects in the collection."]
    impl Collection {
        pub fn attack_patterns<'a>(
            &'a self,
        ) -> impl Iterator<Item = Node<'a, crate::AttackPattern>> {
            self.data
                .get_owner()
                .attack_patterns
                .values()
                .map(move |v| Node::new(v, self))
        }
        pub fn campaigns<'a>(&'a self) -> impl Iterator<Item = Node<'a, crate::Campaign>> {
            self.data
                .get_owner()
                .campaigns
                .values()
                .map(move |v| Node::new(v, self))
        }
        pub fn courses_of_action<'a>(
            &'a self,
        ) -> impl Iterator<Item = Node<'a, crate::CourseOfAction>> {
            self.data
                .get_owner()
                .courses_of_action
                .values()
                .map(move |v| Node::new(v, self))
        }
        pub fn identities<'a>(&'a self) -> impl Iterator<Item = Node<'a, crate::Identity>> {
            self.data
                .get_owner()
                .identities
                .values()
                .map(move |v| Node::new(v, self))
        }
        pub fn intrusion_sets<'a>(&'a self) -> impl Iterator<Item = Node<'a, crate::IntrusionSet>> {
            self.data
                .get_owner()
                .intrusion_sets
                .values()
                .map(move |v| Node::new(v, self))
        }
        pub fn infrastructure<'a>(
            &'a self,
        ) -> impl Iterator<Item = Node<'a, crate::Infrastructure>> {
            self.data
                .get_owner()
                .infrastructure
                .values()
                .map(move |v| Node::new(v, self))
        }
        pub fn locations<'a>(&'a self) -> impl Iterator<Item = Node<'a, crate::Location>> {
            self.data
                .get_owner()
                .locations
                .values()
                .map(move |v| Node::new(v, self))
        }
        pub fn malware<'a>(&'a self) -> impl Iterator<Item = Node<'a, crate::Malware>> {
            self.data
                .get_owner()
                .malware
                .values()
                .map(move |v| Node::new(v, self))
        }
        pub fn marking_definitions<'a>(
            &'a self,
        ) -> impl Iterator<Item = Node<'a, crate::MarkingDefinition>> {
            self.data
                .get_owner()
                .marking_definitions
                .values()
                .map(move |v| Node::new(v, self))
        }
        pub fn relationships<'a>(&'a self) -> impl Iterator<Item = Node<'a, crate::Relationship>> {
            self.data
                .get_owner()
                .relationships
                .values()
                .map(move |v| Node::new(v, self))
        }
        pub fn threat_actors<'a>(&'a self) -> impl Iterator<Item = Node<'a, crate::ThreatActor>> {
            self.data
                .get_owner()
                .threat_actors
                .values()
                .map(move |v| Node::new(v, self))
        }
        pub fn tools<'a>(&'a self) -> impl Iterator<Item = Node<'a, crate::Tool>> {
            self.data
                .get_owner()
                .tools
                .values()
                .map(move |v| Node::new(v, self))
        }
        pub fn vulnerabilities<'a>(
            &'a self,
        ) -> impl Iterator<Item = Node<'a, crate::Vulnerability>> {
            self.data
                .get_owner()
                .vulnerabilities
                .values()
                .map(move |v| Node::new(v, self))
        }
    }
    impl Collection {
        fn data(&self) -> &CollectionBuilder {
            self.data.get_owner()
        }
        fn graph(&self) -> &crate::RelationshipGraph {
            self.data.get_or_init_dependent()
        }
    }
    impl From<crate::Bundle<Declaration>> for Collection {
        fn from(bundle: crate::Bundle<Declaration>) -> Self {
            let mut builder = CollectionBuilder::default();
            builder.add_bundle(bundle);
            builder.build()
        }
    }
    #[doc = r" An ID for a resource that may be in the backing collection."]
    #[doc = r""]
    #[doc = r" `Ref` is used to allow exploration of STIX collections when not every object referenced is"]
    #[doc = r" present in-memory."]
    pub struct Ref<'id, 'collection: 'id, D> {
        id: &'id crate::Id,
        collection: &'collection Collection,
        object_type: ::std::marker::PhantomData<D>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'id, 'collection: 'id, D: ::core::clone::Clone> ::core::clone::Clone
        for Ref<'id, 'collection, D>
    {
        #[inline]
        fn clone(&self) -> Ref<'id, 'collection, D> {
            match *self {
                Ref {
                    id: ref __self_0_0,
                    collection: ref __self_0_1,
                    object_type: ref __self_0_2,
                } => Ref {
                    id: ::core::clone::Clone::clone(&(*__self_0_0)),
                    collection: ::core::clone::Clone::clone(&(*__self_0_1)),
                    object_type: ::core::clone::Clone::clone(&(*__self_0_2)),
                },
            }
        }
    }
    impl<'id, 'collection: 'id, D> Ref<'id, 'collection, D> {
        #[doc = r" The ID the `Ref` will look up in the collection."]
        pub fn id(&self) -> &'id crate::Id {
            self.id
        }
    }
    #[doc = r" The ID of a STIX object of some type which may be present in the collection."]
    impl<'id, 'collection: 'id> Ref<'id, 'collection, Declaration> {
        #[doc = r" Narrow the type of the reference so that it can be resolved to a [`Node`]."]
        #[doc = r" This requires knowing the concrete type of the data associated with the ID."]
        pub fn downcast<D: crate::TypedObject>(self) -> Option<Ref<'id, 'collection, D>> {
            if self.id.object_type() == D::TYPE {
                Some(Ref {
                    id: self.id,
                    collection: self.collection,
                    object_type: ::std::marker::PhantomData::<D>,
                })
            } else {
                None
            }
        }
    }
    impl<'id, 'collection: 'id> Ref<'id, 'collection, crate::AttackPattern> {
        pub fn resolve(self) -> Option<Node<'collection, crate::AttackPattern>> {
            let Self { id, collection, .. } = self;
            Some(Node {
                data: collection.data().attack_patterns.get(id)?,
                collection,
            })
        }
    }
    impl<'id, 'collection: 'id> crate::Resolve for Ref<'id, 'collection, crate::AttackPattern> {
        type Output = Node<'collection, crate::AttackPattern>;
        fn resolve(self) -> Option<Self::Output> {
            Ref::<'id, 'collection, crate::AttackPattern>::resolve(self)
        }
    }
    impl<'id, 'collection: 'id> Ref<'id, 'collection, crate::Campaign> {
        pub fn resolve(self) -> Option<Node<'collection, crate::Campaign>> {
            let Self { id, collection, .. } = self;
            Some(Node {
                data: collection.data().campaigns.get(id)?,
                collection,
            })
        }
    }
    impl<'id, 'collection: 'id> crate::Resolve for Ref<'id, 'collection, crate::Campaign> {
        type Output = Node<'collection, crate::Campaign>;
        fn resolve(self) -> Option<Self::Output> {
            Ref::<'id, 'collection, crate::Campaign>::resolve(self)
        }
    }
    impl<'id, 'collection: 'id> Ref<'id, 'collection, crate::CourseOfAction> {
        pub fn resolve(self) -> Option<Node<'collection, crate::CourseOfAction>> {
            let Self { id, collection, .. } = self;
            Some(Node {
                data: collection.data().courses_of_action.get(id)?,
                collection,
            })
        }
    }
    impl<'id, 'collection: 'id> crate::Resolve for Ref<'id, 'collection, crate::CourseOfAction> {
        type Output = Node<'collection, crate::CourseOfAction>;
        fn resolve(self) -> Option<Self::Output> {
            Ref::<'id, 'collection, crate::CourseOfAction>::resolve(self)
        }
    }
    impl<'id, 'collection: 'id> Ref<'id, 'collection, crate::Identity> {
        pub fn resolve(self) -> Option<Node<'collection, crate::Identity>> {
            let Self { id, collection, .. } = self;
            Some(Node {
                data: collection.data().identities.get(id)?,
                collection,
            })
        }
    }
    impl<'id, 'collection: 'id> crate::Resolve for Ref<'id, 'collection, crate::Identity> {
        type Output = Node<'collection, crate::Identity>;
        fn resolve(self) -> Option<Self::Output> {
            Ref::<'id, 'collection, crate::Identity>::resolve(self)
        }
    }
    impl<'id, 'collection: 'id> Ref<'id, 'collection, crate::IntrusionSet> {
        pub fn resolve(self) -> Option<Node<'collection, crate::IntrusionSet>> {
            let Self { id, collection, .. } = self;
            Some(Node {
                data: collection.data().intrusion_sets.get(id)?,
                collection,
            })
        }
    }
    impl<'id, 'collection: 'id> crate::Resolve for Ref<'id, 'collection, crate::IntrusionSet> {
        type Output = Node<'collection, crate::IntrusionSet>;
        fn resolve(self) -> Option<Self::Output> {
            Ref::<'id, 'collection, crate::IntrusionSet>::resolve(self)
        }
    }
    impl<'id, 'collection: 'id> Ref<'id, 'collection, crate::Infrastructure> {
        pub fn resolve(self) -> Option<Node<'collection, crate::Infrastructure>> {
            let Self { id, collection, .. } = self;
            Some(Node {
                data: collection.data().infrastructure.get(id)?,
                collection,
            })
        }
    }
    impl<'id, 'collection: 'id> crate::Resolve for Ref<'id, 'collection, crate::Infrastructure> {
        type Output = Node<'collection, crate::Infrastructure>;
        fn resolve(self) -> Option<Self::Output> {
            Ref::<'id, 'collection, crate::Infrastructure>::resolve(self)
        }
    }
    impl<'id, 'collection: 'id> Ref<'id, 'collection, crate::Location> {
        pub fn resolve(self) -> Option<Node<'collection, crate::Location>> {
            let Self { id, collection, .. } = self;
            Some(Node {
                data: collection.data().locations.get(id)?,
                collection,
            })
        }
    }
    impl<'id, 'collection: 'id> crate::Resolve for Ref<'id, 'collection, crate::Location> {
        type Output = Node<'collection, crate::Location>;
        fn resolve(self) -> Option<Self::Output> {
            Ref::<'id, 'collection, crate::Location>::resolve(self)
        }
    }
    impl<'id, 'collection: 'id> Ref<'id, 'collection, crate::Malware> {
        pub fn resolve(self) -> Option<Node<'collection, crate::Malware>> {
            let Self { id, collection, .. } = self;
            Some(Node {
                data: collection.data().malware.get(id)?,
                collection,
            })
        }
    }
    impl<'id, 'collection: 'id> crate::Resolve for Ref<'id, 'collection, crate::Malware> {
        type Output = Node<'collection, crate::Malware>;
        fn resolve(self) -> Option<Self::Output> {
            Ref::<'id, 'collection, crate::Malware>::resolve(self)
        }
    }
    impl<'id, 'collection: 'id> Ref<'id, 'collection, crate::MarkingDefinition> {
        pub fn resolve(self) -> Option<Node<'collection, crate::MarkingDefinition>> {
            let Self { id, collection, .. } = self;
            Some(Node {
                data: collection.data().marking_definitions.get(id)?,
                collection,
            })
        }
    }
    impl<'id, 'collection: 'id> crate::Resolve for Ref<'id, 'collection, crate::MarkingDefinition> {
        type Output = Node<'collection, crate::MarkingDefinition>;
        fn resolve(self) -> Option<Self::Output> {
            Ref::<'id, 'collection, crate::MarkingDefinition>::resolve(self)
        }
    }
    impl<'id, 'collection: 'id> Ref<'id, 'collection, crate::Relationship> {
        pub fn resolve(self) -> Option<Node<'collection, crate::Relationship>> {
            let Self { id, collection, .. } = self;
            Some(Node {
                data: collection.data().relationships.get(id)?,
                collection,
            })
        }
    }
    impl<'id, 'collection: 'id> crate::Resolve for Ref<'id, 'collection, crate::Relationship> {
        type Output = Node<'collection, crate::Relationship>;
        fn resolve(self) -> Option<Self::Output> {
            Ref::<'id, 'collection, crate::Relationship>::resolve(self)
        }
    }
    impl<'id, 'collection: 'id> Ref<'id, 'collection, crate::ThreatActor> {
        pub fn resolve(self) -> Option<Node<'collection, crate::ThreatActor>> {
            let Self { id, collection, .. } = self;
            Some(Node {
                data: collection.data().threat_actors.get(id)?,
                collection,
            })
        }
    }
    impl<'id, 'collection: 'id> crate::Resolve for Ref<'id, 'collection, crate::ThreatActor> {
        type Output = Node<'collection, crate::ThreatActor>;
        fn resolve(self) -> Option<Self::Output> {
            Ref::<'id, 'collection, crate::ThreatActor>::resolve(self)
        }
    }
    impl<'id, 'collection: 'id> Ref<'id, 'collection, crate::Tool> {
        pub fn resolve(self) -> Option<Node<'collection, crate::Tool>> {
            let Self { id, collection, .. } = self;
            Some(Node {
                data: collection.data().tools.get(id)?,
                collection,
            })
        }
    }
    impl<'id, 'collection: 'id> crate::Resolve for Ref<'id, 'collection, crate::Tool> {
        type Output = Node<'collection, crate::Tool>;
        fn resolve(self) -> Option<Self::Output> {
            Ref::<'id, 'collection, crate::Tool>::resolve(self)
        }
    }
    impl<'id, 'collection: 'id> Ref<'id, 'collection, crate::Vulnerability> {
        pub fn resolve(self) -> Option<Node<'collection, crate::Vulnerability>> {
            let Self { id, collection, .. } = self;
            Some(Node {
                data: collection.data().vulnerabilities.get(id)?,
                collection,
            })
        }
    }
    impl<'id, 'collection: 'id> crate::Resolve for Ref<'id, 'collection, crate::Vulnerability> {
        type Output = Node<'collection, crate::Vulnerability>;
        fn resolve(self) -> Option<Self::Output> {
            Ref::<'id, 'collection, crate::Vulnerability>::resolve(self)
        }
    }
    #[doc = r" A STIX object in the [`Collection`], exposing the object's data and references"]
    #[doc = r" to associated objects in the same collection."]
    #[doc = r""]
    #[doc = r" Relationships are expressed as instance methods, scoped using the concrete type"]
    #[doc = r" of the object data, e.g. `Node<'a, IntrusionSet>` exposes `uses_attack_patterns`."]
    pub struct Node<'a, D> {
        data: &'a D,
        collection: &'a Collection,
    }
    impl<'a, D> Node<'a, D> {
        fn new(data: &'a D, collection: &'a Collection) -> Self {
            Self { data, collection }
        }
        fn link<E>(&'a self, data: &'a E) -> Node<'a, E> {
            Node::new(data, self.collection)
        }
        fn create_ref<E>(&self, id: &'a crate::Id) -> Ref<'a, 'a, E> {
            Ref {
                id,
                collection: self.collection,
                object_type: ::std::marker::PhantomData::<E>,
            }
        }
    }
    impl<'a, D> ::std::ops::Deref for Node<'a, D> {
        type Target = D;
        fn deref(&self) -> &Self::Target {
            self.data
        }
    }
    impl<D: AsRef<crate::CommonProperties>> AsRef<crate::CommonProperties> for Node<'_, D> {
        fn as_ref(&self) -> &crate::CommonProperties {
            self.data.as_ref()
        }
    }
    impl<'a> Node<'a, crate::IntrusionSet> {
        pub fn attributed_to_threat_actors(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::ThreatActor>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::ThreatActor>(
                        crate::RelationshipType::AttributedTo,
                    ),
                )
                .map(move |id| self.create_ref::<crate::ThreatActor>(id))
        }
        pub fn hosts_infrastructure(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Infrastructure>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Infrastructure>(
                        crate::RelationshipType::Hosts,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Infrastructure>(id))
        }
        pub fn owns_infrastructure(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Infrastructure>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Infrastructure>(
                        crate::RelationshipType::Owns,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Infrastructure>(id))
        }
        pub fn targets_identities(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Identity>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Identity>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Identity>(id))
        }
        pub fn targets_locations(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Location>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Location>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Location>(id))
        }
        pub fn targets_vulnerabilities(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Vulnerability>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Vulnerability>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Vulnerability>(id))
        }
        pub fn uses_attack_patterns(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::AttackPattern>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::AttackPattern>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::AttackPattern>(id))
        }
        pub fn uses_infrastructure(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Infrastructure>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Infrastructure>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Infrastructure>(id))
        }
        pub fn uses_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Malware>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn uses_tools(&'a self) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Tool>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Tool>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Tool>(id))
        }
        pub fn attributed_campaigns(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Campaign>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Campaign>(
                        crate::RelationshipType::AttributedTo,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Campaign>(id))
        }
    }
    impl<'a> Node<'a, crate::Campaign> {
        pub fn attributed_to_intrusion_sets(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::IntrusionSet>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::IntrusionSet>(
                        crate::RelationshipType::AttributedTo,
                    ),
                )
                .map(move |id| self.create_ref::<crate::IntrusionSet>(id))
        }
        pub fn attributed_to_threat_actors(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::ThreatActor>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::ThreatActor>(
                        crate::RelationshipType::AttributedTo,
                    ),
                )
                .map(move |id| self.create_ref::<crate::ThreatActor>(id))
        }
        pub fn compromises_infrastructure(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Infrastructure>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Infrastructure>(
                        crate::RelationshipType::Compromises,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Infrastructure>(id))
        }
        pub fn originates_from_locations(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Location>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Location>(
                        crate::RelationshipType::OriginatesFrom,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Location>(id))
        }
        pub fn targets_identities(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Identity>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Identity>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Identity>(id))
        }
        pub fn targets_locations(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Location>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Location>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Location>(id))
        }
        pub fn targets_vulnerabilities(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Vulnerability>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Vulnerability>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Vulnerability>(id))
        }
        pub fn uses_attack_patterns(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::AttackPattern>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::AttackPattern>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::AttackPattern>(id))
        }
        pub fn uses_infrastructure(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Infrastructure>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Infrastructure>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Infrastructure>(id))
        }
        pub fn uses_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Malware>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn uses_tools(&'a self) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Tool>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Tool>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Tool>(id))
        }
    }
    impl<'a> Node<'a, crate::ThreatActor> {
        pub fn attributed_to_identities(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Identity>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Identity>(
                        crate::RelationshipType::AttributedTo,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Identity>(id))
        }
        pub fn compromises_infrastructure(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Infrastructure>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Infrastructure>(
                        crate::RelationshipType::Compromises,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Infrastructure>(id))
        }
        pub fn hosts_infrastructure(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Infrastructure>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Infrastructure>(
                        crate::RelationshipType::Hosts,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Infrastructure>(id))
        }
        pub fn owns_infrastructure(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Infrastructure>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Infrastructure>(
                        crate::RelationshipType::Owns,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Infrastructure>(id))
        }
        pub fn impersonates_identities(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Identity>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Identity>(
                        crate::RelationshipType::Impersonates,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Identity>(id))
        }
        pub fn located_at_locations(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Location>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Location>(
                        crate::RelationshipType::LocatedAt,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Location>(id))
        }
        pub fn targets_identities(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Identity>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Identity>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Identity>(id))
        }
        pub fn targets_locations(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Location>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Location>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Location>(id))
        }
        pub fn targets_vulnerabilities(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Vulnerability>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Vulnerability>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Vulnerability>(id))
        }
        pub fn uses_attack_patterns(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::AttackPattern>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::AttackPattern>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::AttackPattern>(id))
        }
        pub fn uses_infrastructure(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Infrastructure>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Infrastructure>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Infrastructure>(id))
        }
        pub fn uses_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Malware>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn uses_tools(&'a self) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Tool>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Tool>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Tool>(id))
        }
        pub fn attributed_campaigns(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Campaign>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Campaign>(
                        crate::RelationshipType::AttributedTo,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Campaign>(id))
        }
        pub fn attributed_intrusion_sets(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::IntrusionSet>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::IntrusionSet>(
                        crate::RelationshipType::AttributedTo,
                    ),
                )
                .map(move |id| self.create_ref::<crate::IntrusionSet>(id))
        }
        pub fn authored_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Malware>(
                        crate::RelationshipType::AuthoredBy,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
    }
    impl<'a> Node<'a, crate::Location> {
        pub fn targeted_by_attack_patterns(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::AttackPattern>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::AttackPattern>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::AttackPattern>(id))
        }
        pub fn origin_of_campaigns(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Campaign>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Campaign>(
                        crate::RelationshipType::OriginatesFrom,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Campaign>(id))
        }
        pub fn targeted_by_campaigns(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Campaign>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Campaign>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Campaign>(id))
        }
        pub fn location_of_identities(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Identity>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Identity>(
                        crate::RelationshipType::LocatedAt,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Identity>(id))
        }
        pub fn targeted_by_intrusion_sets(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::IntrusionSet>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::IntrusionSet>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::IntrusionSet>(id))
        }
        pub fn location_of_infrastructure(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Infrastructure>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Infrastructure>(
                        crate::RelationshipType::LocatedAt,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Infrastructure>(id))
        }
        pub fn origin_of_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Malware>(
                        crate::RelationshipType::OriginatesFrom,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn targeted_by_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Malware>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn location_of_threat_actors(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::ThreatActor>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::ThreatActor>(
                        crate::RelationshipType::LocatedAt,
                    ),
                )
                .map(move |id| self.create_ref::<crate::ThreatActor>(id))
        }
        pub fn targeted_by_threat_actors(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::ThreatActor>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::ThreatActor>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::ThreatActor>(id))
        }
        pub fn targeted_by_tools(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Tool>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Tool>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Tool>(id))
        }
    }
    impl<'a> Node<'a, crate::AttackPattern> {
        pub fn compromises_infrastructure(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Infrastructure>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Infrastructure>(
                        crate::RelationshipType::Compromises,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Infrastructure>(id))
        }
        pub fn delivers_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Malware>(
                        crate::RelationshipType::Delivers,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn targets_identities(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Identity>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Identity>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Identity>(id))
        }
        pub fn targets_locations(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Location>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Location>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Location>(id))
        }
        pub fn targets_vulnerabilities(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Vulnerability>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Vulnerability>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Vulnerability>(id))
        }
        pub fn uses_tools(&'a self) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Tool>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Tool>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Tool>(id))
        }
        pub fn uses_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Malware>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn uses_vulnerabilities(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Vulnerability>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Vulnerability>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Vulnerability>(id))
        }
        pub fn used_by_campaigns(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Campaign>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Campaign>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Campaign>(id))
        }
        pub fn mitigated_by_courses_of_action(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::CourseOfAction>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::CourseOfAction>(
                        crate::RelationshipType::Mitigates,
                    ),
                )
                .map(move |id| self.create_ref::<crate::CourseOfAction>(id))
        }
        pub fn used_by_intrusion_sets(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::IntrusionSet>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::IntrusionSet>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::IntrusionSet>(id))
        }
        pub fn used_by_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Malware>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn used_by_threat_actors(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::ThreatActor>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::ThreatActor>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::ThreatActor>(id))
        }
    }
    impl<'a> Node<'a, crate::CourseOfAction> {
        pub fn mitigates_attack_patterns(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::AttackPattern>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::AttackPattern>(
                        crate::RelationshipType::Mitigates,
                    ),
                )
                .map(move |id| self.create_ref::<crate::AttackPattern>(id))
        }
        pub fn mitigates_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Malware>(
                        crate::RelationshipType::Mitigates,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn mitigates_tools(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Tool>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Tool>(
                        crate::RelationshipType::Mitigates,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Tool>(id))
        }
        pub fn mitigates_vulnerabilities(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Vulnerability>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Vulnerability>(
                        crate::RelationshipType::Mitigates,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Vulnerability>(id))
        }
        pub fn remediates_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Malware>(
                        crate::RelationshipType::Remediates,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn remediates_vulnerabilities(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Vulnerability>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Vulnerability>(
                        crate::RelationshipType::Remediates,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Vulnerability>(id))
        }
    }
    impl<'a> Node<'a, crate::Vulnerability> {
        pub fn targeted_by_attack_patterns(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::AttackPattern>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::AttackPattern>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::AttackPattern>(id))
        }
        pub fn used_by_attack_patterns(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::AttackPattern>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::AttackPattern>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::AttackPattern>(id))
        }
        pub fn targeted_by_campaigns(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Campaign>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Campaign>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Campaign>(id))
        }
        pub fn mitigated_by_courses_of_action(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::CourseOfAction>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::CourseOfAction>(
                        crate::RelationshipType::Mitigates,
                    ),
                )
                .map(move |id| self.create_ref::<crate::CourseOfAction>(id))
        }
        pub fn remediated_by_courses_of_action(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::CourseOfAction>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::CourseOfAction>(
                        crate::RelationshipType::Remediates,
                    ),
                )
                .map(move |id| self.create_ref::<crate::CourseOfAction>(id))
        }
        pub fn targeted_by_intrusion_sets(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::IntrusionSet>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::IntrusionSet>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::IntrusionSet>(id))
        }
        pub fn present_in_infrastructure(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Infrastructure>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Infrastructure>(
                        crate::RelationshipType::Has,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Infrastructure>(id))
        }
        pub fn targeted_by_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Malware>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn targeted_by_threat_actors(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::ThreatActor>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::ThreatActor>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::ThreatActor>(id))
        }
        pub fn present_in_tools(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Tool>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Tool>(
                        crate::RelationshipType::Has,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Tool>(id))
        }
        pub fn targeted_by_tools(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Tool>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Tool>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Tool>(id))
        }
    }
    impl<'a> Node<'a, crate::Infrastructure> {
        pub fn controls_infrastructure(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Infrastructure>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Infrastructure>(
                        crate::RelationshipType::Controls,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Infrastructure>(id))
        }
        pub fn controls_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Malware>(
                        crate::RelationshipType::Controls,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn delivers_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Malware>(
                        crate::RelationshipType::Delivers,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn has_vulnerabilities(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Vulnerability>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Vulnerability>(
                        crate::RelationshipType::Has,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Vulnerability>(id))
        }
        pub fn hosts_tools(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Tool>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Tool>(
                        crate::RelationshipType::Hosts,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Tool>(id))
        }
        pub fn hosts_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Malware>(
                        crate::RelationshipType::Hosts,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn located_at_locations(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Location>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Location>(
                        crate::RelationshipType::LocatedAt,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Location>(id))
        }
        pub fn uses_infrastructure(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Infrastructure>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Infrastructure>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Infrastructure>(id))
        }
        pub fn compromised_by_attack_patterns(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::AttackPattern>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::AttackPattern>(
                        crate::RelationshipType::Compromises,
                    ),
                )
                .map(move |id| self.create_ref::<crate::AttackPattern>(id))
        }
        pub fn compromised_by_campaigns(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Campaign>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Campaign>(
                        crate::RelationshipType::Compromises,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Campaign>(id))
        }
        pub fn used_by_campaigns(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Campaign>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Campaign>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Campaign>(id))
        }
        pub fn hosted_by_intrusion_sets(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::IntrusionSet>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::IntrusionSet>(
                        crate::RelationshipType::Hosts,
                    ),
                )
                .map(move |id| self.create_ref::<crate::IntrusionSet>(id))
        }
        pub fn owned_by_intrusion_sets(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::IntrusionSet>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::IntrusionSet>(
                        crate::RelationshipType::Owns,
                    ),
                )
                .map(move |id| self.create_ref::<crate::IntrusionSet>(id))
        }
        pub fn used_by_intrusion_sets(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::IntrusionSet>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::IntrusionSet>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::IntrusionSet>(id))
        }
        pub fn controlled_by_infrastructure(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Infrastructure>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Infrastructure>(
                        crate::RelationshipType::Controls,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Infrastructure>(id))
        }
        pub fn used_by_infrastructure(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Infrastructure>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Infrastructure>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Infrastructure>(id))
        }
        pub fn beaconed_by_to_by_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Malware>(
                        crate::RelationshipType::BeaconsTo,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn exfiltrated_by_to_by_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Malware>(
                        crate::RelationshipType::ExfiltratesTo,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn targeted_by_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Malware>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn used_by_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Malware>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn compromised_by_threat_actors(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::ThreatActor>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::ThreatActor>(
                        crate::RelationshipType::Compromises,
                    ),
                )
                .map(move |id| self.create_ref::<crate::ThreatActor>(id))
        }
        pub fn hosted_by_threat_actors(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::ThreatActor>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::ThreatActor>(
                        crate::RelationshipType::Hosts,
                    ),
                )
                .map(move |id| self.create_ref::<crate::ThreatActor>(id))
        }
        pub fn owned_by_threat_actors(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::ThreatActor>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::ThreatActor>(
                        crate::RelationshipType::Owns,
                    ),
                )
                .map(move |id| self.create_ref::<crate::ThreatActor>(id))
        }
        pub fn used_by_threat_actors(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::ThreatActor>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::ThreatActor>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::ThreatActor>(id))
        }
        pub fn targeted_by_tools(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Tool>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Tool>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Tool>(id))
        }
        pub fn used_by_tools(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Tool>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Tool>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Tool>(id))
        }
    }
    impl<'a> Node<'a, crate::Malware> {
        pub fn authored_by_threat_actors(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::ThreatActor>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::ThreatActor>(
                        crate::RelationshipType::AuthoredBy,
                    ),
                )
                .map(move |id| self.create_ref::<crate::ThreatActor>(id))
        }
        pub fn beacons_to_infrastructure(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Infrastructure>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Infrastructure>(
                        crate::RelationshipType::BeaconsTo,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Infrastructure>(id))
        }
        pub fn controls_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Malware>(
                        crate::RelationshipType::Controls,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn downloads_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Malware>(
                        crate::RelationshipType::Downloads,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn downloads_tools(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Tool>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Tool>(
                        crate::RelationshipType::Downloads,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Tool>(id))
        }
        pub fn drops_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Malware>(
                        crate::RelationshipType::Drops,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn drops_tools(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Tool>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Tool>(
                        crate::RelationshipType::Drops,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Tool>(id))
        }
        pub fn exfiltrates_to_infrastructure(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Infrastructure>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Infrastructure>(
                        crate::RelationshipType::ExfiltratesTo,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Infrastructure>(id))
        }
        pub fn originates_from_locations(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Location>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Location>(
                        crate::RelationshipType::OriginatesFrom,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Location>(id))
        }
        pub fn targets_identities(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Identity>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Identity>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Identity>(id))
        }
        pub fn targets_infrastructure(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Infrastructure>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Infrastructure>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Infrastructure>(id))
        }
        pub fn targets_locations(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Location>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Location>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Location>(id))
        }
        pub fn targets_vulnerabilities(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Vulnerability>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Vulnerability>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Vulnerability>(id))
        }
        pub fn uses_attack_patterns(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::AttackPattern>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::AttackPattern>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::AttackPattern>(id))
        }
        pub fn uses_infrastructure(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Infrastructure>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Infrastructure>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Infrastructure>(id))
        }
        pub fn uses_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Malware>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn uses_tools(&'a self) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Tool>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Tool>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Tool>(id))
        }
        pub fn delivered_by_attack_patterns(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::AttackPattern>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::AttackPattern>(
                        crate::RelationshipType::Delivers,
                    ),
                )
                .map(move |id| self.create_ref::<crate::AttackPattern>(id))
        }
        pub fn used_by_attack_patterns(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::AttackPattern>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::AttackPattern>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::AttackPattern>(id))
        }
        pub fn used_by_campaigns(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Campaign>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Campaign>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Campaign>(id))
        }
        pub fn mitigated_by_courses_of_action(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::CourseOfAction>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::CourseOfAction>(
                        crate::RelationshipType::Mitigates,
                    ),
                )
                .map(move |id| self.create_ref::<crate::CourseOfAction>(id))
        }
        pub fn remediated_by_courses_of_action(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::CourseOfAction>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::CourseOfAction>(
                        crate::RelationshipType::Remediates,
                    ),
                )
                .map(move |id| self.create_ref::<crate::CourseOfAction>(id))
        }
        pub fn used_by_intrusion_sets(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::IntrusionSet>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::IntrusionSet>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::IntrusionSet>(id))
        }
        pub fn controlled_by_infrastructure(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Infrastructure>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Infrastructure>(
                        crate::RelationshipType::Controls,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Infrastructure>(id))
        }
        pub fn delivered_by_infrastructure(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Infrastructure>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Infrastructure>(
                        crate::RelationshipType::Delivers,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Infrastructure>(id))
        }
        pub fn hosted_by_infrastructure(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Infrastructure>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Infrastructure>(
                        crate::RelationshipType::Hosts,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Infrastructure>(id))
        }
        pub fn controlled_by_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Malware>(
                        crate::RelationshipType::Controls,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn downloaded_by_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Malware>(
                        crate::RelationshipType::Downloads,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn dropped_by_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Malware>(
                        crate::RelationshipType::Drops,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn used_by_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Malware>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn used_by_threat_actors(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::ThreatActor>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::ThreatActor>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::ThreatActor>(id))
        }
        pub fn delivered_by_tools(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Tool>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Tool>(
                        crate::RelationshipType::Delivers,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Tool>(id))
        }
        pub fn dropped_by_tools(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Tool>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Tool>(
                        crate::RelationshipType::Drops,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Tool>(id))
        }
    }
    impl<'a> Node<'a, crate::Tool> {
        pub fn delivers_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Malware>(
                        crate::RelationshipType::Delivers,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn drops_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Malware>(
                        crate::RelationshipType::Drops,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn has_vulnerabilities(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Vulnerability>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Vulnerability>(
                        crate::RelationshipType::Has,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Vulnerability>(id))
        }
        pub fn targets_identities(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Identity>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Identity>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Identity>(id))
        }
        pub fn targets_infrastructure(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Infrastructure>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Infrastructure>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Infrastructure>(id))
        }
        pub fn targets_locations(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Location>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Location>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Location>(id))
        }
        pub fn targets_vulnerabilities(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Vulnerability>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Vulnerability>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Vulnerability>(id))
        }
        pub fn uses_infrastructure(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Infrastructure>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Infrastructure>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Infrastructure>(id))
        }
        pub fn used_by_attack_patterns(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::AttackPattern>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::AttackPattern>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::AttackPattern>(id))
        }
        pub fn used_by_campaigns(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Campaign>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Campaign>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Campaign>(id))
        }
        pub fn mitigated_by_courses_of_action(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::CourseOfAction>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::CourseOfAction>(
                        crate::RelationshipType::Mitigates,
                    ),
                )
                .map(move |id| self.create_ref::<crate::CourseOfAction>(id))
        }
        pub fn used_by_intrusion_sets(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::IntrusionSet>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::IntrusionSet>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::IntrusionSet>(id))
        }
        pub fn hosted_by_infrastructure(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Infrastructure>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Infrastructure>(
                        crate::RelationshipType::Hosts,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Infrastructure>(id))
        }
        pub fn downloaded_by_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Malware>(
                        crate::RelationshipType::Downloads,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn dropped_by_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Malware>(
                        crate::RelationshipType::Drops,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn used_by_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Malware>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn used_by_threat_actors(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::ThreatActor>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::ThreatActor>(
                        crate::RelationshipType::Uses,
                    ),
                )
                .map(move |id| self.create_ref::<crate::ThreatActor>(id))
        }
    }
    impl<'a> Node<'a, crate::Identity> {
        pub fn located_at_locations(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Location>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::outgoing::<crate::Location>(
                        crate::RelationshipType::LocatedAt,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Location>(id))
        }
        pub fn targeted_by_attack_patterns(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::AttackPattern>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::AttackPattern>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::AttackPattern>(id))
        }
        pub fn targeted_by_campaigns(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Campaign>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Campaign>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Campaign>(id))
        }
        pub fn targeted_by_intrusion_sets(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::IntrusionSet>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::IntrusionSet>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::IntrusionSet>(id))
        }
        pub fn targeted_by_malware(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Malware>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Malware>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Malware>(id))
        }
        pub fn attributed_threat_actors(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::ThreatActor>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::ThreatActor>(
                        crate::RelationshipType::AttributedTo,
                    ),
                )
                .map(move |id| self.create_ref::<crate::ThreatActor>(id))
        }
        pub fn impersonated_by_threat_actors(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::ThreatActor>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::ThreatActor>(
                        crate::RelationshipType::Impersonates,
                    ),
                )
                .map(move |id| self.create_ref::<crate::ThreatActor>(id))
        }
        pub fn targeted_by_threat_actors(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::ThreatActor>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::ThreatActor>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::ThreatActor>(id))
        }
        pub fn targeted_by_tools(
            &'a self,
        ) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, crate::Tool>> {
            self.collection
                .graph()
                .peers_matching(
                    crate::Object::id(self.data),
                    crate::relationship::Filter::incoming::<crate::Tool>(
                        crate::RelationshipType::Targets,
                    ),
                )
                .map(move |id| self.create_ref::<crate::Tool>(id))
        }
    }
}
mod threat_actor {
    use std::collections::BTreeSet;
    use chrono::{DateTime, Utc};
    use serde::Deserialize;
    use crate::CommonProperties;
    #[typed_object(core)]
    pub struct ThreatActor {
        #[serde(flatten)]
        common: CommonProperties,
        pub name: String,
        #[serde(default)]
        pub description: Option<String>,
        #[serde(default)]
        pub aliases: BTreeSet<String>,
        #[serde(default)]
        pub first_seen: Option<DateTime<Utc>>,
        #[serde(default)]
        pub last_seen: Option<DateTime<Utc>>,
        #[serde(default)]
        pub goals: Vec<String>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ThreatActor {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field<'de> {
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __other(_serde::__private::de::Content<'de>),
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field<'de>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_bool<__E>(
                        self,
                        __value: bool,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Bool(__value),
                        ))
                    }
                    fn visit_i8<__E>(
                        self,
                        __value: i8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(_serde::__private::de::Content::I8(
                            __value,
                        )))
                    }
                    fn visit_i16<__E>(
                        self,
                        __value: i16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I16(__value),
                        ))
                    }
                    fn visit_i32<__E>(
                        self,
                        __value: i32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I32(__value),
                        ))
                    }
                    fn visit_i64<__E>(
                        self,
                        __value: i64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I64(__value),
                        ))
                    }
                    fn visit_u8<__E>(
                        self,
                        __value: u8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(_serde::__private::de::Content::U8(
                            __value,
                        )))
                    }
                    fn visit_u16<__E>(
                        self,
                        __value: u16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U16(__value),
                        ))
                    }
                    fn visit_u32<__E>(
                        self,
                        __value: u32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U32(__value),
                        ))
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U64(__value),
                        ))
                    }
                    fn visit_f32<__E>(
                        self,
                        __value: f32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::F32(__value),
                        ))
                    }
                    fn visit_f64<__E>(
                        self,
                        __value: f64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::F64(__value),
                        ))
                    }
                    fn visit_char<__E>(
                        self,
                        __value: char,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Char(__value),
                        ))
                    }
                    fn visit_unit<__E>(self) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Unit,
                        ))
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field1),
                            "description" => _serde::__private::Ok(__Field::__field2),
                            "aliases" => _serde::__private::Ok(__Field::__field3),
                            "first_seen" => _serde::__private::Ok(__Field::__field4),
                            "last_seen" => _serde::__private::Ok(__Field::__field5),
                            "goals" => _serde::__private::Ok(__Field::__field6),
                            _ => {
                                let __value = _serde::__private::de::Content::String(
                                    _serde::__private::ToString::to_string(__value),
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field1),
                            b"description" => _serde::__private::Ok(__Field::__field2),
                            b"aliases" => _serde::__private::Ok(__Field::__field3),
                            b"first_seen" => _serde::__private::Ok(__Field::__field4),
                            b"last_seen" => _serde::__private::Ok(__Field::__field5),
                            b"goals" => _serde::__private::Ok(__Field::__field6),
                            _ => {
                                let __value =
                                    _serde::__private::de::Content::ByteBuf(__value.to_vec());
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_str<__E>(
                        self,
                        __value: &'de str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field1),
                            "description" => _serde::__private::Ok(__Field::__field2),
                            "aliases" => _serde::__private::Ok(__Field::__field3),
                            "first_seen" => _serde::__private::Ok(__Field::__field4),
                            "last_seen" => _serde::__private::Ok(__Field::__field5),
                            "goals" => _serde::__private::Ok(__Field::__field6),
                            _ => {
                                let __value = _serde::__private::de::Content::Str(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_bytes<__E>(
                        self,
                        __value: &'de [u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field1),
                            b"description" => _serde::__private::Ok(__Field::__field2),
                            b"aliases" => _serde::__private::Ok(__Field::__field3),
                            b"first_seen" => _serde::__private::Ok(__Field::__field4),
                            b"last_seen" => _serde::__private::Ok(__Field::__field5),
                            b"goals" => _serde::__private::Ok(__Field::__field6),
                            _ => {
                                let __value = _serde::__private::de::Content::Bytes(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field<'de> {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ThreatActor>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ThreatActor;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct ThreatActor")
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Option<String>> =
                            _serde::__private::None;
                        let mut __field3: _serde::__private::Option<BTreeSet<String>> =
                            _serde::__private::None;
                        let mut __field4: _serde::__private::Option<Option<DateTime<Utc>>> =
                            _serde::__private::None;
                        let mut __field5: _serde::__private::Option<Option<DateTime<Utc>>> =
                            _serde::__private::None;
                        let mut __field6: _serde::__private::Option<Vec<String>> =
                            _serde::__private::None;
                        let mut __collect = _serde::__private::Vec::<
                            _serde::__private::Option<(
                                _serde::__private::de::Content,
                                _serde::__private::de::Content,
                            )>,
                        >::new();
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "description",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "aliases",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<BTreeSet<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "first_seen",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<DateTime<Utc>>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "last_seen",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<DateTime<Utc>>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::__private::Option::is_some(&__field6) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "goals",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__other(__name) => {
                                    __collect.push(_serde::__private::Some((
                                        __name,
                                        match _serde::de::MapAccess::next_value(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    )));
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field6 = match __field6 {
                            _serde::__private::Some(__field6) => __field6,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field0: CommonProperties = match _serde::de::Deserialize::deserialize(
                            _serde::__private::de::FlatMapDeserializer(
                                &mut __collect,
                                _serde::__private::PhantomData,
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(ThreatActor {
                            common: __field0,
                            name: __field1,
                            description: __field2,
                            aliases: __field3,
                            first_seen: __field4,
                            last_seen: __field5,
                            goals: __field6,
                        })
                    }
                }
                _serde::Deserializer::deserialize_map(
                    __deserializer,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ThreatActor>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl crate::TypedObject for ThreatActor {
        const TYPE: &'static str = "threat-actor";
    }
    impl AsRef<CommonProperties> for ThreatActor {
        fn as_ref(&self) -> &CommonProperties {
            &self.common
        }
    }
}
mod tool {
    use serde::Deserialize;
    use crate::CommonProperties;
    #[typed_object(core)]
    pub struct Tool {
        #[serde(flatten)]
        base: CommonProperties,
        pub name: String,
        #[serde(default)]
        pub description: Option<String>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Tool {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field<'de> {
                    __field1,
                    __field2,
                    __other(_serde::__private::de::Content<'de>),
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field<'de>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_bool<__E>(
                        self,
                        __value: bool,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Bool(__value),
                        ))
                    }
                    fn visit_i8<__E>(
                        self,
                        __value: i8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(_serde::__private::de::Content::I8(
                            __value,
                        )))
                    }
                    fn visit_i16<__E>(
                        self,
                        __value: i16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I16(__value),
                        ))
                    }
                    fn visit_i32<__E>(
                        self,
                        __value: i32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I32(__value),
                        ))
                    }
                    fn visit_i64<__E>(
                        self,
                        __value: i64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I64(__value),
                        ))
                    }
                    fn visit_u8<__E>(
                        self,
                        __value: u8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(_serde::__private::de::Content::U8(
                            __value,
                        )))
                    }
                    fn visit_u16<__E>(
                        self,
                        __value: u16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U16(__value),
                        ))
                    }
                    fn visit_u32<__E>(
                        self,
                        __value: u32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U32(__value),
                        ))
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U64(__value),
                        ))
                    }
                    fn visit_f32<__E>(
                        self,
                        __value: f32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::F32(__value),
                        ))
                    }
                    fn visit_f64<__E>(
                        self,
                        __value: f64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::F64(__value),
                        ))
                    }
                    fn visit_char<__E>(
                        self,
                        __value: char,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Char(__value),
                        ))
                    }
                    fn visit_unit<__E>(self) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Unit,
                        ))
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field1),
                            "description" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = _serde::__private::de::Content::String(
                                    _serde::__private::ToString::to_string(__value),
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field1),
                            b"description" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value =
                                    _serde::__private::de::Content::ByteBuf(__value.to_vec());
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_str<__E>(
                        self,
                        __value: &'de str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field1),
                            "description" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = _serde::__private::de::Content::Str(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_bytes<__E>(
                        self,
                        __value: &'de [u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field1),
                            b"description" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = _serde::__private::de::Content::Bytes(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field<'de> {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Tool>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Tool;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct Tool")
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Option<String>> =
                            _serde::__private::None;
                        let mut __collect = _serde::__private::Vec::<
                            _serde::__private::Option<(
                                _serde::__private::de::Content,
                                _serde::__private::de::Content,
                            )>,
                        >::new();
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "description",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__other(__name) => {
                                    __collect.push(_serde::__private::Some((
                                        __name,
                                        match _serde::de::MapAccess::next_value(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    )));
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field0: CommonProperties = match _serde::de::Deserialize::deserialize(
                            _serde::__private::de::FlatMapDeserializer(
                                &mut __collect,
                                _serde::__private::PhantomData,
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(Tool {
                            base: __field0,
                            name: __field1,
                            description: __field2,
                        })
                    }
                }
                _serde::Deserializer::deserialize_map(
                    __deserializer,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Tool>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl crate::TypedObject for Tool {
        const TYPE: &'static str = "tool";
    }
    impl Tool {
        pub fn name(&self) -> &str {
            &self.name
        }
        pub fn description(&self) -> Option<&str> {
            self.description.as_ref().map(|s| s.as_str())
        }
    }
    impl AsRef<CommonProperties> for Tool {
        fn as_ref(&self) -> &CommonProperties {
            &self.base
        }
    }
}
pub mod vocab {
    #![doc = " Types for working with STIX vocabularies."]
    #![doc = ""]
    #![doc = " From the [specification](https://docs.oasis-open.org/cti/stix/v2.1/cs01/stix-v2.1-cs01.html#_vbsdt43uxrv0):"]
    #![doc = ""]
    #![doc = " > Some STIX properties are defined using open vocabularies or enumerations."]
    #![doc = " > Enumerations and open vocabularies are defined in STIX in order to enhance interoperability"]
    #![doc = " > by increasing the likelihood that different entities use the same exact string to represent"]
    #![doc = " > the same concept. If used consistently, open vocabularies make it less likely that one entity"]
    #![doc = " > refers to the energy sector as “Energy” and another as “Energy Sector”, thereby making comparison"]
    #![doc = " > and correlation easier."]
    #![doc = " An open STIX vocabulary. Vocabularies improver correlation across threat intel from different sources"]
    #![doc = " by ensuring exact string equality whenever there is semantic equality."]
    pub trait Vocabulary {
        #[doc = " The vocabulary identifier. This is typically a kebab-case string ending in `-ov`, e.g. `account-type-ov`."]
        const TYPE: &'static str;
        #[doc = " Whether the contained string is a known value of the vocabulary."]
        fn is_known_value(&self) -> bool;
    }
    pub struct AccountType(::std::borrow::Cow<'static, str>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for AccountType {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                AccountType(ref __self_0_0) => {
                    let mut debug_trait_builder = f.debug_tuple("AccountType");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for AccountType {
        #[inline]
        fn clone(&self) -> AccountType {
            match *self {
                AccountType(ref __self_0_0) => {
                    AccountType(::core::clone::Clone::clone(&(*__self_0_0)))
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for AccountType {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for AccountType {
        #[inline]
        fn eq(&self, other: &AccountType) -> bool {
            match *other {
                AccountType(ref __self_1_0) => match *self {
                    AccountType(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &AccountType) -> bool {
            match *other {
                AccountType(ref __self_1_0) => match *self {
                    AccountType(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl ::core::marker::StructuralEq for AccountType {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for AccountType {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<::std::borrow::Cow<'static, str>>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialOrd for AccountType {
        #[inline]
        fn partial_cmp(
            &self,
            other: &AccountType,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match *other {
                AccountType(ref __self_1_0) => match *self {
                    AccountType(ref __self_0_0) => {
                        match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)) {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                            }
                            cmp => cmp,
                        }
                    }
                },
            }
        }
        #[inline]
        fn lt(&self, other: &AccountType) -> bool {
            match *other {
                AccountType(ref __self_1_0) => match *self {
                    AccountType(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Greater,
                        ) == ::core::cmp::Ordering::Less
                    }
                },
            }
        }
        #[inline]
        fn le(&self, other: &AccountType) -> bool {
            match *other {
                AccountType(ref __self_1_0) => match *self {
                    AccountType(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Greater,
                        ) != ::core::cmp::Ordering::Greater
                    }
                },
            }
        }
        #[inline]
        fn gt(&self, other: &AccountType) -> bool {
            match *other {
                AccountType(ref __self_1_0) => match *self {
                    AccountType(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Less,
                        ) == ::core::cmp::Ordering::Greater
                    }
                },
            }
        }
        #[inline]
        fn ge(&self, other: &AccountType) -> bool {
            match *other {
                AccountType(ref __self_1_0) => match *self {
                    AccountType(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Less,
                        ) != ::core::cmp::Ordering::Less
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Ord for AccountType {
        #[inline]
        fn cmp(&self, other: &AccountType) -> ::core::cmp::Ordering {
            match *other {
                AccountType(ref __self_1_0) => match *self {
                    AccountType(ref __self_0_0) => {
                        match ::core::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                            ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                            cmp => cmp,
                        }
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::hash::Hash for AccountType {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                AccountType(ref __self_0_0) => ::core::hash::Hash::hash(&(*__self_0_0), state),
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for AccountType {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<AccountType>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = AccountType;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "tuple struct AccountType",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: ::std::borrow::Cow<'static, str> = match <::std::borrow::Cow<
                            'static,
                            str,
                        > as _serde::Deserialize>::deserialize(
                            __e
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(AccountType(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            ::std::borrow::Cow<'static, str>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"tuple struct AccountType with 1 element",
                                ));
                            }
                        };
                        _serde::__private::Ok(AccountType(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "AccountType",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<AccountType>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for AccountType {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_newtype_struct(__serializer, "AccountType", &self.0)
            }
        }
    };
    impl AccountType {
        pub const FACEBOOK: Self = Self(::std::borrow::Cow::Borrowed("facebook"));
        pub const LDAP: Self = Self(::std::borrow::Cow::Borrowed("ldap"));
        pub const NIS: Self = Self(::std::borrow::Cow::Borrowed("nis"));
        pub const OPENID: Self = Self(::std::borrow::Cow::Borrowed("openid"));
        pub const RADIUS: Self = Self(::std::borrow::Cow::Borrowed("radius"));
        pub const SKYPE: Self = Self(::std::borrow::Cow::Borrowed("skype"));
        pub const TACACS: Self = Self(::std::borrow::Cow::Borrowed("tacacs"));
        pub const TWITTER: Self = Self(::std::borrow::Cow::Borrowed("twitter"));
        pub const UNIX: Self = Self(::std::borrow::Cow::Borrowed("unix"));
        pub const WINDOWS_LOCAL: Self = Self(::std::borrow::Cow::Borrowed("windows-local"));
        pub const WINDOWS_DOMAIN: Self = Self(::std::borrow::Cow::Borrowed("windows-domain"));
    }
    impl crate::vocab::Vocabulary for AccountType {
        const TYPE: &'static str = "account-type-ov";
        fn is_known_value(&self) -> bool {
            self == "facebook"
                || self == "ldap"
                || self == "nis"
                || self == "openid"
                || self == "radius"
                || self == "skype"
                || self == "tacacs"
                || self == "twitter"
                || self == "unix"
                || self == "windows-local"
                || self == "windows-domain"
        }
    }
    impl ::std::fmt::Display for AccountType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }
    impl ::std::convert::AsRef<str> for AccountType {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }
    impl ::std::cmp::PartialEq<str> for AccountType {
        fn eq(&self, rhs: &str) -> bool {
            self.0 == rhs
        }
    }
    impl ::std::convert::From<String> for AccountType {
        fn from(s: String) -> Self {
            Self(::std::borrow::Cow::Owned(s))
        }
    }
    impl ::std::convert::From<&'static str> for AccountType {
        fn from(s: &'static str) -> Self {
            Self(::std::borrow::Cow::Borrowed(s))
        }
    }
    pub struct AttackMotivation(::std::borrow::Cow<'static, str>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for AttackMotivation {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                AttackMotivation(ref __self_0_0) => {
                    let mut debug_trait_builder = f.debug_tuple("AttackMotivation");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for AttackMotivation {
        #[inline]
        fn clone(&self) -> AttackMotivation {
            match *self {
                AttackMotivation(ref __self_0_0) => {
                    AttackMotivation(::core::clone::Clone::clone(&(*__self_0_0)))
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for AttackMotivation {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for AttackMotivation {
        #[inline]
        fn eq(&self, other: &AttackMotivation) -> bool {
            match *other {
                AttackMotivation(ref __self_1_0) => match *self {
                    AttackMotivation(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &AttackMotivation) -> bool {
            match *other {
                AttackMotivation(ref __self_1_0) => match *self {
                    AttackMotivation(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl ::core::marker::StructuralEq for AttackMotivation {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for AttackMotivation {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<::std::borrow::Cow<'static, str>>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialOrd for AttackMotivation {
        #[inline]
        fn partial_cmp(
            &self,
            other: &AttackMotivation,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match *other {
                AttackMotivation(ref __self_1_0) => match *self {
                    AttackMotivation(ref __self_0_0) => {
                        match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)) {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                            }
                            cmp => cmp,
                        }
                    }
                },
            }
        }
        #[inline]
        fn lt(&self, other: &AttackMotivation) -> bool {
            match *other {
                AttackMotivation(ref __self_1_0) => match *self {
                    AttackMotivation(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Greater,
                        ) == ::core::cmp::Ordering::Less
                    }
                },
            }
        }
        #[inline]
        fn le(&self, other: &AttackMotivation) -> bool {
            match *other {
                AttackMotivation(ref __self_1_0) => match *self {
                    AttackMotivation(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Greater,
                        ) != ::core::cmp::Ordering::Greater
                    }
                },
            }
        }
        #[inline]
        fn gt(&self, other: &AttackMotivation) -> bool {
            match *other {
                AttackMotivation(ref __self_1_0) => match *self {
                    AttackMotivation(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Less,
                        ) == ::core::cmp::Ordering::Greater
                    }
                },
            }
        }
        #[inline]
        fn ge(&self, other: &AttackMotivation) -> bool {
            match *other {
                AttackMotivation(ref __self_1_0) => match *self {
                    AttackMotivation(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Less,
                        ) != ::core::cmp::Ordering::Less
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Ord for AttackMotivation {
        #[inline]
        fn cmp(&self, other: &AttackMotivation) -> ::core::cmp::Ordering {
            match *other {
                AttackMotivation(ref __self_1_0) => match *self {
                    AttackMotivation(ref __self_0_0) => {
                        match ::core::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                            ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                            cmp => cmp,
                        }
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::hash::Hash for AttackMotivation {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                AttackMotivation(ref __self_0_0) => ::core::hash::Hash::hash(&(*__self_0_0), state),
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for AttackMotivation {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<AttackMotivation>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = AttackMotivation;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "tuple struct AttackMotivation",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: ::std::borrow::Cow<'static, str> = match <::std::borrow::Cow<
                            'static,
                            str,
                        > as _serde::Deserialize>::deserialize(
                            __e
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(AttackMotivation(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            ::std::borrow::Cow<'static, str>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"tuple struct AttackMotivation with 1 element",
                                ));
                            }
                        };
                        _serde::__private::Ok(AttackMotivation(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "AttackMotivation",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<AttackMotivation>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for AttackMotivation {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_newtype_struct(
                    __serializer,
                    "AttackMotivation",
                    &self.0,
                )
            }
        }
    };
    impl AttackMotivation {
        pub const ACCIDENTAL: Self = Self(::std::borrow::Cow::Borrowed("accidental"));
        pub const COERCION: Self = Self(::std::borrow::Cow::Borrowed("coercion"));
        pub const DOMINANCE: Self = Self(::std::borrow::Cow::Borrowed("dominance"));
        pub const IDEOLOGY: Self = Self(::std::borrow::Cow::Borrowed("ideology"));
        pub const NOTORIETY: Self = Self(::std::borrow::Cow::Borrowed("notoriety"));
        pub const ORGANIZATIONAL_GAIN: Self =
            Self(::std::borrow::Cow::Borrowed("organizational-gain"));
        pub const PERSONAL_GAIN: Self = Self(::std::borrow::Cow::Borrowed("personal-gain"));
        pub const PERSONAL_SATISFACTION: Self =
            Self(::std::borrow::Cow::Borrowed("personal-satisfaction"));
        pub const REVENGE: Self = Self(::std::borrow::Cow::Borrowed("revenge"));
        pub const UNPREDICTABLE: Self = Self(::std::borrow::Cow::Borrowed("unpredictable"));
    }
    impl crate::vocab::Vocabulary for AttackMotivation {
        const TYPE: &'static str = "attack-motivation-ov";
        fn is_known_value(&self) -> bool {
            self == "accidental"
                || self == "coercion"
                || self == "dominance"
                || self == "ideology"
                || self == "notoriety"
                || self == "organizational-gain"
                || self == "personal-gain"
                || self == "personal-satisfaction"
                || self == "revenge"
                || self == "unpredictable"
        }
    }
    impl ::std::fmt::Display for AttackMotivation {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }
    impl ::std::convert::AsRef<str> for AttackMotivation {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }
    impl ::std::cmp::PartialEq<str> for AttackMotivation {
        fn eq(&self, rhs: &str) -> bool {
            self.0 == rhs
        }
    }
    impl ::std::convert::From<String> for AttackMotivation {
        fn from(s: String) -> Self {
            Self(::std::borrow::Cow::Owned(s))
        }
    }
    impl ::std::convert::From<&'static str> for AttackMotivation {
        fn from(s: &'static str) -> Self {
            Self(::std::borrow::Cow::Borrowed(s))
        }
    }
    pub struct AttackResourceLevel(::std::borrow::Cow<'static, str>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for AttackResourceLevel {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                AttackResourceLevel(ref __self_0_0) => {
                    let mut debug_trait_builder = f.debug_tuple("AttackResourceLevel");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for AttackResourceLevel {
        #[inline]
        fn clone(&self) -> AttackResourceLevel {
            match *self {
                AttackResourceLevel(ref __self_0_0) => {
                    AttackResourceLevel(::core::clone::Clone::clone(&(*__self_0_0)))
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for AttackResourceLevel {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for AttackResourceLevel {
        #[inline]
        fn eq(&self, other: &AttackResourceLevel) -> bool {
            match *other {
                AttackResourceLevel(ref __self_1_0) => match *self {
                    AttackResourceLevel(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &AttackResourceLevel) -> bool {
            match *other {
                AttackResourceLevel(ref __self_1_0) => match *self {
                    AttackResourceLevel(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl ::core::marker::StructuralEq for AttackResourceLevel {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for AttackResourceLevel {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<::std::borrow::Cow<'static, str>>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialOrd for AttackResourceLevel {
        #[inline]
        fn partial_cmp(
            &self,
            other: &AttackResourceLevel,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match *other {
                AttackResourceLevel(ref __self_1_0) => match *self {
                    AttackResourceLevel(ref __self_0_0) => {
                        match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)) {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                            }
                            cmp => cmp,
                        }
                    }
                },
            }
        }
        #[inline]
        fn lt(&self, other: &AttackResourceLevel) -> bool {
            match *other {
                AttackResourceLevel(ref __self_1_0) => match *self {
                    AttackResourceLevel(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Greater,
                        ) == ::core::cmp::Ordering::Less
                    }
                },
            }
        }
        #[inline]
        fn le(&self, other: &AttackResourceLevel) -> bool {
            match *other {
                AttackResourceLevel(ref __self_1_0) => match *self {
                    AttackResourceLevel(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Greater,
                        ) != ::core::cmp::Ordering::Greater
                    }
                },
            }
        }
        #[inline]
        fn gt(&self, other: &AttackResourceLevel) -> bool {
            match *other {
                AttackResourceLevel(ref __self_1_0) => match *self {
                    AttackResourceLevel(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Less,
                        ) == ::core::cmp::Ordering::Greater
                    }
                },
            }
        }
        #[inline]
        fn ge(&self, other: &AttackResourceLevel) -> bool {
            match *other {
                AttackResourceLevel(ref __self_1_0) => match *self {
                    AttackResourceLevel(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Less,
                        ) != ::core::cmp::Ordering::Less
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Ord for AttackResourceLevel {
        #[inline]
        fn cmp(&self, other: &AttackResourceLevel) -> ::core::cmp::Ordering {
            match *other {
                AttackResourceLevel(ref __self_1_0) => match *self {
                    AttackResourceLevel(ref __self_0_0) => {
                        match ::core::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                            ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                            cmp => cmp,
                        }
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::hash::Hash for AttackResourceLevel {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                AttackResourceLevel(ref __self_0_0) => {
                    ::core::hash::Hash::hash(&(*__self_0_0), state)
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for AttackResourceLevel {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<AttackResourceLevel>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = AttackResourceLevel;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "tuple struct AttackResourceLevel",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: ::std::borrow::Cow<'static, str> = match <::std::borrow::Cow<
                            'static,
                            str,
                        > as _serde::Deserialize>::deserialize(
                            __e
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(AttackResourceLevel(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            ::std::borrow::Cow<'static, str>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"tuple struct AttackResourceLevel with 1 element",
                                ));
                            }
                        };
                        _serde::__private::Ok(AttackResourceLevel(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "AttackResourceLevel",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<AttackResourceLevel>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for AttackResourceLevel {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_newtype_struct(
                    __serializer,
                    "AttackResourceLevel",
                    &self.0,
                )
            }
        }
    };
    impl AttackResourceLevel {
        pub const INDIVIDUAL: Self = Self(::std::borrow::Cow::Borrowed("individual"));
        pub const CLUB: Self = Self(::std::borrow::Cow::Borrowed("club"));
        pub const CONTEST: Self = Self(::std::borrow::Cow::Borrowed("contest"));
        pub const TEAM: Self = Self(::std::borrow::Cow::Borrowed("team"));
        pub const ORGANIZATION: Self = Self(::std::borrow::Cow::Borrowed("organization"));
        pub const GOVERNMENT: Self = Self(::std::borrow::Cow::Borrowed("government"));
    }
    impl crate::vocab::Vocabulary for AttackResourceLevel {
        const TYPE: &'static str = "attack-resource-level-ov";
        fn is_known_value(&self) -> bool {
            self == "individual"
                || self == "club"
                || self == "contest"
                || self == "team"
                || self == "organization"
                || self == "government"
        }
    }
    impl ::std::fmt::Display for AttackResourceLevel {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }
    impl ::std::convert::AsRef<str> for AttackResourceLevel {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }
    impl ::std::cmp::PartialEq<str> for AttackResourceLevel {
        fn eq(&self, rhs: &str) -> bool {
            self.0 == rhs
        }
    }
    impl ::std::convert::From<String> for AttackResourceLevel {
        fn from(s: String) -> Self {
            Self(::std::borrow::Cow::Owned(s))
        }
    }
    impl ::std::convert::From<&'static str> for AttackResourceLevel {
        fn from(s: &'static str) -> Self {
            Self(::std::borrow::Cow::Borrowed(s))
        }
    }
    pub struct ImplementationLanguage(::std::borrow::Cow<'static, str>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ImplementationLanguage {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                ImplementationLanguage(ref __self_0_0) => {
                    let mut debug_trait_builder = f.debug_tuple("ImplementationLanguage");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for ImplementationLanguage {
        #[inline]
        fn clone(&self) -> ImplementationLanguage {
            match *self {
                ImplementationLanguage(ref __self_0_0) => {
                    ImplementationLanguage(::core::clone::Clone::clone(&(*__self_0_0)))
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for ImplementationLanguage {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for ImplementationLanguage {
        #[inline]
        fn eq(&self, other: &ImplementationLanguage) -> bool {
            match *other {
                ImplementationLanguage(ref __self_1_0) => match *self {
                    ImplementationLanguage(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ImplementationLanguage) -> bool {
            match *other {
                ImplementationLanguage(ref __self_1_0) => match *self {
                    ImplementationLanguage(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl ::core::marker::StructuralEq for ImplementationLanguage {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for ImplementationLanguage {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<::std::borrow::Cow<'static, str>>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialOrd for ImplementationLanguage {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ImplementationLanguage,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match *other {
                ImplementationLanguage(ref __self_1_0) => match *self {
                    ImplementationLanguage(ref __self_0_0) => {
                        match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)) {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                            }
                            cmp => cmp,
                        }
                    }
                },
            }
        }
        #[inline]
        fn lt(&self, other: &ImplementationLanguage) -> bool {
            match *other {
                ImplementationLanguage(ref __self_1_0) => match *self {
                    ImplementationLanguage(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Greater,
                        ) == ::core::cmp::Ordering::Less
                    }
                },
            }
        }
        #[inline]
        fn le(&self, other: &ImplementationLanguage) -> bool {
            match *other {
                ImplementationLanguage(ref __self_1_0) => match *self {
                    ImplementationLanguage(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Greater,
                        ) != ::core::cmp::Ordering::Greater
                    }
                },
            }
        }
        #[inline]
        fn gt(&self, other: &ImplementationLanguage) -> bool {
            match *other {
                ImplementationLanguage(ref __self_1_0) => match *self {
                    ImplementationLanguage(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Less,
                        ) == ::core::cmp::Ordering::Greater
                    }
                },
            }
        }
        #[inline]
        fn ge(&self, other: &ImplementationLanguage) -> bool {
            match *other {
                ImplementationLanguage(ref __self_1_0) => match *self {
                    ImplementationLanguage(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Less,
                        ) != ::core::cmp::Ordering::Less
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Ord for ImplementationLanguage {
        #[inline]
        fn cmp(&self, other: &ImplementationLanguage) -> ::core::cmp::Ordering {
            match *other {
                ImplementationLanguage(ref __self_1_0) => match *self {
                    ImplementationLanguage(ref __self_0_0) => {
                        match ::core::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                            ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                            cmp => cmp,
                        }
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::hash::Hash for ImplementationLanguage {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                ImplementationLanguage(ref __self_0_0) => {
                    ::core::hash::Hash::hash(&(*__self_0_0), state)
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ImplementationLanguage {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ImplementationLanguage>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ImplementationLanguage;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "tuple struct ImplementationLanguage",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: ::std::borrow::Cow<'static, str> = match <::std::borrow::Cow<
                            'static,
                            str,
                        > as _serde::Deserialize>::deserialize(
                            __e
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(ImplementationLanguage(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            ::std::borrow::Cow<'static, str>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"tuple struct ImplementationLanguage with 1 element",
                                ));
                            }
                        };
                        _serde::__private::Ok(ImplementationLanguage(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "ImplementationLanguage",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ImplementationLanguage>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ImplementationLanguage {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_newtype_struct(
                    __serializer,
                    "ImplementationLanguage",
                    &self.0,
                )
            }
        }
    };
    impl ImplementationLanguage {
        pub const APPLESCRIPT: Self = Self(::std::borrow::Cow::Borrowed("applescript"));
        pub const BASH: Self = Self(::std::borrow::Cow::Borrowed("bash"));
        pub const C: Self = Self(::std::borrow::Cow::Borrowed("c"));
        #[doc = "Value is `\"c++\"`"]
        pub const C_PLUS_PLUS: Self = Self(::std::borrow::Cow::Borrowed("c++"));
        #[doc = "Value is `\"c#\"`"]
        pub const C_SHARP: Self = Self(::std::borrow::Cow::Borrowed("c#"));
        pub const GO: Self = Self(::std::borrow::Cow::Borrowed("go"));
        pub const JAVA: Self = Self(::std::borrow::Cow::Borrowed("java"));
        pub const JAVASCRIPT: Self = Self(::std::borrow::Cow::Borrowed("javascript"));
        pub const LUA: Self = Self(::std::borrow::Cow::Borrowed("lua"));
        pub const OBJECTIVE_C: Self = Self(::std::borrow::Cow::Borrowed("objective-c"));
        pub const PERL: Self = Self(::std::borrow::Cow::Borrowed("perl"));
        pub const PHP: Self = Self(::std::borrow::Cow::Borrowed("php"));
        pub const POWERSHELL: Self = Self(::std::borrow::Cow::Borrowed("powershell"));
        pub const PYTHON: Self = Self(::std::borrow::Cow::Borrowed("python"));
        pub const RUBY: Self = Self(::std::borrow::Cow::Borrowed("ruby"));
        pub const SCALA: Self = Self(::std::borrow::Cow::Borrowed("scala"));
        pub const SWIFT: Self = Self(::std::borrow::Cow::Borrowed("swift"));
        pub const TYPESCRIPT: Self = Self(::std::borrow::Cow::Borrowed("typescript"));
        pub const VISUAL_BASIC: Self = Self(::std::borrow::Cow::Borrowed("visual-basic"));
        pub const X86_32: Self = Self(::std::borrow::Cow::Borrowed("x86-32"));
        pub const X86_64: Self = Self(::std::borrow::Cow::Borrowed("x86-64"));
    }
    impl crate::vocab::Vocabulary for ImplementationLanguage {
        const TYPE: &'static str = "implementation-language-ov";
        fn is_known_value(&self) -> bool {
            self == "applescript"
                || self == "bash"
                || self == "c"
                || self == "c++"
                || self == "c#"
                || self == "go"
                || self == "java"
                || self == "javascript"
                || self == "lua"
                || self == "objective-c"
                || self == "perl"
                || self == "php"
                || self == "powershell"
                || self == "python"
                || self == "ruby"
                || self == "scala"
                || self == "swift"
                || self == "typescript"
                || self == "visual-basic"
                || self == "x86-32"
                || self == "x86-64"
        }
    }
    impl ::std::fmt::Display for ImplementationLanguage {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }
    impl ::std::convert::AsRef<str> for ImplementationLanguage {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }
    impl ::std::cmp::PartialEq<str> for ImplementationLanguage {
        fn eq(&self, rhs: &str) -> bool {
            self.0 == rhs
        }
    }
    impl ::std::convert::From<String> for ImplementationLanguage {
        fn from(s: String) -> Self {
            Self(::std::borrow::Cow::Owned(s))
        }
    }
    impl ::std::convert::From<&'static str> for ImplementationLanguage {
        fn from(s: &'static str) -> Self {
            Self(::std::borrow::Cow::Borrowed(s))
        }
    }
    pub struct MalwareCapabilities(::std::borrow::Cow<'static, str>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for MalwareCapabilities {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                MalwareCapabilities(ref __self_0_0) => {
                    let mut debug_trait_builder = f.debug_tuple("MalwareCapabilities");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for MalwareCapabilities {
        #[inline]
        fn clone(&self) -> MalwareCapabilities {
            match *self {
                MalwareCapabilities(ref __self_0_0) => {
                    MalwareCapabilities(::core::clone::Clone::clone(&(*__self_0_0)))
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for MalwareCapabilities {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for MalwareCapabilities {
        #[inline]
        fn eq(&self, other: &MalwareCapabilities) -> bool {
            match *other {
                MalwareCapabilities(ref __self_1_0) => match *self {
                    MalwareCapabilities(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &MalwareCapabilities) -> bool {
            match *other {
                MalwareCapabilities(ref __self_1_0) => match *self {
                    MalwareCapabilities(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl ::core::marker::StructuralEq for MalwareCapabilities {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for MalwareCapabilities {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<::std::borrow::Cow<'static, str>>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialOrd for MalwareCapabilities {
        #[inline]
        fn partial_cmp(
            &self,
            other: &MalwareCapabilities,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match *other {
                MalwareCapabilities(ref __self_1_0) => match *self {
                    MalwareCapabilities(ref __self_0_0) => {
                        match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)) {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                            }
                            cmp => cmp,
                        }
                    }
                },
            }
        }
        #[inline]
        fn lt(&self, other: &MalwareCapabilities) -> bool {
            match *other {
                MalwareCapabilities(ref __self_1_0) => match *self {
                    MalwareCapabilities(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Greater,
                        ) == ::core::cmp::Ordering::Less
                    }
                },
            }
        }
        #[inline]
        fn le(&self, other: &MalwareCapabilities) -> bool {
            match *other {
                MalwareCapabilities(ref __self_1_0) => match *self {
                    MalwareCapabilities(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Greater,
                        ) != ::core::cmp::Ordering::Greater
                    }
                },
            }
        }
        #[inline]
        fn gt(&self, other: &MalwareCapabilities) -> bool {
            match *other {
                MalwareCapabilities(ref __self_1_0) => match *self {
                    MalwareCapabilities(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Less,
                        ) == ::core::cmp::Ordering::Greater
                    }
                },
            }
        }
        #[inline]
        fn ge(&self, other: &MalwareCapabilities) -> bool {
            match *other {
                MalwareCapabilities(ref __self_1_0) => match *self {
                    MalwareCapabilities(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Less,
                        ) != ::core::cmp::Ordering::Less
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Ord for MalwareCapabilities {
        #[inline]
        fn cmp(&self, other: &MalwareCapabilities) -> ::core::cmp::Ordering {
            match *other {
                MalwareCapabilities(ref __self_1_0) => match *self {
                    MalwareCapabilities(ref __self_0_0) => {
                        match ::core::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                            ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                            cmp => cmp,
                        }
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::hash::Hash for MalwareCapabilities {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                MalwareCapabilities(ref __self_0_0) => {
                    ::core::hash::Hash::hash(&(*__self_0_0), state)
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for MalwareCapabilities {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<MalwareCapabilities>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = MalwareCapabilities;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "tuple struct MalwareCapabilities",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: ::std::borrow::Cow<'static, str> = match <::std::borrow::Cow<
                            'static,
                            str,
                        > as _serde::Deserialize>::deserialize(
                            __e
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(MalwareCapabilities(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            ::std::borrow::Cow<'static, str>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"tuple struct MalwareCapabilities with 1 element",
                                ));
                            }
                        };
                        _serde::__private::Ok(MalwareCapabilities(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "MalwareCapabilities",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<MalwareCapabilities>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for MalwareCapabilities {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_newtype_struct(
                    __serializer,
                    "MalwareCapabilities",
                    &self.0,
                )
            }
        }
    };
    impl MalwareCapabilities {
        pub const ACCESSES_REMOTE_MACHINES: Self =
            Self(::std::borrow::Cow::Borrowed("accesses-remote-machines"));
        pub const ANTI_DEBUGGING: Self = Self(::std::borrow::Cow::Borrowed("anti-debugging"));
        pub const ANTI_DISASSEMBLY: Self = Self(::std::borrow::Cow::Borrowed("anti-disassembly"));
        pub const ANTI_EMULATION: Self = Self(::std::borrow::Cow::Borrowed("anti-emulation"));
        pub const ANTI_MEMORY_FORENSICS: Self =
            Self(::std::borrow::Cow::Borrowed("anti-memory-forensics"));
        pub const ANTI_SANDBOX: Self = Self(::std::borrow::Cow::Borrowed("anti-sandbox"));
        pub const ANTI_VM: Self = Self(::std::borrow::Cow::Borrowed("anti-vm"));
        pub const CAPTURES_INPUT_PERIPHERALS: Self =
            Self(::std::borrow::Cow::Borrowed("captures-input-peripherals"));
        pub const CAPTURES_OUTPUT_PERIPHERALS: Self =
            Self(::std::borrow::Cow::Borrowed("captures-output-peripherals"));
        pub const CAPTURES_SYSTEM_STATE_DATA: Self =
            Self(::std::borrow::Cow::Borrowed("captures-system-state-data"));
        pub const CLEANS_TRACES_OF_INFECTION: Self =
            Self(::std::borrow::Cow::Borrowed("cleans-traces-of-infection"));
        pub const COMMITS_FRAUD: Self = Self(::std::borrow::Cow::Borrowed("commits-fraud"));
        pub const COMMUNICATES_WITH_C2: Self =
            Self(::std::borrow::Cow::Borrowed("communicates-with-c2"));
        pub const COMPROMISES_DATA_AVAILABILITY: Self = Self(::std::borrow::Cow::Borrowed(
            "compromises-data-availability",
        ));
        pub const COMPROMISES_DATA_INTEGRITY: Self =
            Self(::std::borrow::Cow::Borrowed("compromises-data-integrity"));
        pub const COMPROMISES_SYSTEM_AVAILABILITY: Self = Self(::std::borrow::Cow::Borrowed(
            "compromises-system-availability",
        ));
        pub const CONTROLS_LOCAL_MACHINE: Self =
            Self(::std::borrow::Cow::Borrowed("controls-local-machine"));
        pub const DEGRADES_SECURITY_SOFTWARE: Self =
            Self(::std::borrow::Cow::Borrowed("degrades-security-software"));
        pub const DEGRADES_SYSTEM_UPDATES: Self =
            Self(::std::borrow::Cow::Borrowed("degrades-system-updates"));
        pub const DETERMINES_C2_SERVER: Self =
            Self(::std::borrow::Cow::Borrowed("determines-c2-server"));
        pub const EMAILS_SPAM: Self = Self(::std::borrow::Cow::Borrowed("emails-spam"));
        pub const ESCALATES_PRIVILEGES: Self =
            Self(::std::borrow::Cow::Borrowed("escalates-privileges"));
        pub const EVADES_AV: Self = Self(::std::borrow::Cow::Borrowed("evades-av"));
        pub const EXFILTRATES_DATA: Self = Self(::std::borrow::Cow::Borrowed("exfiltrates-data"));
        pub const FINGERPRINTS_HOST: Self = Self(::std::borrow::Cow::Borrowed("fingerprints-host"));
        pub const HIDES_ARTIFACTS: Self = Self(::std::borrow::Cow::Borrowed("hides-artifacts"));
        pub const HIDES_EXECUTING_CODE: Self =
            Self(::std::borrow::Cow::Borrowed("hides-executing-code"));
        pub const INFECTS_FILES: Self = Self(::std::borrow::Cow::Borrowed("infects-files"));
        pub const INFECTS_REMOTE_MACHINES: Self =
            Self(::std::borrow::Cow::Borrowed("infects-remote-machines"));
        pub const INSTALLS_OTHER_COMPONENTS: Self =
            Self(::std::borrow::Cow::Borrowed("installs-other-components"));
        pub const PERSISTS_AFTER_SYSTEM_REBOOT: Self =
            Self(::std::borrow::Cow::Borrowed("persists-after-system-reboot"));
        pub const PREVENTS_ARTIFACT_ACCESS: Self =
            Self(::std::borrow::Cow::Borrowed("prevents-artifact-access"));
        pub const PREVENTS_ARTIFACT_DELETION: Self =
            Self(::std::borrow::Cow::Borrowed("prevents-artifact-deletion"));
        pub const PROBES_NETWORK_ENVIRONMENT: Self =
            Self(::std::borrow::Cow::Borrowed("probes-network-environment"));
        pub const SELF_MODIFIES: Self = Self(::std::borrow::Cow::Borrowed("self-modifies"));
        pub const STEALS_AUTHENTICATION_CREDENTIALS: Self = Self(::std::borrow::Cow::Borrowed(
            "steals-authentication-credentials",
        ));
        pub const VIOLATES_SYSTEM_OPERATIONAL_INTEGRITY: Self = Self(::std::borrow::Cow::Borrowed(
            "violates-system-operational-integrity",
        ));
    }
    impl crate::vocab::Vocabulary for MalwareCapabilities {
        const TYPE: &'static str = "malware-capabilities-ov";
        fn is_known_value(&self) -> bool {
            self == "accesses-remote-machines"
                || self == "anti-debugging"
                || self == "anti-disassembly"
                || self == "anti-emulation"
                || self == "anti-memory-forensics"
                || self == "anti-sandbox"
                || self == "anti-vm"
                || self == "captures-input-peripherals"
                || self == "captures-output-peripherals"
                || self == "captures-system-state-data"
                || self == "cleans-traces-of-infection"
                || self == "commits-fraud"
                || self == "communicates-with-c2"
                || self == "compromises-data-availability"
                || self == "compromises-data-integrity"
                || self == "compromises-system-availability"
                || self == "controls-local-machine"
                || self == "degrades-security-software"
                || self == "degrades-system-updates"
                || self == "determines-c2-server"
                || self == "emails-spam"
                || self == "escalates-privileges"
                || self == "evades-av"
                || self == "exfiltrates-data"
                || self == "fingerprints-host"
                || self == "hides-artifacts"
                || self == "hides-executing-code"
                || self == "infects-files"
                || self == "infects-remote-machines"
                || self == "installs-other-components"
                || self == "persists-after-system-reboot"
                || self == "prevents-artifact-access"
                || self == "prevents-artifact-deletion"
                || self == "probes-network-environment"
                || self == "self-modifies"
                || self == "steals-authentication-credentials"
                || self == "violates-system-operational-integrity"
        }
    }
    impl ::std::fmt::Display for MalwareCapabilities {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }
    impl ::std::convert::AsRef<str> for MalwareCapabilities {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }
    impl ::std::cmp::PartialEq<str> for MalwareCapabilities {
        fn eq(&self, rhs: &str) -> bool {
            self.0 == rhs
        }
    }
    impl ::std::convert::From<String> for MalwareCapabilities {
        fn from(s: String) -> Self {
            Self(::std::borrow::Cow::Owned(s))
        }
    }
    impl ::std::convert::From<&'static str> for MalwareCapabilities {
        fn from(s: &'static str) -> Self {
            Self(::std::borrow::Cow::Borrowed(s))
        }
    }
    pub struct MalwareType(::std::borrow::Cow<'static, str>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for MalwareType {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                MalwareType(ref __self_0_0) => {
                    let mut debug_trait_builder = f.debug_tuple("MalwareType");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for MalwareType {
        #[inline]
        fn clone(&self) -> MalwareType {
            match *self {
                MalwareType(ref __self_0_0) => {
                    MalwareType(::core::clone::Clone::clone(&(*__self_0_0)))
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for MalwareType {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for MalwareType {
        #[inline]
        fn eq(&self, other: &MalwareType) -> bool {
            match *other {
                MalwareType(ref __self_1_0) => match *self {
                    MalwareType(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &MalwareType) -> bool {
            match *other {
                MalwareType(ref __self_1_0) => match *self {
                    MalwareType(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl ::core::marker::StructuralEq for MalwareType {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for MalwareType {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<::std::borrow::Cow<'static, str>>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialOrd for MalwareType {
        #[inline]
        fn partial_cmp(
            &self,
            other: &MalwareType,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match *other {
                MalwareType(ref __self_1_0) => match *self {
                    MalwareType(ref __self_0_0) => {
                        match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)) {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                            }
                            cmp => cmp,
                        }
                    }
                },
            }
        }
        #[inline]
        fn lt(&self, other: &MalwareType) -> bool {
            match *other {
                MalwareType(ref __self_1_0) => match *self {
                    MalwareType(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Greater,
                        ) == ::core::cmp::Ordering::Less
                    }
                },
            }
        }
        #[inline]
        fn le(&self, other: &MalwareType) -> bool {
            match *other {
                MalwareType(ref __self_1_0) => match *self {
                    MalwareType(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Greater,
                        ) != ::core::cmp::Ordering::Greater
                    }
                },
            }
        }
        #[inline]
        fn gt(&self, other: &MalwareType) -> bool {
            match *other {
                MalwareType(ref __self_1_0) => match *self {
                    MalwareType(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Less,
                        ) == ::core::cmp::Ordering::Greater
                    }
                },
            }
        }
        #[inline]
        fn ge(&self, other: &MalwareType) -> bool {
            match *other {
                MalwareType(ref __self_1_0) => match *self {
                    MalwareType(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Less,
                        ) != ::core::cmp::Ordering::Less
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Ord for MalwareType {
        #[inline]
        fn cmp(&self, other: &MalwareType) -> ::core::cmp::Ordering {
            match *other {
                MalwareType(ref __self_1_0) => match *self {
                    MalwareType(ref __self_0_0) => {
                        match ::core::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                            ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                            cmp => cmp,
                        }
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::hash::Hash for MalwareType {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                MalwareType(ref __self_0_0) => ::core::hash::Hash::hash(&(*__self_0_0), state),
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for MalwareType {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<MalwareType>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = MalwareType;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "tuple struct MalwareType",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: ::std::borrow::Cow<'static, str> = match <::std::borrow::Cow<
                            'static,
                            str,
                        > as _serde::Deserialize>::deserialize(
                            __e
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(MalwareType(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            ::std::borrow::Cow<'static, str>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"tuple struct MalwareType with 1 element",
                                ));
                            }
                        };
                        _serde::__private::Ok(MalwareType(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "MalwareType",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<MalwareType>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for MalwareType {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_newtype_struct(__serializer, "MalwareType", &self.0)
            }
        }
    };
    impl MalwareType {
        pub const ADWARE: Self = Self(::std::borrow::Cow::Borrowed("adware"));
        pub const BACKDOOR: Self = Self(::std::borrow::Cow::Borrowed("backdoor"));
        pub const BOT: Self = Self(::std::borrow::Cow::Borrowed("bot"));
        pub const BOOTKIT: Self = Self(::std::borrow::Cow::Borrowed("bootkit"));
        pub const DDOS: Self = Self(::std::borrow::Cow::Borrowed("ddos"));
        pub const DOWNLOADER: Self = Self(::std::borrow::Cow::Borrowed("downloader"));
        pub const DROPPER: Self = Self(::std::borrow::Cow::Borrowed("dropper"));
        pub const EXPLOIT_KIT: Self = Self(::std::borrow::Cow::Borrowed("exploit-kit"));
        pub const KEYLOGGER: Self = Self(::std::borrow::Cow::Borrowed("keylogger"));
        pub const RANSOMWARE: Self = Self(::std::borrow::Cow::Borrowed("ransomware"));
        pub const REMOTE_ACCESS_TROJAN: Self =
            Self(::std::borrow::Cow::Borrowed("remote-access-trojan"));
        pub const RESOURCE_EXPLOITATION: Self =
            Self(::std::borrow::Cow::Borrowed("resource-exploitation"));
        pub const ROGUE_SECURITY_SOFTWARE: Self =
            Self(::std::borrow::Cow::Borrowed("rogue-security-software"));
        pub const ROOTKIT: Self = Self(::std::borrow::Cow::Borrowed("rootkit"));
        pub const SCREEN_CAPTURE: Self = Self(::std::borrow::Cow::Borrowed("screen-capture"));
        pub const SPYWARE: Self = Self(::std::borrow::Cow::Borrowed("spyware"));
        pub const TROJAN: Self = Self(::std::borrow::Cow::Borrowed("trojan"));
        pub const UNKNOWN: Self = Self(::std::borrow::Cow::Borrowed("unknown"));
        pub const VIRUS: Self = Self(::std::borrow::Cow::Borrowed("virus"));
        pub const WEBSHELL: Self = Self(::std::borrow::Cow::Borrowed("webshell"));
        pub const WIPER: Self = Self(::std::borrow::Cow::Borrowed("wiper"));
        pub const WORM: Self = Self(::std::borrow::Cow::Borrowed("worm"));
    }
    impl crate::vocab::Vocabulary for MalwareType {
        const TYPE: &'static str = "malware-type-ov";
        fn is_known_value(&self) -> bool {
            self == "adware"
                || self == "backdoor"
                || self == "bot"
                || self == "bootkit"
                || self == "ddos"
                || self == "downloader"
                || self == "dropper"
                || self == "exploit-kit"
                || self == "keylogger"
                || self == "ransomware"
                || self == "remote-access-trojan"
                || self == "resource-exploitation"
                || self == "rogue-security-software"
                || self == "rootkit"
                || self == "screen-capture"
                || self == "spyware"
                || self == "trojan"
                || self == "unknown"
                || self == "virus"
                || self == "webshell"
                || self == "wiper"
                || self == "worm"
        }
    }
    impl ::std::fmt::Display for MalwareType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }
    impl ::std::convert::AsRef<str> for MalwareType {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }
    impl ::std::cmp::PartialEq<str> for MalwareType {
        fn eq(&self, rhs: &str) -> bool {
            self.0 == rhs
        }
    }
    impl ::std::convert::From<String> for MalwareType {
        fn from(s: String) -> Self {
            Self(::std::borrow::Cow::Owned(s))
        }
    }
    impl ::std::convert::From<&'static str> for MalwareType {
        fn from(s: &'static str) -> Self {
            Self(::std::borrow::Cow::Borrowed(s))
        }
    }
    pub struct ProcessorArchitecture(::std::borrow::Cow<'static, str>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ProcessorArchitecture {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                ProcessorArchitecture(ref __self_0_0) => {
                    let mut debug_trait_builder = f.debug_tuple("ProcessorArchitecture");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for ProcessorArchitecture {
        #[inline]
        fn clone(&self) -> ProcessorArchitecture {
            match *self {
                ProcessorArchitecture(ref __self_0_0) => {
                    ProcessorArchitecture(::core::clone::Clone::clone(&(*__self_0_0)))
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for ProcessorArchitecture {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for ProcessorArchitecture {
        #[inline]
        fn eq(&self, other: &ProcessorArchitecture) -> bool {
            match *other {
                ProcessorArchitecture(ref __self_1_0) => match *self {
                    ProcessorArchitecture(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ProcessorArchitecture) -> bool {
            match *other {
                ProcessorArchitecture(ref __self_1_0) => match *self {
                    ProcessorArchitecture(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl ::core::marker::StructuralEq for ProcessorArchitecture {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for ProcessorArchitecture {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<::std::borrow::Cow<'static, str>>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialOrd for ProcessorArchitecture {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ProcessorArchitecture,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match *other {
                ProcessorArchitecture(ref __self_1_0) => match *self {
                    ProcessorArchitecture(ref __self_0_0) => {
                        match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)) {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                            }
                            cmp => cmp,
                        }
                    }
                },
            }
        }
        #[inline]
        fn lt(&self, other: &ProcessorArchitecture) -> bool {
            match *other {
                ProcessorArchitecture(ref __self_1_0) => match *self {
                    ProcessorArchitecture(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Greater,
                        ) == ::core::cmp::Ordering::Less
                    }
                },
            }
        }
        #[inline]
        fn le(&self, other: &ProcessorArchitecture) -> bool {
            match *other {
                ProcessorArchitecture(ref __self_1_0) => match *self {
                    ProcessorArchitecture(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Greater,
                        ) != ::core::cmp::Ordering::Greater
                    }
                },
            }
        }
        #[inline]
        fn gt(&self, other: &ProcessorArchitecture) -> bool {
            match *other {
                ProcessorArchitecture(ref __self_1_0) => match *self {
                    ProcessorArchitecture(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Less,
                        ) == ::core::cmp::Ordering::Greater
                    }
                },
            }
        }
        #[inline]
        fn ge(&self, other: &ProcessorArchitecture) -> bool {
            match *other {
                ProcessorArchitecture(ref __self_1_0) => match *self {
                    ProcessorArchitecture(ref __self_0_0) => {
                        ::core::option::Option::unwrap_or(
                            ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                            ::core::cmp::Ordering::Less,
                        ) != ::core::cmp::Ordering::Less
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Ord for ProcessorArchitecture {
        #[inline]
        fn cmp(&self, other: &ProcessorArchitecture) -> ::core::cmp::Ordering {
            match *other {
                ProcessorArchitecture(ref __self_1_0) => match *self {
                    ProcessorArchitecture(ref __self_0_0) => {
                        match ::core::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                            ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                            cmp => cmp,
                        }
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::hash::Hash for ProcessorArchitecture {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                ProcessorArchitecture(ref __self_0_0) => {
                    ::core::hash::Hash::hash(&(*__self_0_0), state)
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ProcessorArchitecture {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ProcessorArchitecture>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ProcessorArchitecture;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "tuple struct ProcessorArchitecture",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::__private::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: ::std::borrow::Cow<'static, str> = match <::std::borrow::Cow<
                            'static,
                            str,
                        > as _serde::Deserialize>::deserialize(
                            __e
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(ProcessorArchitecture(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            ::std::borrow::Cow<'static, str>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"tuple struct ProcessorArchitecture with 1 element",
                                ));
                            }
                        };
                        _serde::__private::Ok(ProcessorArchitecture(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "ProcessorArchitecture",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ProcessorArchitecture>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ProcessorArchitecture {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_newtype_struct(
                    __serializer,
                    "ProcessorArchitecture",
                    &self.0,
                )
            }
        }
    };
    impl ProcessorArchitecture {
        pub const ALPHA: Self = Self(::std::borrow::Cow::Borrowed("alpha"));
        pub const ARM: Self = Self(::std::borrow::Cow::Borrowed("arm"));
        pub const IA_64: Self = Self(::std::borrow::Cow::Borrowed("ia-64"));
        pub const MIPS: Self = Self(::std::borrow::Cow::Borrowed("mips"));
        pub const POWERPC: Self = Self(::std::borrow::Cow::Borrowed("powerpc"));
        pub const SPARC: Self = Self(::std::borrow::Cow::Borrowed("sparc"));
        pub const X86: Self = Self(::std::borrow::Cow::Borrowed("x86"));
        pub const X86_64: Self = Self(::std::borrow::Cow::Borrowed("x86-64"));
    }
    impl crate::vocab::Vocabulary for ProcessorArchitecture {
        const TYPE: &'static str = "processor-architecture-ov";
        fn is_known_value(&self) -> bool {
            self == "alpha"
                || self == "arm"
                || self == "ia-64"
                || self == "mips"
                || self == "powerpc"
                || self == "sparc"
                || self == "x86"
                || self == "x86-64"
        }
    }
    impl ::std::fmt::Display for ProcessorArchitecture {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }
    impl ::std::convert::AsRef<str> for ProcessorArchitecture {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }
    impl ::std::cmp::PartialEq<str> for ProcessorArchitecture {
        fn eq(&self, rhs: &str) -> bool {
            self.0 == rhs
        }
    }
    impl ::std::convert::From<String> for ProcessorArchitecture {
        fn from(s: String) -> Self {
            Self(::std::borrow::Cow::Owned(s))
        }
    }
    impl ::std::convert::From<&'static str> for ProcessorArchitecture {
        fn from(s: &'static str) -> Self {
            Self(::std::borrow::Cow::Borrowed(s))
        }
    }
}
mod vulnerability {
    use serde::Deserialize;
    use crate::CommonProperties;
    #[typed_object(core)]
    pub struct Vulnerability {
        #[serde(flatten)]
        base: CommonProperties,
        pub name: String,
        #[serde(default)]
        pub description: Option<String>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Vulnerability {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field<'de> {
                    __field1,
                    __field2,
                    __other(_serde::__private::de::Content<'de>),
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field<'de>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_bool<__E>(
                        self,
                        __value: bool,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Bool(__value),
                        ))
                    }
                    fn visit_i8<__E>(
                        self,
                        __value: i8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(_serde::__private::de::Content::I8(
                            __value,
                        )))
                    }
                    fn visit_i16<__E>(
                        self,
                        __value: i16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I16(__value),
                        ))
                    }
                    fn visit_i32<__E>(
                        self,
                        __value: i32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I32(__value),
                        ))
                    }
                    fn visit_i64<__E>(
                        self,
                        __value: i64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::I64(__value),
                        ))
                    }
                    fn visit_u8<__E>(
                        self,
                        __value: u8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(_serde::__private::de::Content::U8(
                            __value,
                        )))
                    }
                    fn visit_u16<__E>(
                        self,
                        __value: u16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U16(__value),
                        ))
                    }
                    fn visit_u32<__E>(
                        self,
                        __value: u32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U32(__value),
                        ))
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::U64(__value),
                        ))
                    }
                    fn visit_f32<__E>(
                        self,
                        __value: f32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::F32(__value),
                        ))
                    }
                    fn visit_f64<__E>(
                        self,
                        __value: f64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::F64(__value),
                        ))
                    }
                    fn visit_char<__E>(
                        self,
                        __value: char,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Char(__value),
                        ))
                    }
                    fn visit_unit<__E>(self) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(__Field::__other(
                            _serde::__private::de::Content::Unit,
                        ))
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field1),
                            "description" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = _serde::__private::de::Content::String(
                                    _serde::__private::ToString::to_string(__value),
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field1),
                            b"description" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value =
                                    _serde::__private::de::Content::ByteBuf(__value.to_vec());
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_str<__E>(
                        self,
                        __value: &'de str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field1),
                            "description" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = _serde::__private::de::Content::Str(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_bytes<__E>(
                        self,
                        __value: &'de [u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field1),
                            b"description" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = _serde::__private::de::Content::Bytes(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field<'de> {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Vulnerability>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Vulnerability;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct Vulnerability")
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Option<String>> =
                            _serde::__private::None;
                        let mut __collect = _serde::__private::Vec::<
                            _serde::__private::Option<(
                                _serde::__private::de::Content,
                                _serde::__private::de::Content,
                            )>,
                        >::new();
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "description",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__other(__name) => {
                                    __collect.push(_serde::__private::Some((
                                        __name,
                                        match _serde::de::MapAccess::next_value(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    )));
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => _serde::__private::Default::default(),
                        };
                        let __field0: CommonProperties = match _serde::de::Deserialize::deserialize(
                            _serde::__private::de::FlatMapDeserializer(
                                &mut __collect,
                                _serde::__private::PhantomData,
                            ),
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::__private::Ok(Vulnerability {
                            base: __field0,
                            name: __field1,
                            description: __field2,
                        })
                    }
                }
                _serde::Deserializer::deserialize_map(
                    __deserializer,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Vulnerability>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl crate::TypedObject for Vulnerability {
        const TYPE: &'static str = "vulnerability";
    }
    impl Vulnerability {
        pub fn name(&self) -> &str {
            &self.name
        }
        pub fn description(&self) -> Option<&str> {
            self.description.as_ref().map(|s| s.as_str())
        }
    }
    impl AsRef<CommonProperties> for Vulnerability {
        fn as_ref(&self) -> &CommonProperties {
            &self.base
        }
    }
}
pub use attack_pattern::AttackPattern;
pub use bundle::Bundle;
pub use campaign::Campaign;
pub use course_of_action::CourseOfAction;
pub use id::{Id, IdParseError};
#[doc(inline)]
pub use identity::Identity;
pub use infrastructure::Infrastructure;
pub use intrusion_set::IntrusionSet;
pub use location::Location;
pub use malware::Malware;
pub use marking_definition::MarkingDefinition;
pub use object::{CommonProperties, Object, TypedObject};
pub use reference::{ExternalReference, KillChainPhase};
pub use relationship::{Relationship, RelationshipType};
pub use relationship_graph::RelationshipGraph;
pub use threat_actor::ThreatActor;
pub use tool::Tool;
pub use vulnerability::Vulnerability;
pub use stix_derive::*;
#[doc(hidden)]
pub mod export {
    pub use indexmap::IndexMap;
    pub use once_self_cell::sync_once_self_cell;
    pub mod petgraph {
        pub use ::petgraph::{graph::NodeIndex, Graph};
    }
}
#[doc = " Trait for turning a reference in a STIX collection into a data-carrying node."]
pub trait Resolve {
    #[doc = " The node type, containing a reference to the data and the backing collection."]
    type Output;
    #[doc = " Produce a collection-attached node for the object identified by the ID."]
    fn resolve(self) -> Option<Self::Output>;
}
