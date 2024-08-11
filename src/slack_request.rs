use reqwest::{ Error, header};

#[tokio::main]
async fn main() -> Result<(), Error> {
    slack_request().await?;
    Ok(())
}

async fn slack_request() -> Result<(), Error> {
    // Configure URL and headers
    let url = "https://slack.com/api/chat.postMessage";
    let token = "Bearer <your_token_here>";
    let channel = "<your_channel_here>";
    let message = "Hello World!";
    let mut headers = header::HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"));
    headers.insert(header::AUTHORIZATION, header::HeaderValue::from_static(token));

    // Configure body data
	let json = &serde_json::json!({
	    "channel": channel,
	    "text": message
	});

    // Send the request
    let request = reqwest::Client::new()
        .post(url)
        .headers(headers)
        .json(&json)
        .send()
        .await?;

    // Print response
    println!("{:#?}", request);

    // Get the response data
	let response: serde_json::Value = request.json().await?;
    
    // Pretty print the response data
    println!("{}", serde_json::to_string_pretty(&response).unwrap());

    Ok(())

}