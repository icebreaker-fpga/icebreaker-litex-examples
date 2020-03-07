#[doc = "Reader of register MISO"]
pub type R = crate::R<u8, super::MISO>;
#[doc = "Reader of field `miso`"]
pub type MISO_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn miso(&self) -> MISO_R {
        MISO_R::new((self.bits & 0x01) != 0)
    }
}
