using System;
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

        public void SendSample(string name, DateTime timestamp, double value)
        {
            double seconds_since_epoch = new DateTimeOffset(timestamp.ToLocalTime()).ToUnixTimeMilliseconds() * 1.0e-3;
            lognplot_client_send_sample(handle, name, seconds_since_epoch, value);
        }

        [DllImport("clognplot.dll")]
        private static extern IntPtr lognplot_client_new(string address);

        [DllImport("clognplot.dll")]
        private static extern void lognplot_client_send_sample(IntPtr handle, string name, double timestamp, double value);
    }
}
