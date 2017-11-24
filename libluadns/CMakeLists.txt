cmake_minimum_required(VERSION 2.6)
project(luadns)

add_library(luadns STATIC luadns.c)
include_directories(/usr/include/luajit-2.0)
target_link_libraries(luadns luajit-5.1)
target_include_directories(luadns PUBLIC ".")
install(TARGETS luadns DESTINATION ".")

add_executable(luadnsd dnsd.c)
target_link_libraries(luadnsd luadns)
