use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusVersion {
    pub name: String,
    pub protocol: usize
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusPlayer {
    pub name: String,
    pub id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusPlayers {
    pub max: usize,
    pub online: usize,
    pub sample: Vec<StatusPlayer>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusDescription {
    pub text: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub version: StatusVersion,
    pub players: StatusPlayers,
    pub description: StatusDescription
}
