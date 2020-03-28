
#include <math.h>
#include <iostream>
#include <lognplot/client.h>

int main() {

    std::cout << "C++ demo" << std::endl;
    lognplot::TcpClient client;
    client.Connect("localhost:12345");
    std::cout << "Connected!" << std::endl;

    #define NUM_SAMPLES 10000
    const double A = 20.0;
    const double dt = 0.01; // (s)
    const double f = 2.7; // (Hz)
    const double omega = 2.0 * M_PI * f;

    double timestamps[NUM_SAMPLES];
    double values[NUM_SAMPLES];
    double sampled_values[NUM_SAMPLES];
    double t = 0.0;

    for (int i = 0; i < 10000; i++) {
        const double value = A * sin(omega * t);
        client.SendSample("C++ value", t, value);

        timestamps[i] = t;
        values[i] = value + 20.0;
        sampled_values[i] = value + 5.0;

        if (i % 10 == 0) {
            char buf[100];
            snprintf(buf, 100, "i = %i", i);
            client.SendText("C++ text", t, buf);
        }

        t += dt;
    }

    client.SendSamples("C++ multiple values", NUM_SAMPLES, timestamps, values);
    client.SendSampled("C++ sampled values", 2.0, dt, NUM_SAMPLES, sampled_values);

    client.Disconnect();

    std::cout << "That's all!" << std::endl;
}
