#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Bits 24-31 of `UART_PHY_TUNING_WORD`."]
    pub tuning_word3: TUNING_WORD3,
    _reserved1: [u8; 3usize],
    #[doc = "0x04 - Bits 16-23 of `UART_PHY_TUNING_WORD`."]
    pub tuning_word2: TUNING_WORD2,
    _reserved2: [u8; 3usize],
    #[doc = "0x08 - Bits 8-15 of `UART_PHY_TUNING_WORD`."]
    pub tuning_word1: TUNING_WORD1,
    _reserved3: [u8; 3usize],
    #[doc = "0x0c - Bits 0-7 of `UART_PHY_TUNING_WORD`."]
    pub tuning_word0: TUNING_WORD0,
}
#[doc = "Bits 24-31 of `UART_PHY_TUNING_WORD`.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tuning_word3](tuning_word3) module"]
pub type TUNING_WORD3 = crate::Reg<u8, _TUNING_WORD3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TUNING_WORD3;
#[doc = "`read()` method returns [tuning_word3::R](tuning_word3::R) reader structure"]
impl crate::Readable for TUNING_WORD3 {}
#[doc = "`write(|w| ..)` method takes [tuning_word3::W](tuning_word3::W) writer structure"]
impl crate::Writable for TUNING_WORD3 {}
#[doc = "Bits 24-31 of `UART_PHY_TUNING_WORD`."]
pub mod tuning_word3;
#[doc = "Bits 16-23 of `UART_PHY_TUNING_WORD`.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tuning_word2](tuning_word2) module"]
pub type TUNING_WORD2 = crate::Reg<u8, _TUNING_WORD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TUNING_WORD2;
#[doc = "`read()` method returns [tuning_word2::R](tuning_word2::R) reader structure"]
impl crate::Readable for TUNING_WORD2 {}
#[doc = "`write(|w| ..)` method takes [tuning_word2::W](tuning_word2::W) writer structure"]
impl crate::Writable for TUNING_WORD2 {}
#[doc = "Bits 16-23 of `UART_PHY_TUNING_WORD`."]
pub mod tuning_word2;
#[doc = "Bits 8-15 of `UART_PHY_TUNING_WORD`.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tuning_word1](tuning_word1) module"]
pub type TUNING_WORD1 = crate::Reg<u8, _TUNING_WORD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TUNING_WORD1;
#[doc = "`read()` method returns [tuning_word1::R](tuning_word1::R) reader structure"]
impl crate::Readable for TUNING_WORD1 {}
#[doc = "`write(|w| ..)` method takes [tuning_word1::W](tuning_word1::W) writer structure"]
impl crate::Writable for TUNING_WORD1 {}
#[doc = "Bits 8-15 of `UART_PHY_TUNING_WORD`."]
pub mod tuning_word1;
#[doc = "Bits 0-7 of `UART_PHY_TUNING_WORD`.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tuning_word0](tuning_word0) module"]
pub type TUNING_WORD0 = crate::Reg<u8, _TUNING_WORD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TUNING_WORD0;
#[doc = "`read()` method returns [tuning_word0::R](tuning_word0::R) reader structure"]
impl crate::Readable for TUNING_WORD0 {}
#[doc = "`write(|w| ..)` method takes [tuning_word0::W](tuning_word0::W) writer structure"]
impl crate::Writable for TUNING_WORD0 {}
#[doc = "Bits 0-7 of `UART_PHY_TUNING_WORD`."]
pub mod tuning_word0;
