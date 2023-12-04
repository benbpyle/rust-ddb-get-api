use std::collections::HashMap;

use aws_sdk_dynamodb::{types::AttributeValue, Client, Error};

use super::models::Item;

pub async fn fetch_item(client: &Client, table_name: &str, id: &str) -> Result<Item, Error> {
    let key_map: HashMap<String, AttributeValue> = [
        ("pk".to_string(), AttributeValue::S(id.to_string())),
        ("sk".to_string(), AttributeValue::S(id.to_string())),
    ]
    .iter()
    .cloned()
    .collect();

    let result = client
        .get_item()
        .table_name(table_name)
        .set_key(Some(key_map))
        .send()
        .await?;

    let i: Item = result.item.unwrap().into();
    Ok(i)
}
