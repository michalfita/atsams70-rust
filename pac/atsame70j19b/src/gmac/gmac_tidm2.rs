#[doc = "Reader of register GMAC_TIDM2"]
pub type R = crate::R<u32, super::GMAC_TIDM2>;
#[doc = "Writer for register GMAC_TIDM2"]
pub type W = crate::W<u32, super::GMAC_TIDM2>;
#[doc = "Register GMAC_TIDM2 `reset()`'s with value 0"]
impl crate::ResetValue for super::GMAC_TIDM2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TID`"]
pub type TID_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TID`"]
pub struct TID_W<'a> {
    w: &'a mut W,
}
impl<'a> TID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ENID2`"]
pub type ENID2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENID2`"]
pub struct ENID2_W<'a> {
    w: &'a mut W,
}
impl<'a> ENID2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Type ID Match 2"]
    #[inline(always)]
    pub fn tid(&self) -> TID_R {
        TID_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Enable Copying of TID Matched Frames"]
    #[inline(always)]
    pub fn enid2(&self) -> ENID2_R {
        ENID2_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Type ID Match 2"]
    #[inline(always)]
    pub fn tid(&mut self) -> TID_W {
        TID_W { w: self }
    }
    #[doc = "Bit 31 - Enable Copying of TID Matched Frames"]
    #[inline(always)]
    pub fn enid2(&mut self) -> ENID2_W {
        ENID2_W { w: self }
    }
}
