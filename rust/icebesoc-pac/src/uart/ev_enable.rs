#[doc = "Reader of register EV_ENABLE"]
pub type R = crate::R<u8, super::EV_ENABLE>;
#[doc = "Writer for register EV_ENABLE"]
pub type W = crate::W<u8, super::EV_ENABLE>;
#[doc = "Register EV_ENABLE `reset()`'s with value 0"]
impl crate::ResetValue for super::EV_ENABLE {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `enable`"]
pub type ENABLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `enable`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
