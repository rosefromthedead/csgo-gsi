use std::path::PathBuf;
use std::sync::mpsc;

use fehler::throws;
use gotham::handler::HandlerError;
use gotham::helpers::http::response::create_empty_response;
use gotham::hyper::{body, Body, Response, StatusCode};
use gotham::middleware::state::StateMiddleware;
use gotham::pipeline::single::single_pipeline;
use gotham::pipeline::single_middleware;
use gotham::router::builder::{DefineSingleRoute, build_router};
use gotham::router::builder::DrawRoutes;
use gotham::router::Router;
use gotham::state::{State, FromState};

use crate::{GSIConfig, Error, install_dir, update};

pub struct GSIServer {
    port: u16,
    config: GSIConfig,
    installed: bool,
    listeners: Vec<Box<dyn FnMut(&update::Update)>>,
}

impl GSIServer {
    pub fn new(config: GSIConfig, port: u16) -> Self {
        Self {
            port,
            config,
            installed: false,
            listeners: vec![],
        }
    }

    #[throws]
    pub fn install_into<P: Into<PathBuf>>(&mut self, cfg_folder: P) {
        self.config.install_into(cfg_folder, self.port)?;
        self.installed = true;
    }

    #[throws]
    pub fn install(&mut self) {
        self.install_into(install_dir::discover_cfg_folder()?)?;
    }

    pub fn add_listener<F: 'static + FnMut(&update::Update)>(&mut self, listener: F) {
        self.listeners.push(Box::new(listener));
    }

    #[throws]
    pub async fn run(mut self) {
        if !self.installed {
            self.install()?;
        }

        let (tx, rx) = mpsc::sync_channel(128);

        let port = self.port;
        tokio::spawn(gotham::init_server(("127.0.0.1", port), router(tx)));

        for update in rx {
            for callback in &mut self.listeners {
                callback(&update)
            }
        }
    }
}

#[derive(Clone, StateData)]
struct UpdateHandler {
    inner: mpsc::SyncSender<update::Update>,
}

impl UpdateHandler {
    fn new(tx: &mpsc::SyncSender<update::Update>) -> Self {
        Self {
            inner: tx.clone()
        }
    }

    fn send(&self, update: update::Update) {
        self.inner.send(update).expect("failed to send update back to main thread");
    }
}

#[throws((State, HandlerError))]
pub async fn handle_update(mut state: State) -> (State, Response<Body>) {
    let body = state.try_take::<Body>();
    let body = match body {
        Some(body) => body,
        None => {
            let response = create_empty_response(&state, StatusCode::BAD_REQUEST);
            return (state, response);
        }
    };
    let body = body::to_bytes(body).await;
    let body = match body {
        Ok(body) => body,
        Err(err) => {
            eprintln!("{}", err);
            let response = create_empty_response(&state, StatusCode::INTERNAL_SERVER_ERROR);
            return (state, response);
        }
    };
    let json_value = serde_json::from_slice::<serde_json::Value>(body.as_ref());
    let json_value = match json_value {
        Ok(json_value) => json_value,
        Err(err) => {
            println!("JSON parsing error: {}", err);
            if let Ok(data) = ::std::str::from_utf8(body.as_ref()) {
                println!("{}\n", data);
            }
            let response = create_empty_response(&state, StatusCode::INTERNAL_SERVER_ERROR);
            return (state, response);
        }
    };
    let data = serde_json::from_value::<update::Update>(json_value);
    let data = match data {
        Ok(data) => data,
        Err(err) => {
            println!("Update parsing error: {}", err);
            if let Ok(data) = ::std::str::from_utf8(body.as_ref()) {
                println!("{}\n", data);
            }
            let response = create_empty_response(&state, StatusCode::INTERNAL_SERVER_ERROR);
            return (state, response);
        }
    };
    {
        let update_handler = UpdateHandler::borrow_from(&state);
        update_handler.send(data);
    }
    let response = create_empty_response(&state, StatusCode::OK);
    (state, response)
}

fn router(tx: mpsc::SyncSender<update::Update>) -> Router {
    let update_handler = UpdateHandler::new(&tx);

    let middleware = StateMiddleware::new(update_handler);
    let pipeline = single_middleware(middleware);
    let (chain, pipelines) = single_pipeline(pipeline);

    build_router(chain, pipelines, |route| {
        route
            .post("/")
            .to_async(handle_update);
    })
}
