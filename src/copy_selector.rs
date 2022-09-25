use crate::copy_selector::TargetPlacement::BeforeTarget;
use crate::key_path::{KeyPath, OwnedJsonEvent};
use eyre::{eyre, Result};
use json_event_parser::JsonEvent;

enum TargetPlacement {
    BeforeTarget,
    InsideTarget,
    AfterTarget,
}
// 32
struct JsonFileState {
    keys: KeyPath,
    target_placement: TargetPlacement,
    target_index: usize,
    keys_index: usize,
    sub_elements: usize,
}

impl JsonFileState {
    fn new(keys: KeyPath) -> Self {
        Self {
            keys,
            target_placement: BeforeTarget,
            target_index: 0,
            keys_index: 0,
            sub_elements: 0,
        }
    }

    fn in_sub_element(&self) -> bool {
        self.sub_elements != 0
    }

    fn next_element(&mut self, event: &JsonEvent) {
        match event {
            JsonEvent::StartArray | JsonEvent::StartObject => self.sub_elements += 1,
            JsonEvent::EndArray | JsonEvent::EndObject => self.sub_elements -= 1,
            _ => {}
        }
        if self.sub_elements == 0 {
            self.target_index += 1;
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
    count: usize,
    skip: usize,
    no_context: bool,
    json_file_state: JsonFileState,
}

impl CopySelector {
    pub fn new(keys: KeyPath, count: usize, skip: usize, no_context: bool) -> Self {
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
                    if current_key.as_json_event() == event {
                        let _ = state.next_key();
                        return Ok(true);
                    }
                } else if event == JsonEvent::StartArray {
                    state.target_placement = TargetPlacement::InsideTarget;
                    return Ok(true);
                } else {
                    return Err(eyre!("Expecting Json array, found {event:?}"));
                }
                Ok(allow_context)
            }
            TargetPlacement::InsideTarget => {
                if event == JsonEvent::EndArray && !state.in_sub_element() {
                    state.target_placement = TargetPlacement::AfterTarget;
                } else {
                    // Perform the skip logic
                    let index = state.target_index;
                    let skipping = index < self.skip || index >= (self.count + self.skip);
                    state.next_element(&event);
                    return Ok(!skipping);
                }
                Ok(true)
            }
            TargetPlacement::AfterTarget => Ok(allow_context),
        }
    }
}
