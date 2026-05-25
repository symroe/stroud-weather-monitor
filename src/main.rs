use std::collections::{HashMap};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{
    model::AttributeValue, Client, Error as AwsError, types::DisplayErrorContext,
};
use lambda_http::{service_fn, Body, Error, IntoResponse, Request, RequestExt};
use serde_json;


#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(service_fn(process_request)).await?;
    Ok(())
}

async fn process_request(
    request: Request
) -> Result<impl IntoResponse, std::convert::Infallible> {

    let body = request.body();
    println!("{:?}", body);

    if let Body::Text(body_text) = body {
        println!("{}", body_text);
        if let Err(e) = write_to_dynamodb(&body_text).await {
            eprintln!("Error: {}", DisplayErrorContext(e));
        }
    }

    Ok(format!(
        "hello {}",
        request
            .query_string_parameters()
            .first("name")
            .unwrap_or_else(|| "stranger")
    ))
}

async fn write_to_dynamodb(json_string: &String) -> Result<(), AwsError> {
    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-2");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let data: serde_json::Value = serde_json::from_str(json_string).unwrap();
    let uplink = &data["data"]["uplink_message"];

    let device_id = match data["data"]["end_device_ids"]["device_id"].as_str() {
        Some(s) if !s.is_empty() => s.to_string(),
        _ => {
            eprintln!("Skipping record: missing device_id. Payload: {}", json_string);
            return Ok(());
        }
    };
    let timestamp = match uplink["settings"]["time"].as_str() {
        Some(s) if !s.is_empty() => s.to_string(),
        _ => {
            eprintln!("Skipping record: missing timestamp. Payload: {}", json_string);
            return Ok(());
        }
    };

    let mut item = HashMap::from([
        ("device_id".to_string(), AttributeValue::S(device_id)),
        ("timestamp".to_string(), AttributeValue::S(timestamp)),
    ]);

    let decoded = &uplink["decoded_payload"];
    if let Some(temp) = decoded["temperature"].as_f64() {
        item.insert("tempc_ds".to_string(), AttributeValue::N(temp.to_string()));
    }
    if let Some(hum) = decoded["Hum_SHT"].as_f64() {
        item.insert("humidity".to_string(), AttributeValue::N(hum.to_string()));
    }

    let request = client
        .put_item()
        .table_name("TemperatureReadings_v2")
        .set_item(Some(item));
    request.send().await?;

    Ok(())
}
