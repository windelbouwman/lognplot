
# About

Welcome to the adapter folder. There is a theme going on here.
The basic idea is to subscribe to a particular data network
and forward everything or a selection to the lognplot system.
This has the benefit the lognplot tool remains fairly simple
and it also means you can drop your data into the plot with
a simple python scope.

For example, to subscribe to all topics on the MQTT test server
and forward this data to the lognplot GUI via TCP/IP, use the
following command:

    $ python mqtt_adapter.py test.mosquitto.org --topic "#"

Note that this might cause severe traffic, so be sure you can
spare the bandwidth.

List of adapters:

- MQTT
- ADS (planned)
- ROS2 (planned)
