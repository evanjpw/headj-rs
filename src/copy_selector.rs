use crate::copy_selector::TargetPlacement::BeforeTarget;
use crate::key_path::{KeyPath, OwnedJsonEvent};
use eyre::{eyre, Result};
use json_event_parser::JsonEvent;

enum TargetPlacement {
    BeforeTarget,
    InsideTarget,
    AfterTarget,
}

struct JsonFileState {
    keys: KeyPath,
    target_placement: TargetPlacement,
    target_index: u32,
    keys_index: usize,
}

impl JsonFileState {
    fn new(keys: KeyPath) -> Self {
        Self {
            keys,
            target_placement: BeforeTarget,
            target_index: 0,
            keys_index: 0,
        }
    }

    pub fn current_key(&self) -> Option<OwnedJsonEvent> {
        let index = self.keys_index;
        if index < self.keys.len() {
            Some(self.keys[index].clone())
        } else {
            None
        }
    }

    pub fn next_key(&mut self) -> Option<OwnedJsonEvent> {
        if self.keys_index < self.keys.len() {
            self.keys_index += 1;
            self.current_key()
        } else {
            None
        }
    }
}

pub struct CopySelector {
    count: u32,
    skip: u32,
    no_context: bool,
    json_file_state: JsonFileState,
}

impl CopySelector {
    pub fn new(keys: KeyPath, count: u32, skip: u32, no_context: bool) -> Self {
        let json_file_state = JsonFileState::new(keys);
        Self {
            count,
            skip,
            no_context,
            json_file_state,
        }
    }

    pub fn select(&mut self, event: JsonEvent) -> Result<bool> {
        let state = &mut self.json_file_state;
        let allow_context = !self.no_context;
        match &state.target_placement {
            BeforeTarget => {
                if let Some(current_key) = state.current_key() {
                    // See if we're at a key that matches the current key
                } else if event == JsonEvent::StartArray {
                    state.target_placement = TargetPlacement::InsideTarget;
                    return Ok(true);
                } else {
                    return Err(eyre!("Expecting Json array, found {event}"));
                }
                Ok(allow_context)
            }
            InsideTarget => {
                // Perform the skip logic
                Ok(true)
            }
            AfterTarget => Ok(allow_context),
        }
    }
}
