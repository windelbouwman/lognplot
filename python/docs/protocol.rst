
.. _protocol:

Write data protocol
===================

This section describes the protocol used to send data to the
application.

Packet chunking
---------------

The data is send via a normal TCP/IP connection. The data in the
bytestream is encoded into packets. Each packet is transmitted
as a 32 bit big endian integer length prefix, and then the data.

.. code::

    +----------------+--------------+----------------+--------------+
    | 4 bytes        | Len 1 bytes  | 4 bytes        | Len 2 bytes  |
    +----------------+--------------+----------------+--------------+
    | uint32_BE      | uint8[]      | uint32_BE      | uint8[]      |  ...
    +----------------+--------------+----------------+--------------+
    | Length field 1 | Data field 1 | Length field 2 | Data field 2 |
    +----------------+--------------+----------------+--------------+  

This way, the tcp byte stream is used to transport a sequence of
ordered chunks.

Packet format
-------------

Each packet is `cbor <https://en.wikipedia.org/wiki/CBOR>`_ encoded binary data.

The format of each transmitted sample is:

.. code::

    {
        "name": name,     # The name of the sensor
        "t0": t0,         # The timestamp of the first sample
        "dt": dt,         # The time delta between the samples
        "data": samples   # The actual sample data as a list
    }

