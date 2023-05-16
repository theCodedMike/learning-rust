use std::env::Args;
use std::error::Error;
use std::path::Path;
use std::{env, fs};

const ENV_VAR_CASE: &str = "CASE_INSENSITIVE";

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub(crate) fn new(mut args: Args) -> Result<Self, &'static str> {
        args.next(); // 略过第1个参数，因为第1个参数是二进制可执行文件的路径
        let query = match args.next() {
            None => return Err("Didn't get a query string"),
            Some(query) => query,
        };
        let filename = match args.next() {
            None => return Err("Didn't get a file name"),
            Some(file_name) => {
                if !Path::new(&file_name).try_exists().unwrap_or_default() {
                    return Err("not available file path");
                }
                file_name
            }
        };
        // 默认是大小写敏感的
        let case_sensitive = env::var(ENV_VAR_CASE).is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    /*
    for line in results {
        println!("{}", line);
    }
    */
    results.iter().for_each(|line| println!("{}", line));

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // for循环版本
    /*
    let mut results = Vec::new();
    // 使用 lines 方法遍历每一行
    for line in contents.lines() {
        // 用查询字符串搜索每一行
        if line.contains(query) {
            // 存储匹配的行
            results.push(line);
        }
    }
    results
    */
    // 迭代器版本
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = &query.to_lowercase();
    // for循环版本
    /*
    let mut results = Vec::new();
    // 使用 lines 方法遍历每一行
    for line in contents.lines() {
        // 用查询字符串搜索每一行
        if line.to_lowercase().contains(query) {
            // 存储匹配的行
            results.push(line);
        }
    }
    results
    */
    // 迭代器版本
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
            Rust:\n\
            safe, fast, productive.\n\
            Pick three.\
        ";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
            Rust:\n\
            safe, fast, productive.\n\
            Pick three.\n\
            Trust me.\
        ";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
