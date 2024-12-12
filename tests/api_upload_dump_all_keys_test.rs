mod common;

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

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
        handlers::redis::upload_dump_key::upload_dump_all_keys, models::response::Response,
    };

    #[actix_rt::test]
    async fn api_upload_dump_all_keys_test() {
        let (host_redis, port_redis) = load_test_params();
        let test_setup = TestSetup::new(host_redis, port_redis);
        test_setup.drop();
        let pool = test_setup.init_pool();
        let mut con = pool.get().unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool))
                .service(upload_dump_all_keys),
        )
        .await;

        let test_cases = [
            (
                "Проверка загрузки дампа.",
                UploadDump::default()
                    .file_path("./tests/resources/upload_dump_all_keys.rdb")
                    .file_name("upload_dump_all_keys")
                    .build(),
                Response::default()
                    .status("OK")
                    .message("Files uploaded successfully!")
                    .data("")
                    .build(),
            ),
            (
                "Проверка загрузки дампа с некорректными данными.",
                UploadDump::default()
                    .file_path("./tests/resources/error_data_dump_all_keys.rdb")
                    .file_name("error_data_dump_all_keys")
                    .build(),
                Response::default()
                    .status("KO")
                    .message("An error was signalled by the server: DUMP payload version or checksum are wrong")
                    .data("")
                    .build(),
            ),
            (
                "Проверка загрузки пустого дампа.",
                UploadDump::default()
                    .file_path("./tests/resources/empty_dump_all_keys.rdb")
                    .file_name("empty_dump_all_keys")
                    .build(),
                Response::default()
                    .status("KO")
                    .message("Uploaded file is empty.")
                    .data("")
                    .build(),
            )
        ];

        for (_, upload_dump, response) in test_cases {
            let mut file = File::open(upload_dump.get_file_path()).unwrap();
            let mut buffer: Vec<u8> = Vec::new();
            file.read_to_end(&mut buffer).unwrap();
            let payload: Bytes = Bytes::from(buffer);

            let req = test::TestRequest::post()
                .uri("/uploadDumpAllKeys")
                .insert_header(header::ContentType::octet_stream())
                .set_payload(payload)
                .to_request();

            let resp = test::call_service(&app, req).await;
            let response_body = test::read_body(resp).await;
            let result: Response = serde_json::from_slice(&response_body).unwrap();

            // Проверяем ответ от сервера
            assert_eq!(result, response);
            if response.get_status() == "OK" {
                let key_one: String = con.lpop("test").unwrap();
                let key_two: String = con.lpop("542342").unwrap();
                assert_eq!("{}", key_one);
                assert_eq!("{}", key_two);
            }
        }
        test_setup.drop();
    }
}
