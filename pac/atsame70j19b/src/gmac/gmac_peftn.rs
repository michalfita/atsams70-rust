#[doc = "Reader of register GMAC_PEFTN"]
pub type R = crate::R<u32, super::GMAC_PEFTN>;
#[doc = "Reader of field `RUD`"]
pub type RUD_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:29 - Register Update"]
    #[inline(always)]
    pub fn rud(&self) -> RUD_R {
        RUD_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
