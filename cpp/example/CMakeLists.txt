
add_executable(
    client_demo
    demo.cpp
)

target_include_directories(
    client_demo
    PRIVATE
    ..
)

target_link_libraries(
    client_demo
    lognplot_client
)