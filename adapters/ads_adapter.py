""" Beckhoff ADS adapter.

Downloads all beckhof ADS data and sends it to the GUI tool.

DLL:
https://infosys.beckhoff.com/content/1033/tcadsdll2/html/note.htm?id=774456285682383031

Ads specification:
https://infosys.beckhoff.com/content/1033/tcadsamsspec/html/note.htm?id=8851434491951139899

"""
from lognplot.client import LognplotTcpClient
import pyads
import time
import ctypes
import struct
from collections import namedtuple

Entry = namedtuple('Entry', ['name', 'typename', 'comment', 'datatype', 'datatype_size'])

def parse_ads_entries(entries_data: bytes, size: int, count: int):
    parsed_entries = []
    total_bytes = 0
    while total_bytes < size:
        fmt = "<6I3H"
        entry_length, _, _, entry_datatype_size, entry_datatype, _, name_len, type_len, comment_len = struct.unpack_from(fmt, entries_data, total_bytes)

        name_offset = total_bytes + struct.calcsize(fmt)
        name = entries_data[name_offset:name_offset + name_len].decode('ascii')

        type_offset = name_offset + name_len + 1
        typ = entries_data[type_offset:type_offset + type_len].decode('ascii')

        comment_offset = type_offset + type_len + 1
        comment = entries_data[comment_offset:comment_offset + comment_len].decode('ascii')

        total_bytes += entry_length
        parsed_entries.append(Entry(name, typ, comment, entry_datatype, entry_datatype_size))

    assert len(parsed_entries) == count
    return parsed_entries


def main():
    client = LognplotTcpClient()
    client.connect()

    plc = pyads.Connection('10.0.4.58.1.1', 851)
    plc.open()

    @plc.notification(pyads.PLCTYPE_LREAL)
    def callback_lreal(handle, name, timestamp, value):
        client.send_sample(name, time.mktime(timestamp.timetuple())*1e3 + timestamp.microsecond/1e3, value)

    @plc.notification(pyads.PLCTYPE_DINT)
    def callback_dint(handle, name, timestamp, value):
        client.send_sample(name, time.mktime(timestamp.timetuple()) * 1e3 + timestamp.microsecond / 1e3, float(value))

    upload_info = plc.read(pyads.constants.ADSIGRP_SYM_UPLOADINFO, 0, pyads.structs.SAdsSymbolUploadInfo)
    print("nSymSize: " + str(upload_info.nSymSize) + "\nnSymbols: " + str(upload_info.nSymbols))

    entries = plc.read(pyads.constants.ADSIGRP_SYM_UPLOAD, 0, ctypes.c_ubyte * upload_info.nSymSize)

    parsed_entries = parse_ads_entries(bytes(entries), upload_info.nSymSize, upload_info.nSymbols)

    attr_lreal = pyads.NotificationAttrib(ctypes.sizeof(pyads.PLCTYPE_LREAL))
    attr_dint = pyads.NotificationAttrib(ctypes.sizeof(pyads.PLCTYPE_DINT))

    for entry in parsed_entries:
        print(entry)
        if entry.datatype == 5:
            plc.add_device_notification(entry.name, attr_lreal, callback_lreal)
        elif entry.datatype == 3:
            plc.add_device_notification(entry.name, attr_dint, callback_dint)

    # plc.add_device_notification('GVL.test', attr_real, callback_real)
    # plc.add_device_notification('GVL.test2', attr_real, callback_real)

    # Write to the variable to trigger a notification
    # plc.write_by_name('GVL.intvar', 123, pyads.PLCTYPE_INT)


if __name__ == "__main__":
    main()
