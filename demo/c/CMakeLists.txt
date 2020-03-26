
cmake_minimum_required(VERSION 3.14)

project(demo)

add_executable(
    demo
    main.c
)

target_link_libraries(
    demo
    PRIVATE
    clognplot
)

target_link_directories(
    demo
    PRIVATE
    ../../target/release
)

if (MSVC)
    target_link_libraries(
        demo
        PRIVATE
        Ws2_32
        Userenv
    )
endif (MSVC)

if (UNIX)
    target_link_libraries(
        demo
        PRIVATE
        m
    )
endif (UNIX)
