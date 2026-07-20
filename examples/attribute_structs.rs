//! Demonstrates [`Config::attribute_structs`].
//! Emits constants for AUTOSAR E2E / SecOC-style metadata from DBC 'BA_' attributes.

use dbc_codegen::{AttributeField, AttributeScope, AttributeStruct, Config, FieldSource};

const DBC: &str = r#"VERSION ""

NS_ :

BS_:

BU_: ECU1

BO_ 256 Protected: 8 ECU1
 SG_ Speed_CRC : 16|8@1+ (1,0) [0|255] "" ECU1

BA_DEF_ BO_  "SCP_FreshnessValueId" INT 0 65535;
BA_DEF_ SG_  "E2EDataId" INT 0 65535;
BA_DEF_ SG_  "E2EDataLength" INT 0 65535;
BA_DEF_ SG_  "E2EProfile" STRING ;
BA_DEF_DEF_  "SCP_FreshnessValueId" 0;
BA_DEF_DEF_  "E2EDataId" 0;
BA_DEF_DEF_  "E2EDataLength" 0;
BA_DEF_DEF_  "E2EProfile" "none";
BA_ "SCP_FreshnessValueId" BO_ 256 1002;
BA_ "E2EDataId" SG_ 256 Speed_CRC 373;
BA_ "E2EDataLength" SG_ 256 Speed_CRC 48;
BA_ "E2EProfile" SG_ 256 Speed_CRC "P01";
"#;

fn field(name: &'static str, source: FieldSource<'static>) -> AttributeField<'static> {
    AttributeField { name, source }
}

fn main() {
    let e2e = AttributeStruct {
        type_path: "data_protection::E2EDataIdInfo",
        const_name: "E2E",
        scope: AttributeScope::Signal,
        require: "E2EDataId",
        fields: &[
            field("data_id", FieldSource::Attr("E2EDataId")),
            field("start_byte", FieldSource::StartByte),
            field("width_bit", FieldSource::Attr("E2EDataLength")),
            field("profile", FieldSource::Attr("E2EProfile")),
        ],
    };
    let secoc = AttributeStruct {
        type_path: "data_protection::SecOcInfo",
        const_name: "SEC_OC",
        scope: AttributeScope::Message,
        require: "SCP_FreshnessValueId",
        fields: &[field(
            "freshness_id",
            FieldSource::Attr("SCP_FreshnessValueId"),
        )],
    };

    let code = Config::builder()
        .dbc_name("example")
        .dbc_content(DBC)
        .attribute_structs(&[e2e, secoc])
        .build()
        .generate()
        .expect("generate");

    println!("{code}");
}
