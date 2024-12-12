mod common;

#[cfg(test)]
mod tests {
    use crate::common::{
        common::{load_test_params, TestSetup},
        data_structures::status_json::StatusJson,
    };
    
    use actix_web::{
        test::{self, TestRequest},
        web, App,
    };
    use redis_client::{
        handlers::redis::status::status_json,
        models::{response::Response, status::StatusJson as SJM},
    };

    #[actix_rt::test]
    async fn api_status_json_test() {
        let (host_redis, port_redis) = load_test_params();
        let test_setup = TestSetup::new(host_redis, port_redis);
        let keys = [
            "status_json_key_1",
            "status_json_key_2",
            "status_json_key_3",
            "status_json_key",
        ];

        test_setup.drop();
        for key in keys {
            test_setup.setup_test_list_data(key, vec!["test_data"]);
        }
        let pool = test_setup.init_pool();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool))
                .service(status_json),
        )
        .await;

        let test_cases = [
            (
                "Проверка запроса статуса.",
                "OK",
                StatusJson::default()
                    .search_key("status_json_")
                    .type_key("List")
                    .lower_limit(0)
                    .upper_limit(30)
                    .build(),
            ),
            (
                "Проверка привешения лимита (лимит 30 значений).",
                "KO",
                StatusJson::default()
                    .search_key("status_json_")
                    .type_key("List")
                    .lower_limit(0)
                    .upper_limit(40)
                    .build(),
            ),
            (
                "Проверка некорректного лимита upper_limit < lower_limit.",
                "KO",
                StatusJson::default()
                    .search_key("status_json_")
                    .type_key("List")
                    .lower_limit(10)
                    .upper_limit(1)
                    .build(),
            ),
        ];

        for (_, response_status, status) in test_cases {
            let req = TestRequest::get()
                .uri(&format!(
                    "/statusJson?search_key={}&type_key={}&lower_limit={}&upper_limit={}",
                    status.get_search_key(),
                    status.get_type_key(),
                    status.get_lower_limit(),
                    status.get_upper_limit()
                ))
                .to_request();

            let service_response = test::call_service(&app, req).await;
            let response_body = test::read_body(service_response).await;

            // Проверяем ответ от сервера
            if response_status == "OK" {
                let result: SJM = serde_json::from_slice(&response_body).unwrap();
                assert_eq!(result.get_keys().len(), 4);
            } else if status.get_upper_limit() > status.get_lower_limit()
                && status.get_upper_limit() - status.get_lower_limit() > 30
            {
                let result: Response = serde_json::from_slice(&response_body).unwrap();
                assert_eq!(
                    result.get_message(),
                    "The difference between upper_limit and lower_limit must not exceed 30!"
                );
            } else {
                let result: Response = serde_json::from_slice(&response_body).unwrap();
                assert_eq!(
                    result.get_message(),
                    "Lower_limit must be less than upper_limit!"
                );
            }
        }
        test_setup.drop();
    }
}
