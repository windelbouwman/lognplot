""" Import data from CSV (Comma Separated Values) format.

CSV is a text format for data, which can be a bit tricky
to get right. This adapter will probably have a large number
of options to be flexible enough to handle most formats.
"""

import argparse
import csv
from lognplot.client import LognplotTcpClient


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("csv_file")
    parser.add_argument("--delimiter", default=",", help="The value delimiter to use")
    parser.add_argument(
        "--skip-rows",
        default=0,
        type=int,
        help="Amount of rows to skip at the beginning of the file",
    )
    parser.add_argument(
        "--time-column",
        type=int,
        help="The CSV column to use as time column. If not provided, the CSV row will be used as time.",
    )
    parser.add_argument("--lognplot-hostname", default="localhost", type=str)
    parser.add_argument("--lognplot-port", default="12345", type=int)
    args = parser.parse_args()
    print(args)

    lognplot_client = LognplotTcpClient(
        hostname=args.lognplot_hostname, port=args.lognplot_port
    )
    lognplot_client.connect()

    if hasattr(args, "time_column") and args.time_column is not None:
        time_column = int(args.time_column)
    else:
        time_column = None

    with open(args.csv_file, "r") as csv_file:
        reader = csv.reader(csv_file, delimiter=args.delimiter)
        for row_index, row in enumerate(reader):
            if row_index >= args.skip_rows:
                # print(row)

                timestamp = (
                    float(row[time_column]) if time_column is not None else row_index
                )

                for column_index, column in enumerate(row):
                    name = f"csv_column_{column_index}"
                    lognplot_client.send_sample(name, timestamp, float(column))


if __name__ == "__main__":
    main()
