use serde::{Serialize, Deserialize};
use reqwest::Error;

#[derive(Serialize)]
struct CryptolCommand {
    method: String,
    params: serde_json::Value,
    id: u32,
}

#[derive(Deserialize)]
struct CryptolResponse {
    result: Option<serde_json::Value>,
    error: Option<String>,
    _id: u32, // Note the underscore prefix
}

async fn send_cryptol_command(command: CryptolCommand) -> Result<CryptolResponse, Error> {
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:8080/") // Adjust the URL as necessary
        .json(&command)
        .send()
        .await?
        .json::<CryptolResponse>()
        .await?;

    Ok(res)
}

#[tokio::main]
async fn main() {
    let command = CryptolCommand {
        method: "evaluate expression".into(), // Change according to the Cryptol API
        params: serde_json::json!({"expression": "0x123 + 0x456"}), // Change parameters as needed
        id: 1,
    };

    match send_cryptol_command(command).await {
        Ok(response) => {
            if let Some(result) = response.result {
                println!("Result: {:?}", result);
            } else if let Some(error) = response.error {
                println!("Error: {}", error);
            }
        },
        Err(e) => println!("Error sending command: {}", e),
    }
}