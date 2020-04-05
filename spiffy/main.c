typedef unsigned int uint32_t;

volatile int a;
volatile int b;
volatile int w00t;
volatile int blaat;
volatile int sienus_i32;
volatile float sinas_f32;
volatile char sinus_i8;

int main() {
    a = -500;
    sienus_i32 = 2;
    sinus_i8 = 3;
    blaat = 3;
    w00t = 4;
    while(1) {
        if (a > 100) {
            a = -50;
            sienus_i32 = 2;
            sinus_i8 = 3;
            // sinas_f32 = 2.2;
            blaat = 3;
            w00t = 4;
        } else {
            a++;
        }

        if (a > 0) {
            sienus_i32 += 1;
            sinus_i8 += 1;
            // sinas_f32 += 3.14;
            w00t -= 1;
        } else {
            sienus_i32 -= 1;
            sinus_i8 -= 1;
            // sinas_f32 -= 3.14;
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

