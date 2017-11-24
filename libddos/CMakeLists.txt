cmake_minimum_required(VERSION 2.6)
add_library(ddos STATIC ddos.c)
target_include_directories(ddos PUBLIC ".")
install(TARGETS ddos DESTINATION ".")
