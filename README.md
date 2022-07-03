# About
Rusted-invidious is a fork or rather a rewrite of invidious which is an privacy focuse front-end for YouTube.
More information about the original project can be found at http://github.com/iv-org/invidious

This project aims to improve on it and to be node aware from the ground up wich means its meant to be ran on a Kubernetes cluster. It also aims to be completely compitable with the current API of invidious, configs files aren't the same but a migration tool will be provided(TBD).

If you're looking for a simpler solution, please consider using [invidious](http://github.com/iv-org/invidious) as this project is heavy when hosted on one machine.

That said this project is still in the gamma version and far away from useable.
# Local development
For local development you can the provided docker-compose file to start all services necessary for it to run but currently it DOES NOT COMPILE.
You may want to edited the nginx config file to fit your needs.
# Structure
The project consists of multiple services:
- [Notification service](https://github.com/11Tuvork28/rusted-invidious-notifications/)
- [Channel feed refresh service](https://github.com/11Tuvork28/rusted-invidious-refresh-feed/)
- The main app aka this repository
- (TBD)Job management wich manages each instance of the [Channel feed refresh service](https://github.com/11Tuvork28/rusted-invidious-refresh-feed/)
- (TBD)Proxy for the public api and images, this is intended to ran on servers spread around the world to provide exellent latencies.

This project also uses:
- Kafka for communicating internally between services and possible add futures features
- Scylla for the database as its suited to run on a Kubernetes cluster

A more detailed description can be found at [here](https://github.com/11Tuvork28/rusted-invidious/blob/main/notes.md)
# Config parameters that are unsupported or changed
- channel_threads => unsupported
- db_name => db_keyspace
- channel_refresh_interval => unsupported
- feed_threads => unsupported
- database_url => unsupported
- decrypt_pollin => unsupported
- full_refresh => unsupported
- dmca_content => unsupported
- check_tables => unsupported
- cache_annotation => unsupported
- force_resolve => unsupported
- pool_size => unsupported
- use_quic => unsupported
- cookies => unsupported

# Credits
A huge thanks to the [invidious](http://github.com/iv-org/invidious) and [NewPipe](https://github.com/TeamNewPipe/NewPipe) teams for doing the meger work of researching and reverse engineering YouTube in the first place.

Without them and especially without the [invidious team](http://github.com/iv-org/invidious) this project would not be possible.