#[doc = "Reader of register BUS_ERRORS1"]
pub type R = crate::R<u8, super::BUS_ERRORS1>;
#[doc = "Reader of field `bus_errors`"]
pub type BUS_ERRORS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn bus_errors(&self) -> BUS_ERRORS_R {
        BUS_ERRORS_R::new((self.bits & 0xff) as u8)
    }
}
