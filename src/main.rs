use clap::{App, Arg};

fn main() {
    let matches = App::new("recho")
        .version("0.1.0")
        .author("James Sargant <shorinbonsai@gmail.com>")
        .about("Rust implementation of Echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");
    println!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
