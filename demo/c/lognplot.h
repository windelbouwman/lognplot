
#ifndef LOGNPLOT_H
#define LOGNPLOT_H

typedef int* lognplot_client_t;

lognplot_client_t* lognplot_client_new(const char* address);
void lognplot_client_send_sample(lognplot_client_t*, const char*, double, double);

#endif
