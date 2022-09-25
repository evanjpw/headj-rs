extern crate headj;
use clap::Parser;
use eyre::Result;
use headj::copy_loop::copy_loop;
use headj::copy_selector::CopySelector;
use headj::key_path::KeyPath;
use headj::EConsole;
use std::fs::File;
#[allow(unused_imports)]
use std::io::{self, BufRead, Read, Write};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(value_parser)]
    input_file: Option<PathBuf>,
    #[clap(short, long, value_parser)]
    out_file: Option<PathBuf>,
    #[clap(short, long, value_parser)]
    key: Option<String>,
    #[clap(short, long, action)]
    format_output: bool,
    #[clap(short, long, action)]
    quiet: bool,
    #[clap(short, long, action)]
    no_context: bool,
    #[clap(short, long, value_parser, default_value_t = 0)]
    skip: u32,
    #[clap(short, long, value_parser, default_value_t = 100)]
    count: u32,
    #[clap(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

fn perform_copy(args: Args) -> Result<()> {
    let key_path = if let Some(key_str) = args.key {
        KeyPath::from_kp_str(&key_str)?
    } else {
        KeyPath::default()
    };
    let out_writer: Box<dyn Write> = if let Some(out_file) = args.out_file {
        Box::new(File::create(out_file)?)
    } else {
        Box::new(io::stdout())
    };
    let in_reader: Box<dyn BufRead> = if let Some(in_file) = args.input_file {
        Box::new(io::BufReader::new(File::open(in_file)?))
    } else {
        Box::new(io::stdin().lock())
    };
    let mut copy_selector = CopySelector::new(key_path, args.count, args.skip, args.no_context);
    copy_loop(in_reader, out_writer, &mut copy_selector)?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    let quiet = args.quiet;
    let debug = args.debug;
    EConsole::init(quiet, debug, None).expect("EConsole init failed");
    match perform_copy(args) {
        Ok(_) => {}
        Err(e) => {
            //
            EConsole::console().error(&format!("Error: {e}"))
.expect(" if !qui {i}");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Args;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Args::command().debug_assert();
    }
}
