.. lognplot documentation master file, created by
   sphinx-quickstart on Tue Nov 26 19:31:22 2019.
   You can adapt this file completely to your liking, but it should at least
   contain the root `toctree` directive.

Welcome to lognplot's documentation!
====================================

Introduction
------------

The lognplot package aims to ease the logging and plotting
of data of real-time systems. It stores the data in an easy
zoomable form, such that panning and scrolling in the data
works well, even with larger data sets of millions of points.

The project contains:

- Time series database like structures
- PyQt5 widgets for chart rendering

Installation
------------

Install the lognplot library with pip like this:

.. code:: bash

    $ pip install lognplot

Basic example
-------------

To emit measured data from python to a lognplot server/GUI
use the following:

.. code:: python

    import time
    from lognplot.client import LognplotTcpClient

    client = LognplotTcpClient('127.0.0.1', 12345)
    client.connect()

    while True:
        # Complicated code goes here :)

        some_variable = 3.14
        client.send_sample('my_channel', time.time(), some_variable)


Now start the server / GUI tool to receive this data. For example,
one could use the rust implementation of this GUI. When the GUI
is running, run your script and benefit from logging.

Table of Contents
-----------------

.. toctree::
    :maxdepth: 2
    :caption: Contents:

    motivation
    reference


Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`
