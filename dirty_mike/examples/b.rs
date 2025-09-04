use toml::Table;

fn main() {
    let s = include_str!("../Cargo.toml");
    let f = s.parse::<Table>().unwrap();
}
