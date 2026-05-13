use std::env;

enum Langs {
    Python,
    Rust,
}

struct Stats {
    loc_src: i32,
    loc_tests: i32,
    loc_all: i32,
}

fn get_lang_info(lang: &str) -> &str {
    dbg!(lang);
    match lang.to_lowercase().as_str() {
        "python" => ".py",
        "rust" => ".rs",
        _ => todo!(),
    }
}

fn main() {
    println!("Lines of Code CLI Tool");
    println!("Usage: loco --language path(. for current path)");
    // Get command line Arguments
    let args: Vec<String> = env::args().collect();
    println!("{:?}", &args);

    dbg!(&args);
    dbg!(&args.len());

    if args.len() < 3 {
        eprintln!("Error: Missing language argument.");
        std::process::exit(1);
    }

    // Define values.
    let lang = &args[1];
    let extension = get_lang_info(&args[1]);
    let path = &args[2];
    println!("{:?}", extension);
    println!("Extension for language: {} is {}", lang, extension);

    //take path and look up all files in the path and print it for now.
    match path.to_lowercase().as_str() {
        "." => println!("Take current path"),
        _ => println!("Take given path."),
    }
}
