use scylla::{Session, prepared_statement::PreparedStatement, SessionBuilder, transport::{errors::{NewSessionError, QueryError}, query_result::FirstRowError}, IntoTypedRows, cql_to_rust::FromRowError, frame::value::Timestamp};
use tracing::Level;
use super::models::{video::Video, channel_video::ChannelVideo, channel::Channel, user::DBUser, username_uuid::{self, UsernameUuid}};

#[derive(Debug)]
pub enum DbError {
    QueryError(QueryError),
    FromRowError(FromRowError),
    FirstRowError(FirstRowError),
}
pub struct DbManager {
    session: Session,
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
            Ok(session) => {
                Ok(Self { session, prepared_statements: Vec::new() })
            },
            Err(err) => Err(err),
        }
    }

    pub async fn use_keyspace(&self){
        match self.session.use_keyspace("rusted_invidious", false).await{
            Ok(_) => tracing::event!(target:"db", Level::DEBUG, "Successfully set keyspace"),
            Err(_) => panic!("KESPACE NOT FOUND EXISTING...."),
        }
    }
    /// Prepares all select statements used throughout the handlers
    /// Prepared statements are stored in in a vec and the indices are statically maped to specific statements
    /// Not fancy but fast
    /// E.g. index 0 will be the statement to get a video by id from the database
    /// 0 =get_video 1= get_user 2 = get_user_uid 3 = get_channel_video 4 = get_channel 5 = get_session 
    /// 6= insert_video 7 insert_user 8 = insert_user_uid, 9 = insert_channel_video 10 = insert_channel 11 = insert_session 12 = insert_subscription 13 = insert_watched
    pub async fn init_prepared_statements(&mut self){
        let get_video = self.session.prepare("SELECT * FROM videos WHERE video_id = ?");
        let get_user = self.session.prepare("SELECT * FROM users WHERE uid = ? and name = ?");
        let get_user_uid = self.session.prepare("SELECT uid FROM username_uuid WHERE name = ?");
        let get_channel_video = self.session.prepare("SELECT * FROM channel_videos WHERE video_id = ? and channel_id = ? and updated_at < ?");
        let get_channel = self.session.prepare("SELECT * FROM channels WHERE channel_id = ?");
        let get_session = self.session.prepare("SELECT issued FROM sessions WHERE uid = ? and session_id = ?");
        
        let insert_video = self.session.prepare("INSERT INTO videos (video_id, updated_at, channel_id, title, likes, view_count,\
             description, length_in_seconds, genere, genre_url, license, author_verified, subcriber_count, author_name, author_thumbnail_url, is_famliy_safe, publish_date, formats, storyboard_spec_url, continuation_related, continuation_comments ) \
        VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)");
        let insert_user = self.session.prepare("INSERT INTO users (uid, name, created_at, password, access_token, feed_needs_update) VALUES(?,?,?,?,?,?)");
        let insert_user_uid = self.session.prepare("INSERT INTO username_uuid (name, uid) VALUES (?,?)");
        let insert_channel_video = self.session.prepare("INSERT INTO channel_videos (video_id, updated_at, channel_id, title, likes, view_count,\
            description, length_in_seconds, genere, genre_url, license, author_verified, subcriber_count, author_name, author_thumbnail_url, is_famliy_safe, publish_date, formats, storyboard_spec_url, live, premiere_timestamp ) \
       VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)");
        let insert_channel = self.session.prepare("INSERT INTO channels (channel_id, updated_at, subscribed_at, author_name) VALUES (?,?,?,?)");
        let insert_session = self.session.prepare("INSERT INTO sessions (uid, session_id, issued) VALUES(?,?,?)");
        let insert_subscription = self.session.prepare("INSERT INTO user_subscriptions (uid, channel_id) VALUES(?,?)");
        let insert_watched = self.session.prepare("INSERT INTO user_watched (uid, video_id) VALUES(?,?)");

        let results = tokio::join!(get_video, get_user, get_user_uid, get_channel_video, get_channel,get_session, 
            insert_video,insert_user,insert_user_uid, insert_channel_video, insert_channel, insert_session, insert_subscription, insert_watched);

        self.prepared_statements.push(results.0.unwrap());
        self.prepared_statements.push(results.1.unwrap());
        self.prepared_statements.push(results.2.unwrap());
        self.prepared_statements.push(results.3.unwrap());
        self.prepared_statements.push(results.4.unwrap());
        self.prepared_statements.push(results.5.unwrap());
        self.prepared_statements.push(results.6.unwrap());
        self.prepared_statements.push(results.7.unwrap());
        self.prepared_statements.push(results.8.unwrap());
        self.prepared_statements.push(results.9.unwrap());
        self.prepared_statements.push(results.10.unwrap());
        self.prepared_statements.push(results.11.unwrap());
        self.prepared_statements.push(results.12.unwrap());
        self.prepared_statements.push(results.13.unwrap());

    }
    /// Checks if the database exists if not it creates a new keyspace and creates all tables.
    /// With the assumption that the database is a single node cluster.
    /// Should only be called once and before the web server starts
    pub async fn init_database(&self) -> Result<(), QueryError>{
        let keyspace = self.session
        .query(
            "CREATE KEYSPACE IF NOT EXISTS rusted_invidious WITH REPLICATION = \
            {'class' : 'SimpleStrategy', 'replication_factor' : 1};",
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
            "CREATE TABLE IF NOT EXISTS rusted_invidious.videos (video_id TEXT primary key, updated_at timestamp, channel_id TEXT, title TEXT, \
                likes TEXT, view_count TEXT,description TEXT, length_in_seconds TEXT, genere TEXT, genre_url TEXT, license TEXT, author_verified boolean, \
                subcriber_count TEXT, author_name TEXT, author_thumbnail_url TEXT, is_famliy_safe boolean, publish_date TEXT, \
                formats TEXT, storyboard_spec_url TEXT, continuation_related TEXT, continuation_comments TEXT);",
            &[],
        )
        .await?;
        // Creates the users table
        self.session
        .query(
            "CREATE TABLE IF NOT EXISTS rusted_invidious.users (uid UUID, name TEXT,created_at timestamp, password TEXT, access_token TEXT,feed_needs_update boolean, PRIMARY KEY(uid, name));",
            &[],
        )
        .await?;

        self.session
        .query(
            "CREATE TABLE IF NOT EXISTS rusted_invidious.username_uuid (name TEXT PRIMARY KEY, uid UUID);",
            &[],
        )
        .await?;
        self.session
        .query(
            "CREATE TABLE IF NOT EXISTS rusted_invidious.user_subscriptions (uid UUID,subuuid UUID, channel_id ASCII, PRIMARY KEY(uid, subuuid));",
            &[],
        )
        .await?;
        self.session
        .query(
            "CREATE TABLE IF NOT EXISTS rusted_invidious.user_watched (uid UUID PRIMARY KEY, video_id ASCII);",
            &[],
        )
        .await?;
        self.session
        .query(
            "CREATE TABLE IF NOT EXISTS rusted_invidious.sessions (uid UUID, session_id ASCII, issued timestamp, PRIMARY KEY(uid, session_id));",
            &[],
        )
        .await?;
        self.session
        .query(
            "CREATE TABLE IF NOT EXISTS rusted_invidious.channels (channel_id ASCII, updated_at timestamp, subscribed_at timestamp, author_name TEXT, PRIMARY KEY(channel_id));",
            &[],
        )
        .await?;
        self.session
        .query(
            "CREATE TABLE IF NOT EXISTS rusted_invidious.channel_videos (video_id ASCII, updated_at timestamp, channel_id ASCII, title TEXT, \
                likes ASCII, view_count bigint,description TEXT, length_in_seconds bigint, genere ASCII, genre_url ASCII, license ASCII, author_verified boolean, \
                subcriber_count bigint, author_name TEXT, author_thumbnail_url ASCII, is_famliy_safe boolean, publish_date timestamp, \
                formats TEXT, storyboard_spec_url ASCII, live boolean, premiere_timestamp timestamp, PRIMARY KEY((channel_id), video_id, updated_at));",
            &[],
        )
        .await?;
        self.session
        .query(
            "CREATE TABLE IF NOT EXISTS rusted_invidious.nonce (nonce TEXT, issued timestamp, PRIMARY KEY(nonce));",
            &[],
        )
        .await?;
        
        Ok(())
    }
    pub async fn drop_database(&self)-> Result<(), QueryError>{
        let result = self.session
        .query(
            "DROP KEYSPACE rusted_invidious",&[]
        )
        .await;
        match result {
            Ok(_) => tracing::event!(target:"db", Level::DEBUG, "Successfully created keyspace"),
            Err(err) => return Err(err),
        }
        Ok(())
    }
    /// gets a video from the database fails if there is no result
    pub async fn get_video(&self, video_id: String) -> Result<Video, DbError> {
        let res = match self.session.execute(&self.prepared_statements.get(0).unwrap(), (video_id,)).await{
            Ok(res) => res,
            Err(err) => return Err(DbError::QueryError(err)),
        };
        let video: Video = match res.first_row() {
            Ok(row) => match row.into_typed::<Video>(){
                Ok(video) =>video,
                Err(err) => return Err(DbError::FromRowError(err)),
            },
            Err(err) => return Err(DbError::FirstRowError(err)),
        };
        Ok(video)
    }
    pub async fn insert_video(&self, video: Video) -> bool{
        match self.session.execute(&self.prepared_statements.get(6).unwrap(),
        (video,)).await{
            Ok(_) =>  true,
            Err(_) =>  false,
        }
    }
    /// gets a channel video from the database fails if there is no result
    pub async fn get_channe_video(&self, video_id: String, channel_id: String, updated_at: Timestamp) -> Result<ChannelVideo, DbError> {
        let res = match self.session.execute(&self.prepared_statements.get(3).unwrap(), (video_id,channel_id, updated_at,)).await{
            Ok(res) => res,
            Err(err) => return Err(DbError::QueryError(err)),
        };
        let video: ChannelVideo = match res.first_row() {
            Ok(row) => match row.into_typed::<ChannelVideo>(){
                Ok(video) =>video,
                Err(err) => return Err(DbError::FromRowError(err)),
            },
            Err(err) => return Err(DbError::FirstRowError(err)),
        };
        Ok(video)
    }
    /// gets a video from the database
    pub async fn get_channel(&self, channel_id: String) -> Result<Channel, DbError> {
        let res = match self.session.execute(&self.prepared_statements.get(4).unwrap(), (channel_id,)).await{
            Ok(res) => res,
            Err(err) => return Err(DbError::QueryError(err)),
        };
        let channel: Channel = match res.first_row() {
            Ok(row) => match row.into_typed::<Channel>(){
                Ok(channel) =>channel,
                Err(err) => return Err(DbError::FromRowError(err)),
            },
            Err(err) => return Err(DbError::FirstRowError(err)),
        };
        Ok(channel)
    }
    /// gets a video from the database
    pub async fn get_user(&self, username: String) -> Result<DBUser, DbError> {
        let uid = match self.get_user_uid(&username).await {
            Ok(value) => value,
            Err(err) => return Err(err),
        };


        let res = match self.session.execute(&self.prepared_statements.get(1).unwrap(), (uid,username,)).await{
            Ok(res) => res,
            Err(err) => return Err(DbError::QueryError(err)),
        };
        let user: DBUser = match res.first_row() {
            Ok(row) => match row.into_typed::<DBUser>(){
                Ok(user) =>user,
                Err(err) => return Err(DbError::FromRowError(err)),
            },
            Err(err) => return Err(DbError::FirstRowError(err)),
        };
        Ok(user)
    }

    async fn get_user_uid(&self, username: &String) ->  Result<UsernameUuid, DbError> {
        let res = match self.session.execute(&self.prepared_statements.get(2).unwrap(), (username,)).await{
            Ok(res) => res,
            Err(err) => return Err(DbError::QueryError(err)),
        };
        let uid: UsernameUuid = match res.first_row() {
            Ok(row) => match row.into_typed::<UsernameUuid>(){
                Ok(user) =>user,
                Err(err) => return Err(DbError::FromRowError(err)),
            },
            Err(err) => return Err(DbError::FirstRowError(err)),
        };
        Ok(uid)
    }
    /// Add watched video to the database
    pub async fn add_watched(&self, video_id: String,username: &String) -> Result<bool, DbError> {
        let uid = match self.get_user_uid(username).await {
            Ok(value) => value,
            Err(value) => return Err(value),
        };
        match self.session.execute(&self.prepared_statements.get(13).unwrap(),
        (uid,video_id,)).await{
            Ok(_) => return Ok(true),
            Err(err) => return Err(DbError::QueryError(err)),
        }
    }
    /// Add subscription to the database
    pub async fn add_subscription(&self, channel_id: String, username: String) -> Result<bool, DbError> {
        let uid = match self.get_user_uid(&username).await {
            Ok(value) => value,
            Err(value) => return Err(value),
        };
        match self.session.execute(&self.prepared_statements.get(12).unwrap(),
        (uid,channel_id,)).await{
            Ok(_) => return Ok(true),
            Err(err) => return Err(DbError::QueryError(err)),
        }
    }
}