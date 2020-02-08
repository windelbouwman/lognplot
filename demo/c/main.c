
#include <math.h>
#include <stdlib.h>
#include <stdio.h>
#include "lognplot.h"

void send_single_samples(lognplot_client_t* client)
{
    double t, v;
    double dt;
    t = 0.0;
    dt = 0.01;
    for (int i=0; i< 10000;i++) {
        v = 100.0 * sin(t) + 44.0;
        lognplot_client_send_sample(client, "C_signal", t, v);
        t += dt;
    }
}

void send_batch(lognplot_client_t* client)
{
    // Send a batch of samples at once:
    const int amount = 100000;
    double *times = malloc(sizeof(double) * amount);
    double *values = malloc(sizeof(double) * amount);
    const double dt = 0.01;
    double t, v;
    t = 0.0;
    for (int i=0; i < amount; i++) {
        v = 35.0 * cos(t) - 42.0;
        times[i] = t;
        values[i] = v;
        t += dt;
    }
    lognplot_client_send_samples(client, "C_signal_batch", amount, times, values);
    free(times);
    free(values);
}

void send_sampled(lognplot_client_t* client)
{
    const int amount = 100000;
    const double t0 = 50.0;
    const double dt = 0.01;
    double t, v;
    // Send sampled data:
    double *values = malloc(sizeof(double) * amount);
    t = t0;
    for (int i=0; i < amount; i++) {
        v = 35.0 * cos(t * 3.14) + 42.0;
        values[i] = v;
        t += dt;
    }
    lognplot_client_send_sampled_samples(client, "C_signal_sampled_data", t0, dt, amount, values);
    free(values);
}

void main()
{
    lognplot_client_t* client = lognplot_client_new("127.0.0.1:12345");
    if (client) {
        send_single_samples(client);
        send_batch(client);
        send_sampled(client);
    } else {
        printf("Could not connect to data sink!\n");
    }
}
