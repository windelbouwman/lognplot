import math
from .axis import Axis
from .curve import Curve
from ..utils import bench_it
from ..tsdb.metrics import merge_metrics


class Chart:
    """ Chart object.

    Note that this is only the datatype of the chart. All drawing
    is performed in a seperate function. This only binds all the
    axis, traces into a single object.
    """

    def __init__(self, db):
        self.x_axis = Axis()
        self.y_axis = Axis()
        self.curves = []
        self.db = db

    def add_curve(self, name, color):
        curve = Curve(self.db, name, color)
        self.curves.append(curve)

    def info(self):
        print(f"Chart with {len(self.curves)} series")
        for index, curve in enumerate(self.curves):
            print(f"serie {index} with {len(curve)} samples")

    def horizontal_zoom(self, amount):
        """ Zoom in horizontal manner. """
        domain = self.x_axis.domain
        step = domain * amount
        self.x_axis.minimum -= step
        self.x_axis.maximum += step

    def vertical_zoom(self, amount):
        domain = self.y_axis.domain
        step = domain * amount
        self.y_axis.minimum -= step
        self.y_axis.maximum += step

    def horizontal_pan(self, amount):
        domain = self.x_axis.domain
        step = domain * amount
        self.x_axis.minimum += step
        self.x_axis.maximum += step

    def vertical_pan(self, amount):
        domain = self.y_axis.domain
        step = domain * amount
        self.y_axis.minimum += step
        self.y_axis.maximum += step

    def autoscale_y(self):
        """ Automatically adjust the Y-axis to fit data in range. """
        begin = self.x_axis.minimum
        end = self.x_axis.maximum
        timespan = (begin, end)
        metrics = []
        # with bench_it("autoscale_y"):
        for curve in self.curves:
            metric = curve.query_metrics(timespan)
            if metric:
                metrics.append(metric)

        # If we gathered any metrics, group them here:
        if metrics:
            metric = merge_metrics(metrics)
            self.fit_metrics_y_axis(metric)
        # print("query result", type(data), len(data))

    def fit_metrics_y_axis(self, metric):
        """ Adjust Y-axis to fit metrics into view. """
        domain = metric.maximum - metric.minimum

        # If we have a single value, increase the domain.
        if math.isclose(domain, 0):
            domain = 1

        minimum = metric.minimum - domain * 0.05
        maximum = metric.maximum + domain * 0.05
        self.y_axis.set_limits(minimum, maximum)

    def fit_metrics_on_x_axis(self, metric):
        """ Adjust X-axis to fit metrics in view. """

        domain = metric.x2 - metric.x1
        if math.isclose(domain, 0):
            domain = 1

        minimum = metric.x1 - domain * 0.05
        maximum = metric.x2 + domain * 0.05
        self.x_axis.set_limits(minimum, maximum)

    def get_region(self):
        """ Get the current viewed region.
        """
        return (
            self.x_axis.minimum,
            self.y_axis.minimum,
            self.x_axis.maximum,
            self.y_axis.maximum,
        )

    def zoom_fit(self):
        """ Adjust axis to fit all curves. """
        metric = self.metrics()

        # If we have metrics, adjust axis.
        if metric:
            self.fit_metrics_y_axis(metric)
            self.fit_metrics_on_x_axis(metric)

    def metrics(self):
        """ Metrics of all signals in the plot. """

        # Gather bounding boxes of all curves:
        metrics = []
        for curve in self.curves:
            metric = curve.query_summary()
            if metric:
                metrics.append(metric)

        # If we have bounds, merge them and adjust axis.
        if metrics:
            return merge_metrics(metrics)
