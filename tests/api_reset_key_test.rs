mod common;

#[cfg(test)]
mod tests {
    use crate::common::{
        common::{load_test_params, TestSetup},
        data_structures::reset_key::ResetKey,
    };
    
    use actix_web::{
        test::{self, TestRequest},
        web, App,
    };
    use r2d2_redis::redis::Commands;
    use redis_client::{handlers::redis::reset::reset_key, models::response::Response};

    #[actix_rt::test]
    async fn api_reset_key_test() {
        let (host_redis, port_redis) = load_test_params();
        let test_setup = TestSetup::new(host_redis, port_redis);
        test_setup.drop();
        test_setup.setup_test_list_data("reset_key_test", vec!["one"]);
        let pool = test_setup.init_pool();
        let mut con = pool.get().unwrap();

        let app =
            test::init_service(App::new().app_data(web::Data::new(pool)).service(reset_key)).await;

        let test_cases = [
            (
                "Проверка удаления данных по ключу.",
                ResetKey::default().key("reset_key_test"),
                Response::default()
                    .status("OK")
                    .message("Deleted record key (reset_key_test).")
                    .data("")
                    .build(),
            ),
            (
                "Проверка удаления данных по несуществующему ключу.",
                ResetKey::default().key("reset_key_test"),
                Response::default()
                    .status("KO")
                    .message("The key (reset_key_test) does not exist!")
                    .data("")
                    .build(),
            ),
        ];

        for (_, reset, response) in test_cases {
            let req = TestRequest::delete()
                .uri(&format!("/resetKey?key={}", reset.get_key()))
                .to_request();

            let service_response = test::call_service(&app, req).await;
            let response_body = test::read_body(service_response).await;
            let result: Response = serde_json::from_slice(&response_body).unwrap();

            // Проверяем ответ от сервера
            assert_eq!(result, response);

            // Проверка что в Redis нет удалённого
            if response.get_status() == "OK" {
                let exists: bool = con
                    .exists::<String, bool>(reset.get_key().to_string())
                    .unwrap();
                assert!(!exists, "The key ({}) was not deleted!", reset.get_key())
            }
        }
        test_setup.drop();
    }
}
