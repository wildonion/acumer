


use std::collections::HashMap;
use crate::error;

pub type AcumerHttpResponse = Result<actix_web::HttpResponse, error::AcumerErrorResponse>;
pub type RamDb = std::sync::Arc<tokio::sync::Mutex<HashMap<String, String>>>;
pub type LapinPoolConnection = deadpool::managed::Pool<deadpool_lapin::Manager>;
pub type RedisPoolConnection = deadpool::managed::Pool<deadpool_redis::Manager, deadpool_redis::Connection>;
