#include <generated/csr.h>
#include <time.h>
// #include <irq.h>
// #include <rgb.h>
// #include <usb.h>

// // Input a value 0 to 255 to get a color value.
// // The colours are a transition r - g - b - back to r.
// static void color_wheel(uint8_t WheelPos) {
//     WheelPos = 255 - WheelPos;
//     uint8_t r, g, b;
//     if(WheelPos < 85) {
//         r = 255 - WheelPos * 3;
//         g = 0;
//         b = WheelPos * 3;
//     }
//     else if(WheelPos < 170) {
//         WheelPos -= 85;
//         r = 0;
//         g = WheelPos * 3;
//         b = 255 - WheelPos * 3;
//     }
//     else {
//         WheelPos -= 170;
//         r = WheelPos * 3;
//         g = 255 - WheelPos * 3;
//         b = 0;
//     }

//     rgb_set(r, g, b);
// }

void isr(void) {
    //unsigned int irqs;

    //irqs = irq_pending() & irq_getmask();

    //if (irqs & (1 << USB_INTERRUPT))
    //    usb_isr();
    asm("nop");
}


int main(void) {
    // irq_setmask(0);
    // irq_setie(1);
    // usb_init();
    // rgb_init();
    // usb_connect();
    // rgb_write((100000/64000)-1, LEDDBR);
    // int i = 0;
    // while (1) {
    //     color_wheel(i++);
    //     msleep(80);
    // }
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
