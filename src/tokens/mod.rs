use crate::tokens::{keywords::KeywordKind, ops::OpKind};

mod keywords;
mod ops;

pub enum TokenKind {
    Identifier(String),
    Keywords(KeywordKind),
    Ops(OpKind),
}
