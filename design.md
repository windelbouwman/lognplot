
# Database

# Requirements

- Live updating of the database
- Persistent into filesystem
- Time series with aggregates

Time resolution

We require at least 1 ms resolution for real-time systems. It would be even
better to support resolutions from oscilloscopes to be able to ingest
scope data, and be usable as a softscope. Audio is also cool to log, so this
means 1/44kHz resolution (~ 20 micro seconds).

At least nanoseconds should be used.

# Concepts

## single file.

A single file has the benefits of full control of the database and it's paging.
One can lock the file. One can copy the file as a whole.

Possible solutions:

- sqlite3, hdf5

TODO: read about hdf5 for live updates.

## Data metrics / aggregations

To support zoom levels, we require pre-calculated aggregates of the
data. Metrics to support:

- min value -> The smallest value in the data range
- max value -> the biggest value in the data range
- number of samples -> The amount of samples in the date range (uint128_t -> support many values.)
- sum of all values -> the sum of all values. Use this to calculate the average
  (divide by count of samples)
- timestamp of first value
- timestamp of last value

The aggregate should be composable. This means, if we summarize a chunk of data,
we can even further summarize sets of chunked data.

TODO: sum of all values might become too large?

## Timestamp representation

Since in the time series database, time instances and time ranges
are a recurring theme, it's important.

A single point in time is a time instance. A period in time is
called a timespan. A timespan has a certain duration (measured in seconds).

So we have the terminlogy:

- "duration" -> length of a time range
- "timespan" -> time range
- "instant" -> a single moment in time

Time will always be global, and can be expressed in time
since for example EPOCH.

- Unix EPOCH: 1st january 1970
- NTP epoch: 1st january 1900
- Birth of christ?

Resolution requirements.

# References

- youtube lecture on databases by CMU
- youtube lectures from databankenlernen.de
- sqlite sourcecode
- akumuli database
- btrdb -> some time series database?

