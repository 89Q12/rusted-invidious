use serde::Deserialize;
use serde_json::Value;
use crate::api::CommentsTrait;
use crate::api::CommentTrait;
use crate::api::error::{ApiError, Errors};
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct  Comments{
    comments: Vec<Comment>, // A list of comments
    disabled: bool, // Whether or not the comments are disabled
    nextpage: String // A JSON encoded page, which is used for the nextpage endpoint.
}

#[derive(Deserialize,Clone)]
#[serde(rename_all = "camelCase")]
pub struct Comment{
    author: String,// The name of the author of the comment
    comment_id: String, // The comment ID
    comment_text: String, // The text of the comment
    commented_time: String, // The time the comment was made
    commentor_url: String, // The URL of the channel of the comment
    hearted: bool, // Whether or not the comment has been hearted
    like_count: i32, // The number of likes the comment has
    pinned: bool, // Whether or not the comment is pinned
    thumbnail: String, // The thumbnail of the comment
    verified: bool // Whether or not the author of the comment is verified
}

impl TryFrom<Value> for Comments{
    type Error = ApiError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match serde_json::from_value(value){
            Ok(val) => Ok(val),
            Err(err) => Err(ApiError::new(Errors::RequestError, err.to_string())),
        }
    }
}

impl TryFrom<Value> for Comment{
    type Error = ApiError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match serde_json::from_value(value){
            Ok(val) => Ok(val),
            Err(err) => Err(ApiError::new(Errors::RequestError, err.to_string())),
        }
    }
}

impl CommentsTrait for Comments {
    fn is_disabled(&self) -> bool {
        self.disabled.clone()
    }

    fn get_comments(&self) -> Vec<Box<dyn CommentTrait>> {
        let mut ret_vec = Vec::new();
        for comment in self.comments.to_owned(){
            ret_vec.push(Box::new(comment) as Box<dyn CommentTrait>);
        };
        ret_vec
    }

    fn get_nextpage(&self) -> String {
        self.nextpage.clone()
    }
}
impl CommentTrait for Comment{
    fn get_author_name(&self) -> String {
        self.author.clone()
    }

    fn get_author_url(&self) -> String {
        self.commentor_url.clone()
    }

    fn get_author_avatar_url(&self) -> String {
        self.thumbnail.clone()
    }

    fn get_id(&self) -> String {
        self.comment_id.clone()
    }

    fn is_hearted(&self) -> bool {
        self.hearted.clone()
    }

    fn is_pineed(&self) -> bool {
        self.pinned.clone()
    }

    fn get_like_count(&self) -> i32 {
        self.like_count.clone()
    }

    fn is_author_verified(&self) -> bool {
        self.verified.clone()
    }

    fn get_comment_text(&self) -> String {
        self.comment_text.clone()
    }

    fn get_posted_date(&self) -> String {
        self.commented_time.clone()
    }
}