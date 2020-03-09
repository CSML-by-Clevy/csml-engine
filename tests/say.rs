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
/// SAY VALID SYNTAX
////////////////////////////////////////////////////////////////////////////////

#[test]
fn say_0() {
    let result = match format_message("CSML/basic_test/syntax/say/say_0.csml".to_owned()) {
        Ok(_) => true,
        Err(_) => false,
    };

    assert!(result);
}

#[test]
fn say_1() {
    let result = match format_message("CSML/basic_test/syntax/say/say_1.csml".to_owned()) {
        Ok(_) => true,
        Err(_) => false,
    };

    assert!(result);
}

#[test]
fn say_2() {
    let result = match format_message("CSML/basic_test/syntax/say/say_2.csml".to_owned()) {
        Ok(_) => true,
        Err(_) => false,
    };

    assert!(result);
}

////////////////////////////////////////////////////////////////////////////////
/// AS INVALID SYNTAX
////////////////////////////////////////////////////////////////////////////////

#[test]
fn say_3() {
    let result = match format_message("CSML/basic_test/syntax/say/say_3.csml".to_owned()) {
        Ok(_) => false,
        Err(_) => true,
    };

    assert!(result);
}
