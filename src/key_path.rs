use eyre::Result;
use json_event_parser::JsonEvent;
use std::iter::Iterator;
use std::ops::Index;

#[derive(Clone, Debug, PartialEq)]
pub enum OwnedJsonEvent {
    String(String),
    Number(String),
    Boolean(bool),
    Null,
    StartArray,
    EndArray,
    StartObject,
    EndObject,
    ObjectKey(String),
    Eof,
}

impl OwnedJsonEvent {
    pub fn as_json_event(&self) -> JsonEvent {
        match self {
            Self::String(s) => JsonEvent::String(s),
            Self::Number(s) => JsonEvent::Number(s),
            Self::ObjectKey(s) => JsonEvent::ObjectKey(s),
            Self::Boolean(b) => JsonEvent::Boolean(*b),
            Self::Null => JsonEvent::Null,
            Self::StartArray => JsonEvent::StartArray,
            Self::EndArray => JsonEvent::EndArray,
            Self::StartObject => JsonEvent::StartObject,
            Self::EndObject => JsonEvent::EndObject,
            Self::Eof => JsonEvent::Eof,
        }
    }
}

#[derive(Default)]
pub struct KeyPath {
    json_path: Vec<OwnedJsonEvent>,
}

#[allow(clippy::len_without_is_empty)]
impl KeyPath {
    pub fn from_kp_str(key_path_str: &str) -> Result<Self> {
        let mut json_path = Vec::new();
        let mut quote = false;
        let mut current_key = String::new();
        for c in key_path_str.chars() {
            // dbg!(c,&json_path, &current_key, &quote);
            if quote {
                current_key.push(c);
                quote = false;
            } else {
                match c {
                    // TODO: add brackets
                    '\\' => quote = true,
                    '.' => {
                        let element = OwnedJsonEvent::ObjectKey(std::mem::take(&mut current_key));
                        // dbg!(&element);
                        json_path.push(element);
                        current_key = String::new();
                    }
                    _ => current_key.push(c),
                }
            };
        }
        if !current_key.is_empty() {
            let element = OwnedJsonEvent::ObjectKey(std::mem::take(&mut current_key));
            json_path.push(element);
        }
        Ok(Self { json_path })
    }

    pub fn iterator(&self) -> impl Iterator<Item = &OwnedJsonEvent> + '_ {
        self.json_path.iter()
    }

    pub fn len(&self) -> usize {
        self.json_path.len()
    }
}

impl Index<usize> for KeyPath {
    type Output = OwnedJsonEvent;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.json_path[idx]
    }
}

#[cfg(test)]
mod tests {
    use crate::key_path::{KeyPath, OwnedJsonEvent};
    use json_event_parser::JsonEvent;

    #[test]
    fn test_owned_json_event() {
        let e = OwnedJsonEvent::ObjectKey(String::from("key"));
        assert_eq!(e.as_json_event(), JsonEvent::ObjectKey("key"));
        let e = OwnedJsonEvent::String(String::from("string"));
        assert_eq!(e.as_json_event(), JsonEvent::String("string"));
        let e = OwnedJsonEvent::Number(String::from("27"));
        assert_eq!(e.as_json_event(), JsonEvent::Number("27"));
        let e = OwnedJsonEvent::Boolean(true);
        assert_eq!(e.as_json_event(), JsonEvent::Boolean(true));
        assert_eq!(OwnedJsonEvent::Null.as_json_event(), JsonEvent::Null);
        assert_eq!(OwnedJsonEvent::Eof.as_json_event(), JsonEvent::Eof);
        assert_eq!(
            OwnedJsonEvent::StartArray.as_json_event(),
            JsonEvent::StartArray
        );
        assert_eq!(
            OwnedJsonEvent::EndArray.as_json_event(),
            JsonEvent::EndArray
        );
        assert_eq!(
            OwnedJsonEvent::StartObject.as_json_event(),
            JsonEvent::StartObject
        );
        assert_eq!(
            OwnedJsonEvent::EndObject.as_json_event(),
            JsonEvent::EndObject
        );
    }

    #[test]
    fn test_blank_key_path() {
        let key_path = KeyPath::from_kp_str("").unwrap();
        assert_eq!(0, key_path.len())
    }

    #[test]
    fn test_one_element() {
        let key_path = KeyPath::from_kp_str("foo").unwrap();
        assert_eq!(1, key_path.len());
        let event_0 = OwnedJsonEvent::ObjectKey("foo".to_string());
        assert_eq!(event_0, key_path[0]);
    }
}
