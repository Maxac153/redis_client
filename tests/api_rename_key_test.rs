mod common;

#[cfg(test)]
mod tests {
    use crate::common::{
        common::{load_test_params, TestSetup},
        data_structures::rename::RenameKey,
    };
    
    use actix_web::{
        test::{self, TestRequest},
        web, App,
    };
    use r2d2_redis::redis::Commands;
    use redis_client::{handlers::redis::rename_key::rename_key, models::response::Response};

    #[actix_rt::test]
    async fn api_rename_key_test() {
        let (host_redis, port_redis) = load_test_params();
        let test_setup = TestSetup::new(host_redis, port_redis);
        test_setup.drop();
        test_setup.setup_test_list_data("old_name_key_test", vec!["rename_test"]);
        let pool = test_setup.init_pool();
        let mut con = pool.get().unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool))
                .service(rename_key),
        )
        .await;

        let test_cases = [
            (
                "Проверка переименования несуществующего ключа.",
                RenameKey::default()
                    .old_name_key("ERROR_KEY")
                    .new_name_key("new_name_key_test")
                    .build(),
                Response::default()
                    .status("KO")
                    .message("An error was signalled by the server: no such key")
                    .data("")
                    .build(),
            ),
            (
                "Проверка переименования ключа (new name уже есть в базе).",
                RenameKey::default()
                    .old_name_key("old_name_key_test")
                    .new_name_key("old_name_key_test")
                    .build(),
                Response::default()
                    .status("KO")
                    .message("A key (old_name_key_test) with this name already exists!")
                    .data("")
                    .build(),
            ),
            (
                "Проверка изменения имени ключа.",
                RenameKey::default()
                    .old_name_key("old_name_key_test")
                    .new_name_key("new_name_key_test")
                    .build(),
                Response::default()
                    .status("OK")
                    .message("Key (old_name_key_test -> new_name_key_test) successfully renamed")
                    .data("")
                    .build(),
            ),
        ];

        for (_, rename, response) in test_cases {
            let req = TestRequest::patch()
                .uri(&format!(
                    "/renameKey?old_name_key={}&new_name_key={}",
                    rename.get_old_name_key(),
                    rename.get_new_name_key()
                ))
                .to_request();

            let resp = test::call_service(&app, req).await;
            let response_body = test::read_body(resp).await;
            let result: Response = serde_json::from_slice(&response_body).unwrap();

            // Проверяем ответ от сервера
            assert_eq!(result, response);

            // Проверка что в Redis есть данные
            if response.get_status() == "OK" {
                let exists: bool = con
                    .exists::<String, bool>(rename.get_new_name_key().to_string())
                    .unwrap();
                assert!(
                    exists,
                    "The key ({}) does not exist!",
                    rename.get_new_name_key()
                )
            }
        }
        test_setup.drop();
    }
}
