""" This demo reads out /proc/pid/stat and extracts 
information about the processor usage of linux processes.

Field 1 (index 0): pid
Field 2 (index 1): filename
Field 14 (index 13): user code in jiffies (ex children)
Field 15 (index 14): kernel code in jiffies (ex children)
"""

import re
import os
import time
from collections import defaultdict
from lognplot.client import LognplotTcpClient


class RateDetector:
    """ Determine the rate of change of given signal.
    """
    def __init__(self):
        self._previous_measurement = None

    def update(self, measurement):
        if self._previous_measurement:
            assert measurement[1] >= self._previous_measurement[1]
            dt = measurement[0] - self._previous_measurement[0]
            dy = measurement[1] - self._previous_measurement[1]
            assert dy >= 0
            assert dt > 0
            rate = dy / dt
        else:
            rate = 0

        self._previous_measurement = measurement
        return rate


class Monitor:
    def __init__(self):
        self._rate_detectors = defaultdict(RateDetector)

    def connect(self):
        self._client = LognplotTcpClient()
        self._client.connect()

    def update(self):
        for process_folder in os.listdir('/proc'):
            if re.match(r'\d+', process_folder):
                # print("Ow yeah!")
                timestamp = time.time()
                stat_filename = f'/proc/{process_folder}/stat'
                with open(stat_filename, 'r') as f:
                    line = f.read()
                # print(line)
                self.analyze_line(timestamp, line)
                # self.measure(timestamp, pid, filename, user_jiffies, kernel_jiffies)

    def analyze_line(self, timestamp, line):
        parts = line.split(' ')
        pid = int(parts[0])
        filename = parts[1]
        user_jiffies = int(parts[13])
        kernel_jiffies = int(parts[14])
        # print(pid, filename)
        self.measure(f'{filename}_{pid}_user', (timestamp, user_jiffies))
        self.measure(f'{filename}_{pid}_kernel', (timestamp, kernel_jiffies))
    
    def measure(self, signal_name, measurement):
        rate = self._rate_detectors[signal_name].update(measurement)
        # print(signal_name, rate)
        self._client.send_sample(signal_name, float(measurement[0]), float(rate))


def main():
    monitor = Monitor()
    monitor.connect()
    while True:
        monitor.update()





if __name__ == "__main__":
    main()
