#[macro_use]
extern crate clap;

use std::fs::File;
use std::io::{Read, BufReader};
use ansi_term::Colour;
use ansi_term::Colour::RGB;
use clap::{Arg,App};

fn byte_color(b : u8)->Colour {
    match b {
        0 => RGB(0,0,0),
        1..=0x1F => RGB(0xff, 0, 0),
        0x20..=0x7F => RGB(0xc0, 0, 0),
        0x80..=0xFF => RGB(0,0xc0, 0),
    }
}

fn main() -> std::io::Result<()> {
    let matches = App::new("Hex colormap Dump")
        .version("0.0.1")
        .author("theoremoon")
        .arg(Arg::with_name("width")
            .short("w")
            .long("width")
            .takes_value(true)
            .default_value("8"))
        .arg(Arg::with_name("file")
            .required(true)
            .takes_value(true)
            .index(1))
        .get_matches();


    let bufsize : usize = value_t!(matches.value_of("width"), usize).unwrap_or(8);

    let mut reader = BufReader::new(File::open(matches.value_of("file").unwrap())?);
    let mut buf = vec![0u8; bufsize];
    let fg = RGB(255,255,255);
    loop {
        match reader.read(&mut buf)? {
            0 => break,
            n => {
                for x in &buf[..n] {
                    let s = format!("{:02X}", x);
                    print!("{}", fg.on(byte_color(*x)).bold().paint(s));
                }
                println!("");
            }
        }
    }
    return Ok(());

}
