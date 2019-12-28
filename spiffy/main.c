
// LEDs are located on GPIOD pin 12 though 15

// #include <stm32f4xx.h>

typedef unsigned int uint32_t;

volatile int a;
volatile int b;

int main() {
    volatile uint32_t* ITM_STIM0 = 0xE0000000;
    while(1) {
        while ((*ITM_STIM0) == 0) {}
        *ITM_STIM0 = 'A';
        while ((*ITM_STIM0) == 0) {}
        *ITM_STIM0 = 'B';
        while ((*ITM_STIM0) == 0) {}
        *ITM_STIM0 = 'C';

        a++;

        // poor man delay:
        int i,j;
        for (i=0;i<100;i++) {
            for (j=0;j<1000;j++)
            {
                // nop..
                b = i + j;
            }
        }
    }
    return 0;
}

