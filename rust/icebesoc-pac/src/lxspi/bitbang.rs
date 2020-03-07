#[doc = "Reader of register BITBANG"]
pub type R = crate::R<u8, super::BITBANG>;
#[doc = "Writer for register BITBANG"]
pub type W = crate::W<u8, super::BITBANG>;
#[doc = "Register BITBANG `reset()`'s with value 0"]
impl crate::ResetValue for super::BITBANG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `mosi`"]
pub type MOSI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `mosi`"]
pub struct MOSI_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSI_W<'a> {
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
#[doc = "Reader of field `clk`"]
pub type CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clk`"]
pub struct CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `cs_n`"]
pub type CS_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cs_n`"]
pub struct CS_N_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_N_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `dir`"]
pub type DIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dir`"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Output value for MOSI pin, valid whenever ``dir`` is ``0``."]
    #[inline(always)]
    pub fn mosi(&self) -> MOSI_R {
        MOSI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Output value for SPI CLK pin."]
    #[inline(always)]
    pub fn clk(&self) -> CLK_R {
        CLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Output value for SPI CSn pin."]
    #[inline(always)]
    pub fn cs_n(&self) -> CS_N_R {
        CS_N_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Sets the direction for *ALL* SPI data pins except CLK and CSn."]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output value for MOSI pin, valid whenever ``dir`` is ``0``."]
    #[inline(always)]
    pub fn mosi(&mut self) -> MOSI_W {
        MOSI_W { w: self }
    }
    #[doc = "Bit 1 - Output value for SPI CLK pin."]
    #[inline(always)]
    pub fn clk(&mut self) -> CLK_W {
        CLK_W { w: self }
    }
    #[doc = "Bit 2 - Output value for SPI CSn pin."]
    #[inline(always)]
    pub fn cs_n(&mut self) -> CS_N_W {
        CS_N_W { w: self }
    }
    #[doc = "Bit 3 - Sets the direction for *ALL* SPI data pins except CLK and CSn."]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
}
