
#include "client.h"

namespace lognplot {

TcpClient::TcpClient()
: handle(0)
{
}

void TcpClient::Connect(const char* address)
{
    this->handle = lognplot_client_new(address);
    if (!this->handle)
    {
        throw ClientException("Connection to lognplot GUI failed");
    }
}

void TcpClient::Disconnect()
{
    lognplot_client_close(this->handle);
    this->handle = 0;
}

void TcpClient::SendSample(const char* name, const double timestamp, const double value)
{
    lognplot_result_t result = lognplot_client_send_sample(this->handle, name, timestamp, value);
    if (result != LOGNPLOT_RESULT_OK) {
        throw ClientException("Sending sample failed");
    }
}

void TcpClient::SendSamples(const char* name, const size_t count, double* timestamps, double* values)
{
    lognplot_result_t result = lognplot_client_send_samples(this->handle, name, count, timestamps, values);
    if (result != LOGNPLOT_RESULT_OK) {
        throw ClientException("Sending samples failed");
    }
}

void TcpClient::SendSampled(const char* name, const double timestamp, const double dt, const size_t count, double* values)
{
    lognplot_result_t result = lognplot_client_send_sampled_samples(this->handle, name, timestamp, dt, count, values);
    if (result != LOGNPLOT_RESULT_OK) {
        throw ClientException("Sending sampled samples failed");
    }
}

void TcpClient::SendText(const char* name, const double timestamp, const char* text)
{
    lognplot_result_t result = lognplot_client_send_text(this->handle, name, timestamp, text);
    if (result != LOGNPLOT_RESULT_OK) {
        throw ClientException("Sending text failed");
    }
}

ClientException::ClientException(const char* msg)
 : msg(msg)
{
}

const char * ClientException::what () const throw ()
{
    return msg;
}

}
