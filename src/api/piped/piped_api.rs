use std::panic;

use serde_json::Value;

use crate::api::error::{ApiError, Errors};

use super::{channel::Channel, playlist::Playlist, video::Video, comment::Comments, misc::Next};

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
        PipedApi { client: self.client, response: None, api_host: self.api_host.unwrap()}
    }
    pub fn api_host(&self, api_host: String){
        self.api_host =Some(api_host);
    }
}

pub struct PipedApi{
    client: reqwest::Client,
    response: Option<reqwest::Response>,
    api_host: String,
}

impl PipedApi{
    pub async fn get_channel(&self, ucid: String) -> Result<Channel,ApiError>{
        let result = self.get_resource("/channels/".to_owned() + &ucid, None).await;
        let value = match result {
            Ok(val) => val,
            Err(err) => return Err(err),
        };
        match self.parse_channel(value){
            Ok(chan) => Ok(chan),
            Err(err) => Err(err),
        }
    }
    pub async fn get_playlist(&self, plid: String) -> Result<Playlist,ApiError>{
        let result = self.get_resource("/playlists/".to_owned() + &plid, None).await;
        let value = match result {
            Ok(val) => val,
            Err(err) => return Err(err),
        };
        match self.parse_playlist(value){
            Ok(val) => Ok(val),
            Err(err) => Err(err),
        }
    }
    pub async fn get_video(&self, vid: String) -> Result<Video,ApiError>{
        let result = self.get_resource("/streams/".to_owned() + &vid, None).await;
        let value = match result {
            Ok(val) => val,
            Err(err) => return Err(err),
        };
        match self.parse_video(value){
            Ok(val) => Ok(val),
            Err(err) => Err(err),
        }
    }
    pub async fn get_comments(&self, vid: String) -> Result<Comments,ApiError>{
        let result = self.get_resource("/comments/".to_owned() + &vid, None).await;
        let value = match result {
            Ok(val) => val,
            Err(err) => return Err(err),
        };
        match self.parse_comments(value){
            Ok(val) => Ok(val),
            Err(err) => Err(err),
        }
    }
    pub async fn get_next(&self, path: String,id: String, nextpage: String) -> Result<Next,ApiError>{
        let result = self.get_resource(path + &id, Some(nextpage)).await;
        let value = match result {
            Ok(val) => val,
            Err(err) => return Err(err),
        };
        match self.parse_next(value){
            Ok(val) => Ok(val),
            Err(err) => Err(err),
        }
    }
    
    fn parse_channel(&self, value: Value) -> Result<Channel,ApiError>{
        let channel = panic::catch_unwind(|| {
            return Channel::from(value)
        });
        match channel{
            Ok(channel) => Ok(channel),
            Err(_) => Ok(Channel::manual_parse(value)),
        }
    }
    fn parse_playlist(&self, value: Value) -> Result<Playlist,ApiError>{
        let playlist = panic::catch_unwind(|| {
            return Playlist::from(value)
        });
        match playlist{
            Ok(playlist) => Ok(playlist),
            Err(_) => todo!()//Ok(Playlist::manual_parse(value)),
        }
    }
    fn parse_next(&self, value: Value) -> Result<Next,ApiError>{
        let next = panic::catch_unwind(|| {
            return Next::from(value)
        });
        match next{
            Ok(next) => Ok(next),
            Err(_) =>todo!()// Ok(PlaylistNext::manual_parse(value)),
        }
    }
    fn parse_video(&self, value: Value) -> Result<Video,ApiError>{
        let video = panic::catch_unwind(|| {
            return Video::from(value)
        });
        match video{
            Ok(video) => Ok(video),
            Err(_) => todo!()//Ok(Video::manual_parse(value)),
        }
    }
    fn parse_comments(&self, value: Value) -> Result<Comments,ApiError>{
        let comments = panic::catch_unwind(|| {
            return Comments::from(value)
        });
        match comments{
            Ok(comments) => Ok(comments),
            Err(_) => todo!()//Ok(Comments::manual_parse(value)),
        }
    }
    async fn get_resource(&self,url: String, param: Option<String>) -> Result<Value,ApiError>{
        let url = match param{
            Some(param) => self.api_host.to_owned() + &url + &param,
            None => self.api_host.to_owned() + &url,
        };
        let res = self.client.get(url).send().await;
        match res{
            Ok(data) => match data.json::<Value>().await{
                Ok(val) => Ok(val),
                Err(err) => Err(ApiError{
                    kind: Errors::Parse,
                    recoverable: false,
                    message: err.to_string(),
                    action: crate::api::error::Actions::TryInvidious,
                }),
            },
            Err(err) => Err(ApiError{
                kind: Errors::Request,
                recoverable: false,
                message: err.to_string(),
                action: crate::api::error::Actions::TryInvidious,
            }),
        }
    }
}