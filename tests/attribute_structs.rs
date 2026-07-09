#![cfg(feature = "std")]

//! Tests for emitting user-defined structs populated from DBC attributes and
//! message layout.

use dbc_codegen::{AttributeField, AttributeScope, AttributeStruct, Config, FieldSource};

const DBC: &str = r#"VERSION ""

NS_ :

BS_:

BU_: ECU1

BO_ 256 Protected: 8 ECU1
 SG_ TestSig : 16|8@1+ (1,0) [0|255] "" ECU1

BO_ 257 Plain: 8 ECU1
 SG_ OtherSig : 0|8@1+ (1,0) [0|255] "" ECU1

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
"#;

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
    require: "SC_Message",
    fields: &[AttributeField {
        name: "freshness_id",
        source: FieldSource::Attr("SCP_FreshnessValueId"),
    }],
};

fn generate(specs: &[AttributeStruct<'_>]) -> String {
    Config::builder()
        .dbc_name("attribute_structs_test")
        .dbc_content(DBC)
        .attribute_structs(specs)
        .build()
        .generate()
        .expect("generate")
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
fn message_scope_emits_once_per_protected_message() {
    let out = generate(&[SEC_OC]);
    assert!(
        out.contains("pub const SEC_OC: data_protection::SecOcInfo"),
        "{out}"
    );
    assert!(out.contains("freshness_id: 1002"), "{out}");
}

#[test]
fn unprotected_message_and_signal_get_no_const() {
    let out = generate(&[E2E, SEC_OC]);
    assert!(!out.contains("OTHER_SIG_E2E"), "{out}");
    assert_eq!(
        out.matches("pub const SEC_OC").count(),
        1,
        "SEC_OC should appear once\n{out}"
    );
}
