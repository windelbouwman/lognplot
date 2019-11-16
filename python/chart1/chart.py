from .axis import Axis
from .utils import bench_it
from .metrics import merge_metrics


class Chart:
    def __init__(self):
        self.x_axis = Axis()
        self.y_axis = Axis()
        self.series = []

    def add_serie(self, serie):
        self.series.append(serie)

    def info(self):
        print(f"Chart with {len(self.series)} series")
        for index, serie in enumerate(self.series):
            print(f"serie {index} with {len(serie)} samples")

    def autoscale_y(self):
        """ Automatically adjust the Y-axis to fit data in range. """
        begin = self.x_axis.minimum
        end = self.x_axis.maximum
        metrics = []
        with bench_it("autoscale_y"):
            for serie in self.series:
                metric = serie.query_metrics(begin, end)
                if metric:
                    metrics.append(metric)

        # If we gathered any metrics, group them here:
        if metrics:
            metric = merge_metrics(metrics)
            domain = metric.maximum - metric.minimum
            self.y_axis.maximum = metric.maximum + domain * 0.1
            self.y_axis.minimum = metric.minimum - domain * 0.1
        # print("query result", type(data), len(data))

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

        # Gather bounding boxes of all curves:
        metrics = []
        for serie in self.series:
            metric = serie.metrics
            if metric:
                metrics.append(metric)

        # If we have bounds, merge them and adjust axis.
        if metrics:
            metric = merge_metrics(metrics)

            # Adjust Y-axis:
            domain = metric.maximum - metric.minimum
            self.y_axis.maximum = metric.maximum + domain * 0.05
            self.y_axis.minimum = metric.minimum - domain * 0.05

            # Adjust X-axis:
            domain = metric.x2 - metric.x1
            self.x_axis.maximum = metric.x2 + domain * 0.05
            self.x_axis.minimum = metric.x1 - domain * 0.05
