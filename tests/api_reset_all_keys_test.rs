mod common;

#[cfg(test)]
mod tests {
    use actix_web::{
        test::{self, TestRequest},
        web, App,
    };

    use r2d2_redis::redis::Commands;
    use redis_client::{handlers::redis::reset::reset_all_keys, models::response::Response};

    use crate::common::common::{load_test_params, TestSetup};

    #[actix_rt::test]
    async fn api_reset_all_keys_test() {
        let (host_redis, port_redis) = load_test_params();
        let test_setup = TestSetup::new(host_redis, port_redis);
        test_setup.drop();
        test_setup.setup_test_list_data("reset_all_keys_test", vec!["one"]);
        let pool = test_setup.init_pool();
        let mut con = pool.get().unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool))
                .service(reset_all_keys),
        )
        .await;

        let test_cases = [(
            "Проверка удаления всех ключей.",
            Response::default()
                .status("OK")
                .message("Deleted all keys successfully.")
                .data("")
                .build(),
        )];

        for (_, response) in test_cases {
            let req = TestRequest::delete().uri("/resetAllKeys").to_request();

            let service_response = test::call_service(&app, req).await;
            let response_body = test::read_body(service_response).await;
            let result: Response = serde_json::from_slice(&response_body).unwrap();

            // Проверяем ответ от сервера
            assert_eq!(result, response);

            // Проверка пустая ли база
            if response.get_status() == "OK" {
                let keys: Vec<String> = con.keys("*").unwrap();
                assert_eq!(keys.len(), 0);
            }
        }
        test_setup.drop();
    }
}
