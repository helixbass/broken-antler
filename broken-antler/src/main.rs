use std::sync::{mpsc, Arc};
use std::thread;

use sauvignon_axum::{
    axum::{self, Extension},
    simple_app,
};
use tokio::net::TcpListener;
use tracing_chrome::{ChromeLayerBuilder, TraceStyle};
use tracing_subscriber::prelude::*;

use broken_antler::{get_database, get_schema};
use shared::get_db_pool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (chrome_layer, _guard) = ChromeLayerBuilder::new()
        // .trace_style(TraceStyle::Async)
        .build();
    tracing_subscriber::registry().with(chrome_layer).init();

    // let (trace_restarter_sender, trace_restarter_receiver) = mpsc::channel();
    // let trace_restarter_handle = thread::spawn(move || {
    //     for _message in trace_restarter_receiver.iter() {
    //         guard.start_new(None);
    //     }
    // });

    let schema = get_schema();
    let db_pool = get_db_pool().await?;
    let database = get_database(&db_pool).await;

    axum::serve(
        TcpListener::bind("0.0.0.0:3002").await?,
        simple_app(Arc::new(schema), Arc::new(database)), // .layer(Extension(Arc::new(guard))),
    )
    .await?;

    Ok(())
}
