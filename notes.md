## Job Manger
- api to register new channel jobs
- open sockets to workers to push new jobs
- round coubin
- worker nodes register themselfes 
- gives out jobs to worker nodes with shard id 
- crawlse database for jobs on start and receives new when a channel gets subscribed

## API 
- rate limit open endpoints
- rate limit per user
- jwt auth
- geo load balancing
- db manager for user, watched, subscriptions, notifications


## main app
- disallow non proxied video streaming
- use api for notifications, watched, subscribe, add/delete to playlist
- query  proxies for yt calls if possible 
- serves static files, cache busting via filenames that change via ci if files changed
- discovery of instance playlists
- play next button to create a temporary playlist
- predict in cache via referer 


## Worker node
- prefetch new uploaded videos and save to database
- check every 15 and every 1 minute around predicted upload times per channel
- serverside rendering 


## Proxy
- runs api
- proxies images/thumbnails etc
- caches images
- precache images for popular videos and new uploaded videos if channel subscribed
- nginx + local api server as upstream for fetching cache missing content and precaching
- /static maps to cache for images and static files css etc / api to api
- locations usa, europe



## innertube
Don't know if it can be useful, but when using a Safari desktop UA on web player requests of the InnerTube API, you can get a HLS master playlist with a video and audio merged into the same file inside the player response (like for lives and post lives) (itags 91-96 and 300-301) (contrary to the one of the iOS client, on which audio and video streams are separated)!