pub mod data;

use crate::data::{ast::Interval, tokens::Span};
use nom::{
    error::{ErrorKind, ParseError},
    *,
};

pub use data::{CustomError, ErrorInfo};

// Parsing Errors
pub const ERROR_PARENTHESES: &'static str = "list elem type ( ... ) not found";
pub const ERROR_PARENTHESES_END: &'static str =
    "invalid argument expect one ',' between each argument or ')' to end the list";
pub const ERROR_NUMBER_AS_IDENT: &'static str = "int/float can't be used as identifier";
pub const ERROR_RESERVED: &'static str = "reserved keyword can't be used as identifier";
pub const ERROR_PARSING: &'static str =
    "invalid argument use one of [say, do, if, ..] keywords to start an action";
pub const ERROR_REMEMBER: &'static str =
    "remember must be assigning to a variable via '=': remember key = value";
pub const ERROR_USE: &'static str =
    "use must be assigning to a variable via 'as': use value as key";
pub const ERROR_BREAK: &'static str = "break can only be used inside a foreach";
pub const ERROR_HOLD: &'static str = "hold cannot be used inside a foreach";
pub const ERROR_LEFT_BRACE: &'static str = "expect '('";
pub const ERROR_RIGHT_BRACE: &'static str = "expect ')'";
pub const ERROR_RIGHT_BRACKET: &'static str = "expect ']'";
pub const ERROR_GOTO_STEP: &'static str = "missing step name after goto";
pub const ERROR_IMPORT_STEP: &'static str = "missing step name after import";
pub const ERROR_DOUBLE_QUOTE: &'static str = "expect '\"' to end string";
pub const ERROR_UNREACHABLE: &'static str = "unreachable";

// ##Interpreter Errors
// ### Validation
pub const ERROR_STEP_EXIST: &'static str = " step does not exist";
pub const ERROR_INVALID_FLOW: &'static str = "invalid Flow: ";
pub const ERROR_START_INSTRUCTIONS: &'static str =
"to start an action one of the following instruction is expected  : [say, do, if, foreach, goto]";
pub const ERROR_FOREACH: &'static str =
    "foreach only accepts array as iterable elements example: foreach(elem) in [1, 2, 3]";
pub const ERROR_FIND_BY_INDEX: &'static str =
    "index must be of type int or string  => var.[42] or var.[\"key\"]";
pub const ERROR_ASSIGN_IDENT: &'static str = "key must be of type identifier";
pub const ERROR_FUNCTIONS_ARGS: &'static str = "argument in of function must be in a vector";
pub const ERROR_EXPR_TO_LITERAL: &'static str = "Expr can't be converted to Literal";
pub const ERROR_PAYLOAD_EXCEED_MAX_SIZE: &'static str =
    "payload exceed the payload_max_size (16kb)";

// Event
pub const ERROR_EVENT_CONTENT_TYPE: &'static str = "event can only be of ContentType::Event";

// Fn API
pub const ERROR_FN_ID: &'static str =
    "to find the function to call function_id beed to be of type string";
pub const ERROR_FN_ENDPOINT: &'static str =
    "fn call can not be make because fn_endpoint is not set";
pub const ERROR_FAIL_RESPONSE_JSON: &'static str = "failed to read response as JSON";

// ### Import
pub const ERROR_IMPORT_FAIL: &'static str = "import fail at";
pub const ERROR_IMPORT_STEP_FLOW: &'static str = "step not found in flow";

// ### Variables
pub const ERROR_GET_VAR_INFO: &'static str = "expression need to be a variable";
pub const ERROR_JSON_TO_LITERAL: &'static str = "this number is to big to be an int 64 bit";

// ### Memory
pub const ERROR_STEP_MEMORY: &'static str = "variable does not exist in step memory";
pub const ERROR_FIND_MEMORY: &'static str = "is not in in memory";

// ### Built-in
pub const ERROR_TEXT: &'static str =
    "Builtin Text expect one argument of type string | example: Text(\"hola\")";
pub const ERROR_TYPING: &'static str =
    "Builtin Typing expect one argument of type int or float | example: Typing(3, ..)";
pub const ERROR_WAIT: &'static str =
    "Builtin Wait expect one argument of type int or float | example: Wait(3)";
pub const ERROR_BUTTON: &'static str =
    "Builtin Button expect at least one argument of type string | example: Button(\"hola\")";
pub const ERROR_CARD_BUTTON: &'static str = "argument buttons in Builtin Cards need to be of type Array of Button Component example: [ Button(\"b1\"), Button(\"b2\") ]";
pub const ERROR_CARD_TITLE: &'static str =
    "argument title in Builtin Cards need to be of type String";
pub const ERROR_QUESTION: &'static str = "argument buttons in Builtin Question need to be of type Array of Button Component example: [ Button(\"b1\"), Button(\"b2\") ]";
pub const ERROR_CAROUSEL: &'static str =
    "argument cards in Builtin Carousel need to be of type Array of Cards Component";
pub const ERROR_ONE_OF: &'static str =
    "Builtin OneOf expect one value of type Array | example: OneOf( [1, 2, 3] )";
pub const ERROR_SHUFFLE: &'static str =
    "Builtin Shuffle expect one value of type Array | example: Shuffle( [1, 2, 3] )";
pub const ERROR_LENGTH: &'static str =
    "Builtin Length expect one value of type Array or String | example: Length( value )";
pub const ERROR_FIND: &'static str = "Builtin Find expect in to be of type String | example: Contain(value, in = \"hola\", case_sensitive = true)";
pub const ERROR_FLOOR: &'static str =
    "Builtin Floor expect one argument of type float| example: Floor(4.2)";
pub const ERROR_IMAGE: &'static str =
    "Builtin Image expect one argument of type string | example: Image(\"hola\")";
pub const ERROR_URL: &'static str = "Builtin Url expect one argument of type string and 2 optional string arguments: text, title | example: Url(href = \"hola\", text = \"text\", title = \"title\")";
pub const ERROR_VIDEO: &'static str = "Builtin Video expect one argument of type string and 1 optional 'service' argument of type string | example: Video(url = \"hola\", service = \"text\")";
pub const ERROR_AUDIO: &'static str = "Builtin Audio expect one argument of type string and 1 optional 'service' argument of type string | example: Audio(url = \"hola\", service = \"text\")";
pub const ERROR_HTTP: &'static str =
    "Builtin HTTP expect one url of type string | example: HTTP(\"https://clevy.io\")";
pub const ERROR_HTTP_GET_VALUE: &'static str =
"not found in http object please use HTTP(..) to construct the correct object to make a http call";
pub const ERROR_HTTP_QUERY_VALUES: &'static str =
    "must have a value of type String example: {key: \"value\"}";
pub const ERROR_BUILTIN_UNKNOWN: &'static str = "Unknown Built-in";

// ### Primitives
// #### Boolean
pub const ERROR_BOOLEAN_UNKNOWN_METHOD: &'static str = " is not a method of Boolean";

// #### NUMBER
pub const ERROR_NUMBER_POW: &'static str =
    "[pow] take one parameter of type int or float usage: number.pow(42)";

// #### Float
pub const ERROR_FLOAT_UNKNOWN_METHOD: &'static str = " is not a method of Float";

// #### Int
pub const ERROR_INT_UNKNOWN_METHOD: &'static str = " is not a method of Int";

// #### Null
pub const ERROR_NULL_UNKNOWN_METHOD: &'static str = " is not a method of Null";

// #### String
pub const ERROR_STRING_DO_MATCH: &'static str =
    "[do_match] take one parameter of type String usage: string.do_match(\"tag\")";
pub const ERROR_STRING_APPEND: &'static str =
    "[append] take one parameter of type String usage: string.append(\"text to append\")";
pub const ERROR_STRING_CONTAINS: &'static str =
    "[contains] take one parameter of type String usage: string.contains(\"word\")";
pub const ERROR_STRING_CONTAINS_REGEX: &'static str =
    "[contains_regex] take one parameter of type String usage: string.contains_regex(\"regex\")";
pub const ERROR_STRING_VALID_REGEX: &'static str = "parameter must be a valid regex expression"; // link to docs
pub const ERROR_STRING_START_WITH: &'static str =
    "[start_with] take one parameter of type String usage: string.start_with(\"tag\")";
pub const ERROR_STRING_START_WITH_REGEX: &'static str = "[start_with_regex] take one parameter of type String usage: string.start_with_regex(\"regex\")";
pub const ERROR_STRING_END_WITH: &'static str =
    "[end_with] take one parameter of type String usage: string.end_with(\"tag\")";
pub const ERROR_STRING_END_WITH_REGEX: &'static str =
    "[end_with_regex] take one parameter of type String usage: string.end_with_regex(\"regex\")";
pub const ERROR_STRING_MATCH_REGEX: &'static str =
    "[end_match_regex] take one parameter of type String usage: string.match_regex(\"regex\")";
pub const ERROR_STRING_POW: &'static str =
    "[pow] take one parameter of type Float or Int usage: string.pow(number)";
pub const ERROR_STRING_COS: &'static str = "[cos] the string must be of numeric type in order to use cos, you can use 'string.is_number() == true' ";
pub const ERROR_STRING_NUMERIC: &'static str = " the string must be of numeric type in order to use this method, you can use 'string.is_number() == true' to check it";
pub const ERROR_STRING_RHS: &'static str = "rhs need to be of type string";
pub const ERROR_STRING_UNKNOWN_METHOD: &'static str = " is not a method of String";

// #### Array
pub const ERROR_ARRAY_TYPE: &'static str = "value need to be of type array";
pub const ERROR_ARRAY_INDEX_EXIST: &'static str = "index does not exist";
pub const ERROR_ARRAY_INDEX_TYPE: &'static str = "index must be of type int";
pub const ERROR_ARRAY_NEGATIVE: &'static str = "index must be positive.  usage: array[1]";
pub const ERROR_ARRAY_INDEX: &'static str = "index must be lower or equal than array.length()";
pub const ERROR_ARRAY_OVERFLOW: &'static str =
    "[push] Cannot push inside array, since array limit is ";
pub const ERROR_ARRAY_POP: &'static str = "[pop] Cannot pop if array is empty";
pub const ERROR_ARRAY_INSERT_AT: &'static str =
    "[insert_at] take two parameters  usage: array.insert_at(1, elem)";
pub const ERROR_ARRAY_INSERT_AT_INT: &'static str =
    "[insert_at] first parameter must be of type int usage: array.insert_at(1, elem)";
pub const ERROR_ARRAY_REMOVE_AT: &'static str =
    "[remove_at] take one parameter of type int usage: array.remove_at(1) ";
pub const ERROR_ARRAY_JOIN: &'static str =
    "[join] take one parameter of type string usage: array.join(\"elem\") ";
pub const ERROR_ARRAY_INDEX_OF: &'static str =
    "[index_of] take one parameter usage: array.index_of(elem)";
pub const ERROR_ARRAY_FIND: &'static str = "[find] take one parameter usage: array.find(elem)";
pub const ERROR_ARRAY_UNKNOWN_METHOD: &'static str = " is not a method of Array";

// #### HTTP OBJECT
pub const ERROR_HTTP_SET: &'static str =
    "[set] take one parameter of type Object usage: http.set( {\"key\": 42} )";
pub const ERROR_HTTP_QUERY: &'static str =
    "[query] take one parameter of type Object usage: http.query( {\"key\": 42} )";
pub const ERROR_HTTP_POST: &'static str =
    "[post] take one parameter of type Object usage: http.post( {\"key\": 42} )";
pub const ERROR_HTTP_PUT: &'static str =
    "[put] take one parameter of type Object usage: http.put( {\"key\": 42} )";
pub const ERROR_HTTP_PATCH: &'static str =
    "[patch] take one parameter of type Object usage: http.patch( {\"key\": 42} )";
pub const ERROR_HTTP_SEND: &'static str =
    "[send] HTTP Object is bad formatted read doc for correct usage";
pub const ERROR_HTTP_UNKNOWN_METHOD: &'static str = " is not a method of HTTP";

// #### OBJECT
pub const ERROR_OBJECT_TYPE: &'static str = "value need to be of type object";
pub const ERROR_OBJECT_CONTAINS: &'static str =
    "[contains] take one parameter of type String usage: object.contains(\"key\")";
pub const ERROR_OBJECT_GET_GENERICS: &'static str =
    "[get_generics] take one parameter of type String usage: object.get_generics(\"key\")";
pub const ERROR_OBJECT_INSERT: &'static str =
    "[insert] take tow parameters usage: object.insert(string, any_type)";
pub const ERROR_OBJECT_REMOVE: &'static str =
    "[remove] take one parameter of type String usage: object.remove(\"key\")";
pub const ERROR_OBJECT_GET_KEY: &'static str = "key must be of type string";
pub const ERROR_OBJECT_UNKNOWN_METHOD: &'static str = " is not a method of Object";

pub const ERROR_OPS: &'static str = "[!] Ops: Illegal operation";
pub const ERROR_OPS_DIV_INT: &'static str = "[!] Int: Division by zero";
pub const ERROR_OPS_DIV_FLOAT: &'static str = "[!] Float: Division by zero";

pub const ERROR_ILLEGAL_OPERATION: &'static str = "illegal operation:";

pub fn gen_error_info(interval: Interval, message: String) -> ErrorInfo {
    ErrorInfo { interval, message }
}

pub fn gen_nom_error<'a, E>(span: Span<'a>, error: &'static str) -> Err<E>
where
    E: ParseError<Span<'a>>,
{
    Err::Error(E::add_context(
        span,
        error,
        E::from_error_kind(span, ErrorKind::Tag),
    ))
}

pub fn gen_nom_failure<'a, E>(span: Span<'a>, error: &'static str) -> Err<E>
where
    E: ParseError<Span<'a>>,
{
    Err::Failure(E::add_context(
        span,
        error,
        E::from_error_kind(span, ErrorKind::Tag),
    ))
}
