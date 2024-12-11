mod common;

#[cfg(test)]
mod tests {
    use actix_web::{
        test::{self, TestRequest},
        web, App,
    };

    use redis_client::{handlers::redis::read::read_hash, models::response::Response};

    use crate::common::{
        common::{load_test_params, TestSetup},
        data_structures::read::ReadKey,
    };

    #[actix_rt::test]
    async fn api_read_hash_test() {
        let (host_redis, port_redis) = load_test_params();
        let test_setup = TestSetup::new(host_redis, port_redis);
        test_setup.drop();
        test_setup.setup_test_hash_data("read_hash_key", "field", "one");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(test_setup.init_pool()))
                .service(read_hash),
        )
        .await;

        let test_cases = [
            (
                "Проверка чтения из начала очереди.",
                ReadKey::default().key("read_hash_key").build(),
                Response::default()
                    .status("OK")
                    .message("Data read successfully.")
                    .data("{\n  \"field\": {\n    \"data\": \"one\"\n  }\n}")
                    .build(),
            ),
            (
                "Проверка чтения несуществующего ключа.",
                ReadKey::default().key("read_error_key").build(),
                Response::default()
                    .status("KO")
                    .message("Hash not found!")
                    .data("")
                    .build(),
            ),
        ];

        for (_, read_data, response) in test_cases {
            let req = TestRequest::get()
                .uri(&format!("/readHash?key={}", read_data.get_key()))
                .to_request();

            let resp = test::call_service(&app, req).await;
            let response_body = test::read_body(resp).await;
            let result: Response = serde_json::from_slice(&response_body).unwrap();

            // Проверяем ответ от сервера
            assert_eq!(result, response);
        }
        test_setup.drop();
    }
}
