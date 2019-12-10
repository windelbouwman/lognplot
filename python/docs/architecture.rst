
Architecture
============

This section describes how the lognplot software is composed.

Overview
--------

Top level, the software is designed as such that the lognplot library
can be included in an application under test, or a script which
can measure a system. This script will then transmit its measurements
to a another lognplot applicatition, which will store the data
and enable visualization of the data.

Multiple programming languages
------------------------------

Firstly, the software is implemented twice. Once in python and
once in rust. The reason for doing so is that python is a good
choice for rapid prototyping. The rust implementation is most
likely to lag behind, but will probably be more stable and have
a higher performance.

Another benefit is that python developers can include the software
in a way they are familiar with. The same goes for rust developers.

Applications and libraries
--------------------------

Primarily lognplot is a library. This means it can be included
in other applications. The primary application where this library
is used, is in our own application, which is the showcase of the
lognplot library.

Data handling
-------------

To be able to receive data from a wide variety of loggers and trace
tools, a TCP/IP connection was chosen,
as :ref:`described here <protocol>`.

Library contents
----------------

The lognplot library contains the following features:

- Time series database: an in memory database for time series
- Charting: functionality to assist in the creation of plots
- Qt rendering (python only): several widgets for plot visualization
