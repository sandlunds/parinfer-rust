extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate unicode_segmentation;
extern crate unicode_width;


mod parinfer;
mod json;
mod changes;
mod common_wrapper;

use std::env;
use std::io;
use std::io::{Read,Write};
use std::panic;

extern crate getopts;

fn options() -> getopts::Options {
    let mut options = getopts::Options::new();
    options.optflag("j", "json", "read JSON input and write JSON response");
    options
}

fn parse_args() -> getopts::Matches {
    let args: Vec<String> = env::args().collect();
    match options().parse(&args[1..]) {
        Ok(matches) => matches,
        Err(f) => { panic!(f.to_string()); }
    }
}

pub fn main() -> io::Result<()> {
    let opts = parse_args();
    if opts.opt_present("j") {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)?;
        let output = match panic::catch_unwind(|| common_wrapper::internal_run(&input)) {
            Ok(Ok(result)) => result,
            Ok(Err(e)) => serde_json::to_string(&json::Answer::from(e)).unwrap(),
            Err(_) => common_wrapper::panic_result()
        };
        io::stdout().write(output.as_bytes())?;
    }

    Ok(())
}
