use crate::domains::rooms::repository::PresenceRepository;
use redis::{AsyncCommands, RedisResult};

pub struct RedisPresenceRepository {
    pub conn: redis::aio::MultiplexedConnection,
}

#[async_trait::async_trait]
impl PresenceRepository for RedisPresenceRepository {
    async fn get_participant_count(&self, room_id: &str) -> RedisResult<usize> {
        let mut conn = self.conn.clone();
        let count = conn.scard(
            Self::presence_key(room_id)
        ).await?;
        Ok(count)
    }

    async fn add_participant(&self, room_id: &str, user_id: &str) -> RedisResult<usize> {
        let mut conn = self.conn.clone();
        conn
            .sadd(format!("room:{room_id}:presence"), user_id)
            .await
    }

    async fn remove_participant(&self, room_id: &str, user_id: &str) -> RedisResult<usize> {
       let mut conn = self.conn.clone();
        conn
            .srem(format!("room:{room_id}:presence"), user_id)
            .await
    }
}

impl RedisPresenceRepository {
    fn presence_key(room_id: &str) -> String {
        format!("room:{room_id}:presence")
    }
}
