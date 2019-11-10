import math


class Axis:
    def __init__(self):
        self.minimum = -30
        self.maximum = 130

    def get_ticks(self, n_ticks):
        domain = self.domain
        scale = math.floor(math.log10(domain))
        approx = math.pow(10, -scale) * domain / n_ticks
        options = [0.1, 0.2, 0.5, 1.0, 2.0, 5.0]
        best = min(options, key=lambda x: abs(x - approx))

        step_size = best * math.pow(10, scale)
        start = ceil_to_multiple_of(self.minimum, step_size)
        end = self.maximum

        values = float_range(start, end, step_size)
        return [(x, str(x)) for x in values]

    @property
    def domain(self):
        return self.maximum - self.minimum


def float_range(start, end, stepsize):
    values = []
    assert start < end
    assert stepsize > 0
    value = start
    while value < end:
        values.append(value)
        value += stepsize
    return values


def ceil_to_multiple_of(value, step):
    """ Round the given value to integer multiples of step.
    """
    assert step > 0
    offset = value % step
    if offset > 0:
        extra = step - offset
        return value + extra
    else:
        return value
