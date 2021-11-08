pub mod data;

use crate::data::tokens::Span;
use crate::data::{position::Position, warnings::Warnings, Interval};
use nom::{
    error::{ErrorKind, ParseError},
    *,
};

pub use crate::data::error_info::ErrorInfo;
pub use data::CustomError;

// TODO: add link to docs

// Parsing Errors
pub const ERROR_PARENTHESES: &str = "list elem type ( ... ) not found";
pub const ERROR_PARENTHESES_END: &str =
    "Invalid argument. Expecting one ',' between each argument or ')' to end the list";
pub const ERROR_NUMBER_AS_IDENT: &str = "Int/Float can't be used as identifier";
pub const ERROR_FLOW_STEP: &str = "syntax error.";
pub const ERROR_RESERVED: &str = "reserved keyword can't be used as identifier";
pub const ERROR_PARSING: &str =
    "Invalid argument. One of the action keywords [say, do, if, ...] is missing";
pub const ERROR_REMEMBER: &str =
    "'remember' must be assigning to a variable via '='. Example: 'remember key = value'";
pub const ERROR_USE: &str =
    "'use' must be assigning a variable with keyword 'as'. Example: 'use value as key'";
pub const ERROR_ACTION_ARGUMENT: &str =
    "expecting valid argument after action keywords. Example: say value";
pub const ERROR_IMPORT_ARGUMENT: &str =
    "'import' expecting valid function name. Example: 'import function from flow'";
pub const ERROR_BREAK: &str = "break can only be used inside loops";
pub const ERROR_RETURN: &str = "return expects a value to return";
pub const ERROR_LEFT_BRACE: &str = "expecting '{'";
pub const ERROR_RIGHT_BRACE: &str = "expecting '}'";
pub const ERROR_RIGHT_BRACKET: &str = "expecting ']'";
pub const ERROR_GOTO_STEP: &str = "missing step name after goto";
pub const ERROR_IMPORT_STEP: &str = "missing step name after import";
pub const ERROR_DOUBLE_QUOTE: &str = "expecting '\"' to end string";
pub const ERROR_DOUBLE_OPEN_BRACE: &str = "expecting '{{' to begin expandable string";
pub const ERROR_DOUBLE_CLOSE_BRACE: &str = "expecting '}}' to end expandable string";
pub const ERROR_UNREACHABLE: &str = "unreachable";
pub const ERROR_WRONG_ARGUMENT_EXPANDABLE_STRING: &str =
    "wrong argument(s) given to expandable string";
pub const ERROR_FN_SCOPE: &str =
    "invalid action. Use a valid action for this type of scope [do, if, return, ...]"; //\ndoc: https://docs.csml.dev/language/native-csml-functions

// Linter Errors
pub const ERROR_NO_FLOW: &str = "bot must have at least one flow";

// ##Interpreter Errors
// ### Validation
pub const ERROR_STEP_EXIST: &str = "step does not exist";
pub const ERROR_INVALID_FLOW: &str = "invalid flow: ";
pub const ERROR_START_INSTRUCTIONS: &str =
    "to start an action one of the following instructions is expected: [say, do, if, foreach, goto]";
pub const ERROR_FOREACH: &str =
    "foreach only accepts iterable elements like arrays and strings. Example: foreach(elem) in [1, 2, 3]";
pub const ERROR_FIND_BY_INDEX: &str =
    "index must be of type int or string. Example var.[42] or var.[\"key\"]";
pub const ERROR_ASSIGN_IDENT: &str = "key must be of type identifier";
pub const ERROR_SIZE_IDENT: &str = "key can't be longer than 255 character";
pub const ERROR_NUMBER_AS_KEY: &str = "Int/Float can't be used as key";
pub const ERROR_KEY_ALPHANUMERIC: &str = "key must be alphanumeric";
pub const ERROR_FUNCTIONS_ARGS: &str = "function arguments must be in an array";
pub const ERROR_EXPR_TO_LITERAL: &str = "expression can't be converted to Literal";
pub const ERROR_PAYLOAD_EXCEED_MAX_SIZE: &str = "payload exceeds max payload size (16kb)";

pub const ERROR_STEP_LIMIT: &str =
    "[Infinite loop] Step limit reached: 100 steps where executed in a single run";

// Event
pub const ERROR_EVENT_CONTENT_TYPE: &str = "event can only be of ContentType::Event";

// Goto
pub const ERROR_GOTO_VAR: &str = "variables in goto need to resolve as strings";

// Component
pub const ERROR_COMPONENT_NAMESPACE: &str = "component must have a function applied";
pub const ERROR_COMPONENT_UNKNOWN: &str = "function does not exist for component";

// Fn API
pub const ERROR_FN_ID: &str = "App name must be of type string";
pub const ERROR_FN_ENDPOINT: &str = "App can not be called because fn_endpoint is not set in bot";
pub const ERROR_FAIL_RESPONSE_JSON: &str = "failed to read response as JSON";

// ### Import
pub const ERROR_IMPORT_FAIL: &str = "import failed at";
pub const ERROR_IMPORT_STEP_FLOW: &str = "step not found in flow";

// ### Variables
pub const ERROR_GET_VAR_INFO: &str = "Expression must be a variable";
pub const ERROR_JSON_TO_LITERAL: &str = "Number is larget than a 64-bit integer";

// ### Memory
pub const ERROR_STEP_MEMORY: &str = "Variable does not exist in step's memory";
pub const ERROR_FIND_MEMORY: &str = "is used before it was saved in memory";

// ### Functions
pub const ERROR_FN_ARGS: &str = "function arguments are not valid";
pub const ERROR_FN_COLON: &str =
    "Expecting ':' at the end of function prototype. Example: 'fn name():' ";

// ### Built-in
pub const ERROR_TEXT: &str =
    "Text component expects one argument of type string. Example: Text(\"hola\")";
pub const ERROR_TYPING: &str =
    "Typing component expects one argument of type int or float. Example: Typing(3, ..)";
pub const ERROR_WAIT: &str =
    "Wait component expects one argument of type int or float. Example: Wait(3)";
pub const ERROR_BUTTON: &str =
    "Button component expects at least one argument of type string. Example: Button(\"hola\")";
pub const ERROR_CARD_BUTTON: &str = "argument 'buttons' in Card component must be of type Array<Button>. Example: [ Button(\"b1\"), Button(\"b2\") ]";
pub const ERROR_CARD_TITLE: &str = "argument title in Card component must be of type String";
pub const ERROR_QUESTION: &str = "argument 'buttons' in Question component must be of type Array<Button>. Example: [ Button(\"b1\"), Button(\"b2\") ]";
pub const ERROR_CAROUSEL: &str =
    "argument 'cards' in Carousel component must be of type Array<Card>";
pub const ERROR_ONE_OF: &str =
    "OneOf builtin expects one value of type Array. Example: OneOf( [1, 2, 3] )";
pub const ERROR_VAR_EXISTS: &str = "Exists builtin expects one value of type String. Example: Exists( \"var_name\" )";
pub const ERROR_SHUFFLE: &str =
    "Shuffle builtin expects one value of type Array. Example: Shuffle( [1, 2, 3] )";
pub const ERROR_LENGTH: &str =
    "Length builtin expects one value of type Array or String. Example: Length( value )";
pub const ERROR_FIND: &str = "Find builtin expects 'in' param to be of type String. Example: Find(value, in = \"hola\", case_sensitive = true)";
pub const ERROR_FLOOR: &str =
    "Floor builtin expects one argument of type float. Example: Floor(4.2)";
pub const ERROR_UUID: &str =
    "UUID builtin expects one optional argument of type String. Example: UUID(\"v4\") or UUID(\"v1\")";
pub const ERROR_IMAGE: &str =
    "Image component expects one argument of type string. Example: Image(\"hola\")";
pub const ERROR_URL: &str = "Url component expects one argument of type string and 2 optional string arguments: text, title. Example: Url(\"hola\", text = \"text\", title = \"title\")";
pub const ERROR_VIDEO: &str =
    "Video component expects one argument of type string. Example: Video(url = \"hola\")";
pub const ERROR_AUDIO: &str =
    "Audio component expects one argument of type string. Example: Audio(url = \"hola\")";
pub const ERROR_FILE: &str =
    "File component expects one argument of type string. Example: File(url = \"hola\")";
pub const ERROR_HTTP_GET_VALUE: &str =
    "not found in HTTP object. Use the HTTP() builtin to construct the correct object to make HTTP calls";
pub const ERROR_HTTP_QUERY_VALUES: &str =
    "must have a value of type String. Example: {key: \"value\"}";
pub const ERROR_HTTP: &str =
    "HTTP builtin expects one url of type string. Example: HTTP(\"https://clevy.io\")";
pub const ERROR_JWT: &str = "JWT builtin expects payload as argument. Example: JWT({
        \"user\": \"name\",
        \"somekey\": {
          \"somevalue\": 42
        },
        \"exp\": 1618064023,
        \"iss\": \"CSML STUDIO\"
      })";
pub const ERROR_SMTP: &str =
    "SMTP builtin expects SMTP Server Address. Example: SMTP(\"smtp.gmail.com\")";
pub const ERROR_CRYPTO: &str =
    "CRYPTO builtin expects one argument of type string. Example: CRYPTO(\"text\")";
pub const ERROR_BUILTIN_UNKNOWN: &str = "Unknown builtin";

// ### native Components
pub const ERROR_HTTP_NOT_DATA: &str = "bad format: no 'data' in HTTP response";
pub const ERROR_NATIVE_COMPONENT: &str = "native component does not exist";

// ### Primitives
// #### Indexing
pub const ERROR_INDEXING: &str =
    "indexing can only be done in ARRAY, OBJECT or STRING primitive types";

// #### Closure
pub const ERROR_CLOSURE_UNKNOWN_METHOD: &str = "Closure don't have methods";

// #### Boolean
pub const ERROR_BOOLEAN_UNKNOWN_METHOD: &str = "is not a method of Boolean";

// #### NUMBER
pub const ERROR_NUMBER_POW: &str =
    "[pow] takes one parameter of type int or float usage: number.pow(42)";

// #### Float
pub const ERROR_FLOAT_UNKNOWN_METHOD: &str = "is not a method of Float";

// #### Int
pub const ERROR_INT_UNKNOWN_METHOD: &str = "is not a method of Int";

// #### Null
pub const ERROR_NULL_UNKNOWN_METHOD: &str = "is not a method of Null";

// #### String
pub const ERROR_STRING_DO_MATCH: &str =
    "[do_match] takes one parameter of type String. Usage: string.do_match(\"tag\")";
pub const ERROR_STRING_APPEND: &str =
    "[append] takes one parameter of type String. Usage: string.append(\"text to append\")";
pub const ERROR_STRING_CONTAINS: &str =
    "[contains] takes one parameter of type String. Usage: string.contains(\"word\")";
pub const ERROR_STRING_REPLACE: &str =
    "[replace] takes tow parameter of type String. Usage: \"this is old\".replace(\"old\", \"new\")";
pub const ERROR_STRING_REPLACE_ALL: &str =
    "[replace_all] takes tow parameter of type String. Usage: \"old old old old\".replace_all(\"old\", \"new\")";
pub const ERROR_STRING_REPLACE_REGEX: &str =
    "[replace_regex] takes tow parameter of type String. Usage: \"hello world\".replace_regex(\"world\", \"Clevy\")";
pub const ERROR_STRING_CONTAINS_REGEX: &str =
    "[contains_regex] takes one parameter of type String. Usage: string.contains_regex(\"regex\")";
pub const ERROR_STRING_VALID_REGEX: &str = "parameter must be a valid regex expression"; // link to docs
pub const ERROR_STRING_START_WITH: &str =
    "[starts_with] takes one parameter of type String. Usage: string.starts_with(\"tag\")";
pub const ERROR_STRING_START_WITH_REGEX: &str = "[starts_with_regex] takes one parameter of type String. Usage: string.start_with_regex(\"regex\")";
pub const ERROR_STRING_END_WITH: &str =
    "[ends_with] takes one parameter of type String. Usage: string.ends_with(\"tag\")";
pub const ERROR_STRING_END_WITH_REGEX: &str =
    "[ends_with_regex] takes one parameter of type String. Usage: string.ends_with_regex(\"regex\")";
pub const ERROR_STRING_FROM_JSON: &str = "[from_json] [!] string to object failed]";
pub const ERROR_STRING_SPLIT: &str =
    "[split] takes one parameter of type String. Usage: string.split(\"separator\")";
pub const ERROR_STRING_MATCH_REGEX: &str =
    "[match_regex] takes one parameter of type String. Usage: string.match_regex(\"regex\")";
pub const ERROR_STRING_POW: &str =
    "[pow] takes one parameter of type Float or Int. Usage: string.pow(number)";
pub const ERROR_STRING_COS: &str = "[cos] the string must be of numeric type in order to use cos. Verify first with 'string.is_number() == true' ";
pub const ERROR_STRING_NUMERIC: &str = "the string must be of numeric type in order to use this method. Verify first with 'string.is_number() == true' to check it";
pub const ERROR_STRING_RHS: &str = "rhs must be of type string";

pub const ERROR_SLICE_ARG_INT: &str =
    ".slice(start, optional<end>) args need to be of type Integer";
pub const ERROR_SLICE_ARG_LEN: &str =
    ".slice(start, optional<end>) args need to be inferior to the string length";
pub const ERROR_SLICE_ARG2: &str =
    ".slice(start, optional<end>) end need to be superior to start in value ex: .slice(2, 5)";

pub const ERROR_STRING_UNKNOWN_METHOD: &str = "is not a method of String";

// #### Array
pub const ERROR_ARRAY_TYPE: &str = "value must be of type array";
pub const ERROR_ARRAY_INDEX_EXIST: &str = "index does not exist";
pub const ERROR_ARRAY_INDEX_TYPE: &str = "index must be of type int";
pub const ERROR_ARRAY_NEGATIVE: &str = "index must be positive. Udage: array[1]";
pub const ERROR_ARRAY_INDEX: &str = "index must be lower than or equal to array.length()";
pub const ERROR_ARRAY_OVERFLOW: &str = "[push] Cannot push inside array, since array limit is ";
pub const ERROR_ARRAY_POP: &str = "[pop] Cannot pop if array is empty";
pub const ERROR_ARRAY_INSERT_AT: &str =
    "[insert_at] takes two parameters. Usage: array.insert_at(1, elem)";
pub const ERROR_ARRAY_INSERT_AT_INT: &str =
    "[insert_at] first parameter must be of type int. Usage: array.insert_at(1, elem)";
pub const ERROR_ARRAY_REMOVE_AT: &str =
    "[remove_at] takes one parameter of type Int. Usage: array.remove_at(1) ";
pub const ERROR_ARRAY_JOIN: &str =
    "[join] takes one parameter of type String. Usage: array.join(\"elem\") ";
pub const ERROR_ARRAY_INDEX_OF: &str =
    "[index_of] takes one parameter. Usage: array.index_of(elem)";
pub const ERROR_ARRAY_FIND: &str = "[find] takes one parameter. Usage: array.find(elem)";
pub const ERROR_ARRAY_UNKNOWN_METHOD: &str = "is not a method of Array";

// #### CRYPTO OBJECT
// ## HMAC and HASH OBJECT
pub const ERROR_HASH: &str = "Crypto(string) command expect argument of type String";
pub const ERROR_HASH_ALGO: &str =
    "Invalid Algorithm, supported Algorithms are md5 sha1 sha256 sha384 sha512";
pub const ERROR_HMAC_KEY: &str = "HMAC key need to be of type string";

pub const ERROR_DIGEST: &str = "Invalid argument, '.digest' is use incorrectly";
pub const ERROR_DIGEST_ALGO: &str =
    "Invalid Digest Algorithm, supported Algorithms are hex, base64";

// #### JWT OBJECT
pub const ERROR_JWT_ALGO: &str = "Invalid Algorithm, supported Algorithms are HS256, HS384, HS512";
pub const ERROR_JWT_SECRET: &str = "secret must be of type String";

pub const ERROR_JWT_SIGN_CLAIMS: &str =
    "JWT(claims) command expect argument 'claims' of type Object";
pub const ERROR_JWT_SIGN_ALGO: &str =
    "JWT(claims).sign(algo, secret, Optional<Header>) expect first argument 'algo' of type String";
pub const ERROR_JWT_SIGN_SECRET: &str = "JWT(claims).sign(algo, secret, Optional<Header>) expect second argument 'claims' of type String";

pub const ERROR_JWT_TOKEN: &str = "JWT(jwt) command expect argument 'jwt' of type String";

pub const ERROR_JWT_DECODE_ALGO: &str =
    "JWT(jwt).decode(algo, secret) expect first argument 'algo' of type String";
pub const ERROR_JWT_DECODE_SECRET: &str =
    "JWT(jwt).decode(algo, secret) expect second argument 'claims' of type String";

pub const ERROR_JWT_VALIDATION_CLAIMS: &str =
    "JWT(jwt).verify(claims, algo, secret) expect first argument 'claims' of type Object";
pub const ERROR_JWT_VALIDATION_ALGO: &str =
    "JWT(jwt).verify(claims, algo, secret) expect second argument 'algo' of type String";
pub const ERROR_JWT_VALIDATION_SECRETE: &str =
    "JWT(jwt).verify(claims, algo, secret) expect third argument 'secrete' of type String";

// #### HTTP OBJECT
pub const ERROR_HTTP_SET: &str =
    "[set] takes one parameter of type Object. Usage: HTTP(...).set( {\"key\": 42} )";
pub const ERROR_HTTP_QUERY: &str =
    "[query] takes one parameter of type Object. Usage: HTTP(...).query( {\"key\": 42} )";

pub const ERROR_HTTP_SEND: &str = "[send] HTTP Object is bad formatted read doc for correct usage";
pub const ERROR_HTTP_UNKNOWN_METHOD: &str = "is not a method of HTTP";

// #### OBJECT
pub const ERROR_OBJECT_TYPE: &str = "value must be of type Object";
pub const ERROR_OBJECT_GET: &str = "key does not exist";
pub const ERROR_OBJECT_CONTAINS: &str =
    "[contains] takes one parameter of type String. Usage: object.contains(\"key\")";
pub const ERROR_OBJECT_GET_GENERICS: &str =
    "[get_generics] takes one parameter of type String. Usage: object.get_generics(\"key\")";
pub const ERROR_OBJECT_INSERT: &str =
    "[insert] take tow parameters. Usage: object.insert(string, any_type)";
pub const ERROR_OBJECT_REMOVE: &str =
    "[remove] takes one parameter of type String. Usage: object.remove(\"key\")";
pub const ERROR_OBJECT_GET_KEY: &str = "key must be of type String";
pub const ERROR_OBJECT_UNKNOWN_METHOD: &str = "is not a method of Object";

// #### METHODS
pub const ERROR_METHOD_NAMED_ARGS: &str = "arguments in method are not named";

pub const ERROR_OPS: &str = "[!] Ops: Illegal operation";
pub const ERROR_OPS_DIV_INT: &str = "[!] Int: Division by zero";
pub const ERROR_OPS_DIV_FLOAT: &str = "[!] Float: Division by zero";

pub const ERROR_ILLEGAL_OPERATION: &str = "illegal operation:";
pub const OVERFLOWING_OPERATION: &str = "overflowing operation:";

////////////////////////////////////////////////////////////////////////////////
// PRiVTE FUNCTION
////////////////////////////////////////////////////////////////////////////////

fn add_context_to_error_message<'a>(
    flow_slice: Span<'a>,
    message: String,
    line_number: u32,
    column: usize,
    offset: usize,
) -> String {
    use std::fmt::Write;

    let mut result = String::new();

    let prefix = &flow_slice.fragment().as_bytes()[..offset];

    // Find the line that includes the subslice:
    // Find the *last* newline before the substring starts
    let line_begin = prefix
        .iter()
        .rev()
        .position(|&b| b == b'\n')
        .map(|pos| offset - pos)
        .unwrap_or(0);

    // Find the full line after that newline
    let line = flow_slice.fragment()[line_begin..]
        .lines()
        .next()
        .unwrap_or(&flow_slice.fragment()[line_begin..])
        .trim_end();

    write!(
        &mut result,
        "at line {line_number},\n\
            {line}\n\
            {caret:>column$}\n\
            {context}\n\n",
        line_number = line_number,
        context = message,
        line = line,
        caret = '^',
        column = column,
    )
    // Because `write!` to a `String` is infallible, this `unwrap` is fine.
    .unwrap();

    result
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn gen_error_info(position: Position, message: String) -> ErrorInfo {
    ErrorInfo::new(position, message)
}

pub fn gen_warning_info(position: Position, message: String) -> Warnings {
    Warnings { position, message }
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

pub fn convert_error_from_span<'a>(flow_slice: Span<'a>, e: CustomError<Span<'a>>) -> String {
    let message = e.error.to_owned();
    let offset = e.input.location_offset();
    // Count the number of newlines in the first `offset` bytes of input
    let line_number = e.input.location_line();
    // The (1-indexed) column number is the offset of our substring into that line
    let column = e.input.get_column();

    add_context_to_error_message(flow_slice, message, line_number, column, offset)
}

pub fn convert_error_from_interval<'a>(
    flow_slice: Span<'a>,
    message: String,
    interval: Interval,
) -> String {
    let offset = interval.offset;
    // Count the number of newlines in the first `offset` bytes of input
    let line_number = interval.start_line;
    // The (1-indexed) column number is the offset of our substring into that line
    let column = interval.start_column as usize;

    add_context_to_error_message(flow_slice, message, line_number, column, offset)
}

pub fn gen_infinite_loop_error_msg(infinite_loop: Vec<(String, String)>) -> String {
    infinite_loop
        .iter()
        .fold(String::new(), |mut acc, (flow, step)| {
            acc.push_str(&format!("[flow] {}, [step] {}\n", flow, step));
            acc
        })
}
