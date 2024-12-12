mod common;

#[cfg(test)]
mod tests {
    use crate::common::{
        common::{load_test_params, TestSetup},
        data_structures::upload_dump::UploadDump,
    };

    use actix_web::{
        http::header,
        test::{self},
        web::{self, Bytes},
        App,
    };
    use r2d2_redis::redis::Commands;
    use redis_client::{
        handlers::redis::upload_dump_key::upload_dump_key, models::response::Response,
    };
    use std::{fs::File, io::Read};

    #[actix_rt::test]
    async fn api_upload_dump_key_test() {
        let (host_redis, port_redis) = load_test_params();
        let test_setup = TestSetup::new(host_redis, port_redis);
        test_setup.drop();
        let pool = test_setup.init_pool();
        let mut con = pool.get().unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool))
                .service(upload_dump_key),
        )
        .await;

        let test_cases = [(
            "Проверка загрузки дампа.",
            UploadDump::default()
                .file_path("./tests/resources/upload_dump_key.dump")
                .file_name("upload_dump_key.dump")
                .build(),
            Response::default()
                .status("OK")
                .message("Files uploaded successfully!")
                .data("")
                .build(),
        )];

        for (_, upload_dump, response) in test_cases {
            let mut file = File::open(upload_dump.get_file_path()).unwrap();
            let mut buffer: Vec<u8> = Vec::new();
            file.read_to_end(&mut buffer).unwrap();
            let payload: Bytes = Bytes::from(buffer);

            let req = test::TestRequest::post()
                .uri(&format!(
                    "/uploadDumpKey?key_name={}",
                    upload_dump.get_file_name()
                ))
                .insert_header(header::ContentType::octet_stream())
                .set_payload(payload)
                .to_request();

            let resp = test::call_service(&app, req).await;

            // Проверяем ответ от сервера
            assert!(resp.status().is_success());
            let response_body = test::read_body(resp).await;
            let result: Response = serde_json::from_slice(&response_body).unwrap();

            assert_eq!(result, response);
            if response.get_status() == "OK" {
                let key: String = con.lpop(upload_dump.get_file_name()).unwrap();
                assert_eq!("test_data", key);
            }
        }
        test_setup.drop();
    }
}
