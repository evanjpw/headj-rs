use eyre::{eyre, Result};
use json_event_parser::{JsonEvent, JsonReader, JsonWriter};
use std::io::{BufRead, Write};

use crate::copy_selector::CopySelector;

pub fn copy_loop<R: BufRead, W: Write>(
    in_json: R,
    out_json: &mut W,
    cs: &mut CopySelector,
) -> Result<()> {
    let mut inj = JsonReader::from_reader(in_json);
    let mut outj = JsonWriter::from_writer(out_json);
    let mut buff = Vec::new();

    loop {
        let event = inj.read_event(&mut buff)?;
        if event == JsonEvent::Eof {
            return if cs.target_copied() {
                Ok(())
            } else {
                Err(eyre!("Did not complete JSON copy"))
            };
        }
        let copy_to_out = cs.select(event)?;
        if copy_to_out {
            outj.write_event(event)?;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::copy_loop::copy_loop;
    use crate::copy_selector::CopySelector;
    use crate::key_path::KeyPath;
    use eyre::Result;
    use std::io::BufReader;
    use std::str;

    fn run_run_headj(
        json_input_document: &str,
        key_path_str: &str,
        count: usize,
        skip: usize,
        no_context: bool,
    ) -> Result<String> {
        let key_path = KeyPath::from_kp_str(key_path_str)?;
        let mut copy_selector = CopySelector::new(key_path, count, skip, no_context);
        let input_reader = BufReader::new(json_input_document.as_bytes());
        let mut output_writer: Vec<u8> = Vec::new();
        let _ = copy_loop(input_reader, &mut output_writer, &mut copy_selector)?;
        let out_string = str::from_utf8(&output_writer)?;
        Ok(out_string.to_string())
    }

    #[test]
    ///     headj <<- JSON
    ///     [1,2,3,4,5]
    ///     JSON
    ///     # Output: 1, 2, 3, 4, 5]
    fn test_list_no_args() {
        let result = run_run_headj("[1,2,3,4,5]", "", 100, 0, true).unwrap();
        assert_eq!("[1,2,3,4,5]", result.as_str());
    }

    #[test]
    /// headj /dev/null
    /// # No Output
    fn test_no_input() {
        let e = run_run_headj("", "", 100, 0, true).unwrap_err().to_string();
        assert_eq!("unexpected end of file", e.as_str());
    }

    #[test]
    ///     headj -c 1 <<- JSON
    ///     [1,2,3,4,5]
    ///     JSON
    ///     # Output: [1]
    fn test_list_1_count() {
        let result = run_run_headj("[1,2,3,4,5]", "", 1, 0, true).unwrap();
        assert_eq!("[1]", result.as_str());
    }

    #[test]
    ///     headj -c 1 -s 2 <<- JSON
    ///     [1,2,3,4,5]
    ///     JSON
    ///     # Output: [3]
    fn test_list_1_count_2_skip() {
        let result = run_run_headj("[1,2,3,4,5]", "", 1, 2, true).unwrap();
        assert_eq!("[3]", result.as_str());
    }

    #[test]
    ///     headj -c 2 -s 2 <<- JSON
    ///     [1,2,3,4,5]
    ///     JSON
    ///     # Output: [3, 4]
    fn test_list_2_count_2_skip() {
        let result = run_run_headj("[1,2,3,4,5]", "", 2, 2, true).unwrap();
        assert_eq!("[3,4]", result.as_str());
    }
    // false
    #[test]
    /// # Keys: ['foo']
    /// headj -k 'foo' <<- JSON
    /// {"foo":[1,2,3,4,5]}
    /// JSON
    /// # Output: [1, 2, 3, 4, 5]
    fn test_object_1_key_context() {
        let result = run_run_headj("{\"foo\":[1,2,3,4,5]}", "foo", 100, 0, false).unwrap();
        assert_eq!("{\"foo\":[1,2,3,4,5]}", result.as_str())
    }

    #[test]
    ///         # Keys: ['foo']
    ///         headj -k 'foo' <<- JSON
    ///         {"foo":[1,2,3,4,5]}
    ///         JSON
    ///         # Output: [1, 2, 3, 4, 5]
    fn test_object_1_key() {
        let result = run_run_headj("{\"foo\":[1,2,3,4,5]}", "foo", 100, 0, true).unwrap();
        assert_eq!("[1,2,3,4,5]", result.as_str());
    }

    #[test]
    fn test_complex_elements_key_no_context() {
        let result = run_run_headj(
            "{\"foo\":[{\"a\":[1]},{\"b\":[2]},{\"c\":[3]},{\"d\":[4]},{\"e\":[5]}]}",
            "foo",
            2,
            2,
            true,
        )
        .unwrap();
        assert_eq!("[{\"c\":[3]},{\"d\":[4]}]", result.as_str());
    }

    #[test]
    fn test_complex_elements_key() {
        let result = run_run_headj(
            "{\"foo\":[{\"a\":[1]},{\"b\":[2]},{\"c\":[3]},{\"d\":[4]},{\"e\":[5]}]}",
            "foo",
            2,
            2,
            false,
        )
        .unwrap();
        assert_eq!("{\"foo\":[{\"c\":[3]},{\"d\":[4]}]}", result.as_str());
    }

    #[test]
    fn test_complex_elements() {
        let result = run_run_headj(
            "[{\"a\":[1]},{\"b\":[2]},{\"c\":[3]},{\"d\":[4]},{\"e\":[5]}]",
            "",
            2,
            2,
            false,
        )
        .unwrap();
        assert_eq!("[{\"c\":[3]},{\"d\":[4]}]", result.as_str());
    }

    #[test]
    ///         headj -k 'foo.bar' -c 2 -s 2 <<- JSON
    ///         {"foo":{"bar":[1,2,3,4,5]}}
    ///         JSON
    ///         # Output: [3, 4]
    fn test_object_2_keys_2_count_2_skip_context_format() {
        let result =
            run_run_headj("{\"foo\":{\"bar\":[1,2,3,4,5]}}", "foo.bar", 2, 2, false).unwrap();
        assert_eq!("{\"foo\":{\"bar\":[3,4]}}", result.as_str())
    }

    #[test]
    /// headj -k 'foo.bar' -c 2 -s 2 <<- JSON
    /// {"foo":{"bar":[1,2,3,4,5]}}
    /// JSON
    /// # Output: [3, 4]
    fn test_object_2_keys_2_count_2_skip_context() {
        let result =
            run_run_headj("{\"foo\":{\"bar\":[1,2,3,4,5]}}", "foo.bar", 2, 2, false).unwrap();
        assert_eq!("{\"foo\":{\"bar\":[3,4]}}", result.as_str());
    }

    #[test]
    /// headj -k 'foo.bar' -c 2 -s 2 <<- JSON
    /// {"foo":{"bar":[1,2,3,4,5]}}
    /// JSON
    /// # Output: [3, 4]
    fn test_object_2_keys_2_count_2_skip() {
        let result =
            run_run_headj("{\"foo\":{\"bar\":[1,2,3,4,5]}}", "foo.bar", 2, 2, true).unwrap();
        assert_eq!("[3,4]", result.as_str());
    }

    #[test]
    /// headj -k 'foo' -c 2 -s 2 <<- JSON
    /// {"foo":[1,2,3,4,5]}
    /// JSON
    /// # Output: [3, 4]
    fn test_object_1_key_2_count_2_skip_context() {
        let result = run_run_headj("{\"foo\":[1,2,3,4,5]}", "foo", 2, 2, false).unwrap();
        assert_eq!("{\"foo\":[3,4]}", result.as_str());
    }

    #[test]
    /// headj -k 'foo' -c 2 -s 2 <<- JSON
    /// {"foo":[1,2,3,4,5]}
    /// JSON
    /// # Output: [3, 4]
    fn test_object_1_key_2_count_2_skip() {
        let result = run_run_headj("{\"foo\":[1,2,3,4,5]}", "foo", 2, 2, true).unwrap();
        assert_eq!("[3,4]", result.as_str());
    }

    #[test]
    /// headj -k 'foo' /dev/null
    /// # Error: cannot unpack non-iterable NoneType object
    fn test_no_input_with_key() {
        let e = run_run_headj("", "foo", 100, 0, false)
            .unwrap_err()
            .to_string();
        assert_eq!("unexpected end of file", e.as_str());
    }

    #[test]
    /// headj -k 'fooo.bar' -c 2 -s 2 <<- JSON
    /// {"foo":{
    /// "bar":[1,2,3,4,5]}
    /// }
    /// JSON
    /// # Error: Could not find key "fooo" in object "<TransientStreamingJSONObject: TRANSIENT, DONE>".
    fn test_incorect_first_key() {
        let e = run_run_headj(
            "\n                {\"foo\":{\n                \"bar\":[1,2,3,4,5]}\n                }\n",
            "fooo.bar",
            2,
            2,false
        ).unwrap_err().to_string();
        assert_eq!("Did not complete JSON copy", e.as_str());
    }

    #[test]
    /// headj -k 'foo' -c 2 -s 2 <<- JSON
    /// {"bar":[1,2,3,4,5]}
    /// JSON
    /// # Error: Could not find key "foo" in object "<TransientStreamingJSONObject: TRANSIENT, DONE>".
    fn test_incorrect_key() {
        let e = run_run_headj("{\"bar\":[1,2,3,4,5]}", "foo", 2, 2, false)
            .unwrap_err()
            .to_string();
        assert_eq!("Did not complete JSON copy", e.as_str());
    }

    #[test]
    /// headj -k 'foo' <<- JSON
    /// [1,2,3,4,5]
    /// JSON
    /// # Error: Could not look up key "foo" in non-dictionary-object '<TransientStreamingJSONList:  #TRANSIENT, STREAMING>'.
    fn test_array_with_key() {
        let e = run_run_headj("[1,2,3,4,5]", "foo", 100, 0, false)
            .unwrap_err()
            .to_string();
        assert_eq!("Did not complete JSON copy", e.as_str());
    }

    #[test]
    /// headj -k 'foo.barz' -c 2 -s 2 <<- JSON
    /// {"foo":{"bar":[1,2,3,4,5]}}
    /// JSON
    /// # Error: Could not find key "barz" in object "<TransientStreamingJSONObject: TRANSIENT, DONE>".
    fn test_incorrect_second_key() {
        let e = run_run_headj("{\"foo\":{\"bar\":[1,2,3,4,5]}}", "foo.barz", 2, 2, false)
            .unwrap_err()
            .to_string();
        assert_eq!("Did not complete JSON copy", e.as_str());
    }

    #[test]
    /// headj -k 'fooo.bar' -c 2 -s 2 <<- JSON
    /// {"foo":{"bar":[1,2,3,4,5]}}
    /// JSON
    /// # Error: Could not find key "fooo" in object "<TransientStreamingJSONObject: TRANSIENT, DONE>".
    fn test_incorrect_first_key_compact() {
        let e = run_run_headj("{\"foo\":{\"bar\":[1,2,3,4,5]}}", "fooo.bar", 2, 2, false)
            .unwrap_err()
            .to_string();
        assert_eq!("Did not complete JSON copy", e.as_str());
    }
}
