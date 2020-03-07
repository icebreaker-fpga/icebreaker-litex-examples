#[doc = "Reader of register RXEMPTY"]
pub type R = crate::R<u8, super::RXEMPTY>;
#[doc = "Reader of field `rxempty`"]
pub type RXEMPTY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rxempty(&self) -> RXEMPTY_R {
        RXEMPTY_R::new((self.bits & 0x01) != 0)
    }
}
