

Motivation
==========

Debugging real-time systems is not always easy.

When dealing with real-time systems, consisting
of different computers, several processes and software
components written in different programming languages,
one often needs to debug these systems and figure out
what is going on. Simply halting a process with
a debugger is not an option, since the other
processes will continue. Also, you might have a robotic
system which will continue to move when the program
is being halted.

To debug these kind of systems, one can use several
techniques:

- debug printf: simply print lines to a console
- logging: log events to a text file
- print values in a CSV file

Most of these methods have some downsides.

A better option would be to stream data to a
GUI and visualize it right away. This is exactly
was lognplot aims to do.

