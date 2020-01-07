from ..qtapi import QtCore


class ChartLayout:
    def __init__(self, rect: QtCore.QRect, options):
        # Parameters:
        self.axis_width = 40
        self.axis_height = 40

        # Inputs:
        self.options = options
        # print(rect, type(rect))
        self.rect = rect

        # Endless sea of variables :)
        self.do_layout()

    def do_layout(self):
        # self.right = self.rect.right()
        # self.bottom = self.rect.bottom()
        if self.options.show_axis:
            axis_height = self.axis_height
            axis_width = self.axis_width
        else:
            axis_height = 0
            axis_width = 0

        if self.options.show_legend:
            self.legend = QtCore.QRect(self.rect.left() + self.options.padding,
                                       self.rect.top() + self.options.padding,
                                       self.rect.right() - 2 * self.options.padding,
                                       self.options.legend_height)

            self.chart_top = self.legend.bottom() + 10
        else:
            self.chart_top = self.rect.top() + self.options.padding

        self.handles = QtCore.QRect(self.rect.left() + self.options.padding,
                                    self.rect.top(),
                                    self.options.handle_width,
                                    self.rect.height())

        if self.options.show_handles:
            self.chart_left = self.handles.right() + 3
        else:
            self.chart_left = self.rect.left() + self.options.padding


        self.chart_bottom = self.rect.bottom() - self.options.padding - axis_height
        self.chart_right = self.rect.right() - self.options.padding - axis_width
        self.chart_width = self.chart_right - self.chart_left
        self.chart_height = self.chart_bottom - self.chart_top
