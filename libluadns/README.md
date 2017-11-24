# luadns

A lua scriptable server to answer A/AAAA DNS requests. Simple backend for [ddos](https://github.com/spacekookie/ddos)


## How to build

You will need to have several dependencies installed

 - cmake
 - gcc 5+
 - luajit-2.0 headers

On Ubuntu you can install all that with

```
$ sudo apt install libluajit-5.1-dev cmake gcc
```

## How to use

Place an `init.lua` file next to the `luadnsd` binary and run it. Please look at the example lua file contained in this repository for more information.