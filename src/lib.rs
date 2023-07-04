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
    pub struct Message {
        pub id: i32,
        pub sender: User,
        pub content: String,
    }
}
