use std::collections::HashMap;
use tree_sitter::{Node, Parser};

#[cfg(debug_assertions)]
macro_rules! dev_log {
    ($($arg:tt)*) => {{
        println!($($arg)*);
    }};
}

#[cfg(not(debug_assertions))]
macro_rules! dev_log {
    ($($arg:tt)*) => {{}};
}

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
    source: Box<[u8]>, // весь исходный код
    blocks: Vec<LocalVecBlock>, // блок для следующего этапа
    block_vec: Vec<CodeBlock>, // служебный блок, содержит тип блока для следующего этапа
    x_offset: i32, // смещение по х
    y_offset: i32, // смещение по у
    skip_until_brace: bool, // и так все понятно
    is_return: bool, // кажется тоже вопросов быть не должно
    if_else_stack: Vec<IfElseContext>, // стек для отслеживания вложенности if else
    return_to_if: Vec<[i32; 2]>, // x y
    y_if_max: i32, // максимальная у координата в конструкции if
    skip_next_else: bool,
}

#[derive(Debug)]
struct IfElseContext {
    start_x: i32,
    start_y: i32,
    depth: i32,
    has_else: bool,
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
            skip_next_else: false,
            return_to_if: Vec::new()
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
    go_into: Vec<&'static str>,
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
            "if",
            "binary_expression",
            ];

        let go_into = vec![
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

        let skip_children_node = vec!["macro_invocation", "match_pattern", "if", "return_expression", "}"];

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
            go_into,
            skip_children_node,
        }
    }

    fn process(&self, node: &Node, ctx: &mut Context) {
        let node_kind = node.kind();

        if let Some(handler) = self.handlers.get(node_kind) {
            handler(node, ctx);
        } else {
            self.handle_default(node, ctx);
        }
    }

    fn handle_default(&self, node: &Node, ctx: &mut Context) {
        //dev_log!("handle_default");
        let source = ctx.source.clone();
        let text = node.utf8_text(source.as_ref()).unwrap_or_default();
        //dev_log!("{}", text);
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

            if processor.ignor_list.contains(&node.kind()) {
                return;
            }

            if !processor.go_into.contains(&node.kind())
                && !processor.skip_children_node.contains(&node.kind())
            {
                dev_log!("normal proces");
                dev_log!("node is: {}", node.kind());
                dev_log!("text: {}\n", node.utf8_text(&ctx.source).unwrap_or("").to_string());
                processor.process(&node, ctx);
            }
            if processor.skip_children_node.contains(&node.kind()) {
                dev_log!("skip children");
                dev_log!("node is: {}", node.kind());
                dev_log!("text: {}\n", node.utf8_text(&ctx.source).unwrap_or("").to_string());
                processor.process(&node, ctx);
                return;
            }

            dev_log!("go into");
            dev_log!("node is: {}", node.kind());
            dev_log!("text: {}\n", node.utf8_text(&ctx.source).unwrap_or("").to_string());
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
            dev_log!("!stack contents!\n{:#?}", ctx.block_vec);
            panic!("WRONE CODE")
        }

        dev_log!("Final block vector: {:#?}", ctx.blocks);
        ctx.blocks
    }
}

//handlers

fn else_handler(_node: &Node, ctx: &mut Context) {
    dev_log!("create else info block");
    ctx.add_empty_block();
    [ctx.x_offset, ctx.y_offset] = ctx.return_to_if.pop().unwrap();
    ctx.blocks.last_mut().unwrap().x = ctx.x_offset;
    ctx.blocks.last_mut().unwrap().y = ctx.y_offset;
    ctx.blocks.last_mut().unwrap().r#type = BlockType::Else;
    ctx.blocks.last_mut().unwrap().text = "continue".to_string();
    if ctx.skip_next_else{
        ctx.skip_next_else = false;
        return;
    }
    //dev_log!("else for break");
    ctx.block_vec.push(CodeBlock::Else(ctx.x_offset, ctx.y_offset));
}

fn handle_else_clause(node: &Node, ctx: &mut Context) {
    let text = handle_get_node_text(node, ctx);
    let words = text.lines().next().unwrap().trim();
    if words.contains("if") {
        //dev_log!("contains if");
        ctx.skip_next_else = true;
    }

}

fn get_firs_line(text: String) -> String {
    text.lines().next().unwrap().trim().to_string()
}

fn handle_if(node: &Node, ctx: &mut Context) {
    let source = ctx.source.clone();
    let text = node.utf8_text(&source.as_ref()).unwrap();
    let multiple = text.replace("\n", " ").split_whitespace().filter(|&word| word == "if").count() as i32;
    dev_log!("text == {text}");
    let mut text = handle_get_node_text(node, ctx);
    text = get_firs_line(text);

    dev_log!("go if");

    ctx.add_block(BlockType::Condition, &text);
    //let text = handle_get_node_text(node, ctx);
    dev_log!("push new if");
    let new_context = IfElseContext {
        start_x: ctx.x_offset,
        start_y: ctx.y_offset,
        depth: ctx.if_else_stack.len() as i32 + 1,
        has_else: false,
    };

    ctx.if_else_stack.push(new_context);
    ctx.block_vec.push(CodeBlock::If(ctx.x_offset, ctx.y_offset, 100));

    dev_log!("multiple == {multiple}");
    let shift_x = 100 * ctx.if_else_stack.len() as i32 * multiple;
    ctx.x_offset += shift_x;
    let for_return = ctx.x_offset - 2 * shift_x;
    ctx.y_offset += 100;
    ctx.return_to_if.push([for_return, ctx.y_offset]);
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

fn handle_closing_brecket(node: &Node, ctx: &mut Context) {
    let text = handle_get_node_text(node, ctx);
    //dev_log!("found closing brecket {text}");
    //dev_log!("{:?}", ctx.block_vec.last());
    //dev_log!("len of block mass = {}", ctx.block_vec.len());
    let mut r#type = BlockType::End;
    let mut push_text = String::new();
    let mut create_new_block = false;

    match ctx.block_vec.last_mut().unwrap() {
        CodeBlock::If(_x, _y, _offset) => {
            if let Some(context) = ctx.if_else_stack.pop() {
                ctx.y_if_max = ctx.y_offset.max(context.start_y);
                ctx.x_offset = context.start_x;
                ctx.y_offset = context.start_y + if context.has_else { 100 } else { 200 };
                //ctx.return_to_if.push([context.start_y, );
            }
            ctx.block_vec.pop();
        }
        CodeBlock::For(x, y) => {
            //dev_log!("Handling For block at {x}:{y}");
            push_text = format!("{x}:{y}");
            create_new_block = true;
        }
        CodeBlock::While(x, y) => {
            //dev_log!("Handling While block at {x}:{y}");
            push_text = format!("{x}:{y}");
            ctx.block_vec.pop();
            create_new_block = true;
        }
        CodeBlock::Loop(x, y) => {
            //dev_log!("Handling Loop block at {x}:{y}");
            push_text = format!("{x}:{y}");
            ctx.block_vec.pop();
            create_new_block = true;
        }
        CodeBlock::Func => {
            //dev_log!("Handling Func block");
            if ctx.is_return {
                ctx.block_vec.pop();
                ctx.blocks.pop();
                return;
            }
            ctx.add_block(BlockType::End, "");
            ctx.block_vec.pop();
        }
        CodeBlock::Match(back_x, _, count) => {
            if *count > 0 {
                *count -= 1;
                //dev_log!("skip pop");
                //ctx.blocks.last_mut().unwrap().r#type = BlockType::EndMatchArm;
            create_new_block = true;
                r#type = BlockType::EndMatchArm;
            } else {
                ctx.x_offset = *back_x;
                ctx.y_offset = ctx.y_if_max + 50;
                ctx.block_vec.pop();
                //ctx.blocks.last_mut().unwrap().text = String::from("match");
                push_text = "match".to_string();
                //ctx.blocks.last_mut().unwrap().r#type = BlockType::End;
            create_new_block = true;
            }
        }
        CodeBlock::Else(_x, _y) => {
            if let Some(context) = ctx.if_else_stack.pop() { // Явно извлекаем контекст
                ctx.x_offset = context.start_x;
                ctx.y_offset = ctx.y_if_max + 100;
            }
            ctx.block_vec.pop(); // Удаляем Else из block_vec
            create_new_block = true;
        }
    }
    if create_new_block{
        ctx.add_empty_block();
        ctx.blocks.last_mut().unwrap().r#type = r#type;
        ctx.blocks.last_mut().unwrap().text = push_text;
    }
}

fn handle_identifier(node: &Node, ctx: &mut Context) {
    //dev_log!("handle_identifier");
    let text = node.utf8_text(ctx.source.as_ref()).unwrap_or_default();

    if text == "main" {
        ctx.add_block(BlockType::Start, "Начало");
        ctx.block_vec.push(CodeBlock::Func);
        //dev_log!("push НАЧАЛО")
    } else {
        //dev_log!("push {text}")
    }

    //dev_log!("{:?}", ctx.blocks);
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
    ctx.blocks.last_mut().unwrap().r#type = BlockType::Cycle;
    ctx.skip_until_brace = true;
}

fn handle_loop_handler(_node: &Node, ctx: &mut Context) {
    ctx.add_empty_block();
    //dev_log!("push loop");
    ctx.block_vec
        .push(CodeBlock::Loop(ctx.x_offset, ctx.y_offset));
    ctx.blocks.last_mut().unwrap().r#type = BlockType::Cycle;
}

fn handle_for_expression(node: &Node, ctx: &mut Context) {
    ctx.add_empty_block();
    //dev_log!("push for");
    ctx.skip_until_brace = true;
    let text = handle_get_node_text(node, ctx);
    let mut first_line = text.lines().next().unwrap_or("").to_string();
    first_line.pop();
    ctx.blocks.last_mut().unwrap().text = first_line;
    ctx.blocks.last_mut().unwrap().r#type = BlockType::Cycle;
    ctx.block_vec
        .push(CodeBlock::For(ctx.x_offset, ctx.y_offset));
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
    dev_log!(
        "push ctx.blocks\n{:?}\n",
        ctx.blocks.last_mut().unwrap()
    );
    ctx.y_offset += 200;
}

fn handle_return_expression(node: &Node, ctx: &mut Context) {
    let text = handle_get_node_text(node, ctx);
    ctx.add_block(BlockType::End, &text);
    if *ctx.block_vec.last().unwrap() != CodeBlock::Func {
         ctx.is_return = true;
    }
    //dev_log!("push return");
    let text = handle_get_node_text(node, ctx);
    ctx.blocks.last_mut().unwrap().text = text;
    ctx.blocks.last_mut().unwrap().y -= 100;
    ctx.blocks.last_mut().unwrap().r#type = BlockType::End;
    dev_log!(
        "push ctx.blocks.last().\n{:?}\n",
        ctx.blocks.last_mut().unwrap()
    );
    ctx.y_offset += 100;
}

fn handle_match_expression(node: &Node, ctx: &mut Context) {
    let text = handle_get_node_text(node, ctx);
    if text.matches("match").count() > 1 {
        panic!("incorrect use of macth")
    }
    //dev_log!("push match");
    let arrow_count = text.matches("=>").count();
    let inter_block_count = arrow_count - text.matches(",").count();
    ctx.block_vec.push(CodeBlock::Match(
            ctx.x_offset,
            ctx.y_offset + 100,
            inter_block_count,
    ));
    ctx.x_offset -= (arrow_count * 150_usize) as i32;
    let mut first_line = text.lines().next().unwrap_or("").to_string();
    first_line.pop();
    ctx.blocks.last_mut().unwrap().text = first_line;
    ctx.blocks.last_mut().unwrap().r#type = BlockType::Condition;
    ctx.y_offset -= 100;
}

fn handler_match_pattern(_node: &Node, ctx: &mut Context) {
    ctx.add_empty_block();
    if let Some(CodeBlock::Match(_, to_y, _)) = ctx.block_vec.last_mut() {
        ctx.x_offset += 300;
        ctx.y_offset = *to_y;
    } else {
        for i in ctx.block_vec.iter_mut().rev() {
            if let CodeBlock::Match(_, to_y, _) = i {
                ctx.x_offset += 300;
                ctx.y_offset = *to_y;
                break;
            }
        }
    }
    ctx.blocks.last_mut().unwrap().x = ctx.x_offset;
    ctx.blocks.last_mut().unwrap().y = ctx.y_offset;
    //dev_log!("push to blocks match");
    //dev_log!("push local_block\n{:?}\n", ctx.blocks.last_mut().unwrap());
    ctx.y_offset += 100;
}
