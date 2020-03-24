using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

using LognplotClient;

namespace DemoUsage
{
    class Program
    {
        static void Main(string[] args)
        {
            LognplotClient.LognplotClient client = new LognplotClient.LognplotClient();
            client.Connect("localhost:12345");

            double t = 0.0;
            double A = 10.0;
            double f = 0.3;
            double dt = 0.02;

            while (true)
            {
                double value = A * Math.Sin(t * 2 * Math.PI * f);
                client.SendSample("pi", DateTime.Now, value);
                System.Threading.Thread.Sleep((int)(dt * 1.0e3));
                t += dt;
            }
        }
    }
}
