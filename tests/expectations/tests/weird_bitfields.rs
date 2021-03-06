/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum nsStyleSVGOpacitySource {
    eStyleSVGOpacitySource_Normal = 0,
    eStyleSVGOpacitySource_ContextFillOpacity = 1,
    eStyleSVGOpacitySource_ContextStrokeOpacity = 2,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Weird {
    pub mStrokeDasharrayLength: ::std::os::raw::c_uint,
    pub _bitfield_1: [u16; 2usize],
    pub mClipRule: ::std::os::raw::c_uchar,
    pub mColorInterpolation: ::std::os::raw::c_uchar,
    pub mColorInterpolationFilters: ::std::os::raw::c_uchar,
    pub mFillRule: ::std::os::raw::c_uchar,
    pub mImageRendering: ::std::os::raw::c_uchar,
    pub mPaintOrder: ::std::os::raw::c_uchar,
    pub mShapeRendering: ::std::os::raw::c_uchar,
    pub mStrokeLinecap: ::std::os::raw::c_uchar,
    pub mStrokeLinejoin: ::std::os::raw::c_uchar,
    pub mTextAnchor: ::std::os::raw::c_uchar,
    pub mTextRendering: ::std::os::raw::c_uchar,
    pub _bitfield_2: u8,
    pub _bitfield_3: u8,
}
#[test]
fn bindgen_test_layout_Weird() {
    assert_eq!(::std::mem::size_of::<Weird>() , 24usize , concat ! (
               "Size of: " , stringify ! ( Weird ) ));
    assert_eq! (::std::mem::align_of::<Weird>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( Weird ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const Weird ) ) . mStrokeDasharrayLength as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( Weird ) , "::" ,
                stringify ! ( mStrokeDasharrayLength ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const Weird ) ) . mClipRule as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( Weird ) , "::" ,
                stringify ! ( mClipRule ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const Weird ) ) . mColorInterpolation as *
                const _ as usize } , 9usize , concat ! (
                "Alignment of field: " , stringify ! ( Weird ) , "::" ,
                stringify ! ( mColorInterpolation ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const Weird ) ) . mColorInterpolationFilters as
                * const _ as usize } , 10usize , concat ! (
                "Alignment of field: " , stringify ! ( Weird ) , "::" ,
                stringify ! ( mColorInterpolationFilters ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const Weird ) ) . mFillRule as * const _ as
                usize } , 11usize , concat ! (
                "Alignment of field: " , stringify ! ( Weird ) , "::" ,
                stringify ! ( mFillRule ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const Weird ) ) . mImageRendering as * const _
                as usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( Weird ) , "::" ,
                stringify ! ( mImageRendering ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const Weird ) ) . mPaintOrder as * const _ as
                usize } , 13usize , concat ! (
                "Alignment of field: " , stringify ! ( Weird ) , "::" ,
                stringify ! ( mPaintOrder ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const Weird ) ) . mShapeRendering as * const _
                as usize } , 14usize , concat ! (
                "Alignment of field: " , stringify ! ( Weird ) , "::" ,
                stringify ! ( mShapeRendering ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const Weird ) ) . mStrokeLinecap as * const _
                as usize } , 15usize , concat ! (
                "Alignment of field: " , stringify ! ( Weird ) , "::" ,
                stringify ! ( mStrokeLinecap ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const Weird ) ) . mStrokeLinejoin as * const _
                as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( Weird ) , "::" ,
                stringify ! ( mStrokeLinejoin ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const Weird ) ) . mTextAnchor as * const _ as
                usize } , 17usize , concat ! (
                "Alignment of field: " , stringify ! ( Weird ) , "::" ,
                stringify ! ( mTextAnchor ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const Weird ) ) . mTextRendering as * const _
                as usize } , 18usize , concat ! (
                "Alignment of field: " , stringify ! ( Weird ) , "::" ,
                stringify ! ( mTextRendering ) ));
}
impl Clone for Weird {
    fn clone(&self) -> Self { *self }
}
impl Default for Weird {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
