#include <generated/csr.h>
#include <time.h>

void isr(void) {
    asm("nop");
}


int main(void) {
    uint8_t x = 0;
    while (1) {
        uart_rxtx_write('a');
        leds_out_write(x);
        if (x == 0x02) {
            x = 0x01;
        } else {
            x = 0x02;
        }
        msleep(80);
    }
}
