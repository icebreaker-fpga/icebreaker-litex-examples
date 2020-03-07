#[doc = "Reader of register BITBANG_EN"]
pub type R = crate::R<u8, super::BITBANG_EN>;
#[doc = "Writer for register BITBANG_EN"]
pub type W = crate::W<u8, super::BITBANG_EN>;
#[doc = "Register BITBANG_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::BITBANG_EN {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `bitbang_en`"]
pub type BITBANG_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `bitbang_en`"]
pub struct BITBANG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BITBANG_EN_W<'a> {
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
    pub fn bitbang_en(&self) -> BITBANG_EN_R {
        BITBANG_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bitbang_en(&mut self) -> BITBANG_EN_W {
        BITBANG_EN_W { w: self }
    }
}
