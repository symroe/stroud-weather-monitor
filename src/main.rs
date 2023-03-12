use lambda_http::{service_fn, Error, IntoResponse, Request, RequestExt};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(service_fn(process_request)).await?;
    Ok(())
}


async fn process_request(
    request: Request
) ->Result<impl IntoResponse, std::convert::Infallible> {
    let _context = request.lambda_context();

    // save_to_dynamodb();

    println!("{:?}", request.body());

    Ok(format!(
        "hello {}",
        request
            .query_string_parameters()
            .first("name")
            .unwrap_or_else(|| "stranger")
    ))
}
