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
fn functions_syntax() {
    let result = match format_message("CSML/basic_test/syntax/functions.csml".to_owned()) {
        Ok(_) => true,
        Err(_) => false,
    };

    assert!(result);
}
