mod support;

use csmlinterpreter::data::ast::Flow;
use csmlinterpreter::error_format::ErrorInfo;
use csmlinterpreter::parser::parse_flow;

use support::tools::read_file;

fn format_message(filepath: String) -> Result<Flow, ErrorInfo> {
    let text = read_file(filepath).unwrap();

    parse_flow(&text)
}

////////////////////////////////////////////////////////////////////////////////
/// IF VALID SYNTAX
////////////////////////////////////////////////////////////////////////////////

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

////////////////////////////////////////////////////////////////////////////////
/// IF INVALID SYNTAX
////////////////////////////////////////////////////////////////////////////////

#[test]
fn if_2() {
    let result = match format_message("CSML/basic_test/syntax/if/if_2.csml".to_owned()) {
        Ok(_) => false,
        Err(_) => true,
    };

    assert!(result);
}
