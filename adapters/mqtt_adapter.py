"""MQTT adapter.

This script subscribes to the MQTT broker, by default all topics will
be subscribed and forwarded to the plotting tool.

Example usage:
    python mqtt_adapter.py test.mosquitto.org --topic "#"

"""
from lognplot.client import LognplotTcpClient
import paho.mqtt.client as mqtt
import time
import argparse


def main():
    parser = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )

    parser.add_argument("mqtt_hostname", help="Hostname of the mqtt server", type=str)
    parser.add_argument(
        "--mqtt-port", help="Port of the mqtt server", default=1883, type=int
    )
    parser.add_argument("--topic", help="Topic to subscribe to", default="#", type=str)
    parser.add_argument("--lognplot-hostname", default="127.0.0.1", type=str)
    parser.add_argument("--lognplot-port", default="12345", type=int)
    args = parser.parse_args()

    lognplot_client = LognplotTcpClient(
        hostname=args.lognplot_hostname, port=args.lognplot_port
    )
    lognplot_client.connect()

    # The callback for when the client receives a CONNACK response from the server.
    def on_connect(client, userdata, flags, rc):
        print("Connected with result code " + str(rc))

        # Subscribing in on_connect() means that if we lose the connection and
        # reconnect then subscriptions will be renewed.
        client.subscribe(args.topic)

    # The callback for when a PUBLISH message is received from the server.
    def on_message(client, userdata, msg):
        try:
            value = float(msg.payload)
        except ValueError:
            pass
        else:
            timestamp = time.time()
            topic_name = "/mqtt{}".format(msg.topic)
            lognplot_client.send_sample(topic_name, timestamp, value)

    mqtt_client = mqtt.Client()
    mqtt_client.on_connect = on_connect
    mqtt_client.on_message = on_message

    mqtt_client.connect(args.mqtt_hostname, args.mqtt_port, 60)

    # Blocking call that processes network traffic, dispatches callbacks and
    # handles reconnecting.
    mqtt_client.loop_forever()


if __name__ == "__main__":
    main()
