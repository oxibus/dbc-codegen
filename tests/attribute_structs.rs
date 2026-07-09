#![cfg(feature = "std")]

//! Tests for emitting user-defined structs populated from DBC attributes.

use dbc_codegen::{AttributeField, AttributeScope, AttributeStruct, Config, FieldSource};

const DBC: &str = r#"VERSION ""

NS_ :

BS_:

BU_: ECU1

BO_ 256 Protected: 8 ECU1
 SG_ TestSig : 16|8@1+ (1,0) [0|255] "" ECU1

BO_ 257 Plain: 8 ECU1
 SG_ OtherSig : 0|8@1+ (1,0) [0|255] "" ECU1

BO_ 258 BeProtected: 8 ECU1
 SG_ BeSig : 23|8@0+ (1,0) [0|255] "" ECU1

BA_DEF_ BO_  "SC_Message" ENUM  "0","1","2";
BA_DEF_ BO_  "SCP_FreshnessValueId" INT 0 65535;
BA_DEF_ SG_  "E2EDataId" INT 0 65535;
BA_DEF_ SG_  "E2EDataLength" INT 0 65535;
BA_DEF_ SG_  "E2EProfile" STRING ;
BA_DEF_DEF_  "SC_Message" "0";
BA_DEF_DEF_  "SCP_FreshnessValueId" 0;
BA_DEF_DEF_  "E2EDataId" 0;
BA_DEF_DEF_  "E2EDataLength" 0;
BA_DEF_DEF_  "E2EProfile" "none";
BA_ "SC_Message" BO_ 256 1;
BA_ "SCP_FreshnessValueId" BO_ 256 1002;
BA_ "E2EDataId" SG_ 256 TestSig 373;
BA_ "E2EDataLength" SG_ 256 TestSig 48;
BA_ "E2EDataId" SG_ 258 BeSig 500;
BA_ "E2EDataLength" SG_ 258 BeSig 16;
"#;

fn field(name: &'static str, source: FieldSource<'static>) -> AttributeField<'static> {
    AttributeField { name, source }
}

const E2E: AttributeStruct = AttributeStruct {
    type_path: "data_protection::E2EDataIdInfo",
    const_name: "E2E",
    scope: AttributeScope::Signal,
    require: "E2EDataId",
    fields: &[
        AttributeField {
            name: "data_id",
            source: FieldSource::Attr("E2EDataId"),
        },
        AttributeField {
            name: "start_byte",
            source: FieldSource::StartByte,
        },
        AttributeField {
            name: "width_bit",
            source: FieldSource::Attr("E2EDataLength"),
        },
        AttributeField {
            name: "profile",
            source: FieldSource::Attr("E2EProfile"),
        },
    ],
};

const SEC_OC: AttributeStruct = AttributeStruct {
    type_path: "data_protection::SecOcInfo",
    const_name: "SEC_OC",
    scope: AttributeScope::Message,
    require: "SCP_FreshnessValueId",
    fields: &[
        AttributeField {
            name: "freshness_id",
            source: FieldSource::Attr("SCP_FreshnessValueId"),
        },
        AttributeField {
            name: "sc_message",
            source: FieldSource::Attr("SC_Message"),
        },
    ],
};

fn try_generate(specs: &[AttributeStruct<'_>]) -> anyhow::Result<String> {
    Config::builder()
        .dbc_name("attribute_structs_test")
        .dbc_content(DBC)
        .attribute_structs(specs)
        .build()
        .generate()
}

fn generate(specs: &[AttributeStruct<'_>]) -> String {
    try_generate(specs).expect("generate")
}

#[test]
fn default_emits_nothing() {
    let out = generate(&[]);
    assert!(!out.contains("E2EDataIdInfo"), "{out}");
}

#[test]
fn signal_scope_emits_typed_struct_with_derived_layout() {
    let out = generate(&[E2E]);
    assert!(
        out.contains("pub const TEST_SIG_E2E: data_protection::E2EDataIdInfo"),
        "{out}"
    );
    assert!(out.contains("data_id: 373"), "{out}");
    assert!(out.contains("start_byte: 2"), "{out}");
    assert!(out.contains("width_bit: 48"), "{out}");
    assert!(out.contains(r#"profile: "none""#), "{out}");
}

#[test]
fn big_endian_start_byte_uses_start_bit_over_eight() {
    let out = generate(&[E2E]);
    assert!(
        out.contains("pub const BE_SIG_E2E: data_protection::E2EDataIdInfo"),
        "{out}"
    );
    assert!(out.contains("data_id: 500"), "{out}");
    assert!(out.contains("start_byte: 2"), "{out}");
}

#[test]
fn message_scope_emits_once_per_protected_message() {
    let out = generate(&[SEC_OC]);
    assert!(
        out.contains("pub const SEC_OC: data_protection::SecOcInfo"),
        "{out}"
    );
    assert!(out.contains("freshness_id: 1002"), "{out}");
}

#[test]
fn enum_attribute_emitted_as_integer_index() {
    let out = generate(&[SEC_OC]);
    assert!(out.contains("sc_message: 1"), "{out}");
}

#[test]
fn unprotected_message_and_signal_get_no_const() {
    let out = generate(&[E2E, SEC_OC]);
    assert!(!out.contains("OTHER_SIG_E2E"), "{out}");
    assert_eq!(out.matches("pub const SEC_OC").count(), 1, "{out}");
}

#[test]
fn message_scope_with_signal_source_is_rejected() {
    let bad = AttributeStruct {
        type_path: "foo::Bar",
        const_name: "BAD",
        scope: AttributeScope::Message,
        require: "SCP_FreshnessValueId",
        fields: &[field("x", FieldSource::StartByte)],
    };
    let err = format!("{:#}", try_generate(&[bad]).unwrap_err());
    assert!(err.contains("signal-only source"), "{err}");
}

#[test]
fn missing_field_value_is_an_error() {
    let bad = AttributeStruct {
        type_path: "foo::Bar",
        const_name: "BAD",
        scope: AttributeScope::Signal,
        require: "E2EDataId",
        fields: &[field("x", FieldSource::Attr("NoSuchAttr"))],
    };
    let err = format!("{:#}", try_generate(&[bad]).unwrap_err());
    assert!(err.contains("has no value"), "{err}");
}

#[test]
fn duplicate_const_name_is_an_error() {
    let dup = AttributeStruct {
        type_path: "foo::Bar",
        const_name: "SEC_OC",
        scope: AttributeScope::Message,
        require: "SCP_FreshnessValueId",
        fields: &[field("freshness_id", FieldSource::Attr("SCP_FreshnessValueId"))],
    };
    let err = format!("{:#}", try_generate(&[SEC_OC, dup]).unwrap_err());
    assert!(err.contains("collides"), "{err}");
}
