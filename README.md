# About
Rusted-invidious is a fork or rewrite of invidious, a privacy-focused front-end for YouTube.
More information about the original project can be found at http://github.com/iv-org/invidious.

This project aims to improve it and be node aware from the ground up, which means running it on a Kubernetes cluster. It also aims to be fully compatible with the current API of invidious, the configuration files are not the same but a migration tool is provided (TBD).

If you are looking for a simpler solution, consider using [invidious](http://github.com/iv-org/invidious), as this project is very heavy when hosted on a machine.

That being said, this project is still in gamma and far from usable.
# Local development
For local development, you can use the provided docker-compose file to start all the necessary services, but currently it is NOT COMPILED.
You may want to modify the nginx configuration file to suit your needs.
# Structure
The project consists of several services:
- [notification service](https://github.com/11Tuvork28/rusted-invidious-notifications/)
- [Channel feed update service](https://github.com/11Tuvork28/rusted-invidious-refresh-feed/)
- The main application aka this repository.
- (TBD)Job management that manages each instance of the [Channel feed refresh service](https://github.com/11Tuvork28/rusted-invidious-refresh-feed/)
- (TBD)Proxy for the public API and images, this is to run on servers distributed around the world to ensure excellent latency.

This project also uses:
- Kafka for internal communication between services and possible future features.
- Scylla for the database, which is suitable for running in a Kubernetes cluster

A more detailed description can be found [here](https://github.com/11Tuvork28/rusted-invidious/blob/main/notes.md)
# config parameters that are not supported or changed.
- channel_threads => not supported
- db_name => db_keyspace
- channel_refresh_interval => not supported
- feed_threads => not supported
- database_url => not supported
- decrypt_pollin => not supported
- full_refresh => not supported
- dmca_content => not supported
- check_tables => not supported
- cache_annotation => not supported
- force_resolve => not supported
- pool_size => not supported
- use_quic => not supported
- cookies => not supported

# credits
A big thank you to the [invidious](http://github.com/iv-org/invidious) and [NewPipe](https://github.com/TeamNewPipe/NewPipe) teams who did the main work in researching and reverse engineering YouTube.

Without them and especially without the [invidious team](http://github.com/iv-org/invidious) this project would not have been possible.
