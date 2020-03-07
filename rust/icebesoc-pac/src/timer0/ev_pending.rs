#[doc = "Reader of register EV_PENDING"]
pub type R = crate::R<u8, super::EV_PENDING>;
#[doc = "Writer for register EV_PENDING"]
pub type W = crate::W<u8, super::EV_PENDING>;
#[doc = "Register EV_PENDING `reset()`'s with value 0"]
impl crate::ResetValue for super::EV_PENDING {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pending`"]
pub type PENDING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pending`"]
pub struct PENDING_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDING_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pending(&self) -> PENDING_R {
        PENDING_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pending(&mut self) -> PENDING_W {
        PENDING_W { w: self }
    }
}
