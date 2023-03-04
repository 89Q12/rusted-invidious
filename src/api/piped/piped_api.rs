use serde_json::Value;
use crate::api::{error::{ApiError, Errors}, ChannelTrait, VideoBasicInfoTrait,PlaylistTrait,CommentsTrait,TrendingTrait, SearchResultTrait,NextResultTrait, dash_utils};

use super::{Video, Channel, Playlist, Comments, Trending, Search, misc::{SearchFilter, Next, PipedEndpoint}};

pub struct PipedApiBuilder{
    api_host: Option<String>,
    client: reqwest::Client,
}

impl PipedApiBuilder {
    pub fn new(client: reqwest::Client) -> Self{
        Self { api_host: None, client}
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
    pub async fn get_video(&self, id: String) -> Result<Box<dyn VideoBasicInfoTrait>,ApiError>{
        let result = self.get_resource("/streams/".to_owned() + &id, None).await;
        let value = match result {
            Ok(val) => val,
            Err(err) => return Err(err),
        };
        match Video::try_from(value){
            Ok(mut chan) => match chan.has_dash(){
                true => Ok(Box::new(chan)),
                false => {
                    match dash_utils::dash::generate_dash(chan.get_video(),chan.get_audio(), chan.get_duration()).await{
                        Ok(dash) => chan.set_dash(dash),
                        Err(_) => return Ok(Box::new(chan)),
                };
                Ok(Box::new(chan))
            },
            },
            Err(err) => Err(err),
        }
    }
    pub async fn get_playlist(&self, list: String) -> Result<Box<dyn PlaylistTrait>,ApiError>{
        let result = self.get_resource("/playlist/".to_owned() + &list, None).await;
        let value = match result {
            Ok(val) => val,
            Err(err) => return Err(err),
        };
        match Playlist::try_from(value){
            Ok(chan) => Ok(Box::new(chan)),
            Err(err) => Err(err),
        }
    }
    pub async fn get_comments(&self, vid: String) -> Result<Box<dyn CommentsTrait>,ApiError>{
        let result = self.get_resource("/comments/".to_owned() + &vid, None).await;
        let value = match result {
            Ok(val) => val,
            Err(err) => return Err(err),
        };
        match Comments::try_from(value){
            Ok(chan) => Ok(Box::new(chan)),
            Err(err) => Err(err),
        }
    }
    pub async fn get_trending(&self, lang: String) -> Result<Box<dyn TrendingTrait>,ApiError>{
        let result = self.get_resource("/trending?region=".to_owned() + &lang, None).await;
        let value = match result {
            Ok(val) => val,
            Err(err) => return Err(err),
        };
        match Trending::try_from(value){
            Ok(chan) => Ok(Box::new(chan)),
            Err(err) => Err(err),
        }
    }
    pub async fn get_search_results(&self, query: String, filter: SearchFilter) -> Result<Box<dyn SearchResultTrait>,ApiError>{
        let result = self.get_resource("/search?q=".to_owned() + &query, Some(format!("&filter={}", filter.get_filter()))).await;
        let value = match result {
            Ok(val) => val,
            Err(err) => return Err(err),
        };
        match Search::try_from(value){
            Ok(chan) => Ok(Box::new(chan)),
            Err(err) => Err(err),
        }
    }
    pub async fn get_next_results(&self, endpoint: PipedEndpoint, query_param: String) -> Result<Box<dyn NextResultTrait>, ApiError>{
        let result = self.get_resource(endpoint.get_endpoint_uri() + &query_param, None).await;
        let value = match result {
            Ok(val) => val,
            Err(err) => return Err(err),
        };
        match Next::try_from(value){
            Ok(chan) => Ok(Box::new(chan)),
            Err(err) => Err(err),
        }
    }
    pub async fn get_next_search_result(&self,query: String, filter: SearchFilter) -> Result<Box<dyn SearchResultTrait>, ApiError>{
        let result = self.get_resource("/nextpage/search?q=".to_owned() + &query, Some(format!("&filter={}", filter.get_filter()))).await;
        let value = match result {
            Ok(val) => val,
            Err(err) => return Err(err),
        };
        match Search::try_from(value){
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
            Ok(data) => match serde_json::from_str(&data.text().await.unwrap()){
                Ok(val) => Ok(val),
                Err(err) => {
                    Err(ApiError::new(Errors::ParsingError, format!("Line: {}, column: {} category: {:?}", err.line(), err.column(), err.classify())))
                },
            },
            Err(err) => Err(ApiError::new(Errors::RequestError, err.to_string())),
        }
    }
}