use crate::models::config_models::{Config, MutConfig};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use std::{
    fs::{File, OpenOptions},
    io::BufReader,
};

use crate::models::Model;

use crate::middleware::errors_middleware::CustomHttpError;
use crate::middleware::response_middleware::HttpResponseBuilder;

use crate::models::{pool_handler, MySQLPool};

/// This will be exported into by serde. See the `rcms.json` file in the root of the project for config information.
#[derive(Deserialize, Serialize, Clone)]
pub struct LocalConfig {
    pub mysql_username: Option<String>,
    pub mysql_password: Option<String>,
    pub mysql_database: Option<String>,
    pub mysql_url: Option<String>,
    pub mysql_port: Option<u16>,
    pub bind_address: Option<String>,
    pub bind_port: Option<u16>
}

pub async fn update_local_config(
    conf: web::Json<LocalConfig>,
) -> Result<HttpResponse, CustomHttpError> {
    let config_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("./rcms.json")
        .or(Err(CustomHttpError::Unknown))?;

    // delete all previous contents of file.
    config_file.set_len(0).or(Err(CustomHttpError::Unknown))?;
    serde_json::to_writer(config_file, &conf.0).or(Err(CustomHttpError::Unknown))?;

    HttpResponseBuilder::new(200, &"Unimplemented.")
}

pub async fn read_all_local_config() -> Result<HttpResponse, CustomHttpError> {
    let config_file = File::open("./rcms.json").or(Err(CustomHttpError::Unknown))?;
    let reader = BufReader::new(config_file);
    let conf: LocalConfig = serde_json::from_reader(reader).or(Err(CustomHttpError::Unknown))?;

    HttpResponseBuilder::new(200, &conf)
}

pub async fn read_all_database_config(
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;

    let all = Config::read_all(&mysql_pool)?;
    HttpResponseBuilder::new(200, &all)
}

pub async fn read_one_database_config(
    id: web::Path<i32>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;
    let one = Config::read_one(id.to_string(), &mysql_pool)?;
    HttpResponseBuilder::new(200, &one)
}

pub async fn update_database_config(
    id: web::Path<i32>,
    pool: web::Data<MySQLPool>,
    mut_config: web::Json<MutConfig>,
) -> Result<HttpResponse, CustomHttpError> {
    let mysql_pool = pool_handler(pool)?;
    let us = Config::update(id.to_string(), &mut_config, &mysql_pool)?;
    HttpResponseBuilder::new(200, &us)
}
