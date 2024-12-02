mod src;

#[cfg(test)]
mod tests {
    use crate::src::{
        common::{load_test_params, TestSetup},
        data_structures::upload_dump::UploadDump,
    };
    use actix_multipart_test::MultiPartFormDataBuilder;
    use actix_web::{
        test::{self},
        web::{self},
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
                "Проверка загрузки дампа.".to_string(),
                UploadDump::default()
                    .file_path("./tests/resources/upload_dump_all_keys.rdb")
                    .file_name("upload_dump_all_keys.rdb"),
                Response::default()
                    .status("OK")
                    .message("Files uploaded successfully!")
                    .data(""),
            ),
            (
                "Проверка загрузки дампа с некорректными данными.".to_string(),
                UploadDump::default()
                    .file_path("./tests/resources/error_data_dump_all_keys.rdb")
                    .file_name("error_data_dump_all_keys.rdb"),
                Response::default()
                    .status("KO")
                    .message("An error was signalled by the server: DUMP payload version or checksum are wrong")
                    .data(""),
            ),
            (
                "Проверка загрузки пустого дампа.".to_string(),
                UploadDump::default()
                    .file_path("./tests/resources/empty_dump_all_keys.rdb")
                    .file_name("empty_dump_all_keys.rdb"),
                Response::default()
                    .status("KO")
                    .message("Uploaded file is empty.")
                    .data(""),
            ),
            (
                "Проверка загрузки пустого дампа.".to_string(),
                UploadDump::default()
                    .file_path("./tests/resources/incorrect_file_extension_all_keys.txt")
                    .file_name("incorrect_file_extension_all_keys.txt"),
                Response::default()
                    .status("KO")
                    .message("Invalid file format. Expected '.rdb'.")
                    .data(""),
            ),
        ];

        for (_, upload_dump, response) in test_cases {
            let mut multipart_form_data_builder = MultiPartFormDataBuilder::new();
            multipart_form_data_builder.with_file(
                upload_dump.get_file_path(),
                "dump",
                "application/octet-stream",
                upload_dump.get_file_name(),
            );

            multipart_form_data_builder.with_text("dump", upload_dump.get_file_name());
            let (header, body) = multipart_form_data_builder.build();

            let req = test::TestRequest::post()
                .uri("/uploadDumpAllKeys")
                .insert_header(header)
                .set_payload(body)
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
