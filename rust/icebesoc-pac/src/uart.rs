#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub rxtx: RXTX,
    _reserved1: [u8; 3usize],
    #[doc = "0x04 - "]
    pub txfull: TXFULL,
    _reserved2: [u8; 3usize],
    #[doc = "0x08 - "]
    pub rxempty: RXEMPTY,
    _reserved3: [u8; 3usize],
    #[doc = "0x0c - "]
    pub ev_status: EV_STATUS,
    _reserved4: [u8; 3usize],
    #[doc = "0x10 - "]
    pub ev_pending: EV_PENDING,
    _reserved5: [u8; 3usize],
    #[doc = "0x14 - "]
    pub ev_enable: EV_ENABLE,
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxtx](rxtx) module"]
pub type RXTX = crate::Reg<u8, _RXTX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXTX;
#[doc = "`read()` method returns [rxtx::R](rxtx::R) reader structure"]
impl crate::Readable for RXTX {}
#[doc = "`write(|w| ..)` method takes [rxtx::W](rxtx::W) writer structure"]
impl crate::Writable for RXTX {}
#[doc = ""]
pub mod rxtx;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfull](txfull) module"]
pub type TXFULL = crate::Reg<u8, _TXFULL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXFULL;
#[doc = "`read()` method returns [txfull::R](txfull::R) reader structure"]
impl crate::Readable for TXFULL {}
#[doc = ""]
pub mod txfull;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxempty](rxempty) module"]
pub type RXEMPTY = crate::Reg<u8, _RXEMPTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXEMPTY;
#[doc = "`read()` method returns [rxempty::R](rxempty::R) reader structure"]
impl crate::Readable for RXEMPTY {}
#[doc = ""]
pub mod rxempty;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ev_status](ev_status) module"]
pub type EV_STATUS = crate::Reg<u8, _EV_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EV_STATUS;
#[doc = "`read()` method returns [ev_status::R](ev_status::R) reader structure"]
impl crate::Readable for EV_STATUS {}
#[doc = "`write(|w| ..)` method takes [ev_status::W](ev_status::W) writer structure"]
impl crate::Writable for EV_STATUS {}
#[doc = ""]
pub mod ev_status;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ev_pending](ev_pending) module"]
pub type EV_PENDING = crate::Reg<u8, _EV_PENDING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EV_PENDING;
#[doc = "`read()` method returns [ev_pending::R](ev_pending::R) reader structure"]
impl crate::Readable for EV_PENDING {}
#[doc = "`write(|w| ..)` method takes [ev_pending::W](ev_pending::W) writer structure"]
impl crate::Writable for EV_PENDING {}
#[doc = ""]
pub mod ev_pending;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ev_enable](ev_enable) module"]
pub type EV_ENABLE = crate::Reg<u8, _EV_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EV_ENABLE;
#[doc = "`read()` method returns [ev_enable::R](ev_enable::R) reader structure"]
impl crate::Readable for EV_ENABLE {}
#[doc = "`write(|w| ..)` method takes [ev_enable::W](ev_enable::W) writer structure"]
impl crate::Writable for EV_ENABLE {}
#[doc = ""]
pub mod ev_enable;
