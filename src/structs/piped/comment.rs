pub struct  Comments{
    comments: Vec<Comment>, // A list of comments
    disabled: bool, // Whether or not the comments are disabled
    nextpage: String // A JSON encoded page, which is used for the nextpage endpoint.
}
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