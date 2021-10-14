#!/usr/bin/env python3

# This file is Copyright (c) 2019 Sean Cross <sean@xobs.io>
# This file is Copyright (c) 2018 David Shah <dave@ds0.me>
# This file is Copyright (c) 2020 Piotr Esden-Tempski <piotr@esden.net>
# License: BSD

# This target was originally based on the Fomu target.

# This variable defines all the external programs that this module
# relies on.  lxbuildenv reads this variable in order to ensure
# the build will finish without exiting due to missing third-party
# programs.
LX_DEPENDENCIES = ["riscv", "icestorm", "yosys", "nextpnr-ice40"]

# Import lxbuildenv to integrate the deps/ directory
import lxbuildenv

import argparse
from os import path

from migen import *
from migen.genlib.resetsync import AsyncResetSynchronizer

from litex.soc.cores.ram import Up5kSPRAM
from litex.soc.cores.spi_flash import SpiFlash
from litex.soc.cores.clock import iCE40PLL
from litex.soc.integration.soc_core import SoCCore
from litex.soc.integration.builder import Builder, builder_argdict, builder_args
from litex.build.lattice.programmer import IceStormProgrammer
from litex.soc.integration.soc_core import soc_core_argdict, soc_core_args
from litex.soc.integration.doc import AutoDoc
from litex.soc.integration.soc import SoCRegion

from litex_boards.platforms.icebreaker import Platform, break_off_pmod

from litex.soc.cores.uart import UARTWishboneBridge
from rtl.leds import Leds

import litex.soc.doc as lxsocdoc


# CRG ----------------------------------------------------------------------------------------------


class _CRG(Module, AutoDoc):
    """Icebreaker Clock Resource Generator

    The system is clocked by the external 12MHz clock. But if a sys_clk_freq is set to a value
    that is different from the default 12MHz we will feed it through the PLL block and try to
    generate a clock as close as possible to the selected frequency.
    """
    def __init__(self, platform, sys_clk_freq):
        self.clock_domains.cd_sys = ClockDomain()
        self.clock_domains.cd_por = ClockDomain()

        # # #

        # Clocks
        clk12 = platform.request("clk12")
        if sys_clk_freq == 12e6:
            self.comb += self.cd_sys.clk.eq(clk12)
        else:
            self.submodules.pll = pll = iCE40PLL(primitive="SB_PLL40_PAD")
            pll.register_clkin(clk12, 12e6)
            pll.create_clkout(self.cd_sys, sys_clk_freq, with_reset=False)
        platform.add_period_constraint(self.cd_sys.clk, 1e9 / sys_clk_freq)

        # Power On Reset
        self.reset = Signal()
        por_cycles = 4096
        por_counter = Signal(log2_int(por_cycles), reset=por_cycles - 1)
        self.comb += self.cd_por.clk.eq(self.cd_sys.clk)
        platform.add_period_constraint(self.cd_por.clk, 1e9 / sys_clk_freq)
        self.sync.por += If(por_counter != 0, por_counter.eq(por_counter - 1))
        self.comb += self.cd_sys.rst.eq(por_counter != 0)
        self.specials += AsyncResetSynchronizer(self.cd_por, self.reset)


# BaseSoC ------------------------------------------------------------------------------------------

class BaseSoC(SoCCore):
    """A SoC on iCEBreaker, optionally with a softcore CPU"""

    # Statically-define the memory map, to prevent it from shifting across various litex versions.
    SoCCore.mem_map = {
        "sram":             0x10000000,
        "spiflash":         0x20000000,
        "csr":              0xf0000000,
        "vexriscv_debug":   0xf00f0000,
    }

    def __init__(self, debug, flash_offset, sys_clk_freq, **kwargs):
        """Create a basic SoC for iCEBreaker.

        Create a basic SoC for iCEBreaker.  The `sys` frequency will run at 12 MHz.

        Returns:
            Newly-constructed SoC
        """
        platform = Platform()

        # Set cpu name and variant defaults when none are provided
        if "cpu_variant" not in kwargs:
            if debug:
                kwargs["cpu_variant"] = "lite+debug"
            else:
                kwargs["cpu_variant"] = "lite"

        # Force the SRAM size to 0, because we add our own SRAM with SPRAM
        kwargs["integrated_sram_size"] = 0
        kwargs["integrated_rom_size"]  = 0

        kwargs["csr_data_width"] = 32

        # Set CPU reset address
        kwargs["cpu_reset_address"] = self.mem_map["spiflash"] + flash_offset

        # Select "crossover" as soc uart instead of "serial"
        # We have to make that selection before calling the parent initializer
        if debug:
            kwargs["uart_name"]   = "crossover"

        # SoCCore
        SoCCore.__init__(self, platform, sys_clk_freq, **kwargs)

        self.submodules.crg = _CRG(platform, sys_clk_freq)

        # UP5K has single port RAM, which is a dedicated 128 kilobyte block.
        # Use this as CPU RAM.
        spram_size = 128 * 1024
        self.submodules.spram = Up5kSPRAM(size=spram_size)
        self.register_mem("sram", self.mem_map["sram"], self.spram.bus, spram_size)

        # SPI Flash --------------------------------------------------------------------------------
        from litespi.modules import W25Q128JV
        from litespi.opcodes import SpiNorFlashOpCodes as Codes
        self.add_spi_flash(mode="4x", module=W25Q128JV(Codes.READ_1_1_4), with_master=False)

        # Add ROM linker region --------------------------------------------------------------------
        self.bus.add_region("rom", SoCRegion(
            origin = self.mem_map["spiflash"] + flash_offset,
            size   = 8*1024*1024,
            linker = True)
        )

        # In debug mode, add a UART bridge.  This takes over from the normal UART bridge,
        # however you can use the "crossover" UART to communicate with this over the bridge.
        if debug:
            self.submodules.uart_bridge = UARTWishboneBridge(platform.request("serial"), sys_clk_freq, baudrate=115200)
            self.add_wb_master(self.uart_bridge.wishbone)

        platform.add_extension(break_off_pmod)

        self.submodules.leds = Leds(Cat(
            platform.request("user_ledr_n"),
            platform.request("user_ledg_n"),
            platform.request("user_ledr"),
            platform.request("user_ledg", 0),
            platform.request("user_ledg", 1),
            platform.request("user_ledg", 2),
            platform.request("user_ledg", 3)),
            led_polarity=0x03,
            led_name=[
                ["ledr", "The Red LED on the main iCEBreaker board."],
                ["ledg", "The Green LED on the main iCEBreaker board."],
                ["hledr1", "The center Red LED #1 on the iCEBreaker head."],
                ["hledg2", "Green LED #2 on the iCEBreaker head."],
                ["hledg3", "Green LED #3 on the iCEBreaker head."],
                ["hledg4", "Green LED #4 on the iCEBreaker head."],
                ["hledg5", "Green LED #5 on the iCEBreaker head."]])

        self.add_csr("leds")

    def set_yosys_nextpnr_settings(self, nextpnr_seed=0, nextpnr_placer="heap"):
        """Set Yosys/Nextpnr settings by overriding default LiteX's settings.
        Args:
            nextpnr_seed   (int): Seed to use in Nextpnr
            nextpnr_placer (str): Placer to use in Nextpnr
        """
        assert hasattr(self.platform.toolchain, "yosys_template")
        assert hasattr(self.platform.toolchain, "build_template")
        self.platform.toolchain.yosys_template = [
            "{read_files}",
            "attrmap -tocase keep -imap keep=\"true\" keep=1 -imap keep=\"false\" keep=0 -remove keep=0",
            # Use "-relut -dffe_min_ce_use 4" to the synth_ice40 command. The "-reult" adds an additional
            # LUT pass to pack more stuff in, and the "-dffe_min_ce_use 4" flag prevents Yosys from
            # generating a Clock Enable signal for a LUT that has fewer than 4 flip-flops. This increases
            # density, and lets us use the FPGA more efficiently.
            "synth_ice40 -json {build_name}.json -top {build_name} -relut -abc2 -dffe_min_ce_use 4 -relut",
        ]
        self.platform.toolchain.build_template = [
            "yosys -q -l {build_name}.rpt {build_name}.ys",
            "nextpnr-ice40 --json {build_name}.json --pcf {build_name}.pcf --asc {build_name}.txt" +
            " --pre-pack {build_name}_pre_pack.py --{architecture} --package {package}" +
            " --seed {}".format(nextpnr_seed) +
            " --placer {}".format(nextpnr_placer),
            # Disable final deep-sleep power down so firmware words are loaded onto softcore's address bus.
            "icepack -s {build_name}.txt {build_name}.bin"
        ]


# Build --------------------------------------------------------------------------------------------

def main():
    parser = argparse.ArgumentParser(description="LiteX SoC on iCEBreaker")
    parser.add_argument("--flash-offset", default=0x40000, help="Boot offset in SPI Flash")
    parser.add_argument("--sys-clk-freq", type=float, default=21e6, help="Select system clock frequency")
    parser.add_argument("--nextpnr-seed", default=0, help="Select nextpnr pseudo random seed")
    parser.add_argument("--nextpnr-placer", default="heap", choices=["sa", "heap"], help="Select nextpnr placer algorithm")
    parser.add_argument("--debug", action="store_true", help="Enable debug features. (UART has to be used with the wishbone-tool.)")
    parser.add_argument("--document-only", action="store_true", help="Do not build a soc. Only generate documentation.")
    parser.add_argument("--flash", action="store_true", help="Load bitstream")
    builder_args(parser)
    soc_core_args(parser)
    args = parser.parse_args()

    # Create the SOC
    soc = BaseSoC(debug=args.debug, flash_offset=args.flash_offset, sys_clk_freq=int(args.sys_clk_freq), **soc_core_argdict(args))
    soc.set_yosys_nextpnr_settings(nextpnr_seed=args.nextpnr_seed, nextpnr_placer=args.nextpnr_placer)

    # Configure command line parameter defaults
    # Don't build software -- we don't include it since we just jump to SPI flash.
    builder_kwargs = builder_argdict(args)
    builder_kwargs["compile_software"] = False

    if args.document_only:
        builder_kwargs["compile_gateware"] = False
    if builder_kwargs["csr_svd"] is None:
        builder_kwargs["csr_svd"] = "../rust/icebesoc-pac/iCEBESOC.svd"
    if builder_kwargs["memory_x"] is None:
        builder_kwargs["memory_x"] = "../rust/icebesoc-pac/memory.x"

    # Create and run the builder
    builder = Builder(soc, **builder_kwargs)
    builder.build()
    
    lxsocdoc.generate_docs(soc, "build/documentation/", project_name="iCEBreaker LiteX Riscv Example SOC", author="Piotr Esden-Tempski")

    # If requested load the resulting bitstream onto the iCEBreaker
    if args.flash:
        IceStormProgrammer().flash(0x00000000, path.join(builder.gateware_dir,"{}.bin".format(soc.build_name)))


if __name__ == "__main__":
    main()
