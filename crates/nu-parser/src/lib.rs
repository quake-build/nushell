mod deparse;
mod flatten;
mod known_external;
mod lex;
mod lite_parser;
mod parse_keywords;
mod parse_patterns;
mod parse_shape_specs;
mod parser;
mod parser_path;
mod type_check;

pub use deparse::{escape_for_script_arg, escape_quote_string};
pub use flatten::{
    flatten_block, flatten_expression, flatten_pipeline, flatten_pipeline_element, FlatShape,
};
pub use known_external::KnownExternal;
pub use lex::{lex, lex_signature, Token, TokenContents};
pub use lite_parser::{lite_parse, LiteBlock, LiteElement};
pub use parse_keywords::*;
pub use parser_path::*;

pub use parser::*;

#[cfg(feature = "plugin")]
pub use parse_keywords::parse_register;
