use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Team {
    id: usize,
    name: String,
}

impl Team {
    pub fn new(id: usize, name: String) -> Team {
        Team { id: id, name: name }
    }

    pub fn id(&self) -> usize {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}
