use core::mem::{align_of, size_of};
use memoffset::offset_of;

pub type Word = usize;

#[repr(C)]
pub struct Region {
    pub start: Word,
    pub end: Word,
}

#[repr(C)]
pub struct PRegion {
    pub start: Word,
    pub end: Word,
}

#[repr(C)]
pub struct VRegion {
    pub start: Word,
    pub end: Word,
}

#[repr(C)]
pub struct KernelFrame {
    pub paddr: Word,
    pub pptr: Word,
    pub user_available: i32,
}

mod generated {
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    #![allow(clippy::all)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

const _: [(); size_of::<Region>()] = [(); size_of::<generated::region_t>()];
const _: [(); size_of::<PRegion>()] = [(); size_of::<generated::p_region_t>()];
const _: [(); size_of::<VRegion>()] = [(); size_of::<generated::v_region_t>()];
const _: [(); size_of::<KernelFrame>()] = [(); size_of::<generated::kernel_frame_t>()];

const _: [(); align_of::<Region>()] = [(); align_of::<generated::region_t>()];
const _: [(); align_of::<PRegion>()] = [(); align_of::<generated::p_region_t>()];
const _: [(); align_of::<VRegion>()] = [(); align_of::<generated::v_region_t>()];
const _: [(); align_of::<KernelFrame>()] = [(); align_of::<generated::kernel_frame_t>()];

const _: [(); offset_of!(Region, start)] = [(); offset_of!(generated::region_t, start)];
const _: [(); offset_of!(Region, end)] = [(); offset_of!(generated::region_t, end)];

const _: [(); offset_of!(PRegion, start)] = [(); offset_of!(generated::p_region_t, start)];
const _: [(); offset_of!(PRegion, end)] = [(); offset_of!(generated::p_region_t, end)];

const _: [(); offset_of!(VRegion, start)] = [(); offset_of!(generated::v_region_t, start)];
const _: [(); offset_of!(VRegion, end)] = [(); offset_of!(generated::v_region_t, end)];

const _: [(); offset_of!(KernelFrame, paddr)] = [(); offset_of!(generated::kernel_frame_t, paddr)];
const _: [(); offset_of!(KernelFrame, pptr)] = [(); offset_of!(generated::kernel_frame_t, pptr)];
const _: [(); offset_of!(KernelFrame, user_available)] = [(); offset_of!(generated::kernel_frame_t, userAvailable)];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_layout_checks_compile() {
        assert_eq!(size_of::<Region>(), size_of::<generated::region_t>());
        assert_eq!(size_of::<KernelFrame>(), size_of::<generated::kernel_frame_t>());
    }
}
