
#include "client.h"

namespace lognplot {

TcpClient::TcpClient()
: handle(0)
{
}

void TcpClient::Connect(const char* address)
{
    this->handle = lognplot_client_new(address);
}

void TcpClient::Disconnect()
{
    lognplot_client_close(this->handle);
}

void TcpClient::SendSample(const char* name, const double timestamp, const double value)
{
    lognplot_client_send_sample(this->handle, name, timestamp, value);
}

void TcpClient::SendSamples(const char* name, const size_t count, double* timestamps, double* values)
{
    lognplot_client_send_samples(this->handle, name, count, timestamps, values);
}

void TcpClient::SendSampled(const char* name, const double timestamp, const double dt, const size_t count, double* values)
{
    lognplot_client_send_sampled_samples(this->handle, name, timestamp, dt, count, values);
}

void TcpClient::SendText(const char* name, const double timestamp, const char* text)
{
    lognplot_client_send_text(this->handle, name, timestamp, text);
}

}
