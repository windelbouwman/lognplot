
#include <math.h>
#include "lognplot.h"

void main() {
    lognplot_client_t* c = lognplot_client_new("127.0.0.1:12345");
    double t, v;
    double dt;
    t = 0.0;
    dt = 0.0001;
    for (int i=0; i< 10000;i++) {
        v = 100.0 * sin(t) + 44.0;
        lognplot_client_send_sample(c, "C_signal", t, v);
        t += dt;
    }
}
