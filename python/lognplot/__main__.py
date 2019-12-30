""" Usable when executed via

    $ python -m lognplot

"""

import argparse
import logging


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "-v",
        "--verbose",
        action="count",
        default=0,
        help="Increment verbosity of this tool.",
    )
    args = parser.parse_args()
    # print(args)
    # TODO: use argparse to enable more apps.
    verbosity = args.verbose

    if verbosity < 0:
        loglevel = logging.WARNING
    elif verbosity > 0:
        loglevel = logging.DEBUG
    else:
        loglevel = logging.INFO

    logging.basicConfig(level=loglevel)

    from .qt.apps import run_server_gui

    run_server_gui()


if __name__ == "__main__":
    main()
