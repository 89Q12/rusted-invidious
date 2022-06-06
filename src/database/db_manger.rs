use scylla::{Session, batch::Batch, prepared_statement::PreparedStatement, SessionBuilder, transport::errors::{NewSessionError, QueryError}, frame::request::Query};
use tracing::Level;

use super::models::video::Video;


pub struct DbManager {
    session: Session,
    batch_insert: Batch,
    prepared_statements: Vec<PreparedStatement>,

}

impl DbManager {
    /// Initializes a new DbManager struct and creates the database session
    pub async fn new(uri: &str, known_hosts: Option<Vec<String>>) -> Result<Self, NewSessionError>{
        let session_builder;
        if known_hosts.is_some(){
            session_builder = SessionBuilder::new().known_node(uri).known_nodes(&known_hosts.unwrap());
        }else{
            session_builder = SessionBuilder::new().known_node(uri);
        }
 
        match session_builder.build().await {
            Ok(session) => Ok(Self { session, batch_insert: Default::default(), prepared_statements: Vec::new() }),
            Err(err) => Err(err),
        }
    }
    /// Prepares all select statements used throughout the handlers
    /// Prepared statements are stored in in a vec and the indices are statically maped to specific statements
    /// E.g. index 0 will be the statement to get a video by id from the database
    pub fn init_prepared_statements(&self){
        todo!()
    }
    /// Checks if the database exists if not it creates a new keyspace and creates all tables.
    /// With the assumption that the database is a single node cluster.
    /// Should only be called once and before the web server starts
    pub async fn init_database(&self) -> Result<(), QueryError>{
        let keyspace = self.session
        .query(
            "CREATE KEYSPACE IF NOT EXISTS rusted_invidious WITH REPLICATION = \
            {'class' : 'SimpleStrategy', 'replication_factor' : 1}",
            &[],
        )
        .await;
        // Matches the keyspace query, if it fails then the db is already created and the fn returns
        match keyspace {
            Ok(_) => tracing::event!(target:"db", Level::DEBUG, "Successfully created keyspace"),
            Err(err) => return Err(err),
        }
        // Creates the videos table
        self.session
        .query(
            "CREATE TABLE IF NOT EXISTS rusted_invidious.videos (video_id ASCII primary key, updated_at timestamp, channel_id ASCII, title TEXT, \
                likes ASCII, view_count bigint,description TEXT, length_in_seconds bigint, genere ASCII, genere_url ASCII, license ASCII, author_verified boolean, \
                subcriber_count bigint, author_name TEXT, author_thumbnail_url ASCII, is_famliy_safe boolean, publish_date timestamp, \
                formats TEXT, storyboard_spec_url ASCII, continuation_related TEXT, continuation_comments TEXT)",
            &[],
        )
        .await?;
        // Creates the users table
        self.session
        .query(
            "CREATE TABLE IF NOT EXISTS rusted_invidious.users (name TEXT, uuid UUID,created_at timestamp, password TEXT, token TEXT,feed_needs_update boolean, \
                PRIMARY KEY(uuid, name)",
            &[],
        )
        .await?;

        self.session
        .query(
            "CREATE TABLE IF NOT EXISTS rusted_invidious.username_uuid (name TEXT PRIMARY KEY, uuid UUID)",
            &[],
        )
        .await?;
        self.session
        .query(
            "CREATE TABLE IF NOT EXISTS rusted_invidious.user_subscriptions (uuid UUID PRIMARY KEY, channel_id ASCII)",
            &[],
        )
        .await?;
        self.session
        .query(
            "CREATE TABLE IF NOT EXISTS rusted_invidious.user_subscriptions (uuid UUID PRIMARY KEY, channel_id ASCII)",
            &[],
        )
        .await?;
        self.session
        .query(
            "CREATE TABLE IF NOT EXISTS rusted_invidious.user_watched (uuid UUID PRIMARY KEY, video_id ASCII)",
            &[],
        )
        .await?;
        self.session
        .query(
            "CREATE TABLE IF NOT EXISTS rusted_invidious.sessions (uuid UUID, session_id ASCII, issued timestamp, PRIMARY KEY(uuid, session_id))",
            &[],
        )
        .await?;
        self.session
        .query(
            "CREATE TABLE IF NOT EXISTS rusted_invidious.channels (channel_id ASCII, updated_at timestamp, subscribed_at timestamp, author_name TEXT, PRIMARY KEY(channel_id))",
            &[],
        )
        .await?;
        self.session
        .query(
            "CREATE TABLE IF NOT EXISTS rusted_invidious.channel_videos (video_id ASCII, updated_at timestamp, channel_id ASCII, title TEXT, \
                likes ASCII, view_count bigint,description TEXT, length_in_seconds bigint, genere ASCII, genere_url ASCII, license ASCII, author_verified boolean, \
                subcriber_count bigint, author_name TEXT, author_thumbnail_url ASCII, is_famliy_safe boolean, publish_date timestamp, \
                formats TEXT, storyboard_spec_url ASCII, live boolean, premiere_timestamp timestamp, PRIMARY KEY((channel_id), video_id, updated_at))",
            &[],
        )
        .await?;
        self.session
        .query(
            "CREATE TABLE IF NOT EXISTS rusted_invidious.nonce (nonce TEXT, issued timestamp, PRIMARY KEY(nonce))",
            &[],
        )
        .await?;
        
        Ok(())
    }
    /// gets a video from the database fails if there is no result
    pub async fn get_video(&self, video_id: String) -> Result<Video, QueryError> {
        todo!()
    }
    /// gets a channel video from the database fails if there is no result
    pub async fn get_channe_video(&self, video_id: String, channel_id: String) -> Result<Video, QueryError> {
        todo!()
    }
    /// gets a video from the database
    pub async fn get_channel(&self, channel_id: String) -> Result<Video, QueryError> {
        todo!()
    }
    /// gets a video from the database
    pub async fn get_user(&self, username: String) -> Result<Video, QueryError> {
        todo!()
    }
    /// Add watched video to the database
    pub async fn add_watched(&self, video_id: String) -> Result<bool, QueryError> {
        todo!()
    }
    /// Add subscription to the database
    pub async fn add_subscription(&self, channel_id: String, username: String) -> Result<bool, QueryError> {
        todo!()
    }
    

}