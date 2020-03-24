
# .NET client implementation for lognplot

This folder contains a lognplot client library and an
example application written in C#, sending data to
the lognplot GUI.

There are two ways to implement this:

- Wrap the `clognplot` crate, and call it's functions from C#.
- Implement the trace protocol client in C#.

## Wrap clognplot

To use this method, first build the `clognplot.dll` file as follows:

    CMD> cd clognplot
    CMD> cargo build --release

Now copy the file `target/release/clognplot.dll` into the folder
`dotnet/LognplotClient`.



