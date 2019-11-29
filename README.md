
[![Build Status](https://travis-ci.org/windelbouwman/lognplot.svg?branch=master)](https://travis-ci.org/windelbouwman/lognplot)
[![dependency status](https://deps.rs/repo/github/windelbouwman/lognplot/status.svg)](https://deps.rs/repo/github/windelbouwman/lognplot)

# Lognplot

Timeseries database on your laptop!

Features:
- Attosecond timestamps
- Zoom levels
- Triggers on data
- Fast query of data
- python implementation (pyqt5)
- rust implementation (gtk-rs / cairo)

Plan:
- Float only
- Demo log program
- PyQt5 implementation
- gtk-rs implementation

# Usage

To use the python side of this code, start as a demo the softscope:

    $ cd python
    $ python softscope.py

This will popup a plot window. Zooming and panning can be done with the keyboard
keys w,a,s,d and i,j,k,l.

Another demo is the rust side of the code. Start the GUI like this:

    $ cargo run

Next, start the demo datasource, which will send data via TCP to this GUI:

    $ cd demo
    $ python noize_source.py

# Documentation

Documentation for python users can be found here: https://lognplot.readthedocs.io/en/latest/

## Similar projects

- plot juggler https://github.com/facontidavide/PlotJuggler
- grafana (https://grafana.com/)
- KST plot (https://kst-plot.kde.org/)
- tracy profiler (https://bitbucket.org/wolfpld/tracy)
- trace compass: https://www.eclipse.org/tracecompass/
- speedscope: https://www.speedscope.app/

## Project structure

The project is divided into several crates.

- A time series database ala sqlite to store time series and query them.
- A cairo based drawing

# Idea list

- use vulkano-rs instead of openGL
- use cassowary to layout parts of the GUI
- render pretty lines using shaders

