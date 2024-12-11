mod common;

#[cfg(test)]
mod tests {
    use actix_web::{
        http::StatusCode,
        test::{self, TestRequest},
        web, App,
    };

    use redis_client::handlers::redis::download_dump_key::download_dump_key;

    use crate::common::common::{load_test_params, TestSetup};

    #[actix_rt::test]
    async fn api_download_dump_key_test() {
        let (host_redis, port_redis) = load_test_params();
        let test_setup = TestSetup::new(host_redis, port_redis);
        test_setup.drop();
        test_setup.setup_test_list_data("download_dump_key", vec!["test_data"]);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(test_setup.init_pool()))
                .service(download_dump_key),
        )
        .await;

        let test_cases = vec![
            (
                "Проверка скачивания дампа.",
                "download_dump_key",
                StatusCode::OK,
                vec![
                    18, 1, 2, 18, 18, 0, 0, 0, 1, 0, 137, 116, 101, 115, 116, 95, 100, 97, 116, 97,
                    10, 255, 12, 0, 254, 195, 105, 133, 1, 109, 9, 194,
                ],
            ),
            (
                "Проверка некорректного ключа.",
                "error_key",
                StatusCode::NOT_FOUND,
                vec![
                    123, 34, 115, 116, 97, 116, 117, 115, 34, 58, 34, 75, 79, 34, 44, 34, 109, 101,
                    115, 115, 97, 103, 101, 34, 58, 34, 75, 101, 121, 32, 40, 101, 114, 114, 111,
                    114, 95, 107, 101, 121, 41, 32, 100, 111, 101, 115, 32, 110, 111, 116, 32, 101,
                    120, 105, 115, 116, 33, 34, 44, 34, 100, 97, 116, 97, 34, 58, 34, 34, 125,
                ],
            ),
        ];

        for (_, key, expected_status, data) in test_cases {
            let req = TestRequest::get()
                .uri(&format!("/downloadDumpKey?key={}", key))
                .to_request();

            // Проверяем ответ от сервера
            let resp = test::call_service(&app, req).await;
            assert_eq!(resp.status(), expected_status);

            let response_body = test::read_body(resp).await;
            assert_eq!(response_body.as_ref(), data);
        }
        test_setup.drop();
    }
}
