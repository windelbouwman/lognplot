/*
    lognplot C client API.

    Use this API to connect to a lognplot server and send
    data to it.
 */

#ifndef LOGNPLOT_H
#define LOGNPLOT_H

#include <stddef.h>
#include <stdint.h>

typedef int* lognplot_client_t;

#define LOGNPLOT_RESULT_OK 0
#define LOGNPLOT_RESULT_ERR_OTHER 1
#define LOGNPLOT_RESULT_ERR_INVALID_CLIENT_PTR 2
#define LOGNPLOT_RESULT_ERR_INVALID_ARGUMENT 3

typedef uint32_t lognplot_result_t;

/*
    Create a new client and connect to the server.

    \param address the address of the server.
*/
lognplot_client_t* lognplot_client_new(const char* address);

/*
  Close client connection gracefully.
 */
lognplot_result_t lognplot_client_close(lognplot_client_t* client);

/*
    Send a single sample to the server.

    \param client the client pointer.
    \param name the name of the signal
    \param timestamp the timestamp of the sample
    \param value the value of the sample
*/
lognplot_result_t lognplot_client_send_sample(
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
lognplot_result_t lognplot_client_send_samples(
    lognplot_client_t* client,
    const char* name,
    const size_t size,
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
lognplot_result_t lognplot_client_send_sampled_samples(
    lognplot_client_t* client,
    const char* name,
    double t0,
    double dt,
    const size_t size,
    double* values
);

/*
    Send a text event.

    Use this for example for log messages.

    \param client the client structure
    \name the name of the signal
    \timestamp the timestamp of the event
    \text the text message

*/
lognplot_result_t lognplot_client_send_text(
    lognplot_client_t* client,
    const char* name,
    double timestamp,
    const char* text
);

#endif
