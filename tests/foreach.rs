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
/// FOREACH VALID SYNTAX
////////////////////////////////////////////////////////////////////////////////

#[test]
fn foreach_0() {
    let result = match format_message("CSML/basic_test/syntax/foreach/foreach_0.csml".to_owned()) {
        Ok(_) => true,
        Err(_) => false,
    };

    assert!(result);
}

#[test]
fn foreach_1() {
    let result = match format_message("CSML/basic_test/syntax/foreach/foreach_1.csml".to_owned()) {
        Ok(_) => true,
        Err(_) => false,
    };

    assert!(result);
}

#[test]
fn foreach_2() {
    let result = match format_message("CSML/basic_test/syntax/foreach/foreach_2.csml".to_owned()) {
        Ok(_) => true,
        Err(_) => false,
    };

    assert!(result);
}

#[test]
fn foreach_3() {
    let result = match format_message("CSML/basic_test/syntax/foreach/foreach_3.csml".to_owned()) {
        Ok(_) => true,
        Err(_) => false,
    };

    assert!(result);
}

#[test]
fn foreach_4() {
    let result = match format_message("CSML/basic_test/syntax/foreach/foreach_4.csml".to_owned()) {
        Ok(_) => true,
        Err(_) => false,
    };

    assert!(result);
}

#[test]
fn foreach_5() {
    let result = match format_message("CSML/basic_test/syntax/foreach/foreach_5.csml".to_owned()) {
        Ok(_) => true,
        Err(_) => false,
    };

    assert!(result);
}

////////////////////////////////////////////////////////////////////////////////
/// FOREACH INVALID SYNTAX
////////////////////////////////////////////////////////////////////////////////

#[test]
fn foreach_6() {
    let result = match format_message("CSML/basic_test/syntax/foreach/foreach_6.csml".to_owned()) {
        Ok(_) => false,
        Err(_) => true,
    };

    assert!(result);
}

#[test]
fn foreach_7() {
    let result = match format_message("CSML/basic_test/syntax/foreach/foreach_7.csml".to_owned()) {
        Ok(_) => false,
        Err(_) => true,
    };

    assert!(result);
}

#[test]
fn foreach_8() {
    let result = match format_message("CSML/basic_test/syntax/foreach/foreach_8.csml".to_owned()) {
        Ok(_) => false,
        Err(_) => true,
    };

    assert!(result);
}

#[test]
fn foreach_9() {
    let result = match format_message("CSML/basic_test/syntax/foreach/foreach_9.csml".to_owned()) {
        Ok(_) => false,
        Err(_) => true,
    };

    assert!(result);
}

#[test]
fn foreach_10() {
    let result = match format_message("CSML/basic_test/syntax/foreach/foreach_10.csml".to_owned()) {
        Ok(_) => false,
        Err(_) => true,
    };

    assert!(result);
}

#[test]
fn foreach_11() {
    let result = match format_message("CSML/basic_test/syntax/foreach/foreach_11.csml".to_owned()) {
        Ok(_) => false,
        Err(_) => true,
    };

    assert!(result);
}

#[test]
fn foreach_12() {
    let result = match format_message("CSML/basic_test/syntax/foreach/foreach_12.csml".to_owned()) {
        Ok(_) => false,
        Err(_) => true,
    };

    assert!(result);
}
