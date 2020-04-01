
#ifndef LOGNPLOT_CPP_CLIENT
#define LOGNPLOT_CPP_CLIENT

extern "C" {
#include "lognplot.h"
}

#include <exception>

namespace lognplot {

class TcpClient {
    public:
        TcpClient();
        void Connect(const char* address);
        void Disconnect();
        void SendSample(const char* name, const double timestamp, const double value);
        void SendSamples(const char* name, const size_t count, double* timestamps, double* values);
        void SendSampled(const char* name, const double timestamp, const double dt, const size_t count, double* values);
        void SendText(const char* name, const double timestamp, const char* text);

    private:
        lognplot_client_t* handle;
};

class ClientException : public std::exception
{
    public:
        ClientException(const char*);
        virtual const char * what () const throw ();
    
    private:
        const char* msg;
};

}

#endif
