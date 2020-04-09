use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Player {
    id: usize,
    steam_id: String,
    name: String,
    team_id: Option<usize>,
    squad_id: Option<usize>,
}

impl Player {
    pub fn new(
        id: usize,
        steam_id: String,
        name: String,
        team_id: Option<usize>,
        squad_id: Option<usize>,
    ) -> Player {
        Player {
            id: id,
            steam_id: steam_id,
            name: name,
            team_id: team_id,
            squad_id: squad_id,
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
    pub fn team_id(&self) -> Option<usize> {
        self.team_id
    }
    pub fn squad_id(&self) -> Option<usize> {
        self.squad_id
    }
}
