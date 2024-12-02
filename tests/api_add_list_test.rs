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
    use redis_client::{handlers::redis::add::add_list, models::response::Response};

    #[actix_rt::test]
    async fn api_add_list_test() {
        let (host_redis, port_redis) = load_test_params();
        let test_setup = TestSetup::new(host_redis, port_redis);
        test_setup.drop();
        test_setup.setup_test_list_data("add_key", vec!["test_data", "test_data", "test_data"]);
        let pool = test_setup.init_pool();
        let mut con = pool.get().unwrap();

        let app = test::init_service(App::new().app_data(web::Data::new(pool)).service(add_list)).await;

        let test_cases = [
            (
                "Проверка записи в начало очереди.".to_string(),
                AddKey::default()
                    .key("add_key")
                    .add_mod("FIRST")
                    .body_data("{\"data\": \"One\"}"),
                Response::default()
                    .status("OK")
                    .message("Data added successfully.")
                    .data(""),
            ),
            (
                "Проверка записи в конец очереди.".to_string(),
                AddKey::default()
                    .key("add_key")
                    .add_mod("LAST")
                    .body_data("{\"data\": \"Two\"}"),
                Response::default()
                    .status("OK")
                    .message("Data added successfully.")
                    .data(""),
            ),
            (
                "Проверка некорректного add_mode.".to_string(),
                AddKey::default()
                    .key("add_key")
                    .add_mod("ERROR_ADD_MODE")
                    .body_data("{\"data\": \"Hello Redis!\"}"),
                Response::default()
                    .status("KO")
                    .message("Incorrect operation (ERROR_ADD_MODE), expected (FIRST, LAST)!")
                    .data(""),
            ),
        ];

        for (_, add_data, response) in test_cases {
            let req = TestRequest::post()
                .uri(&format!(
                    "/addList?key={}&add_mode={}",
                    add_data.get_key(),
                    add_data.get_add_mod()
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
            let _ = match add_data.get_add_mod() {
                "FIRST" => {
                    let value: String = con.lpop(add_data.get_key()).unwrap();
                    assert_eq!(value, add_data.get_body_data());
                }
                "LAST" => {
                    let value: String = con.rpop(add_data.get_key()).unwrap();
                    assert_eq!(value, add_data.get_body_data());
                }
                _ => {}
            };
        }
        test_setup.drop();
    }
}
