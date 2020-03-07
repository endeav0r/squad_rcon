use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Squad {
    id: usize,
    name: String,
    size: usize,
    team: usize,
    locked: bool,
}

impl Squad {
    pub fn new(id: usize, name: String, size: usize, team: usize, locked: bool) -> Squad {
        Squad {
            id: id,
            name: name,
            size: size,
            team: team,
            locked: locked,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn team(&self) -> usize {
        self.team
    }
    pub fn locked(&self) -> bool {
        self.locked
    }
}
