use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use regex::Regex;

mod rand;
mod base;
mod base_sdk;

fn process_sql_file(file_path: &str, output_file: &mut File, create_table_regex: &Regex) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // 遍历每一行，查找建表语句
    for line in reader.lines() {
        let line = line?;
        if let Some(captures) = create_table_regex.captures(&line) {
            // 如果匹配到了建表语句，提取表名和语句内容
            let table_name = captures.get(1).unwrap().as_str();
            let create_statement = captures.get(2).unwrap().as_str();

            // 将建表语句写入到输出文件中
            writeln!(output_file, "CREATE TABLE {} ({});", table_name, create_statement)?;
        }
    }

    Ok(())
}

fn main() {
    // 获取命令行参数作为文件夹路径
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <folder_path>", args[0]);
        std::process::exit(1);
    }
    let folder_path = &args[1];

    // 打开一个输出文件
    let mut output_file = match File::create("tables.sql") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create output file: {}", e);
            std::process::exit(1);
        }
    };

    // 正则表达式匹配建表语句
    let create_table_regex = match Regex::new(r#"CREATE\s+TABLE\s+(\w+)\s+\((.+?)\);"#) {
        Ok(regex) => regex,
        Err(e) => {
            eprintln!("Failed to create regex: {}", e);
            std::process::exit(1);
        }
    };

    // 遍历 SQL 文件夹
    match fs::read_dir(folder_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_path = entry.path();
                    if let Some(file_name) = file_path.to_str() {
                        match file_path.extension() {
                            Some(ext) if ext == "sql" => {
                                if let Err(e) = process_sql_file(file_name, &mut output_file, &create_table_regex) {
                                    eprintln!("Error processing {}: {}", file_name, e);
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read directory {}: {}", folder_path, e);
            std::process::exit(1);
        }
    }

    println!("Tables extracted successfully to tables.sql");
}
