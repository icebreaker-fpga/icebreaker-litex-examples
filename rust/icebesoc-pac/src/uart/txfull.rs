#[doc = "Reader of register TXFULL"]
pub type R = crate::R<u8, super::TXFULL>;
#[doc = "Reader of field `txfull`"]
pub type TXFULL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn txfull(&self) -> TXFULL_R {
        TXFULL_R::new((self.bits & 0x01) != 0)
    }
}
