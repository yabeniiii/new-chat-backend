use backend::models;
use tokio_postgres::{Client, Error};

pub const USER_CREATE: &str =
    "INSERT INTO users (email, display_name, display_color, avatar_url) VALUES ($1, $2, $3, $4)";
pub const USER_GET: &str =
    "SELECT user_id, email, display_name, display_color, avatar_url FROM users WHERE id=$1";
pub const MESSAGE_CREATE: &str = "INSERT INTO messages (sender_id, content) VALUES ($1, $2)";
pub const MESSAGE_GET: &str = "SELECT * FROM messages JOIN users on messages.sender_id=users.user_id ORDER BY messages.message_id DESC LIMIT $1 OFFSET $2";

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
        .execute(MESSAGE_CREATE, &[&data.sender.id, &data.content])
        .await;
}

pub async fn get_message_query(
    amount: i64,
    starting_from: i64,
    client: Client,
) -> Result<Vec<models::Message>, Error> {
    let db_response = client
        .query(MESSAGE_GET, &[&amount, &starting_from])
        .await?;
    let response = db_response
        .iter()
        .rev()
        .map(|row| models::Message {
            id: row.get("message_id"),
            sender: models::User {
                id: row.get("user_id"),
                email: row.get("email"),
                display_name: row.get("display_name"),
                display_color: row.get("display_color"),
                avatar_url: row.get("avatar_url"),
            },
            content: row.get("content"),
        })
        .collect();
    return Ok(response);
}
