use std::collections::HashMap;

use askama::Template;
use axum::{
    routing::{get, get_service},
    Router,
};
use tower_http::services::ServeDir;

pub type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[tokio::main]
async fn main() -> Result {
    let app = Router::new()
        .route("/", get(home))
        .route("/contact", get(contact))
        // .route("/blog", get(blog))
        .route("/blog/:id", get_service(ServeDir::new("html")))
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("listening at yeah");
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Template)]
#[template(path = "home.html")]
struct Home {
    meow: usize,
}

async fn home() -> Home {
    Home { meow: 4 }
}

#[derive(Template)]
#[template(path = "contact.html")]
struct Contact {
    meow: usize,
}

async fn contact() -> Contact {
    Contact { meow: 4 }
}

// #[derive(Template)]
// #[template(path = "projects.html")]
// struct Blog {
//     posts: HashMap<String, String>,
// }

// #[derive(Deserialize)]
// struct Frontmatter {
//     title: String,
//     blurb: String,
// }

// fn make_markdown

// #[derive(Template)]
// #[template(path = "blog.html")]
// struct Page {
// content: String,
// }
