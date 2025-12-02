
pub fn read_input(day: u8) -> String {
    let filename = format!("input/day{:02}.txt", day);
    let raw = std::fs::read_to_string(filename)
        .expect("Failed to read input file");
    raw.as_str().trim().to_string()
}

pub fn read_demo(day: u8) -> String {
    let filename = format!("input/day{:02}_demo.txt", day);
    let raw = std::fs::read_to_string(filename)
        .expect("Failed to read demo file");
    raw.as_str().trim().to_string()
}