use clap::App;

fn main() {
    let _matches = App::new("recho")
        .version("0.1.0")
        .author("James Sargant <shorinbonsai@gmail.com>")
        .about("Rust implementation of Echo")
        .get_matches();
    println!("Hello, world!");
}
