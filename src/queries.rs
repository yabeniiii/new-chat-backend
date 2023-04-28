use backend::models::User;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::{Client, Error};

pub const USER_CREATE: &str =
    "INSERT INTO users (email, display_name, display_color, avatar_url) VALUES ($1, $2, $3, $4)";
pub const USER_GET: &str =
    "SELECT id, email, display_name, display_color, avatar_url FROM users WHERE id=$1";

pub async fn create_user_query(user: User, mut client: Client) -> Result<u64, Error> {
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

pub async fn get_user_query(id: i32, client: Client) -> Result<User, Error> {
    let response = client.query_one(USER_GET, &[&id]).await?;

    return Ok(User {
        id: response.get("id"),
        email: response.get("email"),
        display_name: response.get("display_name"),
        display_color: response.get("display_color"),
        avatar_url: response.get("avatar_url"),
    });
}

pub fn create_message_query() {}
