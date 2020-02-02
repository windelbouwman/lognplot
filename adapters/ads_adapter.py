""" Beckhoff ADS adapter.

Downloads all beckhof ADS data and sends it to the GUI tool.

"""

# TODO: implement.



from lognplot.client import LognplotTcpClient


def main():
    client = LognplotTcpClient()
    client.connect()
    topic = 'test'
    value = 1337.0
    client.send_sample(topic, 10.0, value)


if __name__ == "__main__":
    main()
