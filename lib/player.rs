use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Player {
    id: usize,
    steam_id: String,
    name: String,
    team: usize,
    squad: Option<usize>,
}

impl Player {
    pub fn new(
        id: usize,
        steam_id: String,
        name: String,
        team: usize,
        squad: Option<usize>,
    ) -> Player {
        Player {
            id: id,
            steam_id: steam_id,
            name: name,
            team: team,
            squad: squad,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }
    pub fn steam_id(&self) -> &str {
        &self.steam_id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn team(&self) -> usize {
        self.team.clone()
    }
    pub fn squad(&self) -> Option<usize> {
        self.squad
    }
}
