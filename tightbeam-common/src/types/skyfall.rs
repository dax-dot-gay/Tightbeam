use std::collections::HashMap;

use iroh::RelayUrl;
use oqs::{kem, sig};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// Profile information, shared with trusted peers
#[derive(Clone, Debug, Serialize, Deserialize, specta::Type)]
pub struct ProfileDef {
    /// Profile ID
    pub id: Uuid,

    /// Profile display name
    pub name: String,

    /// Load-bearing pronouns
    pub pronouns: Vec<String>,

    /// About blurb
    pub about: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, specta::Type)]
pub struct PeerInfoDef {
    pub identity: PublicIdentityDef,
    pub active_profile: Option<Uuid>,
    pub profiles: HashMap<Uuid, ProfileDef>,
    pub routes: HashMap<String, RouteDef>,
}

#[derive(Clone, Debug, Serialize, Deserialize, specta::Type)]
#[serde(rename = "snake_case")]
pub enum PeerDef {
    Trusted(PeerInfoDef),
    Untrusted(PeerInfoDef),
}

/// Client events
#[derive(Clone, Debug, Serialize, Deserialize, specta::Type)]
#[serde(rename = "snake_case", tag = "event")]
pub enum ClientEventDef {
    Connected {
        peer: PeerDef,
    },
    UpdatedPeer {
        peer: PeerDef,
        identity: PublicIdentityDef,
        active_profile: Option<Uuid>,
        profiles: HashMap<Uuid, ProfileDef>,
        routes: HashMap<String, RouteDef>,
    },
    CommandFailure {
        reason: String,
        route: RouteDef,
        path: String,
        segments: Vec<(String, String)>,
        data: Option<Value>,
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug, specta::Type)]
pub struct PublicIdentityDef {
    pub id: String,
    pub identifier: String,
    #[specta(type = String)]
    pub node: iroh::NodeId,
    #[specta(type = String)]
    pub encryption: kem::PublicKey,
    #[specta(type = String)]
    pub signing: sig::PublicKey,
    #[specta(type = String)]
    pub relay: Option<RelayUrl>,
}

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type)]
pub struct RouteDef {
    pub selector: String,
    pub path: String,
    pub data: bool,
    pub stream: bool,
    pub about: Option<String>
}