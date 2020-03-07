#[doc = "Reader of register RXTX"]
pub type R = crate::R<u8, super::RXTX>;
#[doc = "Writer for register RXTX"]
pub type W = crate::W<u8, super::RXTX>;
#[doc = "Register RXTX `reset()`'s with value 0"]
impl crate::ResetValue for super::RXTX {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rxtx`"]
pub type RXTX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rxtx`"]
pub struct RXTX_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rxtx(&self) -> RXTX_R {
        RXTX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rxtx(&mut self) -> RXTX_W {
        RXTX_W { w: self }
    }
}
