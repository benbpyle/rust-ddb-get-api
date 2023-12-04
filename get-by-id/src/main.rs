use std::env;

// use aws_config::meta::region::RegionProviderChain;
use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_sdk_dynamodb::Client;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};

use crate::data::{models::Item, repositories::fetch_item};

mod data;

async fn function_handler(
    client: &Client,
    table_name: &str,
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    //tracing::info!("(Path)={:?}", event.payload.path_parameters);
    match event.payload.path_parameters.get("id") {
        Some(value) => {
            tracing::info!("(Value)={}", value);
            let item: Result<Item, aws_sdk_dynamodb::Error> =
                fetch_item(client, table_name, value).await;
            tracing::info!("Item retrieved");
            tracing::info!("(Item)={:?}", item);
        }
        None => {
            tracing::error!("Key doesn't exist")
        }
    }

    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        is_base64_encoded: Some(false),
        body: Some(event.payload.path.unwrap().into()),
        multi_value_headers: Default::default(),
        headers: Default::default(),
    };

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        //        .without_time()
        .init();

    let stream = env::var("TABLE_NAME").unwrap();
    let str_pointer = stream.as_str();

    let config = aws_config::from_env()
        //.region(region_provider)
        // .profile_name("personal")
        .load()
        .await;
    let client = Client::new(&config);
    let shared_client = &client;

    run(service_fn(
        move |event: LambdaEvent<ApiGatewayProxyRequest>| async move {
            function_handler(&shared_client, str_pointer, event).await
        },
    ))
    .await
}
