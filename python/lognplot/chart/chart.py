import math
from .axis import Axis
from .curve import Curve
from ..utils import bench_it
from ..tsdb import TimeSpan, Aggregation, Metrics


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
        timespan = TimeSpan(self.x_axis.minimum, self.x_axis.maximum)

        summary = self.data_summary(timespan=timespan)
        if summary:
            self.fit_metrics_y_axis(summary.metrics)

    def fit_metrics_y_axis(self, metric: Metrics):
        """ Adjust Y-axis to fit metrics into view. """
        domain = metric.maximum - metric.minimum

        # If we have a single value, increase the domain.
        if math.isclose(domain, 0):
            domain = 1

        minimum = metric.minimum - domain * 0.05
        maximum = metric.maximum + domain * 0.05
        self.y_axis.set_limits(minimum, maximum)

    def fit_timespan_on_x_axis(self, timespan: TimeSpan):
        """ Adjust X-axis to fit timespan in view. """

        domain = timespan.end - timespan.begin
        if math.isclose(domain, 0):
            domain = 1

        minimum = timespan.begin - domain * 0.05
        maximum = timespan.end + domain * 0.05
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
        summary = self.data_summary()

        # If we have metrics, adjust axis.
        if summary:
            self.fit_timespan_on_x_axis(summary.timespan)
            self.fit_metrics_y_axis(summary.metrics)

    def data_summary(self, timespan=None) -> Aggregation:
        """ Metrics of all signals in the plot. """

        # Gather bounding boxes of all curves:
        aggregations = []
        for curve in self.curves:
            aggregation = curve.query_summary(timespan=timespan)

            if aggregation:
                aggregations.append(aggregation)

        # If we have bounds, merge them and adjust axis.
        if aggregations:
            return Aggregation.from_aggregations(aggregations)
