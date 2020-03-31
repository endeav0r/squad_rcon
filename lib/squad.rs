use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Squad {
    id: usize,
    name: String,
    size: usize,
    team_id: usize,
    locked: bool,
}

impl Squad {
    pub fn new(id: usize, name: String, size: usize, team_id: usize, locked: bool) -> Squad {
        Squad {
            id: id,
            name: name,
            size: size,
            team_id: team_id,
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
    pub fn team_id(&self) -> usize {
        self.team_id
    }
    pub fn locked(&self) -> bool {
        self.locked
    }
}
