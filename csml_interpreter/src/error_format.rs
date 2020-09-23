pub mod data;

use crate::data::position::Position;
use crate::data::tokens::Span;
use nom::{
    error::{ErrorKind, ParseError},
    *,
};

pub use crate::data::error_info::ErrorInfo;
pub use data::CustomError;

// Parsing Errors
pub const ERROR_PARENTHESES: &'static str = "list elem type ( ... ) not found";
pub const ERROR_PARENTHESES_END: &'static str =
    "invalid argument expect one ',' between each argument or ')' to end the list";
pub const ERROR_NUMBER_AS_IDENT: &'static str = "Int/Float can't be used as identifier";
pub const ERROR_FLOW_STEP: &'static str =
    "syntax error.";
pub const ERROR_RESERVED: &'static str = "reserved keyword can't be used as identifier";
pub const ERROR_PARSING: &'static str =
    "invalid argument. Use one of the action keywords [say, do, if, ...]";
pub const ERROR_REMEMBER: &'static str =
    "remember must be assigning to a variable via '='. Example: 'remember key = value'";
pub const ERROR_USE: &'static str =
    "'use' must be assigning a variable with keyword 'as'. Example: 'use value as key'";
pub const ERROR_BREAK: &'static str = "break can only be used inside foreach";
pub const ERROR_HOLD: &'static str = "hold cannot be used inside foreach";
pub const ERROR_LEFT_BRACE: &'static str = "expecting '('";
pub const ERROR_RIGHT_BRACE: &'static str = "expecting ')'";
pub const ERROR_RIGHT_BRACKET: &'static str = "expecting ']'";
pub const ERROR_GOTO_STEP: &'static str = "missing step name after goto";
pub const ERROR_IMPORT_STEP: &'static str = "missing step name after import";
pub const ERROR_DOUBLE_QUOTE: &'static str = "expecting '\"' to end string";
pub const ERROR_DOUBLE_OPEN_BRACE: &'static str = "expecting '{{' to begin expandable string";
pub const ERROR_DOUBLE_CLOSE_BRACE: &'static str = "expecting '}}' to end expandable string";
pub const ERROR_UNREACHABLE: &'static str = "unreachable";
pub const ERROR_WRONG_ARGUMENT_EXPANDABLE_STRING: &'static str =
    "wrong argument(s) given to expandable string";

// Linter Errors
pub const ERROR_NO_FLOW: &'static str = "bot must have at least one flow";

// ##Interpreter Errors
// ### Validation
pub const ERROR_STEP_EXIST: &'static str = "step does not exist";
pub const ERROR_INVALID_FLOW: &'static str = "invalid flow: ";
pub const ERROR_START_INSTRUCTIONS: &'static str =
    "to start an action one of the following instructions is expected: [say, do, if, foreach, goto]";
pub const ERROR_FOREACH: &'static str =
    "foreach only accepts array as iterable elements. Example: foreach(elem) in [1, 2, 3]";
pub const ERROR_FIND_BY_INDEX: &'static str =
    "index must be of type int or string. Example var.[42] or var.[\"key\"]";
pub const ERROR_ASSIGN_IDENT: &'static str = "key must be of type identifier";
pub const ERROR_SIZE_IDENT: &'static str = "key can't be longer than 255 character";
pub const ERROR_FUNCTIONS_ARGS: &'static str = "argument in of function must be in an array";
pub const ERROR_EXPR_TO_LITERAL: &'static str = "expression can't be converted to Literal";
pub const ERROR_PAYLOAD_EXCEED_MAX_SIZE: &'static str = "payload exceeds max payload size (16kb)";

// Event
pub const ERROR_EVENT_CONTENT_TYPE: &'static str = "event can only be of ContentType::Event";

// Component
pub const ERROR_COMPONENT_NAMESPACE: &'static str = "component must have a function applied";
pub const ERROR_COMPONENT_UNKNOWN: &'static str = "function does not exist for component";

// Fn API
pub const ERROR_FN_ID: &'static str = "Fn's name must be of type string";
pub const ERROR_FN_ENDPOINT: &'static str =
    "Fn can not be called because fn_endpoint is not set in bot";
pub const ERROR_FAIL_RESPONSE_JSON: &'static str = "failed to read response as JSON";

// ### Import
pub const ERROR_IMPORT_FAIL: &'static str = "import failed at";
pub const ERROR_IMPORT_STEP_FLOW: &'static str = "step not found in flow";

// ### Variables
pub const ERROR_GET_VAR_INFO: &'static str = "expression must be a variable";
pub const ERROR_JSON_TO_LITERAL: &'static str = "number is too big to be an int 64 bit";

// ### Memory
pub const ERROR_STEP_MEMORY: &'static str = "variable does not exist in step's memory";
pub const ERROR_FIND_MEMORY: &'static str = "is used before it was saved in memory";

// ### Built-in
pub const ERROR_TEXT: &'static str =
    "Text component expects one argument of type string. Example: Text(\"hola\")";
pub const ERROR_TYPING: &'static str =
    "Typing component expects one argument of type int or float. Example: Typing(3, ..)";
pub const ERROR_WAIT: &'static str =
    "Wait component expects one argument of type int or float. Example: Wait(3)";
pub const ERROR_BUTTON: &'static str =
    "Button component expects at least one argument of type string. Example: Button(\"hola\")";
pub const ERROR_CARD_BUTTON: &'static str = "argument 'buttons' in Card component must be of type Array<Button>. Example: [ Button(\"b1\"), Button(\"b2\") ]";
pub const ERROR_CARD_TITLE: &'static str =
    "argument title in Card component must be of type String";
pub const ERROR_QUESTION: &'static str = "argument 'buttons' in Question component must be of type Array<Button>. Example: [ Button(\"b1\"), Button(\"b2\") ]";
pub const ERROR_CAROUSEL: &'static str =
    "argument 'cards' in Carousel component must be of type Array<Card>";
pub const ERROR_ONE_OF: &'static str =
    "OneOf builtin expects one value of type Array. Example: OneOf( [1, 2, 3] )";
pub const ERROR_SHUFFLE: &'static str =
    "Shuffle builtin expects one value of type Array. Example: Shuffle( [1, 2, 3] )";
pub const ERROR_LENGTH: &'static str =
    "Length builtin expects one value of type Array or String. Example: Length( value )";
pub const ERROR_FIND: &'static str = "Find builtin expects 'in' param to be of type String. Example: Find(value, in = \"hola\", case_sensitive = true)";
pub const ERROR_FLOOR: &'static str =
    "Floor builtin expects one argument of type float. Example: Floor(4.2)";
pub const ERROR_IMAGE: &'static str =
    "Image component expects one argument of type string. Example: Image(\"hola\")";
pub const ERROR_URL: &'static str = "Url component expects one argument of type string and 2 optional string arguments: text, title. Example: Url(\"hola\", text = \"text\", title = \"title\")";
pub const ERROR_VIDEO: &'static str =
    "Video component expects one argument of type string. Example: Video(url = \"hola\")";
pub const ERROR_AUDIO: &'static str =
    "Audio component expects one argument of type string. Example: Audio(url = \"hola\")";
pub const ERROR_FILE: &'static str =
    "File component expects one argument of type string. Example: File(url = \"hola\")";
pub const ERROR_HTTP: &'static str =
    "HTTP builtin expects one url of type string. Example: HTTP(\"https://clevy.io\")";
pub const ERROR_HTTP_GET_VALUE: &'static str =
"not found in HTTP object. Use the HTTP() builtin to construct the correct object to make HTTP calls";
pub const ERROR_HTTP_QUERY_VALUES: &'static str =
    "must have a value of type String. Example: {key: \"value\"}";
pub const ERROR_BUILTIN_UNKNOWN: &'static str = "Unknown builtin";

// ### native Components
pub const ERROR_HTTP_NOT_DATA: &'static str = "bad format: no 'data' in HTTP response";
pub const ERROR_NATIVE_COMPONENT: &'static str = "native component does not exist";

// ### Primitives
// #### Boolean
pub const ERROR_BOOLEAN_UNKNOWN_METHOD: &'static str = "is not a method of Boolean";

// #### NUMBER
pub const ERROR_NUMBER_POW: &'static str =
    "[pow] takes one parameter of type int or float usage: number.pow(42)";

// #### Float
pub const ERROR_FLOAT_UNKNOWN_METHOD: &'static str = "is not a method of Float";

// #### Int
pub const ERROR_INT_UNKNOWN_METHOD: &'static str = "is not a method of Int";

// #### Null
pub const ERROR_NULL_UNKNOWN_METHOD: &'static str = "is not a method of Null";

// #### String
pub const ERROR_STRING_DO_MATCH: &'static str =
    "[do_match] takes one parameter of type String usage: string.do_match(\"tag\")";
pub const ERROR_STRING_APPEND: &'static str =
    "[append] takes one parameter of type String usage: string.append(\"text to append\")";
pub const ERROR_STRING_CONTAINS: &'static str =
    "[contains] takes one parameter of type String usage: string.contains(\"word\")";
pub const ERROR_STRING_CONTAINS_REGEX: &'static str =
    "[contains_regex] takes one parameter of type String usage: string.contains_regex(\"regex\")";
pub const ERROR_STRING_VALID_REGEX: &'static str = "parameter must be a valid regex expression"; // link to docs
pub const ERROR_STRING_START_WITH: &'static str =
    "[starts_with] takes one parameter of type String usage: string.starts_with(\"tag\")";
pub const ERROR_STRING_START_WITH_REGEX: &'static str = "[starts_with_regex] takes one parameter of type String usage: string.start_with_regex(\"regex\")";
pub const ERROR_STRING_END_WITH: &'static str =
    "[ends_with] takes one parameter of type String usage: string.ends_with(\"tag\")";
pub const ERROR_STRING_END_WITH_REGEX: &'static str =
    "[ends_with_regex] takes one parameter of type String usage: string.ends_with_regex(\"regex\")";
pub const ERROR_STRING_FROM_JSON: &'static str = "[from_json] [!] string to object failed]";
pub const ERROR_STRING_MATCH_REGEX: &'static str =
    "[match_regex] takes one parameter of type String usage: string.match_regex(\"regex\")";
pub const ERROR_STRING_POW: &'static str =
    "[pow] takes one parameter of type Float or Int usage: string.pow(number)";
pub const ERROR_STRING_COS: &'static str = "[cos] the string must be of numeric type in order to use cos. Verify first with 'string.is_number() == true' ";
pub const ERROR_STRING_NUMERIC: &'static str = "the string must be of numeric type in order to use this method. Verify first with 'string.is_number() == true' to check it";
pub const ERROR_STRING_RHS: &'static str = "rhs must be of type string";
pub const ERROR_STRING_UNKNOWN_METHOD: &'static str = "is not a method of String";

// #### Array
pub const ERROR_ARRAY_TYPE: &'static str = "value must be of type array";
pub const ERROR_ARRAY_INDEX_EXIST: &'static str = "index does not exist";
pub const ERROR_ARRAY_INDEX_TYPE: &'static str = "index must be of type int";
pub const ERROR_ARRAY_NEGATIVE: &'static str = "index must be positive. Udage: array[1]";
pub const ERROR_ARRAY_INDEX: &'static str = "index must be lower than or equal to array.length()";
pub const ERROR_ARRAY_OVERFLOW: &'static str =
    "[push] Cannot push inside array, since array limit is ";
pub const ERROR_ARRAY_POP: &'static str = "[pop] Cannot pop if array is empty";
pub const ERROR_ARRAY_INSERT_AT: &'static str =
    "[insert_at] takes two parameters. Usage: array.insert_at(1, elem)";
pub const ERROR_ARRAY_INSERT_AT_INT: &'static str =
    "[insert_at] first parameter must be of type int. Usage: array.insert_at(1, elem)";
pub const ERROR_ARRAY_REMOVE_AT: &'static str =
    "[remove_at] takes one parameter of type Int. Usage: array.remove_at(1) ";
pub const ERROR_ARRAY_JOIN: &'static str =
    "[join] takes one parameter of type String. Usage: array.join(\"elem\") ";
pub const ERROR_ARRAY_INDEX_OF: &'static str =
    "[index_of] takes one parameter. Usage: array.index_of(elem)";
pub const ERROR_ARRAY_FIND: &'static str = "[find] takes one parameter. Usage: array.find(elem)";
pub const ERROR_ARRAY_UNKNOWN_METHOD: &'static str = "is not a method of Array";

// #### HTTP OBJECT
pub const ERROR_HTTP_SET: &'static str =
    "[set] takes one parameter of type Object. Usage: HTTP(...).set( {\"key\": 42} )";
pub const ERROR_HTTP_QUERY: &'static str =
    "[query] takes one parameter of type Object. Usage: HTTP(...).query( {\"key\": 42} )";
pub const ERROR_HTTP_POST: &'static str =
    "[post] takes one parameter of type Object. Usage: HTTP(...).post( {\"key\": 42} )";
pub const ERROR_HTTP_PUT: &'static str =
    "[put] takes one parameter of type Object. Usage: HTTP(...).put( {\"key\": 42} )";
pub const ERROR_HTTP_PATCH: &'static str =
    "[patch] takes one parameter of type Object. Usage: HTTP(...).patch( {\"key\": 42} )";
pub const ERROR_HTTP_SEND: &'static str =
    "[send] HTTP Object is bad formatted read doc for correct usage";
pub const ERROR_HTTP_UNKNOWN_METHOD: &'static str = "is not a method of HTTP";

// #### OBJECT
pub const ERROR_OBJECT_TYPE: &'static str = "value must be of type Object";
pub const ERROR_OBJECT_GET: &'static str = "key does not exist";
pub const ERROR_OBJECT_CONTAINS: &'static str =
    "[contains] takes one parameter of type String. Usage: object.contains(\"key\")";
pub const ERROR_OBJECT_GET_GENERICS: &'static str =
    "[get_generics] takes one parameter of type String. Usage: object.get_generics(\"key\")";
pub const ERROR_OBJECT_INSERT: &'static str =
    "[insert] take tow parameters. Usage: object.insert(string, any_type)";
pub const ERROR_OBJECT_REMOVE: &'static str =
    "[remove] takes one parameter of type String. Usage: object.remove(\"key\")";
pub const ERROR_OBJECT_GET_KEY: &'static str = "key must be of type String";
pub const ERROR_OBJECT_UNKNOWN_METHOD: &'static str = "is not a method of Object";

// #### METHODS
pub const ERROR_METHOD_NAMED_ARGS: &'static str = "arguments in method are not named";

pub const ERROR_OPS: &'static str = "[!] Ops: Illegal operation";
pub const ERROR_OPS_DIV_INT: &'static str = "[!] Int: Division by zero";
pub const ERROR_OPS_DIV_FLOAT: &'static str = "[!] Float: Division by zero";

pub const ERROR_ILLEGAL_OPERATION: &'static str = "illegal operation:";
pub const OVERFLOWING_OPERATION: &'static str = "overflowing operation:";

pub fn gen_error_info(position: Position, message: String) -> ErrorInfo {
    ErrorInfo::new(position, message)
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
