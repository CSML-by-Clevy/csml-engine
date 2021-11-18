use nom_locate::LocatedSpan;

pub type Span<'a> = LocatedSpan<&'a str>;

pub const PORT: &str = "3002";

pub const WHITE_SPACE: &str = " \t\n\r";
pub const INLINE_COMMENT: &str = "//";

pub const START_COMMENT: &str = "/*";
pub const END_COMMENT: &str = "*/";

pub const DOLLAR: &str = "$";

pub const ADDITION: &str = "+";
pub const SUBTRACTION: &str = "-";
pub const DIVIDE: &str = "/";
pub const MULTIPLY: &str = "*";
pub const REMAINDER: &str = "%";
pub const NOT: &str = "!";

pub const EQUAL: &str = "==";
pub const NOT_EQUAL: &str = "!=";
pub const ASSIGN: &str = "=";

pub const OR: &str = "||";
pub const AND: &str = "&&";

pub const SUBTRACTION_ASSIGNMENT: &str = "-=";
pub const ADDITION_ASSIGNMENT: &str = "+=";

pub const GREATER_THAN_EQUAL: &str = ">=";
pub const LESS_THAN_EQUAL: &str = "<=";
pub const GREATER_THAN: &str = ">";
pub const LESS_THAN: &str = "<";

pub const COMMA: &str = ",";
pub const DOT: &str = ".";
pub const SEMICOLON: &str = ";";
pub const COLON: &str = ":";
pub const DOUBLE_QUOTE: &str = "\"";
pub const BACKSLASH_DOUBLE_QUOTE: &str = "\\\"";

pub const UNDERSCORE: char = '_';

pub const L_PAREN: &str = "(";
pub const R_PAREN: &str = ")";
pub const L_BRACE: &str = "{";
pub const R_BRACE: &str = "}";
pub const L_BRACKET: &str = "[";
pub const R_BRACKET: &str = "]";
pub const L2_BRACE: &str = "{{";
pub const R2_BRACE: &str = "}}";

pub const FOREACH: &str = "foreach";
pub const WHILE: &str = "while";
pub const IF: &str = "if";
pub const ELSE: &str = "else";

pub const IMPORT: &str = "import";
pub const FROM: &str = "from";
pub const AS: &str = "as";
pub const IN: &str = "in";
pub const DO: &str = "do";
pub const EVENT: &str = "event";
pub const COMPONENT: &str = "Component";

pub const FLOW: &str = "flow";
pub const STEP: &str = "step";
pub const SAY: &str = "say";
pub const DEBUG_ACTION: &str = "debug";
pub const USE: &str = "use";
pub const HOLD: &str = "hold";
pub const GOTO: &str = "goto";
pub const PREVIOUS: &str = "previous";
pub const MATCH: &str = "match";
pub const NOT_MATCH: &str = "!match";
pub const DEFAULT: &str = "default";
pub const REMEMBER: &str = "remember";
pub const FORGET: &str = "forget";
pub const _METADATA: &str = "_metadata";
pub const _MEMORY: &str = "_memory";
pub const _ENV: &str = "_env";
pub const BREAK: &str = "break";
pub const CONTINUE: &str = "continue";
pub const RETURN: &str = "return";

pub const FN_SCOPE_REJECTED: &[&str] = &[SAY, GOTO, REMEMBER, FORGET, USE, HOLD, BREAK];

pub const TRUE: &str = "true";
pub const FALSE: &str = "false";
pub const NULL: &str = "null";

pub const OBJECT_TYPE: &str = "object";
pub const ARRAY: &str = "array";
pub const TEXT_TYPE: &str = "text";
pub const STRING: &str = "string";
pub const INT: &str = "int";
pub const FLOAT: &str = "float";
pub const BOOLEAN: &str = "boolean";
pub const CLOSURE: &str = "closure";

pub const TYPES: &[&str] = &[
    CLOSURE,
    OBJECT_TYPE,
    ARRAY,
    TEXT_TYPE,
    STRING,
    INT,
    FLOAT,
    BOOLEAN,
    NULL,
];

pub const RESERVED: &[&str] = &[
    FOREACH, WHILE, IF, ELSE, IMPORT, AS, IN, DO, FROM, EVENT, FLOW, FILE, STEP, SAY, USE, HOLD, GOTO,
    MATCH, _METADATA, _MEMORY, _ENV, DEFAULT, REMEMBER, FORGET, TRUE, FALSE, NULL, BREAK,
    COMPONENT,
];

pub const UTILISATION_RESERVED: &[&str] = &[
    FOREACH, WHILE, IF, ELSE, IMPORT, AS, DO, FLOW, STEP, SAY, USE, HOLD, GOTO, MATCH, REMEMBER, FORGET,
    BREAK, COMPONENT,
];

pub const ASSIGNATION_RESERVED: &[&str] = &[
    FOREACH, WHILE , IF, ELSE, IMPORT, AS, DO, EVENT, FLOW, STEP, SAY, USE, HOLD, GOTO, MATCH, REMEMBER,
    FORGET, _METADATA, _MEMORY, _ENV, TRUE, FALSE, NULL, BREAK, COMPONENT,
];

pub const TYPING: &str = "Typing";
pub const WAIT: &str = "Wait";
pub const TEXT: &str = "Text";
pub const URL: &str = "Url";
pub const IMAGE: &str = "Image";
pub const ONE_OF: &str = "OneOf";
pub const SHUFFLE: &str = "Shuffle";
pub const LENGTH: &str = "Length";
pub const FIND: &str = "Find";
pub const RANDOM: &str = "Random";
pub const FLOOR: &str = "Floor";
pub const VIDEO: &str = "Video";
pub const AUDIO: &str = "Audio";

pub const QUESTION: &str = "Question";
pub const BUTTON: &str = "Button";
pub const CAROUSEL: &str = "Carousel";
pub const CARD: &str = "Card";
pub const FN: &str = "Fn";
pub const APP: &str = "App";
pub const HTTP: &str = "HTTP";
pub const SMTP: &str = "SMTP";
pub const JWT: &str = "JWT";
pub const CRYPTO: &str = "Crypto";
pub const BASE64: &str = "Base64";
pub const HEX: &str = "Hex";
pub const FILE: &str = "File";
pub const DEBUG: &str = "Debug";
pub const UUID: &str = "UUID";
pub const TIME: &str = "Time";
pub const EXISTS: &str = "Exists";

pub const OBJECT: &str = "Object";

pub const BUILT_IN: &[&str] = &[
    ONE_OF, SHUFFLE, LENGTH, FIND, RANDOM, FLOOR, FN, APP, HTTP, OBJECT, DEBUG, UUID, BASE64, HEX,
    JWT, CRYPTO, TIME, SMTP, EXISTS,
];

pub const FROM_FILE: &str = "FromFile";
pub const GET_VALUE: &str = "GetValue";
pub const FIRST: &str = "first";

pub const MEMORY: &str = "memory";
