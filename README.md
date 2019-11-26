
[![Build Status](https://travis-ci.org/windelbouwman/quartz.svg?branch=master)](https://travis-ci.org/windelbouwman/quartz)
[![dependency status](https://deps.rs/repo/github/windelbouwman/quartz/status.svg)](https://deps.rs/repo/github/windelbouwman/quartz)

# Lognplot

Timeseries database on your laptop!

Features:
- Attosecond timestamps
- Zoom levels
- Triggers on data
- Fast query of data
- python implementation
- rust implementation

Plan:
- Float only
- Demo log program

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

