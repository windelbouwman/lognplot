
add_library(
    lognplot_client
    STATIC
    client.cpp
)

target_link_libraries(
    lognplot_client
    clognplot
)

target_include_directories(
    lognplot_client
    PUBLIC
    ../../demo/c
)

target_link_directories(
    lognplot_client
    PUBLIC
    ../../target/release
)
