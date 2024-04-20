use askama::Template;
use axum::{
    routing::{get, get_service},
    Router,
};
use tower_http::services::{ServeDir, ServeFile};

pub type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[tokio::main]
async fn main() -> Result {
    let app = Router::new()
        .route("/", get(home))
        .route("/contact", get(contact))
        .nest_service("/blog", ServeFile::new("templates/truesay.html"))
        .route("/project", get(projects))
        .nest_service("/static", ServeDir::new("static"))
        .nest_service("/blogs", ServeDir::new("blog"));

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

#[derive(Template)]
#[template(path = "projects.html")]
struct Project {
    meow: usize,
}

async fn projects() -> Project {
    Project { meow: 4 }
}
