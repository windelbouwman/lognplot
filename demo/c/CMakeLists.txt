
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
