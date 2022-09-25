use eyre::Result;
use json_event_parser::{JsonEvent, JsonReader, JsonWriter};
use std::io::{BufRead, Write};

use crate::copy_selector::CopySelector;

pub fn copy_loop<R: BufRead, W: Write>(
    in_json: R,
    out_json: W,
    cs: &mut CopySelector,
) -> Result<()> {
    let mut inj = JsonReader::from_reader(in_json);
    let mut outj = JsonWriter::from_writer(out_json);
    let mut buff = Vec::new();

    loop {
        let event = inj.read_event(&mut buff)?;
        if event == JsonEvent::Eof {
            return Ok(());
        }
        let copy_to_out = cs.select(event);
        if copy_to_out {
            outj.write_event(event)?;
        }
    }
}
