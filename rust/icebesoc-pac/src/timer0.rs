#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Bits 24-31 of `TIMER0_LOAD`. Load value when Timer is (re-)enabled. In One-Shot mode, the value written to this register specifies the Timer's duration in clock cycles."]
    pub load3: LOAD3,
    _reserved1: [u8; 3usize],
    #[doc = "0x04 - Bits 16-23 of `TIMER0_LOAD`."]
    pub load2: LOAD2,
    _reserved2: [u8; 3usize],
    #[doc = "0x08 - Bits 8-15 of `TIMER0_LOAD`."]
    pub load1: LOAD1,
    _reserved3: [u8; 3usize],
    #[doc = "0x0c - Bits 0-7 of `TIMER0_LOAD`."]
    pub load0: LOAD0,
    _reserved4: [u8; 3usize],
    #[doc = "0x10 - Bits 24-31 of `TIMER0_RELOAD`. Reload value when Timer reaches ``0``. In Periodic mode, the value written to this register specify the Timer's period in clock cycles."]
    pub reload3: RELOAD3,
    _reserved5: [u8; 3usize],
    #[doc = "0x14 - Bits 16-23 of `TIMER0_RELOAD`."]
    pub reload2: RELOAD2,
    _reserved6: [u8; 3usize],
    #[doc = "0x18 - Bits 8-15 of `TIMER0_RELOAD`."]
    pub reload1: RELOAD1,
    _reserved7: [u8; 3usize],
    #[doc = "0x1c - Bits 0-7 of `TIMER0_RELOAD`."]
    pub reload0: RELOAD0,
    _reserved8: [u8; 3usize],
    #[doc = "0x20 - Enable flag of the Timer. Set this flag to ``1`` to enable/start the Timer. Set to ``0`` to disable the Timer."]
    pub en: EN,
    _reserved9: [u8; 3usize],
    #[doc = "0x24 - Update trigger for the current countdown value. A write to this register latches the current countdown value to ``value`` register."]
    pub update_value: UPDATE_VALUE,
    _reserved10: [u8; 3usize],
    #[doc = "0x28 - Bits 24-31 of `TIMER0_VALUE`. Latched countdown value. This value is updated by writing to ``update_value``."]
    pub value3: VALUE3,
    _reserved11: [u8; 3usize],
    #[doc = "0x2c - Bits 16-23 of `TIMER0_VALUE`."]
    pub value2: VALUE2,
    _reserved12: [u8; 3usize],
    #[doc = "0x30 - Bits 8-15 of `TIMER0_VALUE`."]
    pub value1: VALUE1,
    _reserved13: [u8; 3usize],
    #[doc = "0x34 - Bits 0-7 of `TIMER0_VALUE`."]
    pub value0: VALUE0,
    _reserved14: [u8; 3usize],
    #[doc = "0x38 - "]
    pub ev_status: EV_STATUS,
    _reserved15: [u8; 3usize],
    #[doc = "0x3c - "]
    pub ev_pending: EV_PENDING,
    _reserved16: [u8; 3usize],
    #[doc = "0x40 - "]
    pub ev_enable: EV_ENABLE,
}
#[doc = "Bits 24-31 of `TIMER0_LOAD`. Load value when Timer is (re-)enabled. In One-Shot mode, the value written to this register specifies the Timer's duration in clock cycles.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [load3](load3) module"]
pub type LOAD3 = crate::Reg<u8, _LOAD3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOAD3;
#[doc = "`read()` method returns [load3::R](load3::R) reader structure"]
impl crate::Readable for LOAD3 {}
#[doc = "`write(|w| ..)` method takes [load3::W](load3::W) writer structure"]
impl crate::Writable for LOAD3 {}
#[doc = "Bits 24-31 of `TIMER0_LOAD`. Load value when Timer is (re-)enabled. In One-Shot mode, the value written to this register specifies the Timer's duration in clock cycles."]
pub mod load3;
#[doc = "Bits 16-23 of `TIMER0_LOAD`.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [load2](load2) module"]
pub type LOAD2 = crate::Reg<u8, _LOAD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOAD2;
#[doc = "`read()` method returns [load2::R](load2::R) reader structure"]
impl crate::Readable for LOAD2 {}
#[doc = "`write(|w| ..)` method takes [load2::W](load2::W) writer structure"]
impl crate::Writable for LOAD2 {}
#[doc = "Bits 16-23 of `TIMER0_LOAD`."]
pub mod load2;
#[doc = "Bits 8-15 of `TIMER0_LOAD`.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [load1](load1) module"]
pub type LOAD1 = crate::Reg<u8, _LOAD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOAD1;
#[doc = "`read()` method returns [load1::R](load1::R) reader structure"]
impl crate::Readable for LOAD1 {}
#[doc = "`write(|w| ..)` method takes [load1::W](load1::W) writer structure"]
impl crate::Writable for LOAD1 {}
#[doc = "Bits 8-15 of `TIMER0_LOAD`."]
pub mod load1;
#[doc = "Bits 0-7 of `TIMER0_LOAD`.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [load0](load0) module"]
pub type LOAD0 = crate::Reg<u8, _LOAD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOAD0;
#[doc = "`read()` method returns [load0::R](load0::R) reader structure"]
impl crate::Readable for LOAD0 {}
#[doc = "`write(|w| ..)` method takes [load0::W](load0::W) writer structure"]
impl crate::Writable for LOAD0 {}
#[doc = "Bits 0-7 of `TIMER0_LOAD`."]
pub mod load0;
#[doc = "Bits 24-31 of `TIMER0_RELOAD`. Reload value when Timer reaches ``0``. In Periodic mode, the value written to this register specify the Timer's period in clock cycles.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reload3](reload3) module"]
pub type RELOAD3 = crate::Reg<u8, _RELOAD3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RELOAD3;
#[doc = "`read()` method returns [reload3::R](reload3::R) reader structure"]
impl crate::Readable for RELOAD3 {}
#[doc = "`write(|w| ..)` method takes [reload3::W](reload3::W) writer structure"]
impl crate::Writable for RELOAD3 {}
#[doc = "Bits 24-31 of `TIMER0_RELOAD`. Reload value when Timer reaches ``0``. In Periodic mode, the value written to this register specify the Timer's period in clock cycles."]
pub mod reload3;
#[doc = "Bits 16-23 of `TIMER0_RELOAD`.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reload2](reload2) module"]
pub type RELOAD2 = crate::Reg<u8, _RELOAD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RELOAD2;
#[doc = "`read()` method returns [reload2::R](reload2::R) reader structure"]
impl crate::Readable for RELOAD2 {}
#[doc = "`write(|w| ..)` method takes [reload2::W](reload2::W) writer structure"]
impl crate::Writable for RELOAD2 {}
#[doc = "Bits 16-23 of `TIMER0_RELOAD`."]
pub mod reload2;
#[doc = "Bits 8-15 of `TIMER0_RELOAD`.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reload1](reload1) module"]
pub type RELOAD1 = crate::Reg<u8, _RELOAD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RELOAD1;
#[doc = "`read()` method returns [reload1::R](reload1::R) reader structure"]
impl crate::Readable for RELOAD1 {}
#[doc = "`write(|w| ..)` method takes [reload1::W](reload1::W) writer structure"]
impl crate::Writable for RELOAD1 {}
#[doc = "Bits 8-15 of `TIMER0_RELOAD`."]
pub mod reload1;
#[doc = "Bits 0-7 of `TIMER0_RELOAD`.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reload0](reload0) module"]
pub type RELOAD0 = crate::Reg<u8, _RELOAD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RELOAD0;
#[doc = "`read()` method returns [reload0::R](reload0::R) reader structure"]
impl crate::Readable for RELOAD0 {}
#[doc = "`write(|w| ..)` method takes [reload0::W](reload0::W) writer structure"]
impl crate::Writable for RELOAD0 {}
#[doc = "Bits 0-7 of `TIMER0_RELOAD`."]
pub mod reload0;
#[doc = "Enable flag of the Timer. Set this flag to ``1`` to enable/start the Timer. Set to ``0`` to disable the Timer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en](en) module"]
pub type EN = crate::Reg<u8, _EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EN;
#[doc = "`read()` method returns [en::R](en::R) reader structure"]
impl crate::Readable for EN {}
#[doc = "`write(|w| ..)` method takes [en::W](en::W) writer structure"]
impl crate::Writable for EN {}
#[doc = "Enable flag of the Timer. Set this flag to ``1`` to enable/start the Timer. Set to ``0`` to disable the Timer."]
pub mod en;
#[doc = "Update trigger for the current countdown value. A write to this register latches the current countdown value to ``value`` register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [update_value](update_value) module"]
pub type UPDATE_VALUE = crate::Reg<u8, _UPDATE_VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPDATE_VALUE;
#[doc = "`read()` method returns [update_value::R](update_value::R) reader structure"]
impl crate::Readable for UPDATE_VALUE {}
#[doc = "`write(|w| ..)` method takes [update_value::W](update_value::W) writer structure"]
impl crate::Writable for UPDATE_VALUE {}
#[doc = "Update trigger for the current countdown value. A write to this register latches the current countdown value to ``value`` register."]
pub mod update_value;
#[doc = "Bits 24-31 of `TIMER0_VALUE`. Latched countdown value. This value is updated by writing to ``update_value``.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [value3](value3) module"]
pub type VALUE3 = crate::Reg<u8, _VALUE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VALUE3;
#[doc = "`read()` method returns [value3::R](value3::R) reader structure"]
impl crate::Readable for VALUE3 {}
#[doc = "Bits 24-31 of `TIMER0_VALUE`. Latched countdown value. This value is updated by writing to ``update_value``."]
pub mod value3;
#[doc = "Bits 16-23 of `TIMER0_VALUE`.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [value2](value2) module"]
pub type VALUE2 = crate::Reg<u8, _VALUE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VALUE2;
#[doc = "`read()` method returns [value2::R](value2::R) reader structure"]
impl crate::Readable for VALUE2 {}
#[doc = "Bits 16-23 of `TIMER0_VALUE`."]
pub mod value2;
#[doc = "Bits 8-15 of `TIMER0_VALUE`.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [value1](value1) module"]
pub type VALUE1 = crate::Reg<u8, _VALUE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VALUE1;
#[doc = "`read()` method returns [value1::R](value1::R) reader structure"]
impl crate::Readable for VALUE1 {}
#[doc = "Bits 8-15 of `TIMER0_VALUE`."]
pub mod value1;
#[doc = "Bits 0-7 of `TIMER0_VALUE`.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [value0](value0) module"]
pub type VALUE0 = crate::Reg<u8, _VALUE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VALUE0;
#[doc = "`read()` method returns [value0::R](value0::R) reader structure"]
impl crate::Readable for VALUE0 {}
#[doc = "Bits 0-7 of `TIMER0_VALUE`."]
pub mod value0;
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
