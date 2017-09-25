# DDOS

DDOS (**D**ynamic **Do**main **S**erver) is a dynamic DNS client which is configurable in lua and provides a RESTful API to let clients change their IPs. It is supposed to be used as a very minimal dynamic DNS setup where a home-server can report it's IP address to a DNS in the cloud which will route all future requests to said IP.

It is written in Rust and supports custom configuration scripts written in lua.

