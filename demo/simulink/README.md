
# Simulink C-API proof of concept

Idea: use the simulink generated C-API and forward all signals to lognplot using
the C-API.

The simulink coder can generate a C-API which contains structures with information about
signals present in the code. We could iterate this information, and forward
signal values to the lognplot system.

Idea is to write a bridge software which queries simulink, and use the C api of
lognplot to emit the data outwards.

This example does not work out of the box, but requires you to copy and paste some code
from the file `bridge.c` into your own projects glue code.

# Compilation guide

The compilation of this code is rather complex. This simple three step plan might
be of help.

Step 1: generate simulink code, and make sure you can compile and run it.

Step 2: build the clognplot rust project, this will be a static archive linkable in C.

Step 3: Include some code from [`bridge.c`](bridge.c) into your application, add the proper include paths
and libraries, and you should be good to go.
