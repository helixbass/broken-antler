use std::fmt;
use std::sync::{mpsc::Sender, Arc};
use std::task::{Context, Poll};

use sauvignon_axum::{
    axum::{self, extract::Request, middleware::Next, response::Response},
    simple_app,
};
use tokio::net::TcpListener;
use tower::{Layer, Service};

use broken_antler::{get_database, get_schema};
use shared::get_db_pool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let (chrome_layer, guard) = ChromeLayerBuilder::new().build();
    // tracing_subscriber::registry()
    //     .with(chrome_layer)
    //     .with(EnvFilter::new("debug"))
    //     .init();

    // let (trace_restarter_sender, trace_restarter_receiver) = mpsc::channel::<()>();
    // let _trace_restarter_handle = thread::spawn(move || {
    //     for _message in trace_restarter_receiver.iter() {
    //         guard.start_new(None);
    //     }
    // });

    let schema = get_schema();
    let db_pool = get_db_pool().await?;
    let database = get_database(&db_pool).await;

    axum::serve(
        TcpListener::bind("0.0.0.0:3002").await?,
        simple_app(Arc::new(schema), Arc::new(database)), // .layer(RestartTraceLayer {
                                                          //     sender: trace_restarter_sender,
                                                          // }),
    )
    .await?;

    Ok(())
}

#[allow(dead_code)]
#[derive(Clone)]
struct RestartTraceLayer {
    pub sender: Sender<()>,
}

impl<TNested> Layer<TNested> for RestartTraceLayer {
    type Service = RestartTraceService<TNested>;

    fn layer(&self, service: TNested) -> Self::Service {
        RestartTraceService {
            service,
            sender: self.sender.clone(),
        }
    }
}

#[derive(Clone)]
struct RestartTraceService<TNested> {
    pub sender: Sender<()>,
    pub service: TNested,
}

impl<TNested, TRequest> Service<TRequest> for RestartTraceService<TNested>
where
    TNested: Service<TRequest>,
    TRequest: fmt::Debug,
{
    type Response = TNested::Response;
    type Error = TNested::Error;
    type Future = TNested::Future;

    fn poll_ready(&mut self, context: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(context)
    }

    fn call(&mut self, request: TRequest) -> Self::Future {
        self.sender.send(()).unwrap();
        self.service.call(request)
    }
}

#[allow(dead_code)]
async fn restart_trace_middleware(request: Request, next: Next) -> Response {
    let response = next.run(request).await;

    response
}
