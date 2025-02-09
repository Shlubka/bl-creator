use anyhow::{Context, Result};
use inquire::{Select, validator::Validation};
use std::{env, fs, path::PathBuf};

mod mk_json_blocks;
mod lang_vec_stuf;

use crate::{
    lang_vec_stuf::{Language, Rust},
    mk_json_blocks::create_json_blocks,
};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let lang = match get_argument(&args, 1) {
        Ok(l) => l,
        Err(_) => prompt_for_language()?,
    };

    let file_path = match get_argument(&args, 2) {
        Ok(p) => p,
        Err(_) => prompt_for_file_path()?,
    };

    let support_language: Vec<Box<dyn Language>> = vec![
        Box::new(Rust),
    ];

    let selected_language = select_language(&lang, &support_language)
        .context("Failed to select language")?;

    let path = PathBuf::from(&file_path);
    let source_code = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?
        .replace('\t', "");

    let analyzed_vector = selected_language.analyze_to_vec(source_code);
    let final_string = create_json_blocks(analyzed_vector);

    let output_dir = PathBuf::from("outfiles");
    fs::create_dir_all(&output_dir)
        .context("Failed to create output directory")?;

    let output_file_name = path.file_stem()
        .and_then(|s| s.to_str())
        .context("Invalid file name")?;

    let output_path = output_dir.join(format!("{}.json", output_file_name));
    fs::write(&output_path, final_string)
        .with_context(|| format!("Failed to write to {}", output_path.display()))?;

    Ok(())
}

fn get_argument(args: &[String], index: usize) -> Result<String, anyhow::Error> {
    args.get(index)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("Argument at index {} is missing", index))
}

fn prompt_for_language() -> Result<String, anyhow::Error> {
    Select::new("Select language:", vec!["Rust".to_string()])
        .prompt()
        .map_err(Into::into)
}

fn prompt_for_file_path() -> Result<String, anyhow::Error> {
    inquire::Text::new("Enter file path:")
        .with_validator(|input: &str| {
            if PathBuf::from(input).exists() {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("File not found".into()))
            }
        })
        .prompt()
        .map_err(Into::into)
}

fn select_language<'a>(
    lang: &str,
    support_language: &'a [Box<dyn Language>],
) -> Result<&'a Box<dyn Language>, anyhow::Error> {
    if let Some(lang) = support_language.iter().find(|l| l.get_name().eq_ignore_ascii_case(lang)) {
        return Ok(lang);
    }

    let options: Vec<String> = support_language.iter()
        .map(|l| l.get_name().to_string())
        .collect();

    let selected = Select::new("Select language:", options)
        .prompt()?;

    support_language.iter()
        .find(|l| l.get_name() == selected)
        .ok_or_else(|| anyhow::anyhow!("Selected language is not supported"))
}
