#[cfg(feature = "std")]
use std::collections::HashMap;

use prost::alloc::collections::BTreeMap;
use prost::alloc::vec;
#[cfg(not(feature = "std"))]
use prost::alloc::{borrow::ToOwned, string::String, vec::Vec};

use prost::bytes::{BufMut, Bytes};
#[cfg(feature = "std")]
use prost::transfer::{AsyncEncodeRefExt as _, AsyncEncodeTarget, EncodePayload};
use prost::{Enumeration, Message, Oneof};

use crate::check_message;
use crate::check_serialize_equivalent;

#[derive(Clone, PartialEq, Message)]
pub struct RepeatedFloats {
    #[prost(float, tag = "11")]
    pub single_float: f32,
    #[prost(float, repeated, packed = "true", tag = "41")]
    pub repeated_float: Vec<f32>,
}

#[derive(Clone, PartialEq, Message)]
pub struct AsyncScalarOnly {
    #[prost(int32, tag = "1")]
    pub int32: i32,
    #[prost(bool, repeated, packed = "false", tag = "2")]
    pub bools: Vec<bool>,
}

#[test]
fn check_repeated_floats() {
    check_message(&RepeatedFloats {
        single_float: 0.0,
        repeated_float: vec![
            0.1,
            340282300000000000000000000000000000000.0,
            0.000000000000000000000000000000000000011754944,
        ],
    });
}

#[test]
fn check_scalar_types() {
    check_message(&ScalarTypes::default());
}

/// A protobuf message which contains all scalar types.
#[derive(Clone, PartialEq, Message)]
pub struct ScalarTypes {
    #[prost(int32, tag = "001")]
    pub int32: i32,
    #[prost(int64, tag = "002")]
    pub int64: i64,
    #[prost(uint32, tag = "003")]
    pub uint32: u32,
    #[prost(uint64, tag = "004")]
    pub uint64: u64,
    #[prost(sint32, tag = "005")]
    pub sint32: i32,
    #[prost(sint64, tag = "006")]
    pub sint64: i64,
    #[prost(fixed32, tag = "007")]
    pub fixed32: u32,
    #[prost(fixed64, tag = "008")]
    pub fixed64: u64,
    #[prost(sfixed32, tag = "009")]
    pub sfixed32: i32,
    #[prost(sfixed64, tag = "010")]
    pub sfixed64: i64,
    #[prost(float, tag = "011")]
    pub float: f32,
    #[prost(double, tag = "012")]
    pub double: f64,
    #[prost(bool, tag = "013")]
    pub _bool: bool,
    #[prost(string, tag = "014")]
    pub string: String,
    #[prost(bytes = "vec", tag = "015")]
    pub bytes_vec: Vec<u8>,
    #[prost(bytes = "bytes", tag = "016")]
    pub bytes_buf: Bytes,

    #[prost(int32, required, tag = "101")]
    pub required_int32: i32,
    #[prost(int64, required, tag = "102")]
    pub required_int64: i64,
    #[prost(uint32, required, tag = "103")]
    pub required_uint32: u32,
    #[prost(uint64, required, tag = "104")]
    pub required_uint64: u64,
    #[prost(sint32, required, tag = "105")]
    pub required_sint32: i32,
    #[prost(sint64, required, tag = "106")]
    pub required_sint64: i64,
    #[prost(fixed32, required, tag = "107")]
    pub required_fixed32: u32,
    #[prost(fixed64, required, tag = "108")]
    pub required_fixed64: u64,
    #[prost(sfixed32, required, tag = "109")]
    pub required_sfixed32: i32,
    #[prost(sfixed64, required, tag = "110")]
    pub required_sfixed64: i64,
    #[prost(float, required, tag = "111")]
    pub required_float: f32,
    #[prost(double, required, tag = "112")]
    pub required_double: f64,
    #[prost(bool, required, tag = "113")]
    pub required_bool: bool,
    #[prost(string, required, tag = "114")]
    pub required_string: String,
    #[prost(bytes = "vec", required, tag = "115")]
    pub required_bytes_vec: Vec<u8>,
    #[prost(bytes = "bytes", required, tag = "116")]
    pub required_bytes_buf: Bytes,

    #[prost(int32, optional, tag = "201")]
    pub optional_int32: Option<i32>,
    #[prost(int64, optional, tag = "202")]
    pub optional_int64: Option<i64>,
    #[prost(uint32, optional, tag = "203")]
    pub optional_uint32: Option<u32>,
    #[prost(uint64, optional, tag = "204")]
    pub optional_uint64: Option<u64>,
    #[prost(sint32, optional, tag = "205")]
    pub optional_sint32: Option<i32>,
    #[prost(sint64, optional, tag = "206")]
    pub optional_sint64: Option<i64>,

    #[prost(fixed32, optional, tag = "207")]
    pub optional_fixed32: Option<u32>,
    #[prost(fixed64, optional, tag = "208")]
    pub optional_fixed64: Option<u64>,
    #[prost(sfixed32, optional, tag = "209")]
    pub optional_sfixed32: Option<i32>,
    #[prost(sfixed64, optional, tag = "210")]
    pub optional_sfixed64: Option<i64>,
    #[prost(float, optional, tag = "211")]
    pub optional_float: Option<f32>,
    #[prost(double, optional, tag = "212")]
    pub optional_double: Option<f64>,
    #[prost(bool, optional, tag = "213")]
    pub optional_bool: Option<bool>,
    #[prost(string, optional, tag = "214")]
    pub optional_string: Option<String>,
    #[prost(bytes = "vec", optional, tag = "215")]
    pub optional_bytes_vec: Option<Vec<u8>>,
    #[prost(bytes = "bytes", optional, tag = "216")]
    pub optional_bytes_buf: Option<Bytes>,

    #[prost(int32, repeated, packed = "false", tag = "301")]
    pub repeated_int32: Vec<i32>,
    #[prost(int64, repeated, packed = "false", tag = "302")]
    pub repeated_int64: Vec<i64>,
    #[prost(uint32, repeated, packed = "false", tag = "303")]
    pub repeated_uint32: Vec<u32>,
    #[prost(uint64, repeated, packed = "false", tag = "304")]
    pub repeated_uint64: Vec<u64>,
    #[prost(sint32, repeated, packed = "false", tag = "305")]
    pub repeated_sint32: Vec<i32>,
    #[prost(sint64, repeated, packed = "false", tag = "306")]
    pub repeated_sint64: Vec<i64>,
    #[prost(fixed32, repeated, packed = "false", tag = "307")]
    pub repeated_fixed32: Vec<u32>,
    #[prost(fixed64, repeated, packed = "false", tag = "308")]
    pub repeated_fixed64: Vec<u64>,
    #[prost(sfixed32, repeated, packed = "false", tag = "309")]
    pub repeated_sfixed32: Vec<i32>,
    #[prost(sfixed64, repeated, packed = "false", tag = "310")]
    pub repeated_sfixed64: Vec<i64>,
    #[prost(float, repeated, packed = "false", tag = "311")]
    pub repeated_float: Vec<f32>,
    #[prost(double, repeated, packed = "false", tag = "312")]
    pub repeated_double: Vec<f64>,
    #[prost(bool, repeated, packed = "false", tag = "313")]
    pub repeated_bool: Vec<bool>,
    #[prost(string, repeated, packed = "false", tag = "315")]
    pub repeated_string: Vec<String>,
    #[prost(bytes = "vec", repeated, packed = "false", tag = "316")]
    pub repeated_bytes_vec: Vec<Vec<u8>>,
    #[prost(bytes = "bytes", repeated, packed = "false", tag = "317")]
    pub repeated_bytes_buf: Vec<Bytes>,

    #[prost(int32, repeated, tag = "401")]
    pub packed_int32: Vec<i32>,
    #[prost(int64, repeated, tag = "402")]
    pub packed_int64: Vec<i64>,
    #[prost(uint32, repeated, tag = "403")]
    pub packed_uint32: Vec<u32>,
    #[prost(uint64, repeated, tag = "404")]
    pub packed_uint64: Vec<u64>,
    #[prost(sint32, repeated, tag = "405")]
    pub packed_sint32: Vec<i32>,
    #[prost(sint64, repeated, tag = "406")]
    pub packed_sint64: Vec<i64>,
    #[prost(fixed32, repeated, tag = "407")]
    pub packed_fixed32: Vec<u32>,

    #[prost(fixed64, repeated, tag = "408")]
    pub packed_fixed64: Vec<u64>,
    #[prost(sfixed32, repeated, tag = "409")]
    pub packed_sfixed32: Vec<i32>,
    #[prost(sfixed64, repeated, tag = "410")]
    pub packed_sfixed64: Vec<i64>,
    #[prost(float, repeated, tag = "411")]
    pub packed_float: Vec<f32>,
    #[prost(double, repeated, tag = "412")]
    pub packed_double: Vec<f64>,
    #[prost(bool, repeated, tag = "413")]
    pub packed_bool: Vec<bool>,
    #[prost(string, repeated, tag = "415")]
    pub packed_string: Vec<String>,
    #[prost(bytes = "vec", repeated, tag = "416")]
    pub packed_bytes_vec: Vec<Vec<u8>>,
    #[prost(bytes = "bytes", repeated, tag = "417")]
    pub packed_bytes_buf: Vec<Bytes>,
}

#[test]
fn check_tags_inferred() {
    check_message(&TagsInferred::default());
    check_serialize_equivalent(&TagsInferred::default(), &TagsQualified::default());

    let tags_inferred = TagsInferred {
        one: true,
        two: Some(42),
        three: vec![0.0, 1.0, 1.0],
        skip_to_nine: "nine".to_owned(),
        ten: 0,
        eleven: ::alloc::collections::BTreeMap::new(),
        back_to_five: vec![1, 0, 1],
        six: Basic::default(),
    };
    check_message(&tags_inferred);

    let tags_qualified = TagsQualified {
        one: true,
        two: Some(42),
        three: vec![0.0, 1.0, 1.0],
        five: vec![1, 0, 1],
        six: Basic::default(),
        nine: "nine".to_owned(),
        ten: 0,
        eleven: ::alloc::collections::BTreeMap::new(),
    };
    check_serialize_equivalent(&tags_inferred, &tags_qualified);
}

#[derive(Clone, PartialEq, Message)]
pub struct TagsInferred {
    #[prost(bool)]
    pub one: bool,
    #[prost(int32, optional)]
    pub two: Option<i32>,
    #[prost(float, repeated)]
    pub three: Vec<f32>,

    #[prost(tag = "9", string, required)]
    pub skip_to_nine: String,
    #[prost(enumeration = "BasicEnumeration", default = "ONE")]
    pub ten: i32,
    #[prost(btree_map = "string, string")]
    pub eleven: ::alloc::collections::BTreeMap<String, String>,

    #[prost(tag = "5", bytes)]
    pub back_to_five: Vec<u8>,
    #[prost(message, required)]
    pub six: Basic,
}

#[derive(Clone, PartialEq, Message)]
pub struct TagsQualified {
    #[prost(tag = "1", bool)]
    pub one: bool,
    #[prost(tag = "2", int32, optional)]
    pub two: Option<i32>,
    #[prost(tag = "3", float, repeated)]
    pub three: Vec<f32>,

    #[prost(tag = "5", bytes)]
    pub five: Vec<u8>,
    #[prost(tag = "6", message, required)]
    pub six: Basic,

    #[prost(tag = "9", string, required)]
    pub nine: String,
    #[prost(tag = "10", enumeration = "BasicEnumeration", default = "ONE")]
    pub ten: i32,
    #[prost(tag = "11", btree_map = "string, string")]
    pub eleven: ::alloc::collections::BTreeMap<String, String>,
}

/// A prost message with default value.
#[derive(Clone, PartialEq, Message)]
pub struct DefaultValues {
    #[prost(int32, tag = "1", default = "42")]
    pub int32: i32,

    #[prost(int32, optional, tag = "2", default = "88")]
    pub optional_int32: Option<i32>,

    #[prost(string, tag = "3", default = "forty two")]
    pub string: String,

    #[prost(bytes = "vec", tag = "7", default = "b\"foo\\x00bar\"")]
    pub bytes_vec: Vec<u8>,

    #[prost(bytes = "bytes", tag = "8", default = "b\"foo\\x00bar\"")]
    pub bytes_buf: Bytes,

    #[prost(enumeration = "BasicEnumeration", tag = "4", default = "ONE")]
    pub enumeration: i32,

    #[prost(enumeration = "BasicEnumeration", optional, tag = "5", default = "TWO")]
    pub optional_enumeration: Option<i32>,

    #[prost(enumeration = "BasicEnumeration", repeated, tag = "6")]
    pub repeated_enumeration: Vec<i32>,
}

#[test]
fn check_default_values() {
    let default = DefaultValues::default();
    assert_eq!(default.int32, 42);
    assert_eq!(default.optional_int32, None);
    assert_eq!(&default.string, "forty two");
    assert_eq!(&default.bytes_vec.as_ref(), b"foo\0bar");
    assert_eq!(&default.bytes_buf.as_ref(), b"foo\0bar");
    assert_eq!(default.enumeration, BasicEnumeration::ONE as i32);
    assert_eq!(default.optional_enumeration, None);
    assert_eq!(&default.repeated_enumeration, &[]);
    assert_eq!(0, default.encoded_len());
}

/// A protobuf enum.
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, PartialEq, Enumeration)]
pub enum BasicEnumeration {
    ZERO = 0,
    ONE = 1,
    TWO = 2,
    THREE = 3,
}

#[derive(Clone, PartialEq, Message)]
pub struct Basic {
    #[prost(int32, tag = "1")]
    pub int32: i32,

    #[prost(bool, repeated, packed = "false", tag = "2")]
    pub bools: Vec<bool>,

    #[prost(string, tag = "3")]
    pub string: String,

    #[prost(string, optional, tag = "4")]
    pub optional_string: Option<String>,

    #[prost(enumeration = "BasicEnumeration", tag = "5")]
    pub enumeration: i32,

    #[prost(map = "int32, enumeration(BasicEnumeration)", tag = "6")]
    #[cfg(feature = "std")]
    pub enumeration_map: ::std::collections::HashMap<i32, i32>,

    #[prost(hash_map = "string, string", tag = "7")]
    #[cfg(feature = "std")]
    pub string_map: ::std::collections::HashMap<String, String>,

    #[prost(btree_map = "int32, enumeration(BasicEnumeration)", tag = "10")]
    pub enumeration_btree_map: prost::alloc::collections::BTreeMap<i32, i32>,

    #[prost(btree_map = "string, string", tag = "11")]
    pub string_btree_map: prost::alloc::collections::BTreeMap<String, String>,

    #[prost(oneof = "BasicOneof", tags = "8, 9")]
    pub oneof: Option<BasicOneof>,

    #[prost(map = "string, bytes", tag = "12")]
    #[cfg(feature = "std")]
    pub bytes_map: ::std::collections::HashMap<String, Vec<u8>>,
}

#[derive(Clone, PartialEq, Message)]
pub struct Compound {
    #[prost(message, optional, tag = "1")]
    pub optional_message: Option<Basic>,

    #[prost(message, required, tag = "2")]
    pub required_message: Basic,

    #[prost(message, repeated, tag = "3")]
    pub repeated_message: Vec<Basic>,

    #[prost(map = "sint32, message", tag = "4")]
    #[cfg(feature = "std")]
    pub message_map: ::std::collections::HashMap<i32, Basic>,

    #[prost(btree_map = "sint32, message", tag = "5")]
    pub message_btree_map: prost::alloc::collections::BTreeMap<i32, Basic>,
}

#[derive(Clone, PartialEq, Oneof)]
pub enum BasicOneof {
    #[prost(int32, tag = "8")]
    Int(i32),
    #[prost(string, tag = "9")]
    String(String),
}

#[cfg(feature = "std")]
#[derive(Clone, PartialEq, Message)]
pub struct AsyncPayloadNested {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(bytes = "vec", tag = "3")]
    pub payload: Vec<u8>,
}

#[cfg(feature = "std")]
#[derive(Clone, PartialEq, Message)]
pub struct AsyncPayloadE2e {
    #[prost(int32, tag = "1")]
    pub scalar: i32,
    #[prost(bool, repeated, packed = "false", tag = "2")]
    pub flags: Vec<bool>,
    #[prost(string, tag = "3")]
    pub name: String,
    #[prost(bytes = "vec", tag = "4")]
    pub payload: Vec<u8>,
    #[prost(string, repeated, tag = "5")]
    pub repeated_names: Vec<String>,
    #[prost(bytes = "vec", repeated, tag = "6")]
    pub repeated_payloads: Vec<Vec<u8>>,
    #[prost(map = "string, string", tag = "7")]
    pub string_map: HashMap<String, String>,
    #[prost(map = "string, bytes", tag = "8")]
    pub bytes_map: HashMap<String, Vec<u8>>,
    #[prost(message, optional, tag = "9")]
    pub nested: Option<AsyncPayloadNested>,
    #[prost(message, repeated, tag = "10")]
    pub repeated_nested: Vec<AsyncPayloadNested>,
    #[prost(map = "string, message", tag = "11")]
    pub nested_map: HashMap<String, AsyncPayloadNested>,
    #[prost(oneof = "AsyncPayloadOneof", tags = "12, 13")]
    pub payload_oneof: Option<AsyncPayloadOneof>,
    #[prost(oneof = "AsyncNestedOneof", tags = "14, 15")]
    pub nested_oneof: Option<AsyncNestedOneof>,
}

#[cfg(feature = "std")]
#[derive(Clone, PartialEq, Oneof)]
pub enum AsyncPayloadOneof {
    #[prost(bytes = "vec", tag = "12")]
    Payload(Vec<u8>),
    #[prost(string, tag = "13")]
    Text(String),
}

#[cfg(feature = "std")]
#[derive(Clone, PartialEq, Oneof)]
pub enum AsyncNestedOneof {
    #[prost(message, tag = "14")]
    Nested(AsyncPayloadNested),
    #[prost(string, tag = "15")]
    Text(String),
}

#[test]
fn roundtrip() {
    let basic = Basic {
        int32: 123,
        bools: Vec::from([true, false, true]),
        string: "Hello".into(),
        optional_string: Some("World".into()),
        enumeration: 12,
        #[cfg(feature = "std")]
        enumeration_map: HashMap::from([(1, 2), (3, 4)]),
        #[cfg(feature = "std")]
        string_map: HashMap::from([("foo".into(), "bar".into()), ("baz".into(), "boo".into())]),
        enumeration_btree_map: BTreeMap::from([(5, 6), (7, 8)]),
        string_btree_map: BTreeMap::from([
            ("xfoo".into(), "xbar".into()),
            ("xbaz".into(), "xboo".into()),
        ]),
        oneof: Some(BasicOneof::Int(456)),
        #[cfg(feature = "std")]
        bytes_map: HashMap::from([("foo".into(), "bar".into()), ("baz".into(), "boo".into())]),
    };

    let msg = Compound {
        optional_message: Some(basic.clone()),
        required_message: basic.clone(),
        repeated_message: Vec::from([basic.clone()]),
        #[cfg(feature = "std")]
        message_map: HashMap::from([(1, basic.clone()), (2, basic.clone())]),
        message_btree_map: BTreeMap::from([(3, basic.clone()), (4, basic.clone())]),
    };
    check_message(&msg);
}

#[cfg(feature = "std")]
#[test]
fn async_encode_generated_string_bytes_matches_sync() {
    let scalar = ScalarTypes {
        string: "plain string payload".to_owned(),
        bytes_vec: b"plain vec bytes".to_vec(),
        bytes_buf: Bytes::from_static(b"plain Bytes payload"),
        required_string: "required string payload".to_owned(),
        required_bytes_vec: b"required vec bytes".to_vec(),
        required_bytes_buf: Bytes::from_static(b"required Bytes payload"),
        optional_string: Some("optional string payload".to_owned()),
        optional_bytes_vec: Some(b"optional vec bytes".to_vec()),
        optional_bytes_buf: Some(Bytes::from_static(b"optional Bytes payload")),
        repeated_string: vec![
            "repeated string a".to_owned(),
            "repeated string b".to_owned(),
        ],
        repeated_bytes_vec: vec![b"repeated vec a".to_vec(), b"repeated vec b".to_vec()],
        repeated_bytes_buf: vec![
            Bytes::from_static(b"repeated Bytes a"),
            Bytes::from_static(b"repeated Bytes b"),
        ],
        packed_string: vec![
            "packed-name string a".to_owned(),
            "packed-name string b".to_owned(),
        ],
        packed_bytes_vec: vec![b"packed-name vec a".to_vec(), b"packed-name vec b".to_vec()],
        packed_bytes_buf: vec![
            Bytes::from_static(b"packed-name Bytes a"),
            Bytes::from_static(b"packed-name Bytes b"),
        ],
        ..Default::default()
    };
    assert_async_encode_matches_sync(&scalar);

    let basic = Basic {
        int32: 123,
        bools: Vec::from([true, false, true]),
        string: "nested string".into(),
        optional_string: Some("nested optional string".into()),
        enumeration: BasicEnumeration::TWO as i32,
        enumeration_map: HashMap::from([(1, BasicEnumeration::ONE as i32)]),
        string_map: HashMap::from([("hash key".into(), "hash value".into())]),
        enumeration_btree_map: BTreeMap::from([(5, BasicEnumeration::THREE as i32)]),
        string_btree_map: BTreeMap::from([
            ("btree key a".into(), "btree value a".into()),
            ("btree key b".into(), "btree value b".into()),
        ]),
        oneof: Some(BasicOneof::String("oneof string payload".into())),
        bytes_map: HashMap::from([("bytes key".into(), b"bytes map value".to_vec())]),
    };
    assert_async_encode_matches_sync(&basic);

    let compound = Compound {
        optional_message: Some(basic.clone()),
        required_message: basic.clone(),
        repeated_message: Vec::from([basic.clone(), basic.clone()]),
        message_map: HashMap::from([(1, basic.clone())]),
        message_btree_map: BTreeMap::from([(2, basic)]),
    };
    assert_async_encode_matches_sync(&compound);
}

#[cfg(feature = "std")]
#[test]
fn async_encode_scalar_only_is_ready_with_pending_target() {
    let message = AsyncScalarOnly {
        int32: 42,
        bools: Vec::from([true, false, true]),
    };

    let mut expected = Vec::with_capacity(message.encoded_len());
    message.encode(&mut expected).expect("sync encode succeeds");

    let mut actual = Vec::with_capacity(message.encoded_len());
    let mut target = PendingOnceTarget::new(&mut actual);
    let (result, pending_polls) = poll_until_ready(message.encode_async_ref(&mut target));
    result.expect("async encode succeeds");

    assert_eq!(pending_polls, 0);
    assert_eq!(target.pending_payloads, 0);
    assert_eq!(actual, expected);
}

#[cfg(feature = "std")]
#[test]
fn async_encode_pending_resume_plain_payloads_match_sync() {
    let message = ScalarTypes {
        string: "plain string payload".to_owned(),
        bytes_vec: b"plain vec bytes".to_vec(),
        bytes_buf: Bytes::from_static(b"plain Bytes payload"),
        required_string: "required string payload".to_owned(),
        required_bytes_vec: b"required vec bytes".to_vec(),
        required_bytes_buf: Bytes::from_static(b"required Bytes payload"),
        optional_string: Some("optional string payload".to_owned()),
        optional_bytes_vec: Some(b"optional vec bytes".to_vec()),
        optional_bytes_buf: Some(Bytes::from_static(b"optional Bytes payload")),
        ..Default::default()
    };

    let pending_polls = assert_pending_async_encode_matches_sync(&message);
    assert_eq!(pending_polls, 9);
}

#[cfg(feature = "std")]
#[test]
fn async_encode_pending_resume_repeated_payloads_match_sync() {
    let message = ScalarTypes {
        repeated_string: vec![
            "repeated string a".to_owned(),
            "repeated string b".to_owned(),
            "repeated string c".to_owned(),
        ],
        repeated_bytes_vec: vec![b"repeated vec a".to_vec(), b"repeated vec b".to_vec()],
        repeated_bytes_buf: vec![
            Bytes::from_static(b"repeated Bytes a"),
            Bytes::from_static(b"repeated Bytes b"),
        ],
        ..Default::default()
    };

    let pending_polls = assert_pending_async_encode_matches_sync(&message);
    assert_eq!(pending_polls, 10);
}

#[cfg(feature = "std")]
#[test]
fn async_encode_pending_resume_maps_match_sync() {
    let message = Basic {
        string_btree_map: BTreeMap::from([
            ("btree key a".into(), "btree value a".into()),
            ("btree key b".into(), "btree value b".into()),
        ]),
        bytes_map: HashMap::from([("bytes key".into(), b"bytes map value".to_vec())]),
        ..Default::default()
    };

    let pending_polls = assert_pending_async_encode_matches_sync(&message);
    assert_eq!(pending_polls, 6);
}

#[cfg(feature = "std")]
#[test]
fn async_encode_pending_resume_nested_messages_match_sync() {
    let basic = Basic {
        int32: 123,
        bools: Vec::from([true, false, true]),
        string: "nested string".into(),
        optional_string: Some("nested optional string".into()),
        enumeration: BasicEnumeration::TWO as i32,
        enumeration_map: HashMap::from([(1, BasicEnumeration::ONE as i32)]),
        string_map: HashMap::from([("hash key".into(), "hash value".into())]),
        enumeration_btree_map: BTreeMap::from([(5, BasicEnumeration::THREE as i32)]),
        string_btree_map: BTreeMap::from([
            ("btree key a".into(), "btree value a".into()),
            ("btree key b".into(), "btree value b".into()),
        ]),
        oneof: Some(BasicOneof::String("oneof string payload".into())),
        bytes_map: HashMap::from([("bytes key".into(), b"bytes map value".to_vec())]),
    };
    let message = Compound {
        optional_message: Some(basic.clone()),
        required_message: basic.clone(),
        repeated_message: Vec::from([basic.clone(), basic.clone()]),
        message_map: HashMap::from([(1, basic.clone())]),
        message_btree_map: BTreeMap::from([(2, basic)]),
    };

    let pending_polls = assert_pending_async_encode_matches_sync(&message);
    assert!(pending_polls > 20);
}

#[cfg(feature = "std")]
#[test]
fn async_encode_e2e_fake_offload_resume_matches_sync_and_decodes() {
    let nested_a = AsyncPayloadNested {
        id: 7,
        name: "nested-a".into(),
        payload: b"nested-a payload".to_vec(),
    };
    let nested_b = AsyncPayloadNested {
        id: 8,
        name: "nested-b".into(),
        payload: b"nested-b payload".to_vec(),
    };
    let nested_c = AsyncPayloadNested {
        id: 9,
        name: "nested-c".into(),
        payload: b"nested-c payload".to_vec(),
    };
    let message = AsyncPayloadE2e {
        scalar: 99,
        flags: Vec::from([true, false, true]),
        name: "root name payload".into(),
        payload: b"root bytes payload".to_vec(),
        repeated_names: Vec::from(["repeat-a".into(), "repeat-b".into(), "repeat-c".into()]),
        repeated_payloads: Vec::from([
            b"repeated bytes a".to_vec(),
            b"repeated bytes b".to_vec(),
            b"repeated bytes c".to_vec(),
        ]),
        string_map: HashMap::from([
            ("string-map-key-a".into(), "string-map-value-a".into()),
            ("string-map-key-b".into(), "string-map-value-b".into()),
        ]),
        bytes_map: HashMap::from([
            ("bytes-map-key-a".into(), b"bytes-map-value-a".to_vec()),
            ("bytes-map-key-b".into(), b"bytes-map-value-b".to_vec()),
        ]),
        nested: Some(nested_a.clone()),
        repeated_nested: Vec::from([nested_a.clone(), nested_b.clone()]),
        nested_map: HashMap::from([
            ("nested-map-key-a".into(), nested_b.clone()),
            ("nested-map-key-b".into(), nested_c.clone()),
        ]),
        payload_oneof: Some(AsyncPayloadOneof::Payload(b"oneof bytes payload".to_vec())),
        nested_oneof: Some(AsyncNestedOneof::Nested(nested_c)),
    };

    let mut expected = Vec::with_capacity(message.encoded_len());
    message.encode(&mut expected).expect("sync encode succeeds");

    let mut actual = Vec::with_capacity(message.encoded_len());
    let mut target = FakeDsaTarget::new(&mut actual);
    let (result, pending_polls) = poll_until_ready(message.encode_async_ref(&mut target));
    result.expect("fake DSA async encode succeeds");

    assert_eq!(pending_polls, target.pending_payloads);
    assert_eq!(target.pending_payloads, target.completed_payloads);
    assert!(target.pending_payloads > 20);
    assert_eq!(actual, expected);

    let decoded = AsyncPayloadE2e::decode(actual.as_slice()).expect("async bytes decode");
    assert!(decoded == message);
}

#[cfg(feature = "std")]
#[derive(Debug, PartialEq, Eq)]
enum FakeDsaError {
    Encode(prost::EncodeError),
    ResumePayloadChanged { expected: Vec<u8>, actual: Vec<u8> },
}

#[cfg(feature = "std")]
#[derive(Default)]
struct FakeDsaPayloadState {
    staged: Option<Vec<u8>>,
}

#[cfg(feature = "std")]
struct FakeDsaTarget<'a> {
    buf: &'a mut Vec<u8>,
    pending_payloads: usize,
    completed_payloads: usize,
}

#[cfg(feature = "std")]
impl<'a> FakeDsaTarget<'a> {
    fn new(buf: &'a mut Vec<u8>) -> Self {
        Self {
            buf,
            pending_payloads: 0,
            completed_payloads: 0,
        }
    }
}

#[cfg(feature = "std")]
impl AsyncEncodeTarget for FakeDsaTarget<'_> {
    type Error = FakeDsaError;

    type BufMut<'a>
        = &'a mut Vec<u8>
    where
        Self: 'a;

    type PayloadState = FakeDsaPayloadState;

    fn encode_error(error: prost::EncodeError) -> Self::Error {
        FakeDsaError::Encode(error)
    }

    fn buf_mut(&mut self) -> Self::BufMut<'_> {
        self.buf
    }

    fn poll_write_payload(
        &mut self,
        payload: EncodePayload<'_>,
        state: core::pin::Pin<&mut Self::PayloadState>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Result<(), Self::Error>> {
        let payload = payload.as_bytes();
        let state = state.get_mut();
        if let Some(staged) = state.staged.take() {
            if staged.as_slice() != payload {
                return core::task::Poll::Ready(Err(FakeDsaError::ResumePayloadChanged {
                    expected: staged,
                    actual: payload.to_vec(),
                }));
            }

            self.buf.put_slice(&staged);
            self.completed_payloads += 1;
            return core::task::Poll::Ready(Ok(()));
        }

        state.staged = Some(payload.to_vec());
        self.pending_payloads += 1;
        cx.waker().wake_by_ref();
        core::task::Poll::Pending
    }
}

#[cfg(feature = "std")]
#[derive(Default)]
struct PendingPayloadState {
    returned_pending: bool,
}

#[cfg(feature = "std")]
struct PendingOnceTarget<'a> {
    buf: &'a mut Vec<u8>,
    pending_payloads: usize,
    completed_payloads: usize,
}

#[cfg(feature = "std")]
impl<'a> PendingOnceTarget<'a> {
    fn new(buf: &'a mut Vec<u8>) -> Self {
        Self {
            buf,
            pending_payloads: 0,
            completed_payloads: 0,
        }
    }
}

#[cfg(feature = "std")]
impl AsyncEncodeTarget for PendingOnceTarget<'_> {
    type Error = prost::EncodeError;

    type BufMut<'a>
        = &'a mut Vec<u8>
    where
        Self: 'a;

    type PayloadState = PendingPayloadState;

    fn encode_error(error: prost::EncodeError) -> Self::Error {
        error
    }

    fn buf_mut(&mut self) -> Self::BufMut<'_> {
        self.buf
    }

    fn poll_write_payload(
        &mut self,
        payload: EncodePayload<'_>,
        state: core::pin::Pin<&mut Self::PayloadState>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Result<(), Self::Error>> {
        let payload = payload.as_bytes();
        let state = state.get_mut();
        if !state.returned_pending {
            state.returned_pending = true;
            self.pending_payloads += 1;
            cx.waker().wake_by_ref();
            return core::task::Poll::Pending;
        }

        self.buf.put_slice(payload);
        self.completed_payloads += 1;
        *state = PendingPayloadState::default();
        core::task::Poll::Ready(Ok(()))
    }
}

#[cfg(feature = "std")]
fn assert_pending_async_encode_matches_sync<M>(message: &M) -> usize
where
    M: Message,
{
    let mut expected = Vec::with_capacity(message.encoded_len());
    message.encode(&mut expected).expect("sync encode succeeds");

    let mut actual = Vec::with_capacity(message.encoded_len());
    let mut target = PendingOnceTarget::new(&mut actual);
    let (result, pending_polls) = poll_until_ready(message.encode_async_ref(&mut target));
    result.expect("async encode succeeds");

    assert_eq!(target.pending_payloads, pending_polls);
    assert_eq!(target.completed_payloads, target.pending_payloads);
    assert_eq!(actual, expected);

    pending_polls
}

#[cfg(feature = "std")]
fn assert_async_encode_matches_sync<M>(message: &M)
where
    M: Message,
{
    let mut expected = Vec::with_capacity(message.encoded_len());
    message.encode(&mut expected).expect("sync encode succeeds");

    let mut actual = Vec::with_capacity(message.encoded_len());
    {
        let mut target = prost::transfer::BufMutEncodeTarget::new(&mut actual);
        poll_ready(message.encode_async_ref(&mut target)).expect("async encode succeeds");
    }

    assert_eq!(actual, expected);
}

#[cfg(feature = "std")]
fn poll_ready<F>(future: F) -> F::Output
where
    F: core::future::Future,
{
    let (output, pending_polls) = poll_until_ready(future);
    assert_eq!(pending_polls, 0, "CPU async encode returned pending");
    output
}

#[cfg(feature = "std")]
fn poll_until_ready<F>(future: F) -> (F::Output, usize)
where
    F: core::future::Future,
{
    let mut future = std::pin::pin!(future);
    let waker = std::task::Waker::noop();
    let mut cx = std::task::Context::from_waker(waker);
    let mut pending_polls = 0;

    loop {
        match future.as_mut().poll(&mut cx) {
            core::task::Poll::Ready(output) => return (output, pending_polls),
            core::task::Poll::Pending => pending_polls += 1,
        }
    }
}
