mod grads;
mod users;

mod endpoints;
mod persons;

#[macro_use]
extern crate log;

use std::env;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use dotenvy::dotenv;

use axum::{Extension, Router};

use tokio::net::TcpListener;

use color_eyre;
use color_eyre::eyre::Context;
use color_eyre::Report;

use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Surreal;

use async_once::AsyncOnce;

use lazy_static::lazy_static;

lazy_static! {
    static ref DB: AsyncOnce<Surreal<Client>> = {
        AsyncOnce::new(async {
            let db: Surreal<Client> = Surreal::new::<Ws>("127.0.0.1:8000")
                .await
                .expect("couldn't connect to surrealdb");

            db.use_ns("test")
                .use_db("test")
                .await
                .expect("could not use ns and db");

            db
        })
    };
}

#[tokio::main]
async fn main() {
    color_eyre::install()
        .context("Could not start color-eyre tracing")
        .unwrap();

    dotenv().context("Missing .env file").unwrap();
    let port = env::var("PORT")
        .map(|p| p.parse().expect("Invalid 'PORT' in .env file"))
        .unwrap();

    let app = Router::new().nest("/", endpoints::extras_router());

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let tcp_listener = TcpListener::bind(addr)
        .await
        .context("Could not start server")
        .unwrap();

    info!("Initializing server at {}", &addr);

    axum::serve(
        tcp_listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .context("Could not start server")
    .unwrap();
}
