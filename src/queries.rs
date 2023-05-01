use backend::models;
use tokio_postgres::{Client, Error};

pub const USER_CREATE: &str =
    "INSERT INTO users (email, display_name, display_color, avatar_url) VALUES ($1, $2, $3, $4)";
pub const USER_GET: &str =
    "SELECT id, email, display_name, display_color, avatar_url FROM users WHERE id=$1";
pub const MESSAGE_CREATE: &str = "INSERT INTO messages (sender_id, content) VALUES ($1, $2)";
pub const MESSAGE_GET: &str = "SELECT * FROM messages ORDER BY id DESC LIMIT $1";
pub const MESSAGE_GET_SPECIFIC: &str =
    "SELECT * FROM messages WHERE id<$1 ORDER BY id DESC LIMIT $2";

pub async fn create_user_query(user: models::User, client: Client) -> Result<u64, Error> {
    return client
        .execute(
            USER_CREATE,
            &[
                &user.email,
                &user.display_name,
                &user.display_color,
                &user.avatar_url,
            ],
        )
        .await;
}

pub async fn get_user_query(id: i32, client: Client) -> Result<models::User, Error> {
    let response = client.query_one(USER_GET, &[&id]).await?;

    return Ok(models::User {
        id: response.get("id"),
        email: response.get("email"),
        display_name: response.get("display_name"),
        display_color: response.get("display_color"),
        avatar_url: response.get("avatar_url"),
    });
}

pub async fn create_message_query(data: models::Message, client: Client) -> Result<u64, Error> {
    return client
        .execute(MESSAGE_CREATE, &[&data.sender_id, &data.content])
        .await;
}

pub async fn get_message_query(
    amount: i64,
    starting_from: i32,
    client: Client,
) -> Result<Vec<models::Message>, Error> {
    match starting_from {
        0 => {
            let response = client.query(MESSAGE_GET, &[&amount]).await?;
            let response_message = response
                .iter()
                .map(|row| models::Message {
                    id: row.get("id"),
                    sender_id: row.get("sender_id"),
                    content: row.get("content"),
                })
                .collect();
            return Ok(response_message);
        }
        start => {
            let response = client
                .query(MESSAGE_GET_SPECIFIC, &[&start, &amount])
                .await?;
            let response_message = response
                .iter()
                .map(|row| models::Message {
                    id: row.get("id"),
                    sender_id: row.get("sender_id"),
                    content: row.get("content"),
                })
                .collect();
            return Ok(response_message);
        }
    }
}
