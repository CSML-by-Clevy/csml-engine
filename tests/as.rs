mod support;

use csmlinterpreter::data::ast::Flow;
use csmlinterpreter::error_format::ErrorInfo;
use csmlinterpreter::parse_file;

use support::tools::read_file;

fn format_message(filepath: String) -> Result<Flow, ErrorInfo> {
    let text = read_file(filepath).unwrap();

    parse_file(&text)
}

////////////////////////////////////////////////////////////////////////////////
/// AS VALID SYNTAX
////////////////////////////////////////////////////////////////////////////////

#[test]
fn as_0() {
    let result = match format_message("CSML/basic_test/syntax/as/as_0.csml".to_owned()) {
        Ok(_) => true,
        Err(_) => false,
    };

    assert!(result);
}

////////////////////////////////////////////////////////////////////////////////
/// AS INVALID SYNTAX
////////////////////////////////////////////////////////////////////////////////

#[test]
fn as_1() {
    let result = match format_message("CSML/basic_test/syntax/as/as_1.csml".to_owned()) {
        Ok(_) => false,
        Err(_) => true,
    };

    assert!(result);
}

#[test]
fn as_2() {
    let result = match format_message("CSML/basic_test/syntax/as/as_2.csml".to_owned()) {
        Ok(_) => false,
        Err(_) => true,
    };

    assert!(result);
}

#[test]
fn as_3() {
    let result = match format_message("CSML/basic_test/syntax/as/as_3.csml".to_owned()) {
        Ok(_) => false,
        Err(_) => true,
    };

    assert!(result);
}

#[test]
fn as_4() {
    let result = match format_message("CSML/basic_test/syntax/as/as_4.csml".to_owned()) {
        Ok(_) => false,
        Err(_) => true,
    };

    assert!(result);
}

#[test]
fn as_5() {
    let result = match format_message("CSML/basic_test/syntax/as/as_5.csml".to_owned()) {
        Ok(_) => false,
        Err(_) => true,
    };

    assert!(result);
}

#[test]
fn as_6() {
    let result = match format_message("CSML/basic_test/syntax/as/as_6.csml".to_owned()) {
        Ok(_) => false,
        Err(_) => true,
    };

    assert!(result);
}
