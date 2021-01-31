pub mod configurations;
pub mod routes;
pub mod startup;
pub mod telemetry;

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};
