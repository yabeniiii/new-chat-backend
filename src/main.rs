mod queries;

use backend::User;
//#[macro_use]
//extern crate rocket;
use postgres::{Client, NoTls};

//#[get("/")]
//fn index() -> String {
//   let mut client = match Client::connect("host=localhost user=aidanboland", NoTls) {
//      Ok(client) => format!("hey chat!"),
//     Err(err) => {
//        format!("ERROR: database connection error: {}", err)
//   }
// };
//return client;
//}

//#[launch]
//fn rocket() -> _ {
//   rocket::build().mount("/", routes![index])
//}
fn main() {
    let mut client = match Client::connect("host=localhost dbname=chat_app user=aidanboland", NoTls)
    {
        Ok(client) => client,
        Err(err) => panic!("ERROR: error connecting to database: {}", err),
    };

    let mut response_vec: Vec<User> = Vec::new();

    let response = match client.query(queries::USER_GET, &[]) {
        Ok(res) => res,
        Err(err) => panic!("ERROR: error querying database: {}", err),
    };
    for row in response.iter() {
        response_vec.push(User {
            id: row.get(0),
            email: row.get(1),
            display_name: row.get(2),
            display_color: None,
            avatar_url: None,
        })
    }

    let uuhhh = match response_vec.get(0) {
        Some(val) => val,
        None => panic!("ERROR: no users in database"),
    };

    println!("{:?}, {} {}", uuhhh.id, uuhhh.email, uuhhh.display_name);
}
