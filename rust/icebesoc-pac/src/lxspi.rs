#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Bitbang controls for SPI output. Only standard 1x SPI is supported, and as a result all four wires are ganged together. This means that it is only possible to perform half-duplex operations, using this SPI core."]
    pub bitbang: BITBANG,
    _reserved1: [u8; 3usize],
    #[doc = "0x04 - Incoming value of MISO signal."]
    pub miso: MISO,
    _reserved2: [u8; 3usize],
    #[doc = "0x08 - Write a ``1`` here to disable memory-mapped mode and enable bitbang mode."]
    pub bitbang_en: BITBANG_EN,
}
#[doc = "Bitbang controls for SPI output. Only standard 1x SPI is supported, and as a result all four wires are ganged together. This means that it is only possible to perform half-duplex operations, using this SPI core.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bitbang](bitbang) module"]
pub type BITBANG = crate::Reg<u8, _BITBANG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BITBANG;
#[doc = "`read()` method returns [bitbang::R](bitbang::R) reader structure"]
impl crate::Readable for BITBANG {}
#[doc = "`write(|w| ..)` method takes [bitbang::W](bitbang::W) writer structure"]
impl crate::Writable for BITBANG {}
#[doc = "Bitbang controls for SPI output. Only standard 1x SPI is supported, and as a result all four wires are ganged together. This means that it is only possible to perform half-duplex operations, using this SPI core."]
pub mod bitbang;
#[doc = "Incoming value of MISO signal.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [miso](miso) module"]
pub type MISO = crate::Reg<u8, _MISO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISO;
#[doc = "`read()` method returns [miso::R](miso::R) reader structure"]
impl crate::Readable for MISO {}
#[doc = "Incoming value of MISO signal."]
pub mod miso;
#[doc = "Write a ``1`` here to disable memory-mapped mode and enable bitbang mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bitbang_en](bitbang_en) module"]
pub type BITBANG_EN = crate::Reg<u8, _BITBANG_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BITBANG_EN;
#[doc = "`read()` method returns [bitbang_en::R](bitbang_en::R) reader structure"]
impl crate::Readable for BITBANG_EN {}
#[doc = "`write(|w| ..)` method takes [bitbang_en::W](bitbang_en::W) writer structure"]
impl crate::Writable for BITBANG_EN {}
#[doc = "Write a ``1`` here to disable memory-mapped mode and enable bitbang mode."]
pub mod bitbang_en;
