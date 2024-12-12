mod common;

#[cfg(test)]
mod tests {
    use crate::common::{
        common::{load_test_params, TestSetup},
        data_structures::change_ttl::ChangeTtl,
    };
    
    use actix_web::{
        test::{self, TestRequest},
        web, App,
    };
    use r2d2_redis::redis::Commands;
    use redis_client::{handlers::redis::change_ttl::change_ttl, models::response::Response};

    #[actix_rt::test]
    async fn api_change_ttl_test() {
        let (host_redis, port_redis) = load_test_params();
        let test_setup = TestSetup::new(host_redis, port_redis);
        test_setup.drop();
        test_setup.setup_test_list_data("change_ttl_key", vec!["three", "two", "one"]);
        let pool = test_setup.init_pool();
        let mut con = pool.get().unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool))
                .service(change_ttl),
        )
        .await;

        let test_cases = [
            (
                "Проверка записи в начало очереди.",
                ChangeTtl::default().key("change_ttl_key").ttl(120).build(),
                Response::default()
                    .status("OK")
                    .message("TTL successfully changed, key (change_ttl_key), ttl (120 sec).")
                    .data("")
                    .build(),
            ),
            (
                "Проверка записи в конец очереди.",
                ChangeTtl::default().key("change_ttl_key").ttl(0).build(),
                Response::default()
                    .status("OK")
                    .message("Expiration removed for key (change_ttl_key).")
                    .data("")
                    .build(),
            ),
            (
                "Проверка некорректного redis_key.",
                ChangeTtl::default().key("error_key").ttl(120).build(),
                Response::default()
                    .status("KO")
                    .message("Key (error_key) does not exist!")
                    .data("")
                    .build(),
            ),
        ];

        for (_, change_ttl_data, response) in test_cases {
            let req = TestRequest::patch()
                .uri(&format!(
                    "/changeTtl?key={}&ttl={}",
                    change_ttl_data.get_key(),
                    change_ttl_data.get_ttl()
                ))
                .to_request();

            let service_response = test::call_service(&app, req).await;
            let response_body = test::read_body(service_response).await;
            let result: Response = serde_json::from_slice(&response_body).unwrap();

            // Проверяем ответ от сервера
            assert_eq!(result, response);

            // Проверка изменения TTL в Redis
            if response.get_status() == "OK" {
                let ttl: i64 = con.ttl(change_ttl_data.get_key()).unwrap();

                if change_ttl_data.get_ttl() == 0 {
                    assert_eq!(ttl, -1);
                } else {
                    assert_eq!(ttl, change_ttl_data.get_ttl());
                }
            }
        }
        test_setup.drop();
    }
}
