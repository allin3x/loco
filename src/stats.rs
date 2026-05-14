pub struct Stats {
    pub loc_src: i32,
    pub loc_tests: i32,
    pub loc_all: i32,
    pub cnt_src: i32,
    pub cnt_test: i32,
    pub cnt_all: i32,
}

pub fn print_stats(stats: Stats) {
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
