#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Write a ``1`` to this register to reset the SoC."]
    pub reset: RESET,
    _reserved1: [u8; 3usize],
    #[doc = "0x04 - Bits 24-31 of `CTRL_SCRATCH`. Use this register as a scratch space to verify that software read/write accesses to the Wishbone/CSR bus are working correctly. The initial reset value of 0x1234578 can be used to verify endianness."]
    pub scratch3: SCRATCH3,
    _reserved2: [u8; 3usize],
    #[doc = "0x08 - Bits 16-23 of `CTRL_SCRATCH`."]
    pub scratch2: SCRATCH2,
    _reserved3: [u8; 3usize],
    #[doc = "0x0c - Bits 8-15 of `CTRL_SCRATCH`."]
    pub scratch1: SCRATCH1,
    _reserved4: [u8; 3usize],
    #[doc = "0x10 - Bits 0-7 of `CTRL_SCRATCH`."]
    pub scratch0: SCRATCH0,
    _reserved5: [u8; 3usize],
    #[doc = "0x14 - Bits 24-31 of `CTRL_BUS_ERRORS`. Total number of Wishbone bus errors (timeouts) since last reset."]
    pub bus_errors3: BUS_ERRORS3,
    _reserved6: [u8; 3usize],
    #[doc = "0x18 - Bits 16-23 of `CTRL_BUS_ERRORS`."]
    pub bus_errors2: BUS_ERRORS2,
    _reserved7: [u8; 3usize],
    #[doc = "0x1c - Bits 8-15 of `CTRL_BUS_ERRORS`."]
    pub bus_errors1: BUS_ERRORS1,
    _reserved8: [u8; 3usize],
    #[doc = "0x20 - Bits 0-7 of `CTRL_BUS_ERRORS`."]
    pub bus_errors0: BUS_ERRORS0,
}
#[doc = "Write a ``1`` to this register to reset the SoC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset](reset) module"]
pub type RESET = crate::Reg<u8, _RESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESET;
#[doc = "`read()` method returns [reset::R](reset::R) reader structure"]
impl crate::Readable for RESET {}
#[doc = "`write(|w| ..)` method takes [reset::W](reset::W) writer structure"]
impl crate::Writable for RESET {}
#[doc = "Write a ``1`` to this register to reset the SoC."]
pub mod reset;
#[doc = "Bits 24-31 of `CTRL_SCRATCH`. Use this register as a scratch space to verify that software read/write accesses to the Wishbone/CSR bus are working correctly. The initial reset value of 0x1234578 can be used to verify endianness.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scratch3](scratch3) module"]
pub type SCRATCH3 = crate::Reg<u8, _SCRATCH3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCRATCH3;
#[doc = "`read()` method returns [scratch3::R](scratch3::R) reader structure"]
impl crate::Readable for SCRATCH3 {}
#[doc = "`write(|w| ..)` method takes [scratch3::W](scratch3::W) writer structure"]
impl crate::Writable for SCRATCH3 {}
#[doc = "Bits 24-31 of `CTRL_SCRATCH`. Use this register as a scratch space to verify that software read/write accesses to the Wishbone/CSR bus are working correctly. The initial reset value of 0x1234578 can be used to verify endianness."]
pub mod scratch3;
#[doc = "Bits 16-23 of `CTRL_SCRATCH`.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scratch2](scratch2) module"]
pub type SCRATCH2 = crate::Reg<u8, _SCRATCH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCRATCH2;
#[doc = "`read()` method returns [scratch2::R](scratch2::R) reader structure"]
impl crate::Readable for SCRATCH2 {}
#[doc = "`write(|w| ..)` method takes [scratch2::W](scratch2::W) writer structure"]
impl crate::Writable for SCRATCH2 {}
#[doc = "Bits 16-23 of `CTRL_SCRATCH`."]
pub mod scratch2;
#[doc = "Bits 8-15 of `CTRL_SCRATCH`.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scratch1](scratch1) module"]
pub type SCRATCH1 = crate::Reg<u8, _SCRATCH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCRATCH1;
#[doc = "`read()` method returns [scratch1::R](scratch1::R) reader structure"]
impl crate::Readable for SCRATCH1 {}
#[doc = "`write(|w| ..)` method takes [scratch1::W](scratch1::W) writer structure"]
impl crate::Writable for SCRATCH1 {}
#[doc = "Bits 8-15 of `CTRL_SCRATCH`."]
pub mod scratch1;
#[doc = "Bits 0-7 of `CTRL_SCRATCH`.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scratch0](scratch0) module"]
pub type SCRATCH0 = crate::Reg<u8, _SCRATCH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCRATCH0;
#[doc = "`read()` method returns [scratch0::R](scratch0::R) reader structure"]
impl crate::Readable for SCRATCH0 {}
#[doc = "`write(|w| ..)` method takes [scratch0::W](scratch0::W) writer structure"]
impl crate::Writable for SCRATCH0 {}
#[doc = "Bits 0-7 of `CTRL_SCRATCH`."]
pub mod scratch0;
#[doc = "Bits 24-31 of `CTRL_BUS_ERRORS`. Total number of Wishbone bus errors (timeouts) since last reset.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_errors3](bus_errors3) module"]
pub type BUS_ERRORS3 = crate::Reg<u8, _BUS_ERRORS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUS_ERRORS3;
#[doc = "`read()` method returns [bus_errors3::R](bus_errors3::R) reader structure"]
impl crate::Readable for BUS_ERRORS3 {}
#[doc = "Bits 24-31 of `CTRL_BUS_ERRORS`. Total number of Wishbone bus errors (timeouts) since last reset."]
pub mod bus_errors3;
#[doc = "Bits 16-23 of `CTRL_BUS_ERRORS`.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_errors2](bus_errors2) module"]
pub type BUS_ERRORS2 = crate::Reg<u8, _BUS_ERRORS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUS_ERRORS2;
#[doc = "`read()` method returns [bus_errors2::R](bus_errors2::R) reader structure"]
impl crate::Readable for BUS_ERRORS2 {}
#[doc = "Bits 16-23 of `CTRL_BUS_ERRORS`."]
pub mod bus_errors2;
#[doc = "Bits 8-15 of `CTRL_BUS_ERRORS`.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_errors1](bus_errors1) module"]
pub type BUS_ERRORS1 = crate::Reg<u8, _BUS_ERRORS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUS_ERRORS1;
#[doc = "`read()` method returns [bus_errors1::R](bus_errors1::R) reader structure"]
impl crate::Readable for BUS_ERRORS1 {}
#[doc = "Bits 8-15 of `CTRL_BUS_ERRORS`."]
pub mod bus_errors1;
#[doc = "Bits 0-7 of `CTRL_BUS_ERRORS`.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_errors0](bus_errors0) module"]
pub type BUS_ERRORS0 = crate::Reg<u8, _BUS_ERRORS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUS_ERRORS0;
#[doc = "`read()` method returns [bus_errors0::R](bus_errors0::R) reader structure"]
impl crate::Readable for BUS_ERRORS0 {}
#[doc = "Bits 0-7 of `CTRL_BUS_ERRORS`."]
pub mod bus_errors0;
