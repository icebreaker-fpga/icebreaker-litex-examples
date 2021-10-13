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

    // time_init configures timer0 so we can use elapsed()
    time_init();
    int last_event = 0;
    
    while (1) {

        // Do we have an incoming byte?
        if(!uart_rxempty_read()) {
          // Read an incoming byte
          buf = uart_rxtx_read();
          // Tell the UART that we read a byte out of the FIFO
          // and that it can give us another.
          uart_ev_pending_write(UART_EV_RX);
          // Mirror the bytes back
          uart_rxtx_write(buf);
        }

        // Toggle LEDs every 250ms
        if(elapsed(&last_event, (CONFIG_CLOCK_FREQUENCY * 250e-3))){
          if (x == 2) {
              x = 0x00000001;
          } else {
              x = 0x00000002;
          }
        }

        // Write new LED output
        leds_out_write(x | (buf << 2));
    }
}
