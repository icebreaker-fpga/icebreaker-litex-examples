#[doc = "Reader of register EV_STATUS"]
pub type R = crate::R<u8, super::EV_STATUS>;
#[doc = "Writer for register EV_STATUS"]
pub type W = crate::W<u8, super::EV_STATUS>;
#[doc = "Register EV_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::EV_STATUS {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `status`"]
pub type STATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `status`"]
pub struct STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn status(&mut self) -> STATUS_W {
        STATUS_W { w: self }
    }
}
