from .axis import Axis


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
