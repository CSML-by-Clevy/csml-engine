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
/// DO VALID SYNTAX
////////////////////////////////////////////////////////////////////////////////

#[test]
fn do_0() {
    let result = match format_message("CSML/basic_test/syntax/do/do_0.csml".to_owned()) {
        Ok(_) => true,
        Err(_) => false,
    };

    assert!(result);
}

#[test]
fn do_1() {
    let result = match format_message("CSML/basic_test/syntax/do/do_1.csml".to_owned()) {
        Ok(_) => true,
        Err(_) => false,
    };

    assert!(result);
}

#[test]
fn do_2() {
    let result = match format_message("CSML/basic_test/syntax/do/do_2.csml".to_owned()) {
        Ok(_) => true,
        Err(_) => false,
    };

    assert!(result);
}

#[test]
fn do_3() {
    let result = match format_message("CSML/basic_test/syntax/do/do_3.csml".to_owned()) {
        Ok(_) => true,
        Err(_) => false,
    };

    assert!(result);
}

#[test]
fn do_4() {
    let result = match format_message("CSML/basic_test/syntax/do/do_4.csml".to_owned()) {
        Ok(_) => true,
        Err(_) => false,
    };

    assert!(result);
}

#[test]
fn do_5() {
    let result = match format_message("CSML/basic_test/syntax/do/do_5.csml".to_owned()) {
        Ok(_) => true,
        Err(_) => false,
    };

    assert!(result);
}

////////////////////////////////////////////////////////////////////////////////
/// DO INVALID SYNTAX
////////////////////////////////////////////////////////////////////////////////

#[test]
fn do_6() {
    let result = match format_message("CSML/basic_test/syntax/do/do_6.csml".to_owned()) {
        Ok(_) => false,
        Err(_) => true,
    };

    assert!(result);
}

#[test]
fn do_7() {
    let result = match format_message("CSML/basic_test/syntax/do/do_7.csml".to_owned()) {
        Ok(_) => false,
        Err(_) => true,
    };

    assert!(result);
}

#[test]
fn do_8() {
    let result = match format_message("CSML/basic_test/syntax/do/do_8.csml".to_owned()) {
        Ok(_) => false,
        Err(_) => true,
    };

    assert!(result);
}
