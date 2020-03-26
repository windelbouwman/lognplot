
# C client demo

This is a demo on how to integrate the lognplot client into
your C application.

# Build preparations

In order to run this demo, you require the `clognplot` dll to be
in place. Enter the `clognplot` folder and build the project
in release mode.

    $ cd ../../clognplot
    $ cargo build --release

# Usage on linux

On linux, use make:

    $ make

Now run the demo:

    $ ./demo

# Usage on windows

For windows, you can use CMake with visual studio.

    $ mkdir build
    $ cd build
    $ cmake ..

Now open the visual studio solution, and run the demo from there.

# Usage on MAC

TODO