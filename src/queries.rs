use backend::models::User;
use tokio_postgres::{Client, Error};

pub const USER_CREATE: &str =
    "INSERT INTO users (email, display_name, display_color, avatar_url) VALUES ($1, $2, $3, $4)";
pub const USER_GET: &str = "SELECT id, email, display_name FROM users";

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

pub fn get_user_query() {}

pub fn create_message_query() {}
