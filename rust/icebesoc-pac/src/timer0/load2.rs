#[doc = "Reader of register LOAD2"]
pub type R = crate::R<u8, super::LOAD2>;
#[doc = "Writer for register LOAD2"]
pub type W = crate::W<u8, super::LOAD2>;
#[doc = "Register LOAD2 `reset()`'s with value 0"]
impl crate::ResetValue for super::LOAD2 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `load`"]
pub type LOAD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `load`"]
pub struct LOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> LOAD_W<'a> {
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
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn load(&mut self) -> LOAD_W {
        LOAD_W { w: self }
    }
}
