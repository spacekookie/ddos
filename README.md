# DDOS

DDOS (**D**ynamic **Do**main **S**erver) is a dynamic DNS client which is configurable in lua and provides a RESTful API for remote host configuration. It is a very minimal dynamic DNS setup where a home-server can report it's IP address to a DNS in the cloud which will route all future requests to said IP.

It is written in Rust and supports custom configuration scripts written in lua.


## How to use

```
git clone https://github.com/spacekookie/ddos
cd ddos/
git submodule init
git submodule update
```

You will need to have a Rust 1.x toolchain installed to build ddos and `cmake` and a modern `gcc` (version 5.0+). For more details see the [luadns]() dependencies. To prepare running ddos place a `rules.lua` (or similar) file somewhere which you provide as a commandline argument.

On the lua side of things you need to have `cjson` installed as an additional dependency. You can usually do so via `luarocks instal`

If you want to provide your own functionality for `luadns` you need to provide a file which exposes two functions to call.

```lua
ddos = require 'ddos'

function AQuery(record, addr)
  -- ...
  ddos.AQuery(record, addr)
end

function AAAAQuery(record, addr)
  - ...
  ddos.AAAAQuery(record, addr)
end
```

If you don't want to do custom processing you can just include `ddos.lua` which will provide basic functionality as intended. Otherwise you will need to call the `ddos` implementations.

Most things will be assumed by default for you. So the simplest `ddos` setup is the following call which will start the ddos process forked to the background with the default REST API port (8001) and the default configuration file (`ddos.lua`) and the 

```console
$ ~> ./target/debug/ddos start
```