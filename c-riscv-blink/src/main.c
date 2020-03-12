#include <generated/csr.h>
#include <time.h>

void isr(void) {
    asm("nop");
}


int main(void) {
    uint32_t x = 0;
    while (1) {
        uart_rxtx_write('a');
        leds_out_write(x);
        if (x == 0x02) {
            x = 0x00000001;
        } else {
            x = 0x00000002;
        }
        msleep(80);
    }
}
