use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Msg {
    Ping,
    Pong,
    RegistorClient(ClientInfo),
    UnRegistorClient(ClientInfo),
    SessionClients(HashSet<Uuid>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientInfo {
    pub session_uuid: Uuid,
    pub client_uuid: Uuid,
    pub name: String,
}
