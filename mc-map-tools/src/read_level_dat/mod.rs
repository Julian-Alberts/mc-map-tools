use std::path::Path;

pub fn main(save_directory: &Path) {
    let level = save_directory.join("level.dat");
    let level_dat = std::fs::read(level).expect("Failed to read level.dat");
    let a = mc_map_reader::parse_level_dat(&level_dat).expect("Failed to parse level.dat");
    println!("{:#?}", a);
}
