#![cfg(feature = "std")]

//! Tests for emitting user-defined structs populated from DBC attributes
//! (`Config::attribute_structs`).

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
BA_DEF_ BO_  "TxOffset" INT -128 127;
BA_DEF_ BO_  "Gain" FLOAT 0 10;
BA_DEF_ SG_  "E2EDataId" INT 0 65535;
BA_DEF_ SG_  "E2EDataLength" INT 0 65535;
BA_DEF_ SG_  "E2EProfile" STRING ;
BA_DEF_DEF_  "SC_Message" "0";
BA_DEF_DEF_  "SCP_FreshnessValueId" 0;
BA_DEF_DEF_  "TxOffset" 0;
BA_DEF_DEF_  "Gain" 0;
BA_DEF_DEF_  "E2EDataId" 0;
BA_DEF_DEF_  "E2EDataLength" 0;
BA_DEF_DEF_  "E2EProfile" "none";
BA_ "SC_Message" BO_ 256 1;
BA_ "SCP_FreshnessValueId" BO_ 256 1002;
BA_ "TxOffset" BO_ 256 -7;
BA_ "Gain" BO_ 256 2.5;
BA_ "E2EDataId" SG_ 256 TestSig 373;
BA_ "E2EDataLength" SG_ 256 TestSig 48;
BA_ "E2EDataId" SG_ 258 BeSig 500;
BA_ "E2EDataLength" SG_ 258 BeSig 16;
"#;

const FIXTURE: &str = "tests/fixtures/shared-test-files/dbc-cantools/attributes.dbc";

/// Signal-scoped
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

/// Message-scoped
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

/// Non-attribute sources
const LAYOUT: AttributeStruct = AttributeStruct {
    type_path: "layout::SignalLayout",
    const_name: "LAYOUT",
    scope: AttributeScope::Signal,
    require: "E2EDataId",
    fields: &[
        AttributeField {
            name: "start_bit",
            source: FieldSource::StartBit,
        },
        AttributeField {
            name: "bit_width",
            source: FieldSource::BitWidth,
        },
        AttributeField {
            name: "message_size",
            source: FieldSource::MessageSize,
        },
        AttributeField {
            name: "tx_offset",
            source: FieldSource::MessageAttr("TxOffset"),
        },
        AttributeField {
            name: "gain",
            source: FieldSource::MessageAttr("Gain"),
        },
        AttributeField {
            name: "version",
            source: FieldSource::Int(42),
        },
        AttributeField {
            name: "label",
            source: FieldSource::Str("hello"),
        },
    ],
};

#[test]
fn default_emits_nothing() {
    let out = Config::builder()
        .dbc_name("test")
        .dbc_content(DBC)
        .build()
        .generate()
        .unwrap();
    assert!(!out.contains("E2EDataIdInfo"), "{out}");
}

#[test]
fn signal_scope_emits_typed_struct_with_derived_layout() {
    let out = Config::builder()
        .dbc_name("test")
        .dbc_content(DBC)
        .attribute_structs(&[E2E])
        .build()
        .generate()
        .unwrap();
    assert!(
        out.contains("pub const TEST_SIG_E2E: data_protection::E2EDataIdInfo"),
        "{out}"
    );
    assert!(out.contains("data_id: 373"), "{out}");
    assert!(out.contains("start_byte: 2"), "{out}"); // start_bit 16 / 8
    assert!(out.contains("width_bit: 48"), "{out}");
    assert!(out.contains(r#"profile: "none""#), "{out}"); // BA_DEF_DEF_ default
}

#[test]
fn big_endian_start_byte_uses_start_bit_over_eight() {
    // BeSig is `23|8@0+` (Motorola): start byte is still start_bit / 8 == 2.
    let out = Config::builder()
        .dbc_name("test")
        .dbc_content(DBC)
        .attribute_structs(&[E2E])
        .build()
        .generate()
        .unwrap();
    assert!(
        out.contains("pub const BE_SIG_E2E: data_protection::E2EDataIdInfo"),
        "{out}"
    );
    assert!(out.contains("data_id: 500"), "{out}");
    assert!(out.contains("start_byte: 2"), "{out}");
}

#[test]
fn message_scope_emits_once_per_protected_message() {
    let out = Config::builder()
        .dbc_name("test")
        .dbc_content(DBC)
        .attribute_structs(&[SEC_OC])
        .build()
        .generate()
        .unwrap();
    assert!(
        out.contains("pub const SEC_OC: data_protection::SecOcInfo"),
        "{out}"
    );
    assert!(out.contains("freshness_id: 1002"), "{out}");
}

#[test]
fn enum_attribute_emitted_as_integer_index() {
    let out = Config::builder()
        .dbc_name("test")
        .dbc_content(DBC)
        .attribute_structs(&[SEC_OC])
        .build()
        .generate()
        .unwrap();
    // The enum value is emitted as its index (1), not the label.
    assert!(out.contains("sc_message: 1"), "{out}");
}

#[test]
fn unprotected_message_and_signal_get_no_const() {
    let out = Config::builder()
        .dbc_name("test")
        .dbc_content(DBC)
        .attribute_structs(&[E2E, SEC_OC])
        .build()
        .generate()
        .unwrap();
    assert!(!out.contains("OTHER_SIG_E2E"), "{out}");
    assert_eq!(out.matches("pub const SEC_OC").count(), 1, "{out}");
}

#[test]
fn layout_and_literal_sources_resolve() {
    // TestSig is `16|8@1+` on an 8-byte message.
    let out = Config::builder()
        .dbc_name("test")
        .dbc_content(DBC)
        .attribute_structs(&[LAYOUT])
        .build()
        .generate()
        .unwrap();
    assert!(
        out.contains("pub const TEST_SIG_LAYOUT: layout::SignalLayout"),
        "{out}"
    );
    assert!(out.contains("start_bit: 16"), "{out}");
    assert!(out.contains("bit_width: 8"), "{out}");
    assert!(out.contains("message_size: 8"), "{out}");
    assert!(out.contains("tx_offset: -7"), "{out}");
    assert!(out.contains("gain: 2.5"), "{out}");
    assert!(out.contains("version: 42"), "{out}");
    assert!(out.contains(r#"label: "hello""#), "{out}");
}

#[test]
fn message_scope_with_signal_source_is_rejected() {
    let bad = AttributeStruct {
        type_path: "foo::Bar",
        const_name: "BAD",
        scope: AttributeScope::Message,
        require: "SCP_FreshnessValueId",
        fields: &[AttributeField {
            name: "x",
            source: FieldSource::StartByte,
        }],
    };
    let err = Config::builder()
        .dbc_name("test")
        .dbc_content(DBC)
        .attribute_structs(&[bad])
        .build()
        .generate()
        .unwrap_err();
    assert!(format!("{err:#}").contains("signal-only source"), "{err:#}");
}

#[test]
fn missing_field_value_is_an_error() {
    let bad = AttributeStruct {
        type_path: "foo::Bar",
        const_name: "BAD",
        scope: AttributeScope::Signal,
        require: "E2EDataId",
        fields: &[AttributeField {
            name: "x",
            source: FieldSource::Attr("NoSuchAttr"),
        }],
    };
    let err = Config::builder()
        .dbc_name("test")
        .dbc_content(DBC)
        .attribute_structs(&[bad])
        .build()
        .generate()
        .unwrap_err();
    assert!(format!("{err:#}").contains("has no value"), "{err:#}");
}

#[test]
fn duplicate_const_name_is_an_error() {
    let dup = AttributeStruct {
        type_path: "foo::Bar",
        const_name: "SEC_OC", // collides with SEC_OC
        scope: AttributeScope::Message,
        require: "SCP_FreshnessValueId",
        fields: &[AttributeField {
            name: "freshness_id",
            source: FieldSource::Attr("SCP_FreshnessValueId"),
        }],
    };
    let err = Config::builder()
        .dbc_name("test")
        .dbc_content(DBC)
        .attribute_structs(&[SEC_OC, dup])
        .build()
        .generate()
        .unwrap_err();
    assert!(format!("{err:#}").contains("collides"), "{err:#}");
}

#[test]
fn non_screaming_snake_case_const_name_is_rejected() {
    for name in ["1bad", "new", "raw", "with-dash"] {
        let bad = AttributeStruct {
            type_path: "foo::Bar",
            const_name: name,
            scope: AttributeScope::Message,
            require: "SCP_FreshnessValueId",
            fields: &[AttributeField {
                name: "x",
                source: FieldSource::Attr("SCP_FreshnessValueId"),
            }],
        };
        let err = Config::builder()
            .dbc_name("test")
            .dbc_content(DBC)
            .attribute_structs(&[bad])
            .build()
            .generate()
            .unwrap_err();
        assert!(
            format!("{err:#}").contains("SCREAMING_SNAKE_CASE"),
            "{name}: {err:#}"
        );
    }
}

#[test]
fn const_name_colliding_with_reserved_const_is_rejected() {
    let bad = AttributeStruct {
        type_path: "foo::Bar",
        const_name: "MESSAGE_ID", // Collides with the generated const MESSAGE_ID
        scope: AttributeScope::Message,
        require: "SCP_FreshnessValueId",
        fields: &[AttributeField {
            name: "x",
            source: FieldSource::Attr("SCP_FreshnessValueId"),
        }],
    };
    let err = Config::builder()
        .dbc_name("test")
        .dbc_content(DBC)
        .attribute_structs(&[bad])
        .build()
        .generate()
        .unwrap_err();
    assert!(format!("{err:#}").contains("collides"), "{err:#}");
}

#[test]
fn invalid_type_path_is_rejected() {
    let bad = AttributeStruct {
        type_path: "1bad::Type", // not a valid type path
        const_name: "OK",
        scope: AttributeScope::Message,
        require: "SCP_FreshnessValueId",
        fields: &[AttributeField {
            name: "x",
            source: FieldSource::Attr("SCP_FreshnessValueId"),
        }],
    };
    let err = Config::builder()
        .dbc_name("test")
        .dbc_content(DBC)
        .attribute_structs(&[bad])
        .build()
        .generate()
        .unwrap_err();
    assert!(
        format!("{err:#}").contains("not a valid Rust type path"),
        "{err:#}"
    );
}

#[test]
fn invalid_field_name_is_rejected() {
    let bad = AttributeStruct {
        type_path: "foo::Bar",
        const_name: "OK",
        scope: AttributeScope::Message,
        require: "SCP_FreshnessValueId",
        fields: &[AttributeField {
            name: "1x", // not a valid identifier
            source: FieldSource::Attr("SCP_FreshnessValueId"),
        }],
    };
    let err = Config::builder()
        .dbc_name("test")
        .dbc_content(DBC)
        .attribute_structs(&[bad])
        .build()
        .generate()
        .unwrap_err();
    assert!(format!("{err:#}").contains("field name"), "{err:#}");
}

#[test]
fn duplicate_field_names_are_rejected() {
    let bad = AttributeStruct {
        type_path: "foo::Bar",
        const_name: "OK",
        scope: AttributeScope::Message,
        require: "SCP_FreshnessValueId",
        fields: &[
            AttributeField {
                name: "dup",
                source: FieldSource::Attr("SCP_FreshnessValueId"),
            },
            AttributeField {
                name: "dup",
                source: FieldSource::Int(1),
            },
        ],
    };
    let err = Config::builder()
        .dbc_name("test")
        .dbc_content(DBC)
        .attribute_structs(&[bad])
        .build()
        .generate()
        .unwrap_err();
    assert!(format!("{err:#}").contains("duplicate field"), "{err:#}");
}

#[test]
fn fixture_message_scope_resolves_hex_float_enum_and_size() {
    let spec = AttributeStruct {
        type_path: "MsgAttrs",
        const_name: "ATTRS",
        scope: AttributeScope::Message,
        require: "TheHexAttribute",
        fields: &[
            AttributeField {
                name: "hex_attr",
                source: FieldSource::Attr("TheHexAttribute"),
            },
            AttributeField {
                name: "float_attr",
                source: FieldSource::Attr("TheFloatAttribute"),
            },
            AttributeField {
                name: "send_type",
                source: FieldSource::Attr("GenMsgSendType"),
            },
            AttributeField {
                name: "size_bytes",
                source: FieldSource::MessageSize,
            },
        ],
    };
    let bytes = std::fs::read(FIXTURE).unwrap();
    let dbc = can_dbc::decode_cp1252(&bytes).unwrap();
    let out = Config::builder()
        .dbc_name("attributes.dbc")
        .dbc_content(&dbc)
        .attribute_structs(&[spec])
        .build()
        .generate()
        .unwrap();
    assert!(out.contains("hex_attr: 5"), "{out}");
    assert!(out.contains("float_attr: 58.7"), "{out}");
    assert!(out.contains("send_type: 0"), "{out}");
    assert!(out.contains("size_bytes: 8"), "{out}");
    assert_eq!(out.matches("pub const ATTRS").count(), 1, "{out}");
}

#[test]
fn fixture_signal_scope_resolves_string_enum_and_layout() {
    let spec = AttributeStruct {
        type_path: "SigAttrs",
        const_name: "SIG",
        scope: AttributeScope::Signal,
        require: "TheSignalStringAttribute",
        fields: &[
            AttributeField {
                name: "label",
                source: FieldSource::Attr("TheSignalStringAttribute"),
            },
            AttributeField {
                name: "send_type",
                source: FieldSource::Attr("GenSigSendType"),
            },
            AttributeField {
                name: "start_byte",
                source: FieldSource::StartByte,
            },
            AttributeField {
                name: "width",
                source: FieldSource::BitWidth,
            },
        ],
    };
    let bytes = std::fs::read(FIXTURE).unwrap();
    let dbc = can_dbc::decode_cp1252(&bytes).unwrap();
    let out = Config::builder()
        .dbc_name("attributes.dbc")
        .dbc_content(&dbc)
        .attribute_structs(&[spec])
        .build()
        .generate()
        .unwrap();
    assert!(out.contains(r#"label: "TestString""#), "{out}");
    assert!(out.contains("send_type: 1"), "{out}");
    assert!(out.contains("start_byte: 0"), "{out}");
    assert!(out.contains("width: 8"), "{out}");
    assert_eq!(out.matches("_SIG: SigAttrs").count(), 1, "{out}");
}

#[test]
fn fixture_absent_attribute_falls_back_to_default() {
    // GenSigSendType is present on the signal in both messages.
    let spec = AttributeStruct {
        type_path: "SigAttrs",
        const_name: "SIG2",
        scope: AttributeScope::Signal,
        require: "GenSigSendType",
        fields: &[AttributeField {
            name: "label",
            source: FieldSource::Attr("TheSignalStringAttribute"),
        }],
    };
    let bytes = std::fs::read(FIXTURE).unwrap();
    let dbc = can_dbc::decode_cp1252(&bytes).unwrap();
    let out = Config::builder()
        .dbc_name("attributes.dbc")
        .dbc_content(&dbc)
        .attribute_structs(&[spec])
        .build()
        .generate()
        .unwrap();
    assert_eq!(out.matches("pub const THE_SIGNAL_SIG2").count(), 2, "{out}");
    assert!(out.contains(r#"label: "TestString""#), "{out}");
    assert!(out.contains(r#"label: "100""#), "{out}"); // BA_DEF_DEF_ default
}

#[test]
fn fixture_enum_attribute_is_index_not_label() {
    let spec = AttributeStruct {
        type_path: "SendTypeInfo",
        const_name: "ST",
        scope: AttributeScope::Message,
        require: "GenMsgSendType",
        fields: &[AttributeField {
            name: "send_type",
            source: FieldSource::Attr("GenMsgSendType"),
        }],
    };
    let bytes = std::fs::read(FIXTURE).unwrap();
    let dbc = can_dbc::decode_cp1252(&bytes).unwrap();
    let out = Config::builder()
        .dbc_name("attributes.dbc")
        .dbc_content(&dbc)
        .attribute_structs(&[spec])
        .build()
        .generate()
        .unwrap();
    assert!(out.contains("send_type: 0"), "{out}");
    assert!(!out.contains("Cyclic"), "{out}");
}

#[test]
fn fixture_definition_without_default_is_an_error() {
    let spec = AttributeStruct {
        type_path: "Bad",
        const_name: "BAD",
        scope: AttributeScope::Message,
        require: "TheHexAttribute",
        fields: &[AttributeField {
            name: "x",
            source: FieldSource::Attr("TheUnusedAttributeDefinitionWithoutDefault"),
        }],
    };
    let bytes = std::fs::read(FIXTURE).unwrap();
    let dbc = can_dbc::decode_cp1252(&bytes).unwrap();
    let err = Config::builder()
        .dbc_name("attributes.dbc")
        .dbc_content(&dbc)
        .attribute_structs(&[spec])
        .build()
        .generate()
        .unwrap_err();
    assert!(format!("{err:#}").contains("has no value"), "{err:#}");
}
