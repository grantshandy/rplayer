use clap::{app_from_crate, crate_description, crate_authors, crate_name, crate_version, Arg};
use colored::Colorize;
use std::io::BufReader;
use std::path::Path;
use rodio::{Decoder, OutputStream, Sink};

fn main() {
    let matches = app_from_crate!()
        .arg(
            Arg::with_name("FILE")
                .help("Sets the input file to play")
                .required(true)
                .index(1)
        )
        .arg(
            Arg::with_name("volume")
                .help("Set volume to play audio at, 1.0 is normal volume.")
                .long("volume")
                .short("v")
                .takes_value(true)
                .required(false)
        )
        .get_matches();

    let (_stream, stream_handle) = match OutputStream::try_default() {
        Ok((stream, stream_handle)) => (stream, stream_handle),
        Err(error) => {
            clap_error(error.to_string());
            std::process::exit(1);
        },
    };

    let sink = match Sink::try_new(&stream_handle) {
        Ok(data) => data,
        Err(error) => {
            clap_error(error.to_string());
            std::process::exit(1);
        }
    };

    let path = Path::new(matches.value_of("FILE").unwrap());

    if !path.is_file() {
        clap_error("Couldn't find <FILE>");
        std::process::exit(1);
    };

    let file = match std::fs::File::open(path) {
        Ok(data) => data,
        Err(error) => {
            clap_error(error.to_string());
            std::process::exit(1);
        },
    };

    let file = BufReader::new(file);

    let source = match Decoder::new(file) {
        Ok(data) => data,
        Err(error) => {
            clap_error(error.to_string());
            std::process::exit(1);
        }
    };
    sink.append(source);

    match matches.value_of("volume") {
        Some(data) => {
            match data.parse::<f32>() {
                Ok(num) => sink.set_volume(num),
                Err(_) => {
                    clap_error("Volume must be a value like \"3.0\" or \"0.45\"");
                    std::process::exit(1);
                },
            }
        },
        None => (),
    };

    println!("Playing \"{}\"...", path.file_name().unwrap().to_string_lossy());
    sink.sleep_until_end();
}

fn clap_error<T: AsRef<str>>(text: T) {
    eprintln!("{} {}\n", "error:".red().bold(), text.as_ref());
    eprintln!("For more information try {}", "--help".green());
}