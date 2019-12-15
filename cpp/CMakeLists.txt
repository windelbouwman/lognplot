
cmake_minimum_required(VERSION 3.11.0)

project(lognplot)

add_executable(
    lognplot
    main.cpp
)

find_package(Qt5 COMPONENTS Widgets REQUIRED)

target_link_libraries(
    lognplot
    Qt5::Widgets
)
