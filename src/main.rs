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
    let device_id = &data["end_device_ids"]["device_id"];
    let timestamp = &data["uplink_message"]["settings"]["time"];
    let tempc_ds = &data["uplink_message"]["decoded_payload"]["TempC_SHT"];
    let humidity = &data["uplink_message"]["decoded_payload"]["Hum_SHT"];
    let data_map = HashMap::from([
        ( "id".to_string(), AttributeValue::S(timestamp.to_string(),),),
        ( "device_id".to_string(), AttributeValue::S(device_id.to_string(),),),
        ( "timestamp".to_string(), AttributeValue::S(timestamp.to_string(),),),
        ( "tempc_ds".to_string(), AttributeValue::S(tempc_ds.to_string(),),),
        ( "humidity".to_string(), AttributeValue::S(humidity.to_string(),),),
    ],);

    let request = client
        .put_item()
        .table_name("TemperatureReadings")
        .set_item(Some(data_map));
    request.send().await?;

    Ok(())
}
