pub mod dash {
    use reqwest::ClientBuilder;

    use crate::api::{piped::{AudioStream, VideoStream}, error::{ApiError, Errors}, };

    pub async fn generate_dash(vstream: Vec<VideoStream>, astream: Vec<AudioStream>, duration: i32) -> Result<String, ApiError>{
        let client: reqwest::Client = ClientBuilder::new().build().unwrap();
        let res = client.post("http://localhost:3030/").json(&serde_json::json!({
            "videoFormats": vstream,
            "audioFormats": astream,
            "videoLength": duration,

        })).send().await;
        match res{
            Ok(data) => match data.text().await{
                Ok(val) => Ok(val),
                Err(_) => {
                    Err(ApiError::new(Errors::ParsingError, "Parsing Error empty response".to_string()))
                },
            },
            Err(err) => Err(ApiError::new(Errors::RequestError, err.to_string())),
        }
  }
}
