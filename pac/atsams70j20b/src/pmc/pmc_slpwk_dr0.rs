#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PMC_SLPWK_DR0 {
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
#[doc = r"Proxy"]
pub struct _PID7W<'a> {
    w: &'a mut W,
}
impl<'a> _PID7W<'a> {
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
#[doc = r"Proxy"]
pub struct _PID8W<'a> {
    w: &'a mut W,
}
impl<'a> _PID8W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _PID10W<'a> {
    w: &'a mut W,
}
impl<'a> _PID10W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _PID11W<'a> {
    w: &'a mut W,
}
impl<'a> _PID11W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _PID13W<'a> {
    w: &'a mut W,
}
impl<'a> _PID13W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _PID14W<'a> {
    w: &'a mut W,
}
impl<'a> _PID14W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _PID16W<'a> {
    w: &'a mut W,
}
impl<'a> _PID16W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _PID19W<'a> {
    w: &'a mut W,
}
impl<'a> _PID19W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _PID20W<'a> {
    w: &'a mut W,
}
impl<'a> _PID20W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _PID22W<'a> {
    w: &'a mut W,
}
impl<'a> _PID22W<'a> {
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
#[doc = r"Proxy"]
pub struct _PID23W<'a> {
    w: &'a mut W,
}
impl<'a> _PID23W<'a> {
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
#[doc = r"Proxy"]
pub struct _PID24W<'a> {
    w: &'a mut W,
}
impl<'a> _PID24W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _PID25W<'a> {
    w: &'a mut W,
}
impl<'a> _PID25W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _PID26W<'a> {
    w: &'a mut W,
}
impl<'a> _PID26W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _PID27W<'a> {
    w: &'a mut W,
}
impl<'a> _PID27W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _PID28W<'a> {
    w: &'a mut W,
}
impl<'a> _PID28W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _PID29W<'a> {
    w: &'a mut W,
}
impl<'a> _PID29W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _PID30W<'a> {
    w: &'a mut W,
}
impl<'a> _PID30W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _PID31W<'a> {
    w: &'a mut W,
}
impl<'a> _PID31W<'a> {
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
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 7 - Peripheral 7 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid7(&mut self) -> _PID7W {
        _PID7W { w: self }
    }
    #[doc = "Bit 8 - Peripheral 8 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid8(&mut self) -> _PID8W {
        _PID8W { w: self }
    }
    #[doc = "Bit 10 - Peripheral 10 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid10(&mut self) -> _PID10W {
        _PID10W { w: self }
    }
    #[doc = "Bit 11 - Peripheral 11 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid11(&mut self) -> _PID11W {
        _PID11W { w: self }
    }
    #[doc = "Bit 13 - Peripheral 13 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid13(&mut self) -> _PID13W {
        _PID13W { w: self }
    }
    #[doc = "Bit 14 - Peripheral 14 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid14(&mut self) -> _PID14W {
        _PID14W { w: self }
    }
    #[doc = "Bit 16 - Peripheral 16 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid16(&mut self) -> _PID16W {
        _PID16W { w: self }
    }
    #[doc = "Bit 19 - Peripheral 19 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid19(&mut self) -> _PID19W {
        _PID19W { w: self }
    }
    #[doc = "Bit 20 - Peripheral 20 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid20(&mut self) -> _PID20W {
        _PID20W { w: self }
    }
    #[doc = "Bit 22 - Peripheral 22 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid22(&mut self) -> _PID22W {
        _PID22W { w: self }
    }
    #[doc = "Bit 23 - Peripheral 23 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid23(&mut self) -> _PID23W {
        _PID23W { w: self }
    }
    #[doc = "Bit 24 - Peripheral 24 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid24(&mut self) -> _PID24W {
        _PID24W { w: self }
    }
    #[doc = "Bit 25 - Peripheral 25 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid25(&mut self) -> _PID25W {
        _PID25W { w: self }
    }
    #[doc = "Bit 26 - Peripheral 26 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid26(&mut self) -> _PID26W {
        _PID26W { w: self }
    }
    #[doc = "Bit 27 - Peripheral 27 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid27(&mut self) -> _PID27W {
        _PID27W { w: self }
    }
    #[doc = "Bit 28 - Peripheral 28 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid28(&mut self) -> _PID28W {
        _PID28W { w: self }
    }
    #[doc = "Bit 29 - Peripheral 29 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid29(&mut self) -> _PID29W {
        _PID29W { w: self }
    }
    #[doc = "Bit 30 - Peripheral 30 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid30(&mut self) -> _PID30W {
        _PID30W { w: self }
    }
    #[doc = "Bit 31 - Peripheral 31 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid31(&mut self) -> _PID31W {
        _PID31W { w: self }
    }
}