pub mod models {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct User {
        pub id: i32,
        pub email: String,
        pub display_name: String,
        pub display_color: Option<String>,
        pub avatar_url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct IncomingUser {
        pub email: String,
        pub display_name: String,
        pub display_color: Option<String>,
        pub avatar_url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Message {
        pub id: i32,
        pub sender: User,
        pub content: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct IncomingMessage {
        pub sender_id: i32,
        pub content: String,
    }
}
pub mod queries {
    use crate::models;
    use tokio_postgres::{Client, Error};

    pub const USER_CREATE: &str =
    "INSERT INTO users (email, display_name, display_color, avatar_url) VALUES ($1, $2, $3, $4)";
    pub const USER_GET: &str =
        "SELECT user_id, email, display_name, display_color, avatar_url FROM users WHERE id=$1";
    pub const MESSAGE_CREATE: &str = "INSERT INTO messages (sender_id, content) VALUES ($1, $2)";
    pub const MESSAGE_GET: &str = "SELECT * FROM messages JOIN users on messages.sender_id=users.user_id WHERE messages.message_id < $1 ORDER BY messages.message_id DESC LIMIT $2";
    pub const MESSAGE_GET_START: &str = "SELECT * FROM messages JOIN users on messages.sender_id=users.user_id ORDER BY messages.message_id DESC LIMIT $1";

    pub async fn create_user_query(
        user: models::IncomingUser,
        client: Client,
    ) -> Result<u64, Error> {
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

    pub async fn create_message_query(
        data: models::IncomingMessage,
        client: Client,
    ) -> Result<u64, Error> {
        return client
            .execute(MESSAGE_CREATE, &[&data.sender_id, &data.content])
            .await;
    }

    pub async fn get_message_query(
        amount: i64,
        starting_from: i32,
        client: Client,
    ) -> Result<Vec<models::Message>, Error> {
        if starting_from == 0 {
            let db_response = client.query(MESSAGE_GET_START, &[&amount]).await?;
            let response = db_response
                .iter()
                // .rev()
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
        let db_response = client
            .query(MESSAGE_GET, &[&starting_from, &amount])
            .await?;
        let response = db_response
            .iter()
            // .rev()
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
}
