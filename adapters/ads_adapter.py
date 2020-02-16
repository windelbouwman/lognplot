"""Beckhoff ADS adapter.

Subscribes to Beckhof ADS variables and sends it to the GUI tool.

Auto subscribe only works for DINT and LREAL for now.

Example usage:
    python ads_adapter.py --regex GVl.*temperature

"""
from lognplot.client import LognplotTcpClient
import pyads
import time
import ctypes
import struct
import datetime
import re
import argparse
from collections import namedtuple
from typing import Type


class AdsClient:
    Entry = namedtuple(
        "Entry", ["name", "typename", "comment", "datatype", "datatype_size"]
    )

    DATATYPE_MAP = {
        "BOOL": pyads.PLCTYPE_BOOL,
        "BYTE": pyads.PLCTYPE_BYTE,
        "DINT": pyads.PLCTYPE_DINT,
        "DWORD": pyads.PLCTYPE_DWORD,
        "INT": pyads.PLCTYPE_INT,
        "LREAL": pyads.PLCTYPE_LREAL,
        "REAL": pyads.PLCTYPE_REAL,
        "SINT": pyads.PLCTYPE_SINT,
        "UDINT": pyads.PLCTYPE_UDINT,
        "UINT": pyads.PLCTYPE_UINT,
        "USINT": pyads.PLCTYPE_USINT,
        "WORD": pyads.PLCTYPE_WORD,
    }  # type: Dict[str, Type]

    def __init__(self, ams_net_id: str, ams_net_port: str):
        self.notification_handles = []
        self.plc = pyads.Connection(ams_net_id, ams_net_port)
        self.plc.open()

    def __del__(self):
        for handle in self.notification_handles:
            self.plc.del_device_notification(*handle)

        self.plc.close()

    def log(self, name: str, plc_type):
        attr = pyads.NotificationAttrib(ctypes.sizeof(plc_type))
        handles = self.plc.add_device_notification(
            name, attr, self.callback(plc_type), pyads.ADSTRANS_SERVERCYCLE
        )
        self.notification_handles.append(handles)

    def subscribe(self, pattern: str):
        parsed_entries = self.get_ads_entries()

        for entry in parsed_entries:
            if re.match(pattern, entry.name) != None:
                self.log(entry.name, self.DATATYPE_MAP[entry.typename])

    def callback(self, plc_type):
        @self.plc.notification(plc_type)
        def decorated_callback(handle, name, timestamp, value):
            client.send_sample(
                name, AdsClient.format_timestamp(timestamp), float(value),
            )

        return decorated_callback

    def get_ads_entries(self):
        upload_info = self._get_upload_info()
        entries = self.plc.read(
            pyads.constants.ADSIGRP_SYM_UPLOAD, 0, ctypes.c_ubyte * upload_info.nSymSize
        )

        parsed_entries = self._unpack_entries(
            bytes(entries), upload_info.nSymSize, upload_info.nSymbols
        )
        return parsed_entries

    @staticmethod
    def format_timestamp(timestamp: datetime):
        return time.mktime(timestamp.timetuple()) + timestamp.microsecond / 1e6

    def _get_upload_info(self):
        upload_info = self.plc.read(
            pyads.constants.ADSIGRP_SYM_UPLOADINFO,
            0,
            pyads.structs.SAdsSymbolUploadInfo,
        )
        return upload_info

    def _unpack_entry(data: bytes, index: int) -> (int, Entry):
        fmt = "<6I3H"
        (
            entry_length,
            _,
            _,
            entry_datatype_size,
            entry_datatype,
            _,
            name_len,
            type_len,
            comment_len,
        ) = struct.unpack_from(fmt, entries_data, index)

        name_offset = index + struct.calcsize(fmt)
        name = entries_data[name_offset : name_offset + name_len].decode("ascii")

        type_offset = name_offset + name_len + 1
        typ = entries_data[type_offset : type_offset + type_len].decode("ascii")

        comment_offset = type_offset + type_len + 1
        comment = entries_data[comment_offset : comment_offset + comment_len].decode(
            "ascii"
        )

        return (
            entry_length,
            self.Entry(name, typ, comment, entry_datatype, entry_datatype_size),
        )

    def _unpack_entries(self, entries_data: bytes, size: int, count: int):
        parsed_entries = []
        index = 0
        while index < size:
            entry_size, entry = _unpack_entry(data, index)
            index += entry_size
            parsed_entries.append(entry)

        assert len(parsed_entries) == count
        assert index == size
        return parsed_entries


def main():
    parser = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )

    parser.add_argument(
        "--ams-net-id", help="ams net id", default="127.0.0.1.1.1", type=str
    )
    parser.add_argument("--ams-net-port", help="ams net port", default=851, type=int)
    parser.add_argument(
        "--regex",
        help="Regular expression pattern used for filtering",
        default="*",
        type=str,
    )
    parser.add_argument("--lognplot-hostname", default="127.0.0.1", type=str)
    parser.add_argument("--lognplot-port", default="12345", type=int)
    args = parser.parse_args()

    lnp_client = LognplotTcpClient(
        hostname=args.lognplot_hostname, port=args.lognplot_port
    )
    lnp_client.connect()

    ads_client = AdsClient(args.ams_net_id, args.ams_net_port)

    ads_client.subscribe(args.regex)
    ads_client.log("GVL.test", pyads.PLCTYPE_LREAL)


if __name__ == "__main__":
    main()
