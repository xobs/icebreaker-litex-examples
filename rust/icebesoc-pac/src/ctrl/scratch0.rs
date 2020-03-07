#[doc = "Reader of register SCRATCH0"]
pub type R = crate::R<u8, super::SCRATCH0>;
#[doc = "Writer for register SCRATCH0"]
pub type W = crate::W<u8, super::SCRATCH0>;
#[doc = "Register SCRATCH0 `reset()`'s with value 0x78"]
impl crate::ResetValue for super::SCRATCH0 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x78
    }
}
#[doc = "Reader of field `scratch`"]
pub type SCRATCH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `scratch`"]
pub struct SCRATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRATCH_W<'a> {
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
    pub fn scratch(&self) -> SCRATCH_R {
        SCRATCH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn scratch(&mut self) -> SCRATCH_W {
        SCRATCH_W { w: self }
    }
}