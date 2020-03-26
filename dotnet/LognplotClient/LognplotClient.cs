using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;

namespace LognplotClient
{
    public class LognplotClient
    {
        private IntPtr handle;

        public void Connect(string address)
        {
            handle = lognplot_client_new(address);
        }

        public void Disconnect()
        {
            lognplot_client_close(handle);
        }

        public void SendSample(string name, DateTime timestamp, double value)
        {
            double seconds_since_epoch = makeSeconds(timestamp);
            lognplot_client_send_sample(handle, name, seconds_since_epoch, value);
        }

        public void SendSamples(string name, List<Tuple<DateTime, double>> samples)
        {
            List<double> times = samples.Select(s => makeSeconds(s.Item1)).ToList();
            List<double> values = samples.Select(s => s.Item2).ToList();
            int amount = samples.Count;
            throw new NotImplementedException();
            // TODO
            // lognplot_client_send_samples(handle, name, amount, times, values);
        }

        public void SendSampled(string name, DateTime timestamp, double dt, List<double> values)
        {
            double seconds_since_epoch = makeSeconds(timestamp);
            throw new NotImplementedException();
            // TODO
            // lognplot_client_send_sampled_samples(handle, name, seconds_since_epoch, dt, values.asArray());
        }

        public void SendText(string name, DateTime timestamp, string text)
        {
            double seconds_since_epoch = makeSeconds(timestamp);
            lognplot_client_send_text(handle, name, seconds_since_epoch, text);
        }

        private double makeSeconds(DateTime timestamp)
        {
            double seconds_since_epoch = new DateTimeOffset(timestamp.ToLocalTime()).ToUnixTimeMilliseconds() * 1.0e-3;
            return seconds_since_epoch;
        }

        [DllImport("clognplot.dll")]
        private static extern IntPtr lognplot_client_new(string address);

        [DllImport("clognplot.dll")]
        private static extern IntPtr lognplot_client_close(IntPtr handle);

        [DllImport("clognplot.dll")]
        private static extern void lognplot_client_send_sample(IntPtr handle, string name, double timestamp, double value);

        // TODO:
        // [DllImport("clognplot.dll")]
        // private static extern void lognplot_client_send_samples(IntPtr handle, string name, size_t size, double[] times, double[] values);

        // TODO:
        // [DllImport("clognplot.dll")]
        // private static extern void lognplot_client_send_sampled_samples(IntPtr handle, string name, double timestamp, double dt, size_t size, double[] values);

        [DllImport("clognplot.dll")]
        private static extern void lognplot_client_send_text(IntPtr handle, string name, double timestamp, string text);
    }
}