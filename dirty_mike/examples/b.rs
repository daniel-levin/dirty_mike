use toml::Table;

fn main() {
    let s = include_str!("../Cargo.toml");
    let _f = s.parse::<Table>().unwrap();
}
