pub mod models {
    use serde::{Deserialize, Serialize};
    use tokio_pg_mapper::FromTokioPostgresRow;
    use tokio_pg_mapper_derive::PostgresMapper;

    #[derive(Serialize, Deserialize, Debug, PostgresMapper)]
    #[pg_mapper(table = "users")]
    pub struct User {
        pub id: Option<i32>,
        pub email: String,
        pub display_name: String,
        pub display_color: Option<String>,
        pub avatar_url: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Message {
        pub id: Option<i32>,
        pub sender: User,
        pub content: String,
    }
}
