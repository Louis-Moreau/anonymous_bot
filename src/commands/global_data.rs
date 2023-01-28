use serenity::prelude::RwLock;
use serenity::prelude::TypeMapKey;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct VoteData {
    pub question: String,
    pub vote_options: Vec<String>,
    pub message_id: u64,
    pub votes: HashMap<u64, usize>,
}

pub struct Vote;
impl TypeMapKey for Vote {
    type Value = Arc<RwLock<VoteData>>;
}

pub struct AnonymousChannelId;
impl TypeMapKey for AnonymousChannelId {
    type Value = Arc<RwLock<u64>>;
}
