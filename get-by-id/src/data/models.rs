use aws_sdk_dynamodb::{error::SdkError, types::AttributeValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pk: String,
    sk: String,
}

impl Item {
    fn new(pk: String, sk: String) -> Self {
        Item { pk: pk, sk: sk }
    }
}

impl From<HashMap<String, AttributeValue>> for Item {
    fn from(value: HashMap<String, AttributeValue>) -> Self {
        let item = Item::new(
            as_string(value.get("pk"), &"".to_string()),
            as_string(value.get("sk"), &"".to_string()),
        );

        item
    }
}

fn as_string(val: Option<&AttributeValue>, default: &String) -> String {
    if let Some(v) = val {
        if let Ok(s) = v.as_s() {
            return s.to_owned();
        }
    }
    default.to_owned()
}
