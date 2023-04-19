//! Integration tests for /cakes routes.

use axum_example::entity::{cake, prelude::Cake};
use reqwest::{header::LOCATION, StatusCode};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::helpers::{http_client::Client, service::TestService};

// Create a cake using `cake::Model` in the request body.
// The `id` field should be ignored.
#[tokio::test]
async fn create_cake_with_model() {
    let api = TestService::new().await;
    let db = api.database_connection();
    let client = Client::new(api.api_url());

    let new_cake = cake::Model {
        // The `id` field should be ignored.
        id: 987,
        name: "Pancake".to_string(),
    };

    let response = client.post("/cakes").json(&new_cake).send().await;
    let headers = response.headers();

    assert_eq!(response.status(), StatusCode::CREATED);
    // Cake named "Pancake" is found in the DB.
    let cake_id = Cake::find()
        .filter(cake::Column::Name.eq("Pancake"))
        .one(db)
        .await
        .unwrap()
        .expect("No cake named 'Pancake' found in DB")
        .id;

    // Location header included in response.
    assert!(headers.contains_key(LOCATION));
    assert_eq!(
        headers.get(LOCATION).unwrap().to_str().unwrap(),
        format!("/cakes/{cake_id}")
    );

    // `id` specified in request body was not used.
    assert!(new_cake.id != cake_id);
}

// Create a cake using `cake::NewModel` in the request body.
#[tokio::test]
async fn create_cake_with_create_model() {
    let api = TestService::new().await;
    let db = api.database_connection();
    let client = Client::new(api.api_url());

    let new_cake = cake::NewModel {
        name: "Pancake".to_string(),
    };

    let response = client.post("/cakes").json(&new_cake).send().await;
    let headers = response.headers();

    assert_eq!(response.status(), StatusCode::CREATED);
    // Cake named "Pancake" is found in the DB.
    let cake_id = Cake::find()
        .filter(cake::Column::Name.eq("Pancake"))
        .one(db)
        .await
        .unwrap()
        .expect("No cake named 'Pancake' found in DB")
        .id;

    // Location header included in response.
    assert!(headers.contains_key(LOCATION));
    assert_eq!(
        headers.get(LOCATION).unwrap().to_str().unwrap(),
        format!("/cakes/{cake_id}")
    );
}

#[tokio::test]
async fn get_cakes_collection() {
    let api = TestService::new().await;
    let db = api.database_connection();
    let client = Client::new(api.api_url());

    // Arrange: Prepare cakes in DB.
    let new_cake_names = ["chocolate", "strawberry"];
    for name in new_cake_names {
        cake::ActiveModel {
            name: Set(name.to_string()),
            ..Default::default()
        }
        .insert(db)
        .await
        .unwrap();
    }

    let response = client.get("/cakes").send().await;
    assert_eq!(response.status(), StatusCode::OK);

    let cakes: Vec<cake::Model> = response.json().await;
    assert_eq!(new_cake_names.len(), cakes.len());
    for cake in cakes {
        assert!(new_cake_names.contains(&cake.name.as_ref()));
    }
}

#[tokio::test]
async fn get_cake_by_id_ok() {
    let api = TestService::new().await;
    let db = api.database_connection();
    let client = Client::new(api.api_url());

    // Arrange: Prepare a cake in DB.
    let cake_id = cake::ActiveModel {
        name: Set("Coffee Cake".to_string()),
        ..Default::default()
    }
    .insert(db)
    .await
    .unwrap()
    .id;

    let response = client.get(&format!("/cakes/{cake_id}")).send().await;

    assert_eq!(response.status(), StatusCode::OK);
    let cake: cake::Model = response.json().await;
    assert_eq!(cake.id, cake_id);
    assert_eq!(cake.name, "Coffee Cake");
}

#[tokio::test]
async fn get_cake_by_id_not_found() {
    let api = TestService::new().await;
    let client = Client::new(api.api_url());

    // No cake with ID 987 exists in the database.
    let response = client.get("/cakes/987").send().await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn delete_cake_by_id_ok() {
    let api = TestService::new().await;
    let db = api.database_connection();
    let client = Client::new(api.api_url());

    // Arrange: Prepare a cake in DB.
    let cake_id = cake::ActiveModel {
        name: Set("Coffee Cake".to_string()),
        ..Default::default()
    }
    .insert(db)
    .await
    .unwrap()
    .id;

    // Act: Delete the cake by ID.
    let response = client.delete(&format!("/cakes/{cake_id}")).send().await;

    assert_eq!(response.status(), StatusCode::OK);
    // Cake matching cake_id cannot be found in DB
    assert!(Cake::find_by_id(cake_id).one(db).await.unwrap().is_none());
}

#[tokio::test]
async fn delete_cake_by_id_not_found() {
    let api = TestService::new().await;
    let client = Client::new(api.api_url());

    // Act: Delete the cake by ID.
    // No cake with ID 987 exists in the database.
    let response = client.delete("/cakes/987").send().await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
