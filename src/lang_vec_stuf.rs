use std::collections::HashMap;
use tree_sitter::{Node, Parser};

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
pub struct CodeBlock {
    pub r#type: BlockType,
    pub text: String,
    pub x: i32,
    pub y: i32,
}

struct DiagramBuilder {
    source: String,
    blocks: Vec<CodeBlock>,
    position: (i32, i32),
    block_stack: Vec<BlockScope>,
}

#[derive(Debug)]
enum BlockScope {
    Function,
    If(i32),
    Loop(i32),
    Match(i32, usize),
}

impl DiagramBuilder {
    fn new(source: String) -> Self {
        Self {
            source,
            blocks: Vec::new(),
            position: (0, 0),
            block_stack: Vec::new(),
        }
    }

    fn add_block(&mut self, block_type: BlockType, text: impl Into<String>) {
        let block = CodeBlock {
            r#type: block_type,
            text: text.into(),
            x: self.position.0,
            y: self.position.1,
        };
        self.blocks.push(block);
    }

    fn enter_scope(&mut self, scope: BlockScope, x_shift: i32, y_shift: i32) {
        self.block_stack.push(scope);
        self.position.0 += x_shift;
        self.position.1 += y_shift;
    }

    fn exit_scope(&mut self) {
        if let Some(scope) = self.block_stack.pop() {
            match scope {
                BlockScope::Function => self.position.1 += 100,
                BlockScope::If(depth) => {
                    self.position.0 -= 100 * depth;
                    self.position.1 += 200;
                }
                BlockScope::Loop(_) => {
                    self.position.1 += 100;
                }
                BlockScope::Match(_, _) => {
                    self.position.0 -= 300;
                    self.position.1 += 50;
                }
            }
        }
    }
}

struct AstProcessor {
    handlers: HashMap<&'static str, fn(&Node, &mut DiagramBuilder)>,
}

impl AstProcessor {
    fn new() -> Self {
        let mut handlers = HashMap::new();

        handlers.insert("function_item", Self::handle_function);
        handlers.insert("if_expression", Self::handle_if);
        handlers.insert("else_clause", Self::handle_else);
        handlers.insert("for_expression", Self::handle_loop);
        handlers.insert("while_expression", Self::handle_loop);
        handlers.insert("loop_expression", Self::handle_loop);
        handlers.insert("return_expression", Self::handle_return);
        handlers.insert("macro_invocation", Self::handle_macro);
        handlers.insert("match_expression", Self::handle_match);
        handlers.insert("match_arm", Self::handle_match_arm);

        Self { handlers }
    }

    fn process_node(&self, node: &Node, builder: &mut DiagramBuilder) {
        let kind = node.kind();

        if let Some(handler) = self.handlers.get(kind) {
            handler(node, builder);
        } else {
            self.handle_generic(node, builder);
        }

        self.process_children(node, builder);
    }

    fn process_children(&self, node: &Node, builder: &mut DiagramBuilder) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.process_node(&child, builder);
        }
    }

    fn handle_generic(&self, node: &Node, builder: &mut DiagramBuilder) {
        let text = node.utf8_text(builder.source.as_bytes()).unwrap_or_default();
        if !text.trim().is_empty() {
            builder.add_block(BlockType::Action, text);
            builder.position.1 += 50;
        }
    }

    fn handle_function(node: &Node, builder: &mut DiagramBuilder) {
        let name = node.child(1).and_then(|n| n.utf8_text(builder.source.as_bytes()).ok())
            .unwrap_or("anonymous");

        builder.add_block(BlockType::Start, name);
        builder.enter_scope(BlockScope::Function, 0, 100);
    }

    fn handle_if(node: &Node, builder: &mut DiagramBuilder) {
        let depth = builder.block_stack.iter()
            .filter(|s| matches!(s, BlockScope::If(_)))
            .count() as i32 + 1;

        builder.add_block(BlockType::Condition, "if");
        builder.enter_scope(BlockScope::If(depth), 100 * depth, 100);
    }

    fn handle_else(node: &Node, builder: &mut DiagramBuilder) {
        if let Some(BlockScope::If(depth)) = builder.block_stack.last() {
            builder.add_block(BlockType::Else, "else");
            builder.position.0 -= 100 * depth;
            builder.position.1 += 100;
        }
    }

    fn handle_loop(node: &Node, builder: &mut DiagramBuilder) {
        let loop_type = match node.kind() {
            "for_expression" => "for",
            "while_expression" => "while",
            _ => "loop",
        };

        builder.add_block(BlockType::Cycle, loop_type);
        builder.enter_scope(BlockScope::Loop(1), 100, 100);
    }

    fn handle_return(node: &Node, builder: &mut DiagramBuilder) {
        builder.add_block(BlockType::End, "return");
        builder.position.1 += 100;
    }

    fn handle_macro(node: &Node, builder: &mut DiagramBuilder) {
        let text = node.utf8_text(builder.source.as_bytes()).unwrap_or_default();
        let block_type = if text.contains("print") {
            BlockType::Print
        } else {
            BlockType::Action
        };

        builder.add_block(block_type, text);
        builder.position.1 += 100;
    }

    fn handle_match(node: &Node, builder: &mut DiagramBuilder) {
        let arm_count = node.children(&mut node.walk())
            .filter(|n| n.kind() == "match_arm")
            .count();

        builder.add_block(BlockType::Condition, "match");
        builder.enter_scope(BlockScope::Match(300, arm_count), -300, 100);
    }

    fn handle_match_arm(node: &Node, builder: &mut DiagramBuilder) {
        builder.position.0 += 300;
        builder.add_block(BlockType::Action, "arm");
        builder.position.1 += 100;
    }
}

pub struct RustAnalyzer;

impl RustAnalyzer {
    pub fn analyze(source: String) -> Vec<CodeBlock> {
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_rust::language()).unwrap();

        let tree = parser.parse(&source, None).unwrap();
        let mut builder = DiagramBuilder::new(source);
        let processor = AstProcessor::new();

        processor.process_node(&tree.root_node(), &mut builder);
        builder.blocks
    }
}
