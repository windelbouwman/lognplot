
[![Build Status](https://travis-ci.org/windelbouwman/lognplot.svg?branch=master)](https://travis-ci.org/windelbouwman/lognplot)
[![dependency status](https://deps.rs/repo/github/windelbouwman/lognplot/status.svg)](https://deps.rs/repo/github/windelbouwman/lognplot)
[![Documentation Status](https://readthedocs.org/projects/lognplot/badge/?version=latest)](https://lognplot.readthedocs.io)
[![docs.rs Documentation](https://docs.rs/lognplot/badge.svg)](https://docs.rs/lognplot)
[![crates.io page](https://img.shields.io/crates/v/lognplot.svg)](https://crates.io/crates/lognplot)
[![PyPI](https://img.shields.io/pypi/v/lognplot)](https://pypi.org/project/lognplot)

![logo](logo/logo.png)

Live timeseries analysis on your desktop!

# About

Lognplot is a graphical viewer for time series data. Unlike many other
projects in this area, lognplot is not a hosted web application.
It is a desktop application which visualizes your data from your
application. This can be an embedded, robotic, mobile, PC or PLC application.

Features:
- Plot data live
- Fast pan and zoom
- python GUI implementation (based on PyQt5)
- rust GUI implementation (based on gtk-rs / cairo)
- Send data over TCP/IP link to GUI.

# Video

These are recorded videos of the tool:

- [lognplot @ FOSDEM 2020](https://fosdem.org/2020/schedule/event/ema_lognplot/)

# Screenshots

This is an example screenshot of the lognplot python application, visualizing
10 million datapoints. Note that zooming is still smoothly performed.

![screenshot3](screenshots/screenshot3.png)

This is an example screenshot of the GTK gui implemented with rust, visualizing also 10 million
datapoints.

![screenshot4](screenshots/screenshot4.png)

This is an example of the plot window, when zoomed out.
Note that not all points are displayed, but aggregates
of the data are visualized as min/max/mean/stddev lines.

![screenshot1](screenshots/screenshot1.png)

When zooming into the data, the individual data points come
into picture.

![screenshot2](screenshots/screenshot2.png)

# Installation

Note that at this moment, you will want to grab the latest
git version from github.

First clone this repository:

    $ git clone https://github.com/windelbouwman/lognplot.git

For python, follow this steps:

    $ cd lognplot/python
    $ pip install --editable .
    $ python -m lognplot

For rust, follow these steps:

    $ cd lognplotgtk
    $ cargo run --release

Packages are released for rust and python

Rust crate: https://crates.io/crates/lognplot

Python package: https://pypi.org/project/lognplot/

## Requirements

For the GTK variant of the tool, you need the following to be installed:

- cairo-gobject-devel
- pango-devel
- atk-devel
- gdk-pixbuf2-devel
- gtk3-devel
- hdf5-devel

# Usage

To use the python side of this code, start as a demo the softscope:

    $ cd python
    $ python softscope.py

This will popup a plot window. Zooming and panning can be done with the keyboard
keys w,a,s,d and i,j,k,l. Press space or enter to autofit. The data is
a 10 kHz generated signal.

Another demo is the softscope server. This will open a TCP/IP port
which can receive data.

    $ cd python
    $ python -m lognplot

The softscope is now
ready to receive streaming data via network.

Next, start the demo datasource, which will send data via TCP to this GUI:

    $ cd demo
    $ python noize_source.py

Another server demo is the rust side of the code. Start the GUI like this:

    $ cd lognplotgtk
    $ cargo run

This application will be able to receive data via TCP/IP.

# Documentation

Documentation for python users can be found here: https://lognplot.readthedocs.io/en/latest/
Documentation for rust users can be found here: https://docs.rs/lognplot

# Plan

This is a list of things to do:

- PyQt5 implementation
- gtk-rs implementation

# Requirements for live data visualization

- Append data structure to enable appending new data
- Data point aggregation for zooming out and showing min/max/mean lines

# Similar projects

There is an interesting list of similar projects. Do you know of another
project? Please submit a pull request or an issue!

- [getcurve.io](https://getcurve.io/)
- [grafana](https://grafana.com/)
    - Web based
- [KST plot](https://kst-plot.kde.org/)
    - Can view large CSV files
- [plot juggler](https://github.com/facontidavide/PlotJuggler)
    - Qt interface
    - Interfaces with ROS
- [Plc-Lab](http://plc-lab.nl/)
    - Fancy GUI
- [RQT](https://wiki.ros.org/rqt)
    - Comes with ROS
- [sigrok](https://sigrok.org)
    - Slick Qt interface
    - Interfacing with many logic analyzers and oscilloscopes
- [speedscope](https://www.speedscope.app/)
    - Callstack tracer
- [trace compass](https://www.eclipse.org/tracecompass/)
    - Eclipse based
    - Supports many trace formats
- [tracy profiler](https://bitbucket.org/wolfpld/tracy)
