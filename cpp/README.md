
# C++ integration

This folder contains C++ code for lognplot integration.

Folders:

- `example` contains a C++ lognplot client example.
- `lognplot` contains the C++ lognplot library, which you can use in your application.

# building

As a preparation, build the `clognplot` library:

    $ cd ../clognplot
    $ cargo build --release

Use cmake to build this folder:

    $ mkdir build
    $ cmake ..
    $ make

# Idea section

Use C++ with Qt for user interface.

Use QML for user interface.

Use RUST for library of time series database.

Use C++ to bind the two things together.

