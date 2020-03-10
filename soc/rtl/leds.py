# This file is Copyright (c) 2020 Piotr Esden-Tempski <piotr@esden.net>
# License: BSD

from migen import *

from litex.soc.interconnect.csr import *
from litex.soc.integration.doc import AutoDoc, ModuleDoc


class Leds(Module, AutoCSR, AutoDoc):
    """iCEBreaker LED control.

    With this module you can control up to 8 LEDs. The class also handles documenting which bit
    corresponds to which LED as well as polarity corrections.

    Attributes:
        led_pin: Signals of the LED pin outputs.
        led_polarity: Bit pattern to adjust polarity. 0 stays the same 1 inverts the signal.
        led_name: Array of the LED names and descriptions. [["name1", "description1"], ["name2", "description2"]]
    """
    def __init__(self, led_pin, led_polarity=0x00, led_name=[]):
        # Documentation
        self.intro = ModuleDoc("""iCEBreaker LED control.
        The LEDs are inverted as these are negative logic LED. This means that if you set the
        corresponding LED bit to 1 the LED will be off and if you set it to 0 the LED will be on.
        """)

        # HDL Implementationj
        self._out = CSRStorage(len(led_pin), fields=[
            CSRField(fld[0], description=fld[1]) for fld in led_name
        ])
        self.comb += led_pin.eq(self._out.storage ^ led_polarity)
