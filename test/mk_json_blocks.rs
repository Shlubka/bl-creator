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

pub fn create_json_blocks(analyzed_vector: Vec<LocalVecBlock>) -> String {
    let mut cycle_acum = Vec::<usize>::new(); //indexing cycle index
    let mut x_min_max_acum = [0, 0]; //max min x for correct arrow adding for cycle
    let mut is_cycle = 0;
    let mut last_condition_index = Vec::new();
    let mut match_arm = Vec::<usize>::new();
    let mut look_for_cond_xy = Vec::new();

    let mut local_full_blocks = FullJson {
        blocks: Vec::<JsBlock>::new(),
        arrows: Vec::<Arrow>::new(),
        x0: 0,
        y0: 0,
    };

    let mut iterator = 0;

    for i in &analyzed_vector {
        let mut local_block = JsBlock::new(i.x, i.y);
        let mut local_arrow = Arrow::new(iterator.clone());
        println!("coords: x={}; y={}", i.x, i.y);
        if is_cycle > 0 {
            if i.x < x_min_max_acum[0] {
                x_min_max_acum[0] = i.x;
                println!("more");
            }
            if i.x > x_min_max_acum[1] {
                x_min_max_acum[1] = i.x;
                println!("not more");
            }
        } else {
            x_min_max_acum = [100, -100];
        }

        let mut local_text = String::new();
        if i.text.len() > 16 {
            //println!("enter");
            let mut chars = i.text.chars();
            let mid = chars.clone().count() / 2;
            let first_half: String = chars.by_ref().take(mid).collect();
            let second_half: String = chars.collect();
            local_text = format!("{first_half}\n{second_half}");
        }

        match i.r#type {
            BlockType::Start => {
                //println!("found start {} in vec {} {}", i.text, i.x, i.y);
                local_block.text.clone_from(&i.text);
                local_block.r#type = String::from("Начало / конец");
                local_full_blocks.arrows.pop();
            }
            BlockType::Condition => {
                last_condition_index.push(iterator);
                //println!("found Condition {} in vec {} {}", i.text, i.x, i.y);
                local_block.text.clone_from(&i.text);
                local_block.r#type = String::from("Условие");
                if i.text.contains("match") {
                    local_arrow.start_connector_index = 2;
                } else {
                    local_arrow.start_connector_index = 1;
                    look_for_cond_xy.push((i.x, i.y, iterator));
                }
            }
            BlockType::Action => {
                //println!("found {} in vec {} {}", i.text, i.y, i.x);
                local_block.text = if local_text.is_empty() {
                    i.text.clone()
                } else {
                    local_text
                };

                //костыль, надо переделать
                /*if i.text.contains("else") || i.text.contains("}") {
                    continue;
                }*/
            }
            BlockType::EndMatchArm => {
                match_arm.push(iterator - 1);
                println!("push match arm");
                continue;
            }
            BlockType::End => {
                if i.text == "condition" {
                    //continue;
                    //if следующий блок != else
                    let (_, _, to_iterator) = look_for_cond_xy.pop().unwrap();
                    local_arrow.start_index = to_iterator;
                    local_arrow.end_index = iterator;
                    local_arrow.start_connector_index = 3;
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
                        &(0 as usize)
                    };

                    // Разделение текста на части по символу ':'
                    let parts: Vec<&str> = i.text.split(':').collect();
                    assert!(parts.len() == 2, "Invalid coordinate format");

                    // Парсинг координат
                    let to_y = parts[1].parse::<i32>().expect("Invalid y coordinate");
                    // Обновление состояния цикла и координат
                    is_cycle -= 1;
                    x_min_max_acum[0] -= 30;
                    x_min_max_acum[1] += 300;

                    local_block.r#type = String::from("Блок");
                    local_block.text = String::from("iter++");

                    // из крайнего блока цикла в цикл
                    let value = vec![
                        Node {
                            x: i.x,
                            y: i.y - 30,
                        },
                        Node { x: i.x, y: i.y },
                        Node {
                            x: x_min_max_acum[0] - 60,
                            y: i.y,
                        },
                        Node {
                            x: x_min_max_acum[0],
                            y: to_y,
                        },
                    ];
                    let local_arrow_local = Arrow {
                        start_index: iterator,
                        end_index: *cycle_index,
                        start_connector_index: 3,
                        end_connector_index: 3,
                        nodes: Vec::from(value),
                        counts: vec![1, 1, 1, 1, 1],
                    };
                    local_full_blocks.arrows.push(local_arrow_local);

                    // Обновление стрелки
                    local_arrow.start_index = cycle_acum.pop().unwrap_or(0);
                    local_arrow.end_index = iterator + 1;
                    local_arrow.start_connector_index = 1;
                    local_arrow.end_connector_index = 0;
                    println!("log {}", x_min_max_acum[0]);
                    // из цикла в блок после }
                    local_arrow.nodes.extend([
                        Node {
                            x: x_min_max_acum[0] - 130,
                            y: to_y,
                        },
                        Node {
                            x: x_min_max_acum[0] - 130,
                            y: i.y + 45,
                        },
                        Node {
                            x: i.x,
                            y: i.y + 45,
                        },
                    ]);
                    x_min_max_acum[1] += 10;
                } else {
                    local_block.r#type = String::from("Начало / конец");
                    local_block.text = if i.text.is_empty() {
                        "Конец".to_string()
                    } else {
                        i.text.clone()
                    };
                }
            }
            BlockType::Print => {
                //println!("found print in vec {} {}", i.y, i.x);
                local_block.r#type = String::from("Ввод / вывод");
                if i.text.is_empty() {
                    local_block.text = String::from("Вывод строки");
                } else {
                    local_block.text = String::from(i.text.as_str());
                }
            }
            BlockType::Cycle => {
                //local_block.y += 20;
                is_cycle += 1;
                cycle_acum.push(iterator);
                //println!("enter");
                //println!("found cycle in vec {} {}", i.y, i.x);
                local_block.r#type = String::from("Цикл for");
                local_block.text.clone_from(&i.text.to_string())
            }
            BlockType::Else => {
                //local_full_blocks.arrows.pop();
                println!("else goyda");
                local_full_blocks.arrows.pop();
                local_arrow.start_index = last_condition_index.pop().unwrap();
                local_arrow.start_connector_index = 3;
                local_arrow.end_index = iterator;
                local_arrow.end_connector_index = 0;
                local_full_blocks.arrows.push(local_arrow);
                continue;
            }
        }
        local_full_blocks.blocks.push(local_block);
        local_full_blocks.arrows.push(local_arrow);
        iterator += 1;
    }
    local_full_blocks.arrows.pop();
    to_string_pretty(&local_full_blocks).unwrap()
}

fn _add_arrow_after() {}
fn _add_arrow_before() {}
fn _add_arrow_to() {}
