
add_executable(
    lognplot_gui
    main.cpp
)

find_package(Qt5 COMPONENTS Widgets REQUIRED)

target_link_libraries(
    lognplot_gui
    Qt5::Widgets
    clognplot
    pthread
    dl
)

target_link_directories(
    lognplot_gui
    PRIVATE
    ../../target/release
)
