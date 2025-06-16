mod common;

//`actix_rt::test`isthetestingequivalentof`actix_web::main`.
// Italsosparesyoufromhavingtospecifythe`#[test]` attribute.
//
// Use `cargoaddactix-rt--dev--vers2`toadd `actix-rt`
// under`[dev-dependencies]` inCargo.toml
//
// You caninspectwhatcodegetsgeneratedusing
//`cargoexpand--testhealth_check` (<-nameofthetestfile)
#[actix_rt::test]
async fn health_check_works() {
    let client = reqwest::Client::new();
    //Act
    let response = client
        .get(
            common::spawn_app().await.address
                + "/health_check",
        )
        .send()
        .await
        .expect("Failed to execute request.");
    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
