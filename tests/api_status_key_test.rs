mod common;

#[cfg(test)]
mod tests {
    use actix_web::{
        test::{self, TestRequest},
        web, App,
    };

    use redis_client::models::status_key::StatusKey as SKM;
    use redis_client::{handlers::redis::status::status_key, models::response::Response};

    use crate::common::{
        common::{load_test_params, TestSetup},
        data_structures::status_key::StatusKey,
    };

    #[actix_rt::test]
    async fn api_status_key_test() {
        let (host_redis, port_redis) = load_test_params();
        let test_setup = TestSetup::new(host_redis, port_redis);
        let key = "status_key";

        test_setup.drop();
        test_setup.setup_test_list_data(&key, vec!["test_data"]);

        let pool = test_setup.init_pool();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool))
                .service(status_key),
        )
        .await;

        let test_cases = [
            (
                "Проверка запроса статуса ключа.",
                "OK",
                StatusKey::default().search_key("status_key").build(),
            ),
            (
                "Проверка получения информации по несуществующему ключу.",
                "KO",
                StatusKey::default().search_key("error_key").build(),
            ),
        ];

        for (_, response_status, status) in test_cases {
            let req = TestRequest::get()
                .uri(&format!(
                    "/statusKey?search_key={}",
                    status.get_search_key(),
                ))
                .to_request();

            let service_response = test::call_service(&app, req).await;
            let response_body = test::read_body(service_response).await;

            // Проверяем ответ от сервера
            if response_status == "OK" {
                let result: SKM = serde_json::from_slice(&response_body).unwrap();
                assert_eq!(key, result.get_key());
                assert_eq!(result.get_type_key(), Some("list"));
                assert_eq!(result.get_len(), 1);
                assert_eq!(result.get_memory_usage(), 80);
                assert_eq!(result.get_ttl(), -1)
            } else if status.get_search_key() == "error_key" {
                let result: Response = serde_json::from_slice(&response_body).unwrap();
                assert_eq!(result.get_status(), "KO");
                assert_eq!(result.get_message(), "Key (error_key) does not exist!");
            }
        }
        test_setup.drop();
    }
}
