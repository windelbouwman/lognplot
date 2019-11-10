""" Page handling in file.
"""

import struct


class Pager:
    def __init__(self, page_size=4096):
        self.page_size = page_size

    def open(self, filename):
        self.f = open(filename, "wb")
        # If we re-open a database, load it.
        # otherwise create a header in the file.
        self.write_header()

    def write_header(self):
        # Go to file offset 0:
        self.f.seek(0)
        header_fmt = ">10sI"
        header_data = struct.pack(header_fmt, "COOLDOWNDB", 1)
        self.f.write(header_data)

    def close(self):
        self.f.close()

    def get_page(self, page_id):
        return Page()

    def new_page(self):
        page = Page()
        return page


class PageDirectory:
    """ A directory index where to find pages in the file.
    """

    def __init__(self):
        pass


class Page:
    def __init__(self):
        self.id = 1

    def __repr__(self):
        return f"Page ( id = {self.id })"


class SlottedPage:
    """ Page divided into slots.
    """

    def __init__(self):
        pass
