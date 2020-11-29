///////////////////////////////////////////////////////////////////////////////////////////////////
// Main Webserver Junk

#[path = "config.rs"]
mod config;

#[path = "handlers.rs"]
mod handlers;

use actix_web::{ 
    web,
    App,
    HttpServer,
    middleware
};

use dotenv::dotenv;

#[actix_web::main]
async fn main() -> Result<()> {
    // starting our logger...
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    
    // getting our .env settings
    let cfg = config::Config::from_env().unwrap(); 

    // I don't know if I like these routes... 
    // Should these be version specific or maybe nah?
    // TODO(XVI): add a match for the payload in each route to check PX vers
    let server = HttpServer::new( move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/px2").route(web::post()
                .to(handlers::px_generators::genpx2)
                )
            )
            .service(web::resource("/px3").route(web::post()
                .to(handlers::px_generators::genpx3)
                )
            )
    })
    .bind(cfg.api_addr.clone())?
    .run();
    server.await
}

