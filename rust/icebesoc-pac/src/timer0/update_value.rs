#[doc = "Reader of register UPDATE_VALUE"]
pub type R = crate::R<u8, super::UPDATE_VALUE>;
#[doc = "Writer for register UPDATE_VALUE"]
pub type W = crate::W<u8, super::UPDATE_VALUE>;
#[doc = "Register UPDATE_VALUE `reset()`'s with value 0"]
impl crate::ResetValue for super::UPDATE_VALUE {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `update_value`"]
pub type UPDATE_VALUE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `update_value`"]
pub struct UPDATE_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDATE_VALUE_W<'a> {
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
    pub fn update_value(&self) -> UPDATE_VALUE_R {
        UPDATE_VALUE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn update_value(&mut self) -> UPDATE_VALUE_W {
        UPDATE_VALUE_W { w: self }
    }
}
