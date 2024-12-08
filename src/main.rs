use dialoguer::Input;
use inquire::Select;
use std::{env, fs, path::PathBuf};
//use tree_sitter_c::LANGUAGE as tree_sitter_c;

mod mk_json_blocks;
use crate::mk_json_blocks::create_json_blocks;
mod lang_vec_stuf;
use crate::lang_vec_stuf::{Rust, C};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let lang = get_argument(&args, 1).unwrap_or_else(|_| prompt_for_language());
    let file_path = get_argument(&args, 2).unwrap_or_else(|_| prompt_for_file_path());
    let support_language: Vec<Box<dyn lang_vec_stuf::Language>> = vec![Box::new(Rust), Box::new(C)];

    let selected_language = select_language(&lang, &support_language)?;
    let path = PathBuf::from(file_path);

    let source_code = fs::read_to_string(&path)?.replace('\t', "");

    let analyzed_vector = selected_language.analyze_to_vec(source_code);
    let final_string = create_json_blocks(analyzed_vector);

    let output_file_name = path.file_stem().unwrap().to_str().unwrap();
    let mut output_file_path = PathBuf::from("outfiles");
    output_file_path.push(format!("{output_file_name}.json"));
    fs::write(output_file_path, final_string)?;

    Ok(())
}


fn get_argument(args: &[String], index: usize) -> Result<String, String> {
    if args.len() > index {
        Ok(args[index].clone())
    } else {
        Err(format!("Argument at index {index} is missing"))
    }
}

fn prompt_for_language() -> String {
    Input::new()
        .with_prompt("Enter the language")
        .interact()
        .expect("Failed to read language")
}

fn prompt_for_file_path() -> String {
    Input::new()
        .with_prompt("Enter the file path")
        .interact()
        .expect("Failed to read file path")
}

fn select_language<'a>(
    lang: &str,
    support_language: &'a [Box<dyn lang_vec_stuf::Language>],
) -> Result<&'a Box<dyn lang_vec_stuf::Language>, String> {
    let selected_language: &str = match lang.to_lowercase().as_str() {
        "rust" => "Rust",
        "java" => "Java",
        "c" => "C",
        "cpp" | "c++" | "cplusplus" => "CPlusPlus",
        _ => Select::new(
            "Language?",
            support_language.iter().map(|x| x.get_name()).collect(),
        )
        .prompt()
        .map_err(|e| e.to_string())?,
    };

    support_language
        .iter()
        .find(|x| x.get_name() == selected_language)
        .ok_or_else(|| format!("Language {selected_language} is not supported"))
}
