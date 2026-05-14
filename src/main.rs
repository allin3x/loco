use std::{env, io};
use walkdir::WalkDir;

mod stats;
mod utils;

fn main() {
    println!("Lines of Code CLI Tool");
    println!("Usage: loco --language path(. for current path)");
    // Get command line Arguments
    let args: Vec<String> = env::args().collect();
    // Stats
    let mut stats = stats::Stats {
        loc_src: 0,
        loc_tests: 0,
        loc_all: 0,
        cnt_src: 0,
        cnt_test: 0,
        cnt_all: 0,
    };
    println!("{:?}", &args);

    if args.len() < 3 {
        eprintln!("Error: Missing language argument.");
        std::process::exit(1);
    }

    let lang = &args[1];
    let extension = utils::get_lang_info(&args[1]);
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
            crate::utils::process_file("src", &mut stats, entry.path());
        }

        if entry_str.contains("test") && entry_str.contains(extension) {
            println!("{} - {:?}", entry.path().display(), f_name);
            stats.cnt_test += 1;
            stats.cnt_all += 1;

            crate::utils::process_file("test", &mut stats, entry.path());
        }
    }

    // print stats
    stats::print_stats(stats);
}
