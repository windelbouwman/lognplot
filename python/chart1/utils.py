import time
import contextlib


def chunk(sequence, chunk_size):
    """ Split an iterable into batches of items.
    """
    c = []
    for item in sequence:
        c.append(item)
        if len(c) >= chunk_size:
            yield c
            c = []
    if c:
        yield c


@contextlib.contextmanager
def bench_it(name):
    t1 = time.time()
    yield
    t2 = time.time()
    duration = t2 - t1
    print(f"{name} took {duration} seconds")
