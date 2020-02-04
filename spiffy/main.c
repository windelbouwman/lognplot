typedef unsigned int uint32_t;

volatile int a;
volatile int b;

int main() {
    a = -500;
    while(1) {
        if (a > 1000) {
            a = -500;
        } else {
            a++;
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

