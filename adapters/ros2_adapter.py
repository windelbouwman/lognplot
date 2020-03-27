""" Snoops all ROS traffic, and forwards it into lognplot.

Strategy for this adapter:
- Create a rclpy node
- Regularly, check all available topics
- Subscribe to each topic
- On message reception, recurse into the message type
- For each message field which looks remotely like a float, forward it.
"""

import argparse
import math
import time
import socket
import struct
import importlib
import numpy as np

# ROS imports:
import rclpy
from rclpy.qos import qos_profile_sensor_data
from rcl_interfaces.msg import Log

from lognplot.client import LognplotTcpClient


def main(args=None):
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--lognplot-host",
        default="localhost",
        type=str,
        help="The host where the lognplot GUI is running.",
    )
    parser.add_argument(
        "--lognplot-port",
        default="12345",
        type=int,
        help="The port of the lognplot GUI.",
    )
    args2 = parser.parse_args(args)
    rclpy.init(args=args)
    ros_to_lognplot = RosToLogNPlot(args2.lognplot_host, args2.lognplot_port)
    ros_to_lognplot.connect()
    if ros_to_lognplot.is_connected():
        ros_to_lognplot.run()


class RosToLogNPlot:
    def __init__(self, lognplot_host, lognplot_port):
        self._subscriptions = {}
        self._lognplot_host = lognplot_host
        self._lognplot_port = lognplot_port

    def connect(self):
        try:
            self._client = LognplotTcpClient(
                hostname=self._lognplot_host, port=self._lognplot_port
            )
            self._client.connect()
        except ConnectionRefusedError:
            print("Error connecting to lognplot GUI!")
            self._client = None

    def is_connected(self):
        return bool(self._client)

    def run(self):
        self.node = rclpy.create_node("ros_to_lognplot")
        self.timer = self.node.create_timer(2.0, self._check_topics)
        self.node.create_subscription(
            Log, "/rosout", self.on_ros_out_msg, 0
        )
        rclpy.spin(self.node)
        rclpy.shutdown()

    def on_ros_out_msg(self, msg):
        signal_name = f'/rosout/{msg.name}'
        timestamp = time.time()
        text = msg.msg
        self.send_text(signal_name, timestamp, text)

    def _check_topics(self):
        """ Check which topics are present in the system, and subscribe to them all!
        """
        topics = self.node.get_topic_names_and_types()
        for topic_name, topic_type_name in topics:
            if not self.is_subscribed(topic_name):
                print("-", topic_name, "---", topic_type_name)
                topic_type = load_type(topic_type_name[0])
                self._subscribe_on_topic(topic_name, topic_type)

    def is_subscribed(self, topic_name):
        return topic_name in self._subscriptions

    def _subscribe_on_topic(self, topic_name, topic_type):
        assert topic_name not in self._subscriptions

        def handler(msg):
            timestamp = time.time()
            self.process_message(topic_name, topic_type, timestamp, msg)

        subscription = self.node.create_subscription(
            topic_type, topic_name, handler, qos_profile_sensor_data
        )
        self._subscriptions[topic_name] = subscription

    def process_message(self, topic_name, topic_type, timestamp, msg):
        """ Process an incoming ROS message.
        """
        self.process_value(topic_name, topic_type, timestamp, msg)

    def process_value(self, full_name, value_type, timestamp, value):
        if hasattr(value, "get_fields_and_field_types"):
            for field_name, field_type in value.get_fields_and_field_types().items():
                field_value = getattr(value, field_name)
                full_field_name = f"{full_name}.{field_name}"
                self.process_value(full_field_name, field_type, timestamp, field_value)
        else:
            if isinstance(value, (float, np.float32, int)):
                self.send_sample(full_name, timestamp, float(value))
            elif isinstance(value, (list, np.ndarray)):
                for element_index, element_value in enumerate(value):
                    element_name = f"{full_name}[{element_index}]"
                    element_type = None
                    self.process_value(
                        element_name, element_type, timestamp, element_value
                    )
            else:
                # Great panic! What now?
                # Ignore for now..
                pass

    def send_sample(self, signal_name: str, timestamp, value):
        """ Emit a single sample to the lognplot GUI. """
        if self._client:
            self._client.send_sample(signal_name, timestamp, value)

    def send_text(self, signal_name: str, timestamp, text):
        """ Emit a single text to the lognplot GUI. """
        if self._client:
            self._client.send_text(signal_name, timestamp, text)


def load_type(type_name):
    """ Load/import ros msg type from proper module. """
    package_name, *message_name = type_name.split("/")
    module_name = package_name + ".msg"
    module = importlib.import_module(module_name)
    return getattr(module, message_name[-1])


def ros_stamp_to_x(stamp):
    """ Convert ROS timestamp to own timestamp. """
    return stamp.sec + (stamp.nanosec * 1e-9)


if __name__ == "__main__":
    main()
