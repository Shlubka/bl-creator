use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;

use crate::lang_vec_stuf::{BlockType, LocalVecBlock};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct FullJson {
    blocks: Vec<JsBlock>,
    arrows: Vec<Arrow>,
    x0: i32,
    y0: i32,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct JsBlock {
    x: i32,
    y: i32,
    text: String,
    width: i32,
    height: i32,
    r#type: String,
    is_menu_block: bool,
    font_size: i32,
    text_height: i32,
    is_bold: bool,
    is_italic: bool,
    text_align: String,
    labels_position: i32,
}

impl JsBlock {
    fn new(x: i32, y: i32) -> Self {
        JsBlock {
            x,
            y,
            text: String::new(),
            width: 120,
            height: 60,
            r#type: String::from("Блок"),
            is_menu_block: false,
            font_size: 14,
            text_height: 14,
            is_bold: false,
            is_italic: false,
            text_align: String::new(),
            labels_position: 1,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Node {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Arrow {
    start_index: usize,
    end_index: usize,
    start_connector_index: usize,
    end_connector_index: usize,
    nodes: Vec<Node>,
    counts: Vec<usize>,
}

impl Arrow {
    fn new(iterator: usize) -> Self {
        Arrow {
            start_index: iterator,
            end_index: iterator + 1,
            start_connector_index: 2,
            end_connector_index: 0,
            nodes: Vec::<Node>::new(),
            counts: vec![1, 1, 1],
        }
    }
}

pub fn create_json_blocks(mut analyzed_vector: Vec<LocalVecBlock>) -> String {
    let mut cycle_acum = Vec::<usize>::new(); //indexing cycle index
    let mut x_min_max_acum = [0, 0]; //max min x for correct arrow adding for cycle
    let mut is_cycle = 0;
    //let mut is_match = false;
    let mut else_arrow_iter = 0;
    let mut last_condition_index = Vec::new();
    let mut match_arm = Vec::<usize>::new();
    let mut look_for_cond_xy = Vec::new();
    //let mut nesting_if_else_iter = Vec::new();

    let mut local_full_blocks = FullJson {
        blocks: Vec::<JsBlock>::new(),
        arrows: Vec::<Arrow>::new(),
        x0: 0,
        y0: 0,
    };

    let mut iterator = 0;
    let mut sluzba = 0;
    for i in &mut analyzed_vector {
        println!("thish block ready for processing {i:?}");
        let local_text = text_analyzer(i);
        let mut local_block = JsBlock::new(i.x, i.y);
        let mut local_arrow = Arrow::new(iterator.clone());
        println!("coords: x={}; y={}", i.x, i.y);

        match i.r#type {
            BlockType::Start => {
                //nesting_if_else_iter.clear();
                block_start(
                    &mut x_min_max_acum,
                    &mut local_full_blocks,
                    &mut local_block,
                    i,
                    iterator,
                );
            }
            BlockType::Condition => {
                block_condition(
                    &mut x_min_max_acum,
                    &mut local_block,
                    &mut local_arrow,
                    i,
                    &mut look_for_cond_xy,
                    &mut last_condition_index,
                    iterator,
                    //is_match: bool
                    &mut sluzba,
                );
            }
            BlockType::Action => {
                check_x(is_cycle, i.x, &mut x_min_max_acum);
                //println!("found {} in vec {} {}", i.text, i.y, i.x);
                local_block.text = if local_text.is_empty() {
                    i.text.clone()
                } else {
                    local_text
                };
            }
            BlockType::EndMatchArm => {
                match_arm.push(iterator - 1);
                println!("push match arm");
                continue;
            }
            BlockType::End => {
                if i.text == "end if" {
                    println!("pop look_for");
                    let (_, _, to_iterator) =
                        look_for_cond_xy.pop().expect("look_for_cond_xy is empty");
                    //.unwrap();
                    local_arrow.start_index = to_iterator;
                    local_arrow.end_index = iterator;
                    local_arrow.start_connector_index = 3;
                    local_arrow.end_connector_index = 0;
                    local_arrow.nodes = Vec::new();
                    local_arrow.counts = vec![1, 1, 1, 1, 1];
                    local_full_blocks.arrows.push(local_arrow);
                    continue;
                }
                if i.text == "end else" {
                    local_arrow.start_index = else_arrow_iter;
                    else_arrow_iter = 0;
                    local_arrow.end_index = iterator;
                    local_arrow.start_connector_index = 2;
                    local_arrow.end_connector_index = 0;
                    local_arrow.nodes = Vec::new();
                    local_arrow.counts = vec![1, 1, 1, 1, 1];
                    local_full_blocks.arrows.push(local_arrow);
                    continue;
                }
                if i.text.contains("match") {
                    //drop(local_arrow);
                    for i in match_arm.iter() {
                        let mut local_arrow_local = local_arrow.clone();
                        local_arrow_local.start_index = *i;
                        local_arrow_local.end_index = iterator;
                        local_arrow_local.start_connector_index = 2;
                        local_arrow_local.end_connector_index = 0;
                        local_arrow_local.nodes = Vec::new();
                        local_arrow_local.counts = vec![1, 1, 1, 1, 1];
                        local_full_blocks.arrows.push(local_arrow_local);
                    }
                    continue;
                }
                // Обработка блока "Конец"
                if i.text.contains(":") {
                    // Получение индекса цикла из стека
                    let cycle_index = if let Some(index) = cycle_acum.last() {
                        println!("index: {index}");
                        index
                    } else {
                        println!("index stack is empty");
                        // Возвращаем значение по умолчанию, если стек пуст
                        &(0_usize)
                    };

                    // Разделение текста на части по символу ':'
                    let parts: Vec<&str> = i.text.split(':').collect();
                    assert!(parts.len() == 2, "Invalid coordinate format");

                    // Парсинг координат
                    let to_y = parts[1].parse::<i32>().expect("Invalid y coordinate");
                    let to_x = parts[0].parse::<i32>().expect("Invalid y coordinate");
                    // Обновление состояния цикла и координат
                    is_cycle -= 1;
                    x_min_max_acum[0] -= 10;
                    x_min_max_acum[1] += 10;

                    local_block.r#type = String::from("Блок");
                    local_block.text = String::from("iter++");
                    add_arrow_to_cycle(
                        &mut local_full_blocks,
                        iterator,
                        i,
                        &mut x_min_max_acum,
                        to_y,
                        to_x,
                        cycle_index,
                    );

                    // Обновление стрелки
                    add_arrow_from_cycle(
                        &mut local_arrow,
                        i,
                        cycle_acum.pop().unwrap(),
                        iterator,
                        &mut x_min_max_acum,
                        to_y,
                        to_x,
                    );
                } else {
                    local_block.r#type = String::from("Начало / конец");
                    local_block.text = if i.text.is_empty() || i.text == "}" {
                        //local_block.y -= 100;
                        "Конец".to_string()
                    } else {
                        i.text.clone()
                    };
                }
            }
            BlockType::Print => {
                println!("text in json block {}, iterator {iterator}", i.text);
                //rem_unused(&mut local_full_blocks, iterator, &sluzba);
                check_x(is_cycle, i.x, &mut x_min_max_acum);
                local_block.r#type = String::from("Ввод / вывод");
                if i.text.is_empty() {
                    local_block.text = String::from("Вывод строки");
                } else {
                    local_block.text = String::from(i.text.as_str());
                }
            }
            BlockType::Cycle => {
                is_cycle += 1;
                cycle_acum.push(iterator);
                local_block.r#type = String::from("Цикл for");
                local_block.text.clone_from(&i.text.to_string())
            }
            BlockType::Else => {
                local_full_blocks
                    .arrows
                    .remove(local_full_blocks.arrows.len() - 2); //чисто на рандом тыкнул, мб когда нибуль аукнится
                                                                 // ну я хз, это костыль, но работает
                if i.text == "continue" {
                    else_arrow_iter = iterator - 1;
                    continue;
                }
                println!("pop iterator {:?}", last_condition_index);
                add_else_arrow(
                    local_arrow,
                    &mut local_full_blocks,
                    iterator,
                    last_condition_index.pop().unwrap(),
                );
                //x_min_max_acum[0] -= 100;
                continue;
            }
        }
        local_full_blocks.blocks.push(local_block);
        local_full_blocks.arrows.push(local_arrow);
        iterator += 1;
    }
    local_full_blocks.arrows.pop();
    //local_full_blocks.arrows.clear();
    to_string_pretty(&local_full_blocks).unwrap()
}

fn check_x(is_cycle: i32, current_x: i32, x_min_max_acum: &mut [i32; 2]) {
    print!("enter in check funk\n");
    if is_cycle > 0 {
        println!("{is_cycle}");
        if current_x < x_min_max_acum[0] {
            x_min_max_acum[0] = current_x - 60;
            println!("not more");
        }
        if current_x > x_min_max_acum[1] {
            x_min_max_acum[1] = current_x + 60;
            println!("more");
        }
    }
    println!("after check {x_min_max_acum:?}");
}

//arrows hendlers
fn add_arrow_from_cycle(
    local_arrow: &mut Arrow,
    current: &LocalVecBlock,
    cycle_acum: usize,
    iterator: usize,
    x_min_max_acum: &mut [i32; 2],
    to_y: i32,
    to_x: i32,
) {
    println!("arrow after cycle");
    println!("");
    println!(
        "metirial:\nx_m_m_a == {x_min_max_acum:?}\nall current: x == {}, y == {}\nto: x == {to_x}; y == {to_y}",
        current.x, current.y
    );
    println!();
    // из цикла в блок после }
    // ни при каких обстоятельствах не трогать!!!!!!!!!!!!!!!!!
    let value = vec![
        Node {
            //x: x_min_max_acum[1],
            x: to_x + 60,
            y: to_y,
        },
        Node {
            x: x_min_max_acum[1] + 10,
            //x: 170,
            y: to_y,
        },
        Node {
            x: x_min_max_acum[1] + 10,
            y: current.y + 50,
        },
        Node {
            x: current.x,
            y: current.y + 50,
        },
        Node {
            x: current.x,
            y: current.y + 70,
        },
    ];
    local_arrow.start_index = cycle_acum;
    local_arrow.end_index = iterator + 1;
    local_arrow.start_connector_index = 1;
    local_arrow.end_connector_index = 0;
    local_arrow.counts = vec![1, 1, 1, 1, 1];
    local_arrow.nodes = Vec::from(value);
    x_min_max_acum[1] += 10;
}
fn add_arrow_to_cycle(
    local_full_blocks: &mut FullJson,
    iterator: usize,
    current: &LocalVecBlock,
    x_min_max_acum: &mut [i32; 2],
    to_y: i32,
    to_x: i32,
    cycle_index: &usize,
) {
    println!(
        "\nadd arrow to cycle\nmetirial:\nx_m_m_a == {x_min_max_acum:?}\nall current: x == {}, y == {}\nto: x == {to_x}; y == {to_y}",
        current.x, current.y
    );
    // из крайнего блока цикла в цикл
    // ни при каких обстоятельствах не трогать!!!!!!!!!!!!!!!!!
    let value = vec![
        Node {
            // связь стрелка блок
            x: current.x - 60,
            y: current.y,
        },
        Node {
            // угол
            x: current.x - 90,
            y: current.y,
        },
        Node {
            x: x_min_max_acum[0] - 20,
            //x: x_min_max_acum[0],
            //y: current.y,
            y: to_y,
        },
        Node {
            x: to_x - 80,
            y: to_y,
        },
        Node {
            x: to_x - 60,
            y: to_y,
        },
    ];
    x_min_max_acum[0] -= 10;
    let local_arrow_local = Arrow {
        start_index: iterator,
        end_index: *cycle_index,
        start_connector_index: 3,
        end_connector_index: 3,
        nodes: Vec::from(value),
        counts: vec![1, 1, 1, 1, 1],
    };
    local_full_blocks.arrows.push(local_arrow_local);
}
fn _add_standart_arrow() {}
fn add_else_arrow(
    mut local_arrow: Arrow,
    local_full_blocks: &mut FullJson,
    iterator: usize,
    last_index: usize,
) {
    local_arrow.start_index = last_index;
    local_arrow.start_connector_index = 3;
    local_arrow.end_index = iterator;
    local_arrow.end_connector_index = 0;
    local_full_blocks.arrows.push(local_arrow);
}

//blocks hendlers
fn block_start(
    x_min_max_acum: &mut [i32; 2],
    local_full_blocks: &mut FullJson,
    local_block: &mut JsBlock,
    i: &mut LocalVecBlock,
    iterator: usize,
) {
    *x_min_max_acum = [0, 0];
    //println!("found start {} in vec {} {}", i.text, i.x, i.y);
    local_block.text.clone_from(&i.text);
    local_block.r#type = String::from("Начало / конец");
    local_full_blocks.arrows.pop();
    //Костыль!! надо найти место, где эта стрелка вылазит
    if let Some(last_arrow) = local_full_blocks.arrows.last() {
        if last_arrow.end_index == iterator {
            local_full_blocks.arrows.pop();
        }
    }
}

fn block_condition(
    x_min_max_acum: &mut [i32; 2],
    local_block: &mut JsBlock,
    local_arrow: &mut Arrow,
    i: &mut LocalVecBlock,
    look_for_cond_xy: &mut Vec<(i32, i32, usize)>,
    last_condition_index: &mut Vec<usize>,
    iterator: usize,
    //is_match: bool
    sluzba: &mut usize,
) {
    /*local_full_blocks.arrows.retain(|arrow| {
        !(arrow.end_index == iterator
            && !arrow.nodes.is_empty()
            && i.y < arrow.nodes[0].y)
    });*/
    *sluzba = iterator;
    x_min_max_acum[0] -= 10;
    last_condition_index.push(iterator);
    println!("push iterator {last_condition_index:?}");
    //println!("found Condition {} in vec {} {}", i.text, i.x, i.y);
    local_block.text.clone_from(&i.text);
    local_block.r#type = String::from("Условие");
    if i.text.contains("match") {
        local_arrow.start_connector_index = 2;
        //is_match = true;
    } else {
        local_arrow.start_connector_index = 1;
        println!("push look_for {}", i.text);
        look_for_cond_xy.push((i.x, i.y, iterator));
    }
}

fn text_analyzer(i: &mut LocalVecBlock) -> String {
    i.text = i.text.replace("\t", "");
    let mut local_text = String::new();
    if i.text.len() > 1 {
        let mid = i.text.chars().count() / 2;
        let first_half: String = i.text.chars().take(mid).collect();
        let second_half: String = i.text.chars().skip(mid).collect();
        local_text = format!("{}\n{}", first_half, second_half);
    } else {
        local_text = i.text.clone();
    }
    local_text
}

//для чистки стрелок между веток в if else
fn rem_unused(local_full_blocks: &mut FullJson, iterator: usize, sluzba: &usize) {
    let start_index = *sluzba;
    local_full_blocks.arrows.retain(|arrow| {
        if arrow.end_index == iterator {
            if arrow.start_index == start_index {
                true // Оставить элемент, если start_index равен заданному числу
            } else {
                false // Удалить элемент
            }
        } else {
            true // Оставить элемент, если end_index не равен iterator
        }
    });
}
