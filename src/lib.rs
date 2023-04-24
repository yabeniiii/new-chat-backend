pub struct User {
    pub id: i32,
    pub email: String,
    pub display_name: String,
    pub display_color: Option<String>,
    pub avatar_url: Option<String>,
}

pub struct Message {
    pub id: i32,
    pub sender: User,
    pub content: String,
}

// async fn get_recent_messages() {}
