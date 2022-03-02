// use crate::bindgen;

// #[repr(C)]
// #[derive(Debug, Clone)]
// pub struct Image {
//     native: bindgen::filament_ibl_Image,
// }

// // impl Image {
// //     #[inline]
// //     pub fn native(&self) -> *const bindgen::filament_ibl_Image {
// //         self.native.as_ptr()
// //     }

// //     #[inline]
// //     pub fn native_mut(&mut self) -> *mut bindgen::filament_ibl_Image {
// //         self.native.as_ptr()
// //     }
// // }

// impl Image {
//     #[inline]
//     pub unsafe fn new_size(width: usize, height: usize) -> Self {
//         Self {
//             native: bindgen::filament_ibl_Image::new1(width, height, 0),
//         }
//     }

//     #[inline]
//     pub unsafe fn new_size_stride(width: usize, height: usize, stride: usize) -> Self {
//         Self {
//             native: bindgen::filament_ibl_Image::new1(width, height, stride),
//         }
//     }

//     #[inline]
//     pub unsafe fn reset(&mut self) {
//         bindgen::filament_ibl_Image_reset(&mut self.native)
//     }

//     #[inline]
//     pub unsafe fn set(&mut self, image: &Image) {
//         bindgen::filament_ibl_Image_set(&mut self.native, &image.native)
//     }

//     #[inline]
//     pub unsafe fn subset(&mut self, image: &Image, x: usize, y: usize, w: usize, h: usize) {
//         bindgen::filament_ibl_Image_subset(&mut self.native, &image.native, x, y, w, h)
//     }

//     #[inline]
//     pub unsafe fn get_pixel_ref(&self, x: usize, y: usize) -> *mut ::core::ffi::c_void {
//         bindgen::filament_ibl_Image_getPixelRef(&self.native, x, y)
//     }
// }
