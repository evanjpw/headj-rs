extern crate headj;
use clap::Parser;
use env_logger::builder;
use eyre::Result;
use headj::copy_loop::copy_loop;
use headj::copy_selector::CopySelector;
use headj::key_path::KeyPath;
use log::{error, LevelFilter};
use std::fs::File;
#[allow(unused_imports)]
use std::io::{self, BufRead, Read, Write};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    ///The JSON file to read from. If none is specified, reads from Standard Input
    #[clap(value_parser)]
    input_file: Option<PathBuf>,
    /// File to write the JSON results to (default: Standard Output)
    #[clap(short, long, value_parser)]
    out_file: Option<PathBuf>,
    /// The JSON key of the array to copy from. If none specified, treat the input JSON as an array.
    #[clap(short, long, value_parser)]
    key: Option<String>,
    /// Nicely format the output JSON with indentation & newlines.
    #[clap(short, long, action)]
    format_output: bool,
    /// Don't print any status, diagnostic or error messages
    #[clap(short, long, action)]
    quiet: bool,
    /// Output _only_ the target JSON array
    #[clap(short, long, action)]
    no_context: bool,
    /// Number of elements to skip before copying (default: 0)
    #[clap(short, long, value_parser, default_value_t = 0)]
    skip: usize,
    /// Number of elements to copy to the output (default: 100)
    #[clap(short, long, value_parser, default_value_t = 100)]
    count: usize,
    /// Activate extra debugging output
    #[clap(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

fn perform_copy(args: Args) -> Result<()> {
    let key_path = if let Some(key_str) = args.key {
        KeyPath::from_kp_str(&key_str)?
    } else {
        KeyPath::default()
    };
    let mut out_writer: Box<dyn Write> = if let Some(out_file) = args.out_file {
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
    copy_loop(in_reader, &mut out_writer, &mut copy_selector)?;
    Ok(())
}

fn setup_logging(quiet: bool, debug: u8, _use_stderr: Option<bool>) {
    let level = if quiet {
        LevelFilter::Off
    } else {
        match debug {
            0 => LevelFilter::Info,
            1 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        }
    };
    builder()
        .filter_level(level)
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .init()
}
fn main() {
    let args = Args::parse();
    let quiet = args.quiet;
    let debug = args.debug;
    setup_logging(quiet, debug, None);
    match perform_copy(args) {
        Ok(_) => {}
        Err(e) => {
            error!("Error: {e}");
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
