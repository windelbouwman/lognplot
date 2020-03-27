using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

using Lognplot;

namespace DemoUsage
{
    class Program
    {
        static void Main(string[] args)
        {
            LognplotClient client = new LognplotClient();
            client.Connect("localhost:12345");
            DemoSingle(client);
            DemoSendSampled(client);
            DemoSendMultiple(client);
            client.Disconnect();
        }

        static void DemoSingle(LognplotClient client)
        {
            double t = 0.0;
            double A = 10.0;
            double f = 0.3;
            double dt = 0.02;
            int count = 100;

            DateTime t2 = DateTime.Now;

            while (count-- > 0)
            {
                double value = A * Math.Sin(t * 2 * Math.PI * f);
                client.SendSample("C# value", t2, value);
                client.SendText("C# Log", t2, $"Moi {count}");
                // System.Threading.Thread.Sleep((int)(dt * 1.0e3));
                t += dt;
                t2 = t2.AddSeconds(dt);
            }
        }
        static void DemoSendMultiple(LognplotClient client)
        {
            double dt = 0.01;
            double t = 10.0;
            double f = 2.7;
            double A = 20.0;
            int count = 10000;
            List<Tuple<DateTime, double>> values = new List<Tuple<DateTime, double>>();
            DateTime t2 = DateTime.Now;

            for (int i = 0; i < count; i++)
            {
                double omega = t * 2 * Math.PI * f;
                double value = A * Math.Sin(omega);
                values.Add(Tuple.Create(t2, value));
                t += dt;
                t2 = t2.AddSeconds(dt);
            }
            client.SendSamples("C# Batch data", values);
        }

        static void DemoSendSampled(LognplotClient client)
        {
            double dt = 0.01;
            double t = 10.0;
            double f = 3.14;
            double A = 30.0;
            int count = 10000;
            List<double> values = new List<double>();

            for (int i =0;i<count; i++)
            {
                double omega = t * 2 * Math.PI * f;
                double value = A * Math.Sin(omega);
                values.Add(value);
                t += dt;
            }
            client.SendSampled("C# Sampled data", DateTime.Now, dt, values);
        }
    }
}
