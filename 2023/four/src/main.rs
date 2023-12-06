fn main() {
    let _lines = lines_from_file("src/input.txt");
}

fn lines_from_file(path: impl AsRef<std::path::Path>) -> std::io::Result<Vec<String>> {
    let file = std::fs::File::open(path)?;
    let buf = std::io::BufReader::new(file);
    std::io::BufRead::lines(buf).collect()
}
