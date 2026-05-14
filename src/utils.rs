use crate::stats::Stats;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn get_lang_info(lang: &str) -> &str {
    dbg!(lang);
    match lang.to_lowercase().as_str() {
        "python" => ".py",
        "rust" => ".rs",
        _ => todo!(),
    }
}

pub fn verify_line_is_code(line: String) -> bool {
    line.is_empty()
}

pub fn process_file(stat_type: &str, stats: &mut Stats, file_path: &Path) -> io::Result<()> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            if !verify_line_is_code(line) {
                continue;
            }
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
            return Err(std::io::Error::other("bad".to_string()));
        }
    }
    Ok(())
}
