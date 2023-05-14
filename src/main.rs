mod queries;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use backend::models;
use serde_json;
use tokio;
use tokio_postgres::NoTls;

#[get("/message/get/{number}/{starting}")]
async fn get_messages(data: web::Path<(i64, i32)>) -> impl Responder {
    let (number, starting_from) = data.into_inner();
    let (client, connection) = match tokio_postgres::connect(
        "host=localhost 
        dbname=chat_app 
        user=aidanboland",
        NoTls,
    )
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
    match queries::get_message_query(number, starting_from, client).await {
        Ok(response_vec) => {
            let json = match serde_json::to_string(&response_vec) {
                Ok(json) => json,
                Err(err) => return HttpResponse::InternalServerError().body(format!("{}", err)),
            };
            return HttpResponse::Ok().body(format!("{}", json));
        }
        Err(err) => return HttpResponse::InternalServerError().body(format!("{}", err)),
    }
}

#[post("/message/post")]
async fn create_message(data: web::Json<models::Message>) -> impl Responder {
    let (client, connection) = match tokio_postgres::connect(
        "host=localhost 
        dbname=chat_app 
        user=aidanboland",
        NoTls,
    )
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

    match queries::create_message_query(data.into_inner(), client).await {
        Ok(_) => return HttpResponse::Ok().body("success"),
        Err(err) => return HttpResponse::InternalServerError().body(format!("{}", err)),
    };
}

#[post("/user/create")]
async fn create_user(user: web::Json<models::User>) -> impl Responder {
    let (client, connection) = match tokio_postgres::connect(
        "host=localhost 
        dbname=chat_app 
        user=aidanboland",
        NoTls,
    )
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

    let db_response = match queries::create_user_query(user.into_inner(), client).await {
        Ok(rows_changed) => rows_changed,
        Err(err) => return HttpResponse::InternalServerError().body(format!("{}", err)),
    };

    return HttpResponse::Ok().body(format!("lol {}", db_response));
}

#[get("/user/{user_id}")]
async fn get_user(id: web::Path<i32>) -> impl Responder {
    let (client, connection) = match tokio_postgres::connect(
        "host=localhost 
        dbname=chat_app 
        user=aidanboland",
        NoTls,
    )
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
    let db_response = match queries::get_user_query(id.into_inner(), client).await {
        Ok(response) => response,
        Err(err) => return HttpResponse::InternalServerError().body(format!("{}", err)),
    };
    return HttpResponse::Ok().body(format!("{:?}", db_response));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("started server at http://localhost:8080/");
    HttpServer::new(|| {
        App::new()
            .service(create_user)
            .service(get_user)
            .service(create_message)
            .service(get_messages)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
