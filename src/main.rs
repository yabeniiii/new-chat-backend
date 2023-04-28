mod queries;

use std::fmt::format;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use backend::models;
use tokio;
use tokio_postgres::{Error, NoTls};

#[post("/user/create")]
async fn index(user: web::Json<models::User>) -> impl Responder {
    // let user = models::User {
    //     id: None,
    //     email: format!("nut@nut.nut"),
    //     display_name: format!("nut"),
    //     display_color: None,
    //     avatar_url: None,
    // };

    // Connect to the database.
    let (client, connection) =
        match tokio_postgres::connect("host=localhost dbname=chat_app user=aidanboland", NoTls)
            .await
        {
            Ok(client) => client,
            Err(err) => return HttpResponse::InternalServerError().body(format!("{}", err)),
        };

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let db_response = match queries::create_user_query(user.into_inner(), client).await {
        Ok(rows_changed) => rows_changed,
        Err(err) => return HttpResponse::InternalServerError().body(format!("{}", err)),
    };

    return HttpResponse::Ok().body(format!("lol {}", db_response));
}

#[get("/user/{user_id}")]
async fn get_user(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let (client, connection) =
        match tokio_postgres::connect("host=localhost dbname=chat_app user=aidanboland", NoTls)
            .await
        {
            Ok(client) => client,
            Err(err) => return HttpResponse::InternalServerError().body(format!("{}", err)),
        };
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let db_response = match queries::get_user_query(id, client).await {
        Ok(response) => response,
        Err(err) => return HttpResponse::InternalServerError().body(format!("{}", err)),
    };
    return HttpResponse::Ok().body(format!("{:?}", db_response));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(get_user))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
