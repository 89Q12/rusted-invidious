
use serde_json::Value;
use crate::api::{error::{ApiError, Errors}, ChannelTrait};

use super::{Video, Channel};

pub struct PipedApiBuilder{
    api_host: Option<String>,
    client: reqwest::Client,
    response: Option<reqwest::Response>,
}

impl PipedApiBuilder {
    pub fn new(client: reqwest::Client) -> Self{
        Self { api_host: None, client, response: None }
    }
    pub fn build(self) -> PipedApi {
        PipedApi { client: self.client, api_host: self.api_host.unwrap()}
    }
    pub fn api_host(mut self, api_host: String) -> Self{
        self.api_host =Some(api_host);
        self
    }
}

pub struct PipedApi{
    client: reqwest::Client,
    api_host: String,
}

impl PipedApi{
    pub async fn get_channel(&self, ucid: String) -> Result<Box<dyn ChannelTrait>,ApiError>{
        let result = self.get_resource("/channel/".to_owned() + &ucid, None).await;
        let value = match result {
            Ok(val) => val,
            Err(err) => return Err(err),
        };
        match Channel::try_from(value){
            Ok(chan) => Ok(Box::new(chan)),
            Err(err) => Err(err),
        }
    }
    async fn get_resource(&self,url: String, param: Option<String>) -> Result<Value,ApiError>{
        let url = match param{
            Some(param) => self.api_host.to_owned() + &url + &param,
            None => self.api_host.to_owned() + &url,
        };
        print!("{}",url);
        let res = self.client.get(url).send().await;
        match res{
            Ok(data) => match data.json().await{
                Ok(val) => Ok(val),
                Err(err) => Err(ApiError::new(Errors::RequestError, err.to_string())),
            },
            Err(err) => Err(ApiError::new(Errors::RequestError, err.to_string())),
        }
    }
}