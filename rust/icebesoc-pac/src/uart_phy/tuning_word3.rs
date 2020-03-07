#[doc = "Reader of register TUNING_WORD3"]
pub type R = crate::R<u8, super::TUNING_WORD3>;
#[doc = "Writer for register TUNING_WORD3"]
pub type W = crate::W<u8, super::TUNING_WORD3>;
#[doc = "Register TUNING_WORD3 `reset()`'s with value 0x02"]
impl crate::ResetValue for super::TUNING_WORD3 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `tuning_word`"]
pub type TUNING_WORD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tuning_word`"]
pub struct TUNING_WORD_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNING_WORD_W<'a> {
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
    pub fn tuning_word(&self) -> TUNING_WORD_R {
        TUNING_WORD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn tuning_word(&mut self) -> TUNING_WORD_W {
        TUNING_WORD_W { w: self }
    }
}
