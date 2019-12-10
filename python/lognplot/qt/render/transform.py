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
