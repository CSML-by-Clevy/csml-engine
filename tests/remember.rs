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
/// REMEMBER VALID SYNTAX
////////////////////////////////////////////////////////////////////////////////

#[test]
fn remember_0() {
    let result = match format_message("CSML/basic_test/syntax/remember/remember_0.csml".to_owned())
    {
        Ok(_) => true,
        Err(_) => false,
    };

    assert!(result);
}

#[test]
fn remember_1() {
    let result = match format_message("CSML/basic_test/syntax/remember/remember_1.csml".to_owned())
    {
        Ok(_) => true,
        Err(_) => false,
    };

    assert!(result);
}

#[test]
fn remember_2() {
    let result = match format_message("CSML/basic_test/syntax/remember/remember_2.csml".to_owned())
    {
        Ok(_) => true,
        Err(_) => false,
    };

    assert!(result);
}

#[test]
fn remember_3() {
    let result = match format_message("CSML/basic_test/syntax/remember/remember_3.csml".to_owned())
    {
        Ok(_) => true,
        Err(_) => false,
    };

    assert!(result);
}

////////////////////////////////////////////////////////////////////////////////
/// USE INVALID SYNTAX
////////////////////////////////////////////////////////////////////////////////

#[test]
fn remember_4() {
    let result = match format_message("CSML/basic_test/syntax/remember/remember_4.csml".to_owned())
    {
        Ok(_) => false,
        Err(_) => true,
    };

    assert!(result);
}

#[test]
fn remember_5() {
    let result = match format_message("CSML/basic_test/syntax/remember/remember_5.csml".to_owned())
    {
        Ok(_) => false,
        Err(_) => true,
    };

    assert!(result);
}

#[test]
fn remember_6() {
    let result = match format_message("CSML/basic_test/syntax/remember/remember_6.csml".to_owned())
    {
        Ok(_) => false,
        Err(_) => true,
    };

    assert!(result);
}
