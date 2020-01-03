from ...utils import clip


def to_x_pixel(value, axis, layout):
    """ Transform the given X value to a pixel position.
    """
    domain = axis.domain
    a = layout.chart_width / domain
    x = layout.chart_left + a * (value - axis.minimum)
    return clip(x, layout.chart_left, layout.chart_right)


def to_y_pixel(value, axis, layout):
    """ Transform the given Y value to a pixel position.
    """
    domain = axis.domain
    a = layout.chart_height / domain
    y = layout.chart_bottom - a * (value - axis.minimum)
    return clip(y, layout.chart_top, layout.chart_bottom)


def to_x_value(pixel, axis, layout):
    """ Given a pixel, determine its domain value. """
    domain = axis.domain
    a = domain / layout.chart_width
    value = axis.minimum + a * (pixel - layout.chart_left)
    return value
    # return clip(x, layout.chart_left, layout.chart_right)


def x_pixels_to_domain(pixels, axis, layout):
    """ Convert a pixel distance to a domain distance """
    domain = axis.domain
    a = domain / layout.chart_width
    shift = a * pixels
    return shift
