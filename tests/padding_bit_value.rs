use dbc_codegen::Config;

const SIMPLE_DBC: &str = r#"
VERSION ""

NS_ :
	NS_DESC_
	CM_
	BA_DEF_
	BA_
	VAL_
	CAT_DEF_
	CAT_
	FILTER
	BA_DEF_DEF_
	EV_DATA_
	ENVVAR_DATA_
	SGTYPE_
	SGTYPE_VAL_
	BA_DEF_SGTYPE_
	BA_SGTYPE_
	SIG_TYPE_REF_
	VAL_TABLE_
	SIG_GROUP_
	SIG_VALTYPE_
	SIGTYPE_VALTYPE_
	BO_TX_BU_
	BA_DEF_REL_
	BA_REL_
	BA_DEF_DEF_REL_
	BU_SG_REL_
	BU_EV_REL_
	BU_BO_REL_
	SG_MUL_VAL_

BS_:

BU_: TEST_NODE

BO_ 1 TEST_MESSAGE: 1 TEST_NODE
 SG_ FLAG : 0|1@1+ (1,0) [0|1] "" TEST_NODE

BO_ 2 PARTIAL_MESSAGE: 2 TEST_NODE
 SG_ PARTIAL : 0|3@1+ (1,0) [0|7] "" TEST_NODE
"#;

fn generated_code(padding_bit_value: bool) -> String {
    Config::builder()
        .dbc_name("padding_bit_value")
        .dbc_content(SIMPLE_DBC)
        .padding_bit_value(padding_bit_value)
        .build()
        .generate()
        .expect("code generation should succeed")
}

#[test]
fn constructor_uses_zero_padding_by_default() {
    let code = generated_code(false);
    assert!(
        code.contains("let mut res = Self { raw: [0x00; 1] };")
            || code.contains("let res = Self { raw: [0x00; 1] };"),
        "expected constructor to initialize payload with 0x00, got:\n{code}"
    );
}

#[test]
fn constructor_uses_one_padding_when_enabled() {
    let code = generated_code(true);
    assert!(
        code.contains("let mut res = Self { raw: [0xFF; 1] };")
            || code.contains("let res = Self { raw: [0xFF; 1] };"),
        "expected constructor to initialize payload with 0xFF, got:\n{code}"
    );
}

#[test]
fn non_byte_aligned_signal_keeps_unused_bits_padded() {
    let code = generated_code(true);

    assert!(
        code.contains("let mut res = Self { raw: [0xFF; 2] };")
            || code.contains("let res = Self { raw: [0xFF; 2] };"),
        "expected partial-message constructor to initialize both bytes with 0xFF, got:\n{code}"
    );

    assert!(
        code.contains("res.set_partial(partial)?;"),
        "expected constructor to set the 3-bit signal after initialization, got:\n{code}"
    );

    assert!(
        code.contains("self.raw.view_bits_mut::<Lsb0>()[0..3].store_le(value);"),
        "expected only the signal bit range to be overwritten, got:\n{code}"
    );
}
