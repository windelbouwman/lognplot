
# logshipper

This utility enables shipping of log files into the lognplot GUI.

Building:

    $ cd logshipper
    $ cargo build --release

Example usage:

    $ strace --timestamps=unix,ns -o "|../target/release/logshipper" ls

This will `strace` the `ls` program and forward the events to the lognplot GUI.
