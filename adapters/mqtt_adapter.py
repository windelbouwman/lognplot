""" MQTT adapter.

This script subscribes to the MQTT broker, all topics will
be subscribed and forwarded to the plotting tool.

"""

# TODO: implement this :)
# from mqtt import client?

from lognplot.client import LognplotTcpClient


def main():
    client = LognplotTcpClient()
    client.connect()
    topic = 'test'
    value = 1337.0
    client.send_sample(topic, 10.0, value)


if __name__ == "__main__":
    main()
