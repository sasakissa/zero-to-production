pub mod configurations;
pub mod routes;
pub mod startup;

use std::net::TcpListener;

// lib.rs
use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};
