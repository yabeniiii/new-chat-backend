pub struct User {
    id: i32,
    email: String,
    display_name: String,
    display_color: Option<String>,
    avatar_url: Option<String>,
    messages: Vec<Message>,
}

pub struct Message {
    id: i32,
    sender: User,
    contend: String,
}

async fn get_recent_messages() {}
