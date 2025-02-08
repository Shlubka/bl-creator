use std::collections::HashMap;
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

//вместо того, чтобы таскать кучу переменных, их можно собрать в контекст и таксать только контекст)
#[derive(Debug)]
struct Context {
    source: Box<[u8]>,
    blocks: Vec<LocalVecBlock>,
    block_vec: Vec<CodeBlock>,
    x_offset: i32,
    y_offset: i32,
    skip_until_brace: bool,
    is_return: bool,
    if_else_stack: Vec<(i32, i32)>,
    y_if_max: i32,
}

impl Context {
    fn new(source: Box<[u8]>) -> Self {
        Self {
            source,
            blocks: Vec::new(),
            block_vec: Vec::new(),
            x_offset: 0,
            y_offset: 0,
            skip_until_brace: false,
            is_return: false,
            if_else_stack: Vec::new(),
            y_if_max: 0,
        }
    }

    fn add_empty_block(&mut self) {
        let block = LocalVecBlock {
            r#type: BlockType::Action,
            text: String::new(),
            x: self.x_offset,
            y: self.y_offset,
        };
        self.blocks.push(block);
    }
    fn add_block(&mut self, block_type: BlockType, text: &str) {
        let block = LocalVecBlock {
            r#type: block_type,
            text: text.to_string(),
            x: self.x_offset,
            y: self.y_offset,
        };
        self.blocks.push(block);
    }

    fn push_code_block(&mut self, code_block: CodeBlock) {
        self.block_vec.push(code_block);
    }

    fn skip_children(&self) -> bool {
        self.skip_until_brace
    }
}

//Пока припиздил из нейросети, надо только понять, как это работает
type NodeHandler = fn(&Node, &mut Context);

struct NodeProcessor {
    handlers: HashMap<String, NodeHandler>,
    ignor_list: Vec<&'static str>,
    not_go_into: Vec<&'static str>,
    skip_children_node: Vec<&'static str>,
}

impl NodeProcessor {
    fn for_rust() -> Self {
        let mut handlers = HashMap::new();
        let ignor_list = vec![
            "impl_item",
            "enum_item",
            "reference_type",
            "struct_item",
            "let_declaration",
            "parameters",
            "->",
            "primitive_type",
            ";",
            "block_comment",
            "line_comment",
            "match",
            "=>",
            "use_declaration",
            "generic_type",
            "type_item",
            "attribute_item",
            "struct_item",
            "{",
            "line_comment",
            "struct_item",
        ];

        let not_go_into = vec![
            "loop_expression",
            "function_item",
            "block",
            "match_block",
            "expression_statement",
            "source_file",
            "match_arm",
            "fn",
            "block",
            "match_expression",
            "string_literal",
        ];

        let skip_children_node = vec!["macro_invocation", "match_pattern", "if"];

        // Регистрируем основные обработчики
        handlers.insert(
            String::from("match_pattern"),
            handler_match_pattern as NodeHandler,
        );
        handlers.insert(
            String::from("macro_invocation"),
            handle_macro_invocation as NodeHandler,
        );
        handlers.insert(String::from("loop"), handle_loop_handler as NodeHandler);
        handlers.insert(String::from("if_expression"), handle_if as NodeHandler);
        handlers.insert(
            String::from("else_clause"),
            handle_else_clause as NodeHandler,
        );
        handlers.insert(
            String::from("for_expression"),
            handle_for_expression as NodeHandler,
        );
        handlers.insert(
            String::from("while_expression"),
            handle_while_expression as NodeHandler,
        );
        handlers.insert(
            String::from("return_expression"),
            handle_return_expression as NodeHandler,
        );
        handlers.insert(String::from("else"), else_handler as NodeHandler);
        handlers.insert(String::from("identifier"), handle_identifier as NodeHandler);
        handlers.insert(String::from("}"), handle_closing_brecket as NodeHandler);
        handlers.insert(
            String::from("match_expression"),
            handle_match_expression as NodeHandler,
        );

        Self {
            handlers,
            ignor_list,
            not_go_into,
            skip_children_node,
        }
    }

    fn process(&self, node: &Node, ctx: &mut Context) {
        let node_kind = node.kind();

        //println!("process node: {}\n", node.kind());
        if let Some(handler) = self.handlers.get(node_kind) {
            handler(node, ctx);
        } else {
            self.handle_default(node, ctx);
        }
    }

    fn handle_default(&self, node: &Node, ctx: &mut Context) {
        let source = ctx.source.clone();
        let text = node.utf8_text(source.as_ref()).unwrap_or_default();
        ctx.add_block(BlockType::Action, text);
        ctx.y_offset += 100;
    }
}

#[derive(Default)]
pub struct Rust;

impl Language for Rust {
    fn get_name(&self) -> &'static str {
        "Rust"
    }
    fn analyze_to_vec(&self, source_code: String) -> Vec<LocalVecBlock> {
        fn traverse_ast(node: Node, processor: &NodeProcessor, ctx: &mut Context) {
            let text = node.utf8_text(&ctx.source).unwrap_or("").to_string();

            //processor.process(&node, ctx);

            if processor.ignor_list.contains(&node.kind()) {
                //println!("ignore ");
                //println!("node is: {}", node.kind());
                //println!("text: {text}\n");
                return;
            }

            if !processor.not_go_into.contains(&node.kind())
                && !processor.skip_children_node.contains(&node.kind())
            {
                println!("normal proces");
                println!("node is: {}", node.kind());
                println!("text: {text}\n");
                processor.process(&node, ctx);
            }
            if processor.skip_children_node.contains(&node.kind()) {
                println!("skip children");
                println!("node is: {}", node.kind());
                println!("text: {text}\n");
                processor.process(&node, ctx);
                return;
            }

            println!("go into ");
            println!("node is: {}", node.kind());
            println!("text: {text}\n");
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                traverse_ast(child, processor, ctx);
            }
        }

        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_rust::language())
            .expect("Error loading Rust grammar");
        let tree = parser.parse(source_code.clone(), None).unwrap();

        let root_node = tree.root_node();

        let mut ctx: Context = Context::new(source_code.into_bytes().into_boxed_slice());
        let processor: NodeProcessor = NodeProcessor::for_rust();

        traverse_ast(root_node, &processor, &mut ctx);
        if !ctx.block_vec.is_empty() {
            println!("!stack contents!\n{:#?}", ctx.block_vec);
            panic!("WRONE CODE")
        }
        println!("Final block vector: {:#?}", ctx.blocks);
        ctx.blocks
    }
}

//handlers

fn else_handler(_node: &Node, ctx: &mut Context) {
    println!("create else info block");
    ctx.blocks.last_mut().unwrap().x = ctx.x_offset;
    ctx.blocks.last_mut().unwrap().r#type = BlockType::Else;
    ctx.blocks.last_mut().unwrap().text = "continue".to_string();
    ctx.y_offset -= 100;
}

fn handle_else_clause(node: &Node, ctx: &mut Context) {
    let text = handle_get_node_text(node, ctx);
    println!("text in else_clause: {text}");
    println!(
        "pop from if_else_stack. it contains {:?}",
        ctx.if_else_stack
    );
    let return_to = ctx.if_else_stack.pop().unwrap();
    ctx.x_offset = return_to.0;
    ctx.y_offset = return_to.1;
    println!("after pop x: {}; y: {}", ctx.x_offset, ctx.y_offset);
    println!("mr penis");
    ctx.blocks.last_mut().unwrap().text = String::from("mr penis");
    println!("return to in vec x:{} y:{}", return_to.0, return_to.1);
    println!("local coords x{} y:{}", ctx.x_offset, ctx.y_offset);
    ctx.block_vec
        .push(CodeBlock::Else(ctx.x_offset, ctx.y_offset));
    ctx.blocks.last_mut().unwrap().r#type = BlockType::Else;
    ctx.blocks.last_mut().unwrap().x = ctx.x_offset;
    ctx.blocks.last_mut().unwrap().y = ctx.y_offset - 200;
}

fn handle_if(node: &Node, ctx: &mut Context) {
    let text = handle_get_node_text(node, ctx);
    ctx.add_block(BlockType::Condition, &text);
    ctx.block_vec
        .push(CodeBlock::If(ctx.x_offset, ctx.y_offset, 100));
    ctx.x_offset += 100;
}

fn handle_get_node_text(node: &Node, ctx: &Context) -> String {
    node.utf8_text(ctx.source.as_ref())
        .unwrap_or_default()
        .lines()
        .next()
        .unwrap_or_default()
        .trim_end_matches('{')
        .to_string()
}

fn handle_closing_brecket(_node: &Node, ctx: &mut Context) {
    ctx.add_empty_block();
    println!("{:?}", ctx.block_vec.last());
    println!("len of block mass = {}", ctx.block_vec.len());

    match ctx.block_vec.last_mut().unwrap() {
        CodeBlock::Return => {
            println!("add return");
            ctx.blocks.last_mut().unwrap().text = "Конец".to_string();
            ctx.blocks.last_mut().unwrap().r#type = BlockType::End;
            //ctx.blocks.last_mut().unwrap()..x -= 200;
            ctx.block_vec.pop();
        }
        CodeBlock::If(x, y, offset) => {
            println!("Handling If block at {x}:{y}");
            ctx.blocks.last_mut().unwrap().r#type = BlockType::End;
            //ctx.blocks.last_mut().unwrap()..text = format!("{x}:{y}");
            ctx.blocks.last_mut().unwrap().text = "end if".to_string();
            //let (x, y, offset) = block_vec.pop().unwrap();
            ctx.y_if_max = ctx.y_offset;
            //ctx.x_offset -= 100;
            ctx.y_offset -= 100;
            ctx.x_offset -= *offset;
            ctx.block_vec.pop().unwrap();
        }
        CodeBlock::For(x, y) => {
            println!("Handling For block at {x}:{y}");
            ctx.blocks.last_mut().unwrap().r#type = BlockType::End;
            ctx.blocks.last_mut().unwrap().text = format!("{x}:{y}");
            ctx.block_vec.pop();
        }
        CodeBlock::While(x, y) => {
            println!("Handling While block at {x}:{y}");
            ctx.blocks.last_mut().unwrap().r#type = BlockType::End;
            ctx.blocks.last_mut().unwrap().text = format!("{x}:{y}");
            ctx.block_vec.pop();
        }
        CodeBlock::Loop(x, y) => {
            println!("Handling Loop block at {x}:{y}");
            ctx.blocks.last_mut().unwrap().r#type = BlockType::End;
            ctx.blocks.last_mut().unwrap().text = format!("{x}:{y}");
            ctx.block_vec.pop();
        }
        CodeBlock::Func => {
            println!("Handling Func block");
            ctx.blocks.last_mut().unwrap().r#type = BlockType::End;
            ctx.block_vec.pop();
            if ctx.is_return {
                ctx.is_return = false;
                ctx.blocks.last_mut().unwrap().text = "drop".to_string();
                println!("skip brecket");
                //drop(*ctx.blocks.last_mut().unwrap().);
                return;
            }
        }
        CodeBlock::Continue => {
            ctx.block_vec.pop();
            return;
        }
        CodeBlock::Match(back_x, _, count) => {
            if *count > 0 {
                *count -= 1;
                println!("skip pop");
                ctx.blocks.last_mut().unwrap().r#type = BlockType::EndMatchArm;
            } else {
                ctx.x_offset = *back_x;
                ctx.y_offset = ctx.y_if_max + 50;
                ctx.block_vec.pop();
                ctx.blocks.last_mut().unwrap().text = String::from("match");
                ctx.blocks.last_mut().unwrap().r#type = BlockType::End;
            }
        }
        CodeBlock::Else(_, _) => {
            println!("pop else");
            ctx.blocks.last_mut().unwrap().r#type = BlockType::End;
            ctx.blocks.last_mut().unwrap().text = "end else".to_string();
            //ctx.blocks.last_mut().unwrap()..r#type = BlockType::End;
            ctx.block_vec.pop();
            //if_else_stack.pop();
            if ctx.y_if_max > ctx.y_offset {
                ctx.y_offset = ctx.y_if_max
            }
            ctx.x_offset += 100;
        }
    }
}

fn handle_identifier(node: &Node, ctx: &mut Context) {
    let text = node.utf8_text(ctx.source.as_ref()).unwrap_or_default();

    if text == "main" {
        ctx.add_block(BlockType::Start, "Начало");
        ctx.block_vec.push(CodeBlock::Func);
        println!("push НАЧАЛО")
    } else {
        println!("push {text}")
    }

    println!("{:?}", ctx.blocks);
    ctx.y_offset += 100;
}

fn handle_while_expression(node: &Node, ctx: &mut Context) {
    ctx.add_empty_block();
    let text = handle_get_node_text(node, ctx);
    let mut first_line = text.lines().next().unwrap_or("").to_string();
    first_line.pop();
    ctx.blocks.last_mut().unwrap().text = first_line;
    ctx.block_vec
        .push(CodeBlock::While(ctx.x_offset, ctx.y_offset));
    //пока хз как себя поведет
    if ctx.y_offset > ctx.y_if_max {
        ctx.y_if_max = ctx.y_offset;
    }
    ctx.blocks.last_mut().unwrap().r#type = BlockType::Cycle;
    ctx.skip_until_brace = true;
}

fn handle_loop_handler(_node: &Node, ctx: &mut Context) {
    ctx.add_empty_block();
    println!("push loop");
    ctx.block_vec
        .push(CodeBlock::Loop(ctx.x_offset, ctx.y_offset));
    //пока хз как себя поведет
    if ctx.y_offset > ctx.y_if_max {
        ctx.y_if_max = ctx.y_offset;
    }
    ctx.blocks.last_mut().unwrap().r#type = BlockType::Cycle;
}

fn handle_for_expression(node: &Node, ctx: &mut Context) {
    ctx.add_empty_block();
    println!("push for");
    ctx.skip_until_brace = true;
    let text = handle_get_node_text(node, ctx);
    let mut first_line = text.lines().next().unwrap_or("").to_string();
    first_line.pop();
    ctx.blocks.last_mut().unwrap().text = first_line;
    ctx.blocks.last_mut().unwrap().r#type = BlockType::Cycle;
    //пока хз как себя поведет
    if ctx.y_offset > ctx.y_if_max {
        ctx.y_if_max = ctx.y_offset;
    }
    ctx.block_vec
        .push(CodeBlock::For(ctx.x_offset, ctx.y_offset));
    //return;
}

fn handle_macro_invocation(node: &Node, ctx: &mut Context) {
    let text = handle_get_node_text(node, ctx);
    ctx.add_block(BlockType::Action, "");
    if text.contains("print") {
        if text.contains("}") {
            ctx.blocks.last_mut().unwrap().text = String::from("Вывод переменной");
        } else {
            ctx.blocks.last_mut().unwrap().text = String::from("Вывод строки");
        }
        ctx.blocks.last_mut().unwrap().r#type = BlockType::Print;
    }
    println!(
        "push ctx.blocks.last_mut().unwrap().\n{:?}\n",
        ctx.blocks.last_mut().unwrap()
    );
    //ctx.blocks.push(ctx.blocks.last_mut().unwrap());
    //пока хз как себя поведет
    if ctx.y_offset > ctx.y_if_max {
        ctx.y_if_max = ctx.y_offset;
    }
    ctx.y_offset += 100;
}

fn handle_return_expression(node: &Node, ctx: &mut Context) {
    ctx.is_return = true;
    println!("push return");
    /*if text.trim_start() == "return;" {
        ctx.blocks.last_mut().unwrap()..text = "Конец".to_string();
    } else {
    }*/
    let text = handle_get_node_text(node, ctx);
    ctx.blocks.last_mut().unwrap().text = text;
    ctx.blocks.last_mut().unwrap().r#type = BlockType::End;
    //пока хз как себя поведет
    if ctx.y_offset > ctx.y_if_max {
        ctx.y_if_max = ctx.y_offset;
    }
    println!(
        "push ctx.blocks.last_mut().unwrap().\n{:?}\n",
        ctx.blocks.last_mut().unwrap()
    );
    //ctx.blocks.push(ctx.blocks.last_mut().unwrap());
    ctx.y_offset += 100;
}

fn handle_match_expression(node: &Node, ctx: &mut Context) {
    let text = handle_get_node_text(node, ctx);
    if text.matches("match").count() > 1 {
        panic!("incorrect use of macth")
    }
    println!("push match");
    let arrow_count = text.matches("=>").count();
    /*let mut arrow_count = text.matches(&case_keyword).count();
    if text.contains(&default_patetrn) {
        arrow_count += 1;
    }*/
    let inter_block_count = arrow_count - text.matches(",").count();
    ctx.block_vec.push(CodeBlock::Match(
        ctx.x_offset,
        ctx.y_offset + 100,
        inter_block_count,
    ));
    ctx.x_offset -= (arrow_count * 150 as usize) as i32;
    //block_vec.push(CodeBlock::Match(ctx.x_offset, ctx.y_offset));
    let mut first_line = text.lines().next().unwrap_or("").to_string();
    first_line.pop();
    ctx.blocks.last_mut().unwrap().text = first_line;
    ctx.blocks.last_mut().unwrap().r#type = BlockType::Condition;
    //blocks.push(ctx.blocks.last_mut().unwrap().);
    ctx.y_offset -= 100;
}

fn handler_match_pattern(node: &Node, ctx: &mut Context) {
    ctx.add_empty_block();
    //возможно насрал, посмотрим по поведению
    if let Some(CodeBlock::Match(_, to_y, _)) = ctx.block_vec.last_mut() {
        //*count -= 1;
        ctx.x_offset += 300;
        ctx.y_offset = *to_y;
    } else {
        for i in ctx.block_vec.iter_mut().rev() {
            if let CodeBlock::Match(_, to_y, _) = i {
                //*count -= 1;
                ctx.x_offset += 300;
                ctx.y_offset = *to_y;
                break;
            }
        }
    }
    ctx.blocks.last_mut().unwrap().x = ctx.x_offset;
    ctx.blocks.last_mut().unwrap().y = ctx.y_offset;
    println!("push to blocks match");
    println!("push local_block\n{:?}\n", ctx.blocks.last_mut().unwrap());
    //blocks.push(local_block);
    ctx.y_offset += 100;
}
