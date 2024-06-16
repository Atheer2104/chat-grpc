use anyhow::anyhow;
use redis::{AsyncCommands, RedisResult};

use crate::server::RedisCon;

#[tracing::instrument(
    name = "Store auth_token into redis"
    skip(redis_connection, auth_token)
)]
pub async fn store_token_redis(
    redis_connection: RedisCon,
    user_id: &i32,
    auth_token: &str,
) -> Result<(), anyhow::Error> {
    let mut redis_con = redis_connection.lock().await;

    let res: RedisResult<()> = redis_con.set(user_id, auth_token).await;

    if res.is_err() {
        return Err(anyhow!("couldn't save auth token in redis"));
    }

    Ok(())
}

#[tracing::instrument(
    name = "get auth_token into redis"
    skip(redis_connection, user_id)
)]
pub async fn get_token_redis(
    redis_connection: RedisCon,
    user_id: &i32,
) -> Result<String, anyhow::Error> {
    let mut redis_con = redis_connection.lock().await;

    let token = match redis_con.get(user_id).await {
        Ok(token) => token,
        Err(_) => return Err(anyhow!("couldn't get auth token in redis")),
    };

    Ok(token)
}
