#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RTC_TIMALR {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type SEC_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _SECW<'a> {
    w: &'a mut W,
}
impl<'a> _SECW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SECEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SECENW<'a> {
    w: &'a mut W,
}
impl<'a> _SECENW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type MIN_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _MINW<'a> {
    w: &'a mut W,
}
impl<'a> _MINW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type MINEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MINENW<'a> {
    w: &'a mut W,
}
impl<'a> _MINENW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type HOUR_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _HOURW<'a> {
    w: &'a mut W,
}
impl<'a> _HOURW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type AMPM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _AMPMW<'a> {
    w: &'a mut W,
}
impl<'a> _AMPMW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type HOUREN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _HOURENW<'a> {
    w: &'a mut W,
}
impl<'a> _HOURENW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:6 - Second Alarm"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits() & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Second Alarm Enable"]
    #[inline(always)]
    pub fn secen(&self) -> SECEN_R {
        SECEN_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - Minute Alarm"]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new(((self.bits() >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Minute Alarm Enable"]
    #[inline(always)]
    pub fn minen(&self) -> MINEN_R {
        MINEN_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - Hour Alarm"]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new(((self.bits() >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - AM/PM Indicator"]
    #[inline(always)]
    pub fn ampm(&self) -> AMPM_R {
        AMPM_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Hour Alarm Enable"]
    #[inline(always)]
    pub fn houren(&self) -> HOUREN_R {
        HOUREN_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - Second Alarm"]
    #[inline(always)]
    pub fn sec(&mut self) -> _SECW {
        _SECW { w: self }
    }
    #[doc = "Bit 7 - Second Alarm Enable"]
    #[inline(always)]
    pub fn secen(&mut self) -> _SECENW {
        _SECENW { w: self }
    }
    #[doc = "Bits 8:14 - Minute Alarm"]
    #[inline(always)]
    pub fn min(&mut self) -> _MINW {
        _MINW { w: self }
    }
    #[doc = "Bit 15 - Minute Alarm Enable"]
    #[inline(always)]
    pub fn minen(&mut self) -> _MINENW {
        _MINENW { w: self }
    }
    #[doc = "Bits 16:21 - Hour Alarm"]
    #[inline(always)]
    pub fn hour(&mut self) -> _HOURW {
        _HOURW { w: self }
    }
    #[doc = "Bit 22 - AM/PM Indicator"]
    #[inline(always)]
    pub fn ampm(&mut self) -> _AMPMW {
        _AMPMW { w: self }
    }
    #[doc = "Bit 23 - Hour Alarm Enable"]
    #[inline(always)]
    pub fn houren(&mut self) -> _HOURENW {
        _HOURENW { w: self }
    }
}