use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};
// use sqlx::PgPool;
// use dotenv::dotenv;

// #[get("/")]
// async fn index(pool: web::Data<PgPool>) -> impl Responder {
//     let restaurants = sqlx::query!(
//         r#"
//         SELECT * FROM pending_restaurants
//         "#
//     )
//     .fetch_all(pool.get_ref())
//     .await
//     .expect("Failed to fetch restaurants");

//     let mut response = String::new();
//     for restaurant in restaurants {
//         let line = format!(
//             "Id: {}, Name: {}, Information: {}, IsApproved: {}\n",
//             restaurant.id, restaurant.name, restaurant.information, restaurant.isapproved
//         );
//         response.push_str(&line);
//     }
//     HttpResponse::Ok().body(response)
// }

#[get("/again")]
async fn again() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // dotenv().ok();
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let db_pool = PgPool::connect(&database_url).await.expect("Failed to create pool.");

    log::info!("Starting HTTP server: go to http://localhost:7779");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            // .app_data(web::Data::new(db_pool.clone()))
            // .service(index)
            .service(again)
    })
    .bind(("0.0.0.0", 7779))?
    .run()
    .await
}