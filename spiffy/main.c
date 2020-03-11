typedef unsigned int uint32_t;

volatile int a;
volatile int b;
volatile int w00t;
volatile int blaat;
volatile int sienus;

int main() {
    a = -500;
    sienus = 2;
    blaat = 3;
    w00t = 4;
    while(1) {
        if (a > 1000) {
            a = -500;
            sienus = 2;
            blaat = 3;
            w00t = 4;
        } else {
            a++;
        }

        if (a > 0) {
            sienus += 1;
            w00t -= 1;
        } else {
            sienus -= 1;
            w00t -= 2;
        }

        // poor man delay:
        int i,j;
        for (i=0;i<100;i++) {
            for (j=0;j<100;j++)
            {
                // nop..
                b = i + j;
            }
        }
    }
    return 0;
}

