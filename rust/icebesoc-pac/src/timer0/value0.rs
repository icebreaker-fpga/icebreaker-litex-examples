#[doc = "Reader of register VALUE0"]
pub type R = crate::R<u8, super::VALUE0>;
#[doc = "Reader of field `value`"]
pub type VALUE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xff) as u8)
    }
}
