use std::fmt::Debug;
use std::fs;
use std::io::BufRead;
use std::path::Path;
use std::{env, io};
use walkdir::WalkDir;

struct Stats {
    loc_src: i32,
    loc_tests: i32,
    loc_all: i32,
    cnt_src: i32,
    cnt_test: i32,
    cnt_all: i32,
}

fn get_lang_info(lang: &str) -> &str {
    dbg!(lang);
    match lang.to_lowercase().as_str() {
        "python" => ".py",
        "rust" => ".rs",
        _ => todo!(),
    }
}

fn verify_line_is_code(line: String) {
    todo!("not implemented yet")
}

fn process_file(stat_type: &str, stats: &mut Stats, file_path: &Path) -> io::Result<()> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            //println!("{}", line);
            match stat_type {
                "src" => {
                    stats.loc_src += 1;
                    stats.loc_all += 1;
                }
                "test" => {
                    stats.loc_tests += 1;
                    stats.loc_all += 1;
                }
                _ => {}
            }
        } else {
            eprintln!("Error reading line from file '{}'", file_path.display());
        }
    }
    Ok(())
}

fn print_stats(stats: Stats) {
    println!("Total Section");
    println!("Total: {} files", stats.cnt_all);
    println!("Total: {} LoC", stats.loc_all);
    println!("----------------------------------");
    println!("SRC Section");
    println!("SRC: {} files", stats.cnt_src);
    println!("SRC: {} LoC", stats.loc_src);
    println!("----------------------------------");
    println!("Test Section");
    println!("Test: {} files", stats.cnt_test);
    println!("Test: {} LoC", stats.loc_tests);
}

fn main() {
    println!("Lines of Code CLI Tool");
    println!("Usage: loco --language path(. for current path)");
    // Get command line Arguments
    let args: Vec<String> = env::args().collect();
    // Stats
    let mut stats = Stats {
        loc_src: 0,
        loc_tests: 0,
        loc_all: 0,
        cnt_src: 0,
        cnt_test: 0,
        cnt_all: 0,
    };
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
    //let path = &args[2];
    let path = if args[2] == "." {
        std::env::current_dir().expect("Cannot get current directory")
    } else {
        std::path::PathBuf::from(&args[2])
    };
    println!("{:?}", extension);
    println!("Extension for language: {} is {}", lang, extension);

    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        let f_name = entry.file_name();
        let file_extension = entry.path().extension();
        let entry_str = entry.path().to_str().unwrap();

        if entry_str.contains("src") && entry_str.ends_with(extension) {
            println!("{} - {:?}", entry.path().display(), f_name);
            stats.cnt_src += 1;
            stats.cnt_all += 1;

            //read file
            process_file("src", &mut stats, entry.path());
        }

        if entry_str.contains("test") && entry_str.contains(extension) {
            println!("{} - {:?}", entry.path().display(), f_name);
            stats.cnt_test += 1;
            stats.cnt_all += 1;

            process_file("test", &mut stats, entry.path());
        }
    }

    // print stats
    print_stats(stats);
}
