use std::{collections::HashMap, fs::File, io::Read};

use actix_web::{post, Responder};

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{web, HttpResponse};
use r2d2::{Pool, PooledConnection};
use r2d2_redis::{
    redis::{self, Commands, RedisResult},
    RedisConnectionManager,
};

use crate::models::response::Response;

fn restore(con: &mut redis::Connection, key: String, data: Vec<u8>) -> RedisResult<String> {
    let status: String = redis::cmd("RESTORE").arg(key).arg(0).arg(data).query(con)?;
    Ok(status)
}

fn validate_file_name(file_name: &str, file_extension: &str) -> Result<String, String> {
    if file_name.ends_with(file_extension) {
        match file_name.split_once('.') {
            Some((name, _extension)) => Ok(name.to_string()),
            None => Err("Error: No extension found.".to_string()),
        }
    } else {
        Err(format!(
            "Invalid file format. Expected '{}'.",
            file_extension
        ))
    }
}

#[utoipa::path(
    tag = "Redis Client",
    description = "Redis Upload Dump Key - Загрузка дампа в Redis",
    post,
    path = "/uploadDumpKey"
)]
#[post("/uploadDumpKey")]
pub async fn upload_dump_key(
    pool: web::Data<Pool<RedisConnectionManager>>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> impl Responder {
    let mut con: PooledConnection<RedisConnectionManager> = match pool.get() {
        Ok(connection) => connection,
        Err(_) => {
            return HttpResponse::InternalServerError().json(Response::error(
                "Failed to get a connection from the pool!".to_string(),
            ));
        }
    };

    let file_name: String = match validate_file_name(&form.dump.file_name.unwrap(), ".dump") {
        Ok(name) => name,
        Err(err) => return HttpResponse::Ok().json(Response::error(err)),
    };

    let file_path: &std::path::Path = form.dump.file.path();

    let mut file: File = match std::fs::File::open(&file_path) {
        Ok(file) => file,
        Err(err) => return HttpResponse::Ok().json(Response::error(err.to_string())),
    };

    let mut contents: Vec<u8> = Vec::new();
    if let Err(err) = file.read_to_end(&mut contents) {
        return HttpResponse::Ok().json(Response::error(err.to_string()));
    }

    match con.del::<&String, ()>(&file_name) {
        Ok(_) => {}
        Err(err) => return HttpResponse::Ok().json(Response::error(err.to_string())),
    };

    let res = match restore(&mut con, file_name.clone(), contents) {
        Ok(_) => Response::ok("Files uploaded successfully!".to_string(), "".to_string()),
        Err(err) => Response::error(err.to_string()),
    };

    HttpResponse::Ok().json(res)
}

fn vec_to_hashmap(data: Vec<u8>) -> Result<HashMap<String, Vec<u8>>, String> {
    let mut result: HashMap<String, Vec<u8>> = HashMap::new();
    let mut index: usize = 0;

    while index < data.len() {
        if index + 4 > data.len() {
            return Err("Insufficient data for key length".to_string());
        }

        let key_length: usize =
            u32::from_le_bytes(data[index..index + 4].try_into().unwrap()) as usize;
        index += 4;

        if index + key_length > data.len() {
            return Err("Insufficient data for key".to_string());
        }

        let key_result = String::from_utf8(data[index..index + key_length].to_vec());
        let key = match key_result {
            Ok(k) => k,
            Err(_) => return Err("Invalid UTF-8 sequence in key".to_string()),
        };
        index += key_length;

        if index + 4 > data.len() {
            return Err("Insufficient data for value length".to_string());
        }

        let value_length: usize =
            u32::from_le_bytes(data[index..index + 4].try_into().unwrap()) as usize;
        index += 4;

        if index + value_length > data.len() {
            return Err("Insufficient data for value".to_string());
        }

        let value: Vec<u8> = data[index..index + value_length].to_vec();
        index += value_length;

        result.insert(key, value);
    }

    Ok(result)
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    dump: TempFile,
}

#[utoipa::path(
    tag = "Redis Client",
    description = "Redis Upload Dump All Keys - Загрузка дампа со всеми ключами в Redis",
    post,
    path = "/uploadDumpAllKeys",
    request_body(
        content_type = "multipart/form-data",
        description = "Файл для загрузки",
    )
)]
#[post("/uploadDumpAllKeys")]
pub async fn upload_dump_all_keys(
    pool: web::Data<Pool<RedisConnectionManager>>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> impl Responder {
    let mut con: PooledConnection<RedisConnectionManager> = match pool.get() {
        Ok(connection) => connection,
        Err(_) => {
            return HttpResponse::BadRequest().json(Response::error(
                "Failed to get a connection from the pool!".to_string(),
            ));
        }
    };

    match validate_file_name(&form.dump.file_name.unwrap(), ".rdb") {
        Ok(name) => name,
        Err(err) => return HttpResponse::Ok().json(Response::error(err)),
    };

    let file_path: &std::path::Path = form.dump.file.path();
    let mut file: File = match File::open(&file_path) {
        Ok(file) => file,
        Err(err) => return HttpResponse::Ok().json(Response::error(err.to_string())),
    };

    let mut contents: Vec<u8> = Vec::new();
    if let Err(err) = file.read_to_end(&mut contents) {
        return HttpResponse::Ok().json(Response::error(err.to_string()));
    }

    if contents.is_empty() {
        return HttpResponse::BadRequest()
            .json(Response::error("Uploaded file is empty.".to_string()));
    }

    let dump: HashMap<String, Vec<u8>> = match vec_to_hashmap(contents) {
        Ok(contents) => contents,
        Err(_) => {
            return HttpResponse::InternalServerError().json(Response::error(
                "An error was signalled by the server: DUMP payload version or checksum are wrong"
                    .to_string(),
            ));
        }
    };

    for (key, data) in dump {
        match con.del::<String, ()>(key.to_string()) {
            Ok(_) => {}
            Err(err) => return HttpResponse::Ok().json(Response::error(err.to_string())),
        };

        match restore(&mut con, key.to_string(), data.to_vec()) {
            Ok(_) => continue,
            Err(err) => return HttpResponse::Ok().json(Response::error(err.to_string())),
        };
    }
    HttpResponse::Ok().json(Response::ok(
        "Files uploaded successfully!".to_string(),
        "".to_string(),
    ))
}
