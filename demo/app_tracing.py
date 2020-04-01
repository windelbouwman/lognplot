""" Demo on how to track python script execution.
"""

from lognplot import install_tracer

install_tracer()


def do_bar(y):
    print(f'y = {y}')


def do_spiffy(x):
    for y in [1, 2, 3]:
        print(y, x)
        do_bar(y)


l = ["a", "bla", "bla-di-bla"]
for t in l:
    do_spiffy(t)

