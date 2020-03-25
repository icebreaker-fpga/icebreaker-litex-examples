#include <generated/csr.h>
#include <time.h>

#define UART_EV_TX 0x1
#define UART_EV_RX 0x2

void isr(void) {
    asm("nop");
}


int main(void) {
    uint32_t x = 0;
    uint32_t buf = 0;
    while (1) {

        // Wait for an incoming byte
        while (uart_rxempty_read()) {
            // Prevent the while loop from being optimized out
            asm("nop");
        }

        // Read an incoming byte
        buf = uart_rxtx_read();
        // Tell the UART that we read a byte out of the FIFO
        // and that it can give us another.
        uart_ev_pending_write(UART_EV_RX);
        // Mirror the bytes back
        uart_rxtx_write(buf);

        // Write new LED output and toggle
        leds_out_write(x | (buf << 2));
        if (x == 2) {
            x = 0x00000001;
        } else {
            x = 0x00000002;
        }
    }
}
