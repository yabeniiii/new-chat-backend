mod queries;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use backend::models;
// use serde_json;
use tokio;
use tokio_postgres::NoTls;

#[get("/message/get/{number}/{starting}")]
async fn get_messages(data: web::Path<(i64, i64)>) -> impl Responder {
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
            let mut html: String = String::new();
            response_vec.iter().for_each(|message| {
                let message_sender_display_color = match &message.sender.display_color {
                    Some(color) => color.clone(),
                    None => String::from("ffffff")
                };
                let message_sender_avatar_url = match &message.sender.avatar_url {
                    Some(url) => url.clone(),
                    None => String::from("none")
                };
                html.push_str(format!("
                        <div class='message-box'>
                            <div class='avatar' style='background-image: {message_sender_avatar}' />
                            <div class='message-content'>
                                <div class='message-bar'>
                                    <h1 class='display-name' style='color: {message_sender_color}'>{message_sender_name}</h1>
                                    <p class='message-info'>usr_id: {message_sender_id} msg_id: {message_id}</p>
                                </div>
                                <p class='message-text'>{message_content}</p>
                            </div>
                        </div>
                    ", message_sender_avatar = message_sender_avatar_url, message_sender_color = message_sender_display_color, message_sender_name = message.sender.display_name, message_sender_id = message.sender.id, message_id = message.id, message_content = message.content).as_str());
            });
            return HttpResponse::Ok().body(html);
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
