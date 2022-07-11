use clap::{Arg, Command};

fn main() {
    let matches = Command::new("recho")
        .version("0.1.0")
        .author("James Sargant <shorinbonsai@gmail.com>")
        .about("Rust implementation of Echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();

    let text: Vec<String> = matches
        .get_many("text")
        .expect("`text`is required")
        .cloned()
        .collect();
    let omit_newline = matches.is_present("omit_newline");
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
