use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::Mutex;

#[path = "../handlers.rs"]
mod handlers;
#[path = "../state.rs"]
mod state;
#[path ="../routers.rs"]
mod routers;

fn main() {}