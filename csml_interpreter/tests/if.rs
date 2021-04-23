mod support;

use csml_interpreter::data::ast::Flow;
use csml_interpreter::error_format::ErrorInfo;
use csml_interpreter::parser::parse_flow;

use support::tools::read_file;

fn format_message(filepath: String) -> Result<Flow, ErrorInfo> {
    let text = read_file(filepath).unwrap();

    parse_flow(&text, "Test")
}

#[test]
fn if_0() {
    let result = match format_message("CSML/basic_test/syntax/if/if_0.csml".to_owned()) {
        Ok(_) => true,
        Err(_) => false,
    };

    assert!(result);
}

#[test]
fn if_1() {
    let result = match format_message("CSML/basic_test/syntax/if/if_1.csml".to_owned()) {
        Ok(_) => true,
        Err(_) => false,
    };

    assert!(result);
}

#[test]
fn if_2() {
    let result = match format_message("CSML/basic_test/syntax/if/if_2.csml".to_owned()) {
        Ok(_) => false,
        Err(_) => true,
    };

    assert!(result);
}
