use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::search;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    //let config = Config::new(&args); // 추가: 명령줄 인자에서 검색어와 파일 경로를 추출하는 부분을 함수로 분리
    //let config = parse_config(&args); // 추가: 명령줄 인자에서 검색어와 파일 경로를 추출하는 부분을 함수로 분리
    // let query = &args[1]; // 추가: 명령줄 인자에서 검색어와 파일 경로를 추출하는 부분을 함수로 분리
    // let file_path = &args[2]; // 추가: 명령줄 인자에서 검색어와 파일 경로를 추출하는 부분을 함수로 분리

    // println!("Searching for {query}");
    // println!("In file {file_path}");
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

    // let contents = fs::read_to_string(file_path)
    // let contents = fs::read_to_string(config.file_path)
    //     .expect("Should have been able to read the file");

    // println!("With text:\n{contents}");
}

// fn run(config: Config) {
//     let contents = fs::read_to_string(config.file_path)
//         .expect("Should have been able to read the file");

//     println!("With text:\n{contents}");
// }
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    //println!("With text:\n{contents}");
    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     Config{query, file_path}
// }

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config{query, file_path}
    }

    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}