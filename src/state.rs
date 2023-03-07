use serde_tuple::{Deserialize_tuple, Serialize_tuple};

/// The state object.
#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug)]
pub struct State {
    pub count: u64,
}

impl State {
    pub fn increment(&mut self) {
        self.count += 1;
    }
}
