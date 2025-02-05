use tree_sitter::{Node, Parser};

pub trait Language {
    fn get_name(&self) -> &'static str;
    fn analyze_to_vec(&self, source_code: String) -> Vec<LocalVecBlock>;
}

#[derive(Debug, PartialEq)]
enum CodeBlock {
    Else(i32, i32),
    Match(i32, i32, usize),
    If(i32, i32, i32),
    For(i32, i32),
    While(i32, i32),
    Loop(i32, i32),
    Func,
    Return,
    Continue,
}

#[derive(Debug, PartialEq)]
pub enum BlockType {
    Start,
    End,
    Action,
    Print,
    Condition,
    Cycle,
    Else,
    EndMatchArm,
}

#[derive(Debug)]
pub struct LocalVecBlock {
    pub r#type: BlockType,
    pub text: String,
    pub x: i32,
    pub y: i32,
}

struct Context {
    blocks: Vec<LocalVecBlock>,
    block_stack: Vec<CodeBlock>,
    if_else_stack: Vec<(i32, i32)>,
    x_offset: i32,
    y_offset: i32,
    y_if_max: i32,
    skip_until_brace: bool,
    is_return: bool,
}

impl Context {
    fn new() -> Self {
        Self {
            blocks: Vec::new(),
            block_stack: Vec::new(),
            if_else_stack: Vec::new(),
            x_offset: 0,
            y_offset: 0,
            y_if_max: 0,
            skip_until_brace: false,
            is_return: false,
        }
    }
}

enum NodeType {
    Function,
    If,
    Else,
    Loop,
    For,
    While,
    Match,
    Return,
    Macro,
    Other,
}
