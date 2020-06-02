extern crate chrono;
extern crate env_logger;
extern crate iron;
extern crate logger;
extern crate router;
extern crate rustc_serialize;
extern crate uuid;

mod database;
mod models;
mod handlers;

use models::*;
use database::Database;
use handlers::*;

use iron::prelude::Chain;
use iron::Iron;
use router::Router;
use logger::Logger;
use uuid::Uuid;

fn main() {
    env_logger::init();
    let (logger_before, logger_after) = Logger::new(None);

    let mut db = Database::new();
    let p = Post::new(
        "the first Post",
        "this is the in our API",
        "osama",
        chrono::offset::Utc::now(),
        Uuid::nil(),
    );
    db.add_post(p);

    let p2 = Post::new(
        "the next Post is better",
        "Iron is really cool and rust is awsome",
        "matalman",
        chrono::offset::Utc::now(),
        Uuid::nil(),
    );
    db.add_post(p2);

    let handlers = Handlers::new(db);
    let json_content_middleware = JsonAfterMiddleware;

    let mut router = Router::new();
    router.get("/post_feed", handlers.post_feed, "post feed");
    router.post("/post", handlers.post_post, "post post");
    router.get("/post/:id", handlers.post, "post");

    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_after(json_content_middleware);
    chain.link_after(logger_after);

    Iron::new(chain).http("localhost:8000").unwrap();
    
}
