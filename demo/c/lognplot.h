/*
    lognplot C client API.

    Use this API to connect to a lognplot server and send
    data to it.
 */

#ifndef LOGNPLOT_H
#define LOGNPLOT_H

typedef int* lognplot_client_t;

/*
    Create a new client and connect to the server.

    \param address the address of the server.
*/
lognplot_client_t* lognplot_client_new(const char* address);

/*
    Send a single sample to the server.

    \param client the client pointer.
    \param name the name of the signal
    \param timestamp the timestamp of the sample
    \param value the value of the sample
*/
void lognplot_client_send_sample(
    lognplot_client_t* client,
    const char* name,
    double timestamp,
    double value
);

/*
    Send a batch of samples to the server.

    This can be handy if you have a lot of data and want to
    send it in batches.

    \param client the client structure
    \param name the name of the signal
    \param size the amount of samples
    \param times the timestamps of the samples
    \param value the values of the samples
 */
void lognplot_client_send_samples(
    lognplot_client_t* client,
    const char* name,
    const int size,
    double* times,
    double* values
);

/*
    Send a batch of sampled data to the server.

    Use this if your data is sampled at regular intervals
    for example, you have an array of values measured
    at intervals of 1 second.

    \param client the client structure
    \param name the name of the signal
    \param t0 the timestamp of the first data value
    \param dt the time interval in seconds
    \param size the amount of values
    \param values the actual data values
*/
void lognplot_client_send_sampled_samples(
    lognplot_client_t* client,
    const char* name,
    double t0,
    double dt,
    const int size,
    double* values
);

#endif
