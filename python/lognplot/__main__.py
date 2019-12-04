""" Usable when executed via

    $ python -m lognplot

"""

import logging


def main():
    # TODO: use argparse to enable more apps.
    logging.basicConfig(level=logging.DEBUG)

    from .qt.apps import run_server_gui

    run_server_gui()


if __name__ == "__main__":
    main()
