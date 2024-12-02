mod src;

#[cfg(test)]
mod tests {
    use actix_web::{
        http::header,
        test::{self, TestRequest},
        web, App,
    };

    use r2d2_redis::redis::Commands;

    use crate::src::{
        common::{load_test_params, TestSetup},
        data_structures::add::AddKey,
    };
    use redis_client::{handlers::redis::add::add_hash, models::response::Response};

    #[actix_rt::test]
    async fn api_add_hash_test() {
        let (host_redis, port_redis) = load_test_params();
        let test_setup = TestSetup::new(host_redis, port_redis);
        test_setup.drop();
        let pool = test_setup.init_pool();
        let mut con = pool.get().unwrap();

        let app =
            test::init_service(App::new().app_data(web::Data::new(pool)).service(add_hash)).await;

        let test_cases = [
            (
                "Проверка добавлеения поля в Hash (поле field_one).".to_string(),
                AddKey::default()
                    .key("add_hash_key")
                    .field("field_one")
                    .body_data("{\"data\": \"One\"}"),
                Response::default()
                    .status("OK")
                    .message("Data added successfully.")
                    .data(""),
            ),
            (
                "Проверка добавления второго поля (поле field_two).".to_string(),
                AddKey::default()
                    .key("add_hash_key")
                    .field("field_two")
                    .body_data("{\"data\": \"Two\"}"),
                Response::default()
                    .status("OK")
                    .message("Data added successfully.")
                    .data(""),
            ),
        ];

        for (_, add_data, response) in test_cases {
            let req = TestRequest::post()
                .uri(&format!(
                    "/addHash?key={}&field={}",
                    add_data.get_key(),
                    add_data.get_field()
                ))
                .insert_header(header::ContentType::json())
                .set_payload(add_data.get_body_data().to_string())
                .to_request();

            let service_response = test::call_service(&app, req).await;
            let response_body = test::read_body(service_response).await;
            let result: Response = serde_json::from_slice(&response_body).unwrap();

            // Проверяем ответ от сервера
            assert_eq!(result, response);

            // Проверка что в Redis есть данные
            if response.get_status() == "OK" {
                let result: String = con.hget(add_data.get_key(), add_data.get_field()).unwrap();
                assert_eq!(result, add_data.get_body_data());
            }
        }
        test_setup.drop();
    }
}
