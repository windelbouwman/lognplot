
# About

The lognplot library is a graphical viewer for time series data.
It is designed to work for desktop, and can be used to debug embedded
PLC, or robotic application.

Features:
- Send data over plain TCP/IP to GUI
- Two GUI implementations:
    - Python GUI based on PyQt5
    - Rust GUI based om gtk-rs / cairo
- Client libraries for:
    - Python
    - Rust (with C wrappers)
- Data adapters:
    - MQTT: subscribe to all topics and forwards signals
    - ROS (planned)
    - ADS (planned)

This is the python implementation, there is an redundant rust implementation
as well.

# Installation

To install this library use:

    $ pip install lognplot

# Example usage

To start a data visualization GUI, use the following command:

    $ python -m lognplot

To send data from a python script to this GUI, use for example:

```python
lognplot_client = LognplotTcpClient(hostname='localhost')
lognplot_client.connect()
lognplot_client.send_sample("test signal", timestamp=100.0, value=3.14)
```
