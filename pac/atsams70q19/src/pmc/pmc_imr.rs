#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PMC_IMR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type MOSCXTS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type LOCKA_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type MCKRDY_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type LOCKU_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PCKRDY0_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PCKRDY1_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PCKRDY2_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PCKRDY3_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PCKRDY4_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PCKRDY5_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PCKRDY6_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type MOSCSELS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type MOSCRCS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type CFDEV_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type XT32KERR_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Main Crystal Oscillator Status Interrupt Mask"]
    #[inline(always)]
    pub fn moscxts(&self) -> MOSCXTS_R {
        MOSCXTS_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - PLLA Lock Interrupt Mask"]
    #[inline(always)]
    pub fn locka(&self) -> LOCKA_R {
        LOCKA_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Master Clock Ready Interrupt Mask"]
    #[inline(always)]
    pub fn mckrdy(&self) -> MCKRDY_R {
        MCKRDY_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - UTMI PLL Lock Interrupt Mask"]
    #[inline(always)]
    pub fn locku(&self) -> LOCKU_R {
        LOCKU_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Programmable Clock Ready 0 Interrupt Mask"]
    #[inline(always)]
    pub fn pckrdy0(&self) -> PCKRDY0_R {
        PCKRDY0_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Programmable Clock Ready 1 Interrupt Mask"]
    #[inline(always)]
    pub fn pckrdy1(&self) -> PCKRDY1_R {
        PCKRDY1_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Programmable Clock Ready 2 Interrupt Mask"]
    #[inline(always)]
    pub fn pckrdy2(&self) -> PCKRDY2_R {
        PCKRDY2_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Programmable Clock Ready 3 Interrupt Mask"]
    #[inline(always)]
    pub fn pckrdy3(&self) -> PCKRDY3_R {
        PCKRDY3_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Programmable Clock Ready 4 Interrupt Mask"]
    #[inline(always)]
    pub fn pckrdy4(&self) -> PCKRDY4_R {
        PCKRDY4_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Programmable Clock Ready 5 Interrupt Mask"]
    #[inline(always)]
    pub fn pckrdy5(&self) -> PCKRDY5_R {
        PCKRDY5_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Programmable Clock Ready 6 Interrupt Mask"]
    #[inline(always)]
    pub fn pckrdy6(&self) -> PCKRDY6_R {
        PCKRDY6_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Main Clock Source Oscillator Selection Status Interrupt Mask"]
    #[inline(always)]
    pub fn moscsels(&self) -> MOSCSELS_R {
        MOSCSELS_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Main RC Status Interrupt Mask"]
    #[inline(always)]
    pub fn moscrcs(&self) -> MOSCRCS_R {
        MOSCRCS_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Clock Failure Detector Event Interrupt Mask"]
    #[inline(always)]
    pub fn cfdev(&self) -> CFDEV_R {
        CFDEV_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 32.768 kHz Crystal Oscillator Error Interrupt Mask"]
    #[inline(always)]
    pub fn xt32kerr(&self) -> XT32KERR_R {
        XT32KERR_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
}