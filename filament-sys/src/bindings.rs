#![allow(clippy::all)]
#![allow(unknown_lints)]
#![allow(deref_nullptr)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!("fix.rs");

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct std_input_iterator_tag {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std_input_iterator_tag() {
    assert_eq!(
        ::std::mem::size_of::<std_input_iterator_tag>(),
        1usize,
        concat!("Size of: ", stringify!(std_input_iterator_tag))
    );
    assert_eq!(
        ::std::mem::align_of::<std_input_iterator_tag>(),
        1usize,
        concat!("Alignment of ", stringify!(std_input_iterator_tag))
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct std_forward_iterator_tag {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std_forward_iterator_tag() {
    assert_eq!(
        ::std::mem::size_of::<std_forward_iterator_tag>(),
        1usize,
        concat!("Size of: ", stringify!(std_forward_iterator_tag))
    );
    assert_eq!(
        ::std::mem::align_of::<std_forward_iterator_tag>(),
        1usize,
        concat!("Alignment of ", stringify!(std_forward_iterator_tag))
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct std_iterator {
    pub _address: u8,
}
pub type std_iterator_iterator_category<_Category> = _Category;
pub type std_iterator_value_type<_Tp> = _Tp;
pub type std_iterator_difference_type<_Distance> = _Distance;
pub type std_iterator_pointer<_Pointer> = _Pointer;
pub type std_iterator_reference<_Reference> = _Reference;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_math_details_TVecAddOperators {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_math_details_TVecProductOperators {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_math_details_TVecUnaryOperators {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_math_details_TVecComparisonOperators {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_math_details_TVecFunctions {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_math_details_TQuatProductOperators {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_math_details_TQuatFunctions {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_math_details_TMatProductOperators {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_math_details_TMatSquareFunctions {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_math_details_TMatHelpers {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_math_details_TMatTransform {
    pub _address: u8,
}
#[doc = " An axis aligned 3D box represented by its center and half-extent."]
#[repr(C)]
pub struct filament_Box {
    #[doc = " Center of the 3D box"]
    pub center: filament_math_float3,
    #[doc = " Half extent from the center on all 3 axis"]
    pub halfExtent: filament_math_float3,
}
#[test]
fn bindgen_test_layout_filament_Box() {
    assert_eq!(
        ::std::mem::size_of::<filament_Box>(),
        24usize,
        concat!("Size of: ", stringify!(filament_Box))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_Box>(),
        4usize,
        concat!("Alignment of ", stringify!(filament_Box))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<filament_Box>())).center as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_Box),
            "::",
            stringify!(center)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<filament_Box>())).halfExtent as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_Box),
            "::",
            stringify!(halfExtent)
        )
    );
}
impl Default for filament_Box {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " \\privatesection"]
#[doc = " FilamentAPI is used to define an API in filament."]
#[doc = " It ensures the class defining the API can't be created, destroyed"]
#[doc = " or copied by the caller."]
#[repr(C)]
#[derive(Debug, Default)]
pub struct filament_FilamentAPI {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_filament_FilamentAPI() {
    assert_eq!(
        ::std::mem::size_of::<filament_FilamentAPI>(),
        1usize,
        concat!("Size of: ", stringify!(filament_FilamentAPI))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_FilamentAPI>(),
        1usize,
        concat!("Alignment of ", stringify!(filament_FilamentAPI))
    );
}
#[doc = " \\privatesection"]
#[doc = " BuilderBase is used to hide the implementation details of builders and ensure a higher"]
#[doc = " level of backward binary compatibility."]
#[doc = " The actual implementation is in src/FilamentAPI-impl.h\""]
#[repr(C)]
#[derive(Debug)]
pub struct filament_BuilderBase<T> {
    pub mImpl: *mut T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for filament_BuilderBase<T> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " A PresentCallable is a callable object that, when called, schedules a frame for presentation on"]
#[doc = " a SwapChain."]
#[doc = ""]
#[doc = " Typically, Filament's backend is responsible scheduling a frame's presentation. However, there"]
#[doc = " are certain cases where the application might want to control when a frame is scheduled for"]
#[doc = " presentation."]
#[doc = ""]
#[doc = " For example, on iOS, UIKit elements can be synchronized to 3D content by scheduling a present"]
#[doc = " within a CATransation:"]
#[doc = ""]
#[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
#[doc = " void myFrameScheduledCallback(PresentCallable presentCallable, void* user) {"]
#[doc = "     [CATransaction begin];"]
#[doc = "     // Update other UI elements..."]
#[doc = "     presentCallable();"]
#[doc = "     [CATransaction commit];"]
#[doc = " }"]
#[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
#[doc = ""]
#[doc = " To obtain a PresentCallable, set a SwapChain::FrameScheduledCallback on a SwapChain with the"]
#[doc = " SwapChain::setFrameScheduledCallback method. The callback is called with a PresentCallable object"]
#[doc = " and optional user data:"]
#[doc = ""]
#[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
#[doc = " swapChain->setFrameScheduledCallback(myFrameScheduledCallback, nullptr);"]
#[doc = " if (renderer->beginFrame(swapChain)) {"]
#[doc = "     renderer->render(view);"]
#[doc = "     renderer->endFrame();"]
#[doc = " }"]
#[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
#[doc = ""]
#[doc = " @remark Only Filament's Metal backend supports PresentCallables and frame callbacks. Other"]
#[doc = " backends ignore the callback (which will never be called) and proceed normally."]
#[doc = ""]
#[doc = " @remark The SwapChain::FrameScheduledCallback is called on an arbitrary thread."]
#[doc = ""]
#[doc = " Applications *must* call each PresentCallable they receive. Each PresentCallable represents a"]
#[doc = " frame that is waiting to be presented. If an application fails to call a PresentCallable, a"]
#[doc = " memory leak could occur. To \"cancel\" the presentation of a frame, pass false to the"]
#[doc = " PresentCallable, which will cancel the presentation of the frame and release associated memory."]
#[doc = ""]
#[doc = " @see Renderer, SwapChain::setFrameScheduledCallback"]
#[repr(C)]
#[derive(Debug)]
pub struct filament_backend_PresentCallable {
    pub mPresentFn: filament_backend_PresentCallable_PresentFn,
    pub mUser: *mut ::std::os::raw::c_void,
}
pub type filament_backend_PresentCallable_PresentFn = ::std::option::Option<
    unsafe extern "C" fn(presentFrame: bool, user: *mut ::std::os::raw::c_void),
>;
#[test]
fn bindgen_test_layout_filament_backend_PresentCallable() {
    assert_eq!(
        ::std::mem::size_of::<filament_backend_PresentCallable>(),
        16usize,
        concat!("Size of: ", stringify!(filament_backend_PresentCallable))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_backend_PresentCallable>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(filament_backend_PresentCallable)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_backend_PresentCallable>())).mPresentFn as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_backend_PresentCallable),
            "::",
            stringify!(mPresentFn)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_backend_PresentCallable>())).mUser as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_backend_PresentCallable),
            "::",
            stringify!(mUser)
        )
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament7backend15PresentCallableC1EPFvbPvES2_"]
    pub fn filament_backend_PresentCallable_PresentCallable(
        this: *mut filament_backend_PresentCallable,
        fn_: filament_backend_PresentCallable_PresentFn,
        user: *mut ::std::os::raw::c_void,
    );
}
impl Default for filament_backend_PresentCallable {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl filament_backend_PresentCallable {
    #[inline]
    pub unsafe fn new(
        fn_: filament_backend_PresentCallable_PresentFn,
        user: *mut ::std::os::raw::c_void,
    ) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_backend_PresentCallable_PresentCallable(__bindgen_tmp.as_mut_ptr(), fn_, user);
        __bindgen_tmp.assume_init()
    }
}
#[doc = "!< Automatically selects an appropriate driver for the platform."]
pub const filament_backend_Backend_DEFAULT: filament_backend_Backend = 0;
#[doc = "!< Selects the OpenGL/ES driver (default on Android)"]
pub const filament_backend_Backend_OPENGL: filament_backend_Backend = 1;
#[doc = "!< Selects the Vulkan driver if the platform supports it (default on Linux/Windows)"]
pub const filament_backend_Backend_VULKAN: filament_backend_Backend = 2;
#[doc = "!< Selects the Metal driver if the platform supports it (default on MacOS/iOS)."]
pub const filament_backend_Backend_METAL: filament_backend_Backend = 3;
#[doc = "!< Selects the no-op driver for testing purposes."]
pub const filament_backend_Backend_NOOP: filament_backend_Backend = 4;
#[doc = " Selects which driver a particular Engine should use."]
pub type filament_backend_Backend = u8;
#[doc = " Defines a viewport, which is the origin and extent of the clip-space."]
#[doc = " All drawing is clipped to the viewport."]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_backend_Viewport {
    #[doc = "!< left coordinate in window space."]
    pub left: i32,
    #[doc = "!< bottom coordinate in window space."]
    pub bottom: i32,
    #[doc = "!< width in pixels"]
    pub width: u32,
    #[doc = "!< height in pixels"]
    pub height: u32,
}
#[test]
fn bindgen_test_layout_filament_backend_Viewport() {
    assert_eq!(
        ::std::mem::size_of::<filament_backend_Viewport>(),
        16usize,
        concat!("Size of: ", stringify!(filament_backend_Viewport))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_backend_Viewport>(),
        4usize,
        concat!("Alignment of ", stringify!(filament_backend_Viewport))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<filament_backend_Viewport>())).left as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_backend_Viewport),
            "::",
            stringify!(left)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_backend_Viewport>())).bottom as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_backend_Viewport),
            "::",
            stringify!(bottom)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<filament_backend_Viewport>())).width as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_backend_Viewport),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_backend_Viewport>())).height as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_backend_Viewport),
            "::",
            stringify!(height)
        )
    );
}
#[doc = "!< An error occured. The Fence condition is not satisfied."]
pub const filament_backend_FenceStatus_ERROR: filament_backend_FenceStatus = -1;
#[doc = "!< The Fence condition is satisfied."]
pub const filament_backend_FenceStatus_CONDITION_SATISFIED: filament_backend_FenceStatus = 0;
#[doc = "!< wait()'s timeout expired. The Fence condition is not satisfied."]
pub const filament_backend_FenceStatus_TIMEOUT_EXPIRED: filament_backend_FenceStatus = 1;
#[doc = " Error codes for Fence::wait()"]
#[doc = " @see Fence, Fence::wait()"]
pub type filament_backend_FenceStatus = i8;
#[doc = "! For testing"]
pub const filament_backend_ShaderModel_UNKNOWN: filament_backend_ShaderModel = 0;
#[doc = "!< Mobile level functionality"]
pub const filament_backend_ShaderModel_GL_ES_30: filament_backend_ShaderModel = 1;
#[doc = "!< Desktop level functionality"]
pub const filament_backend_ShaderModel_GL_CORE_41: filament_backend_ShaderModel = 2;
#[doc = " Shader model."]
#[doc = ""]
#[doc = " These enumerants are used across all backends and refer to a level of functionality, rather"]
#[doc = " than to an OpenGL specific shader model."]
pub type filament_backend_ShaderModel = u8;
#[doc = "!< points"]
pub const filament_backend_PrimitiveType_POINTS: filament_backend_PrimitiveType = 0;
#[doc = "!< lines"]
pub const filament_backend_PrimitiveType_LINES: filament_backend_PrimitiveType = 1;
#[doc = "!< line strip"]
pub const filament_backend_PrimitiveType_LINE_STRIP: filament_backend_PrimitiveType = 3;
#[doc = "!< triangles"]
pub const filament_backend_PrimitiveType_TRIANGLES: filament_backend_PrimitiveType = 4;
#[doc = "!< triangle strip"]
pub const filament_backend_PrimitiveType_TRIANGLE_STRIP: filament_backend_PrimitiveType = 5;
pub const filament_backend_PrimitiveType_NONE: filament_backend_PrimitiveType = 255;
#[doc = " Primitive types"]
pub type filament_backend_PrimitiveType = u8;
pub const filament_backend_UniformType_BOOL: filament_backend_UniformType = 0;
pub const filament_backend_UniformType_BOOL2: filament_backend_UniformType = 1;
pub const filament_backend_UniformType_BOOL3: filament_backend_UniformType = 2;
pub const filament_backend_UniformType_BOOL4: filament_backend_UniformType = 3;
pub const filament_backend_UniformType_FLOAT: filament_backend_UniformType = 4;
pub const filament_backend_UniformType_FLOAT2: filament_backend_UniformType = 5;
pub const filament_backend_UniformType_FLOAT3: filament_backend_UniformType = 6;
pub const filament_backend_UniformType_FLOAT4: filament_backend_UniformType = 7;
pub const filament_backend_UniformType_INT: filament_backend_UniformType = 8;
pub const filament_backend_UniformType_INT2: filament_backend_UniformType = 9;
pub const filament_backend_UniformType_INT3: filament_backend_UniformType = 10;
pub const filament_backend_UniformType_INT4: filament_backend_UniformType = 11;
pub const filament_backend_UniformType_UINT: filament_backend_UniformType = 12;
pub const filament_backend_UniformType_UINT2: filament_backend_UniformType = 13;
pub const filament_backend_UniformType_UINT3: filament_backend_UniformType = 14;
pub const filament_backend_UniformType_UINT4: filament_backend_UniformType = 15;
#[doc = "!< a 3x3 float matrix"]
pub const filament_backend_UniformType_MAT3: filament_backend_UniformType = 16;
#[doc = "!< a 4x4 float matrix"]
pub const filament_backend_UniformType_MAT4: filament_backend_UniformType = 17;
pub const filament_backend_UniformType_STRUCT: filament_backend_UniformType = 18;
#[doc = " Supported uniform types"]
pub type filament_backend_UniformType = u8;
pub const filament_backend_Precision_LOW: filament_backend_Precision = 0;
pub const filament_backend_Precision_MEDIUM: filament_backend_Precision = 1;
pub const filament_backend_Precision_HIGH: filament_backend_Precision = 2;
pub const filament_backend_Precision_DEFAULT: filament_backend_Precision = 3;
pub type filament_backend_Precision = u8;
#[doc = "!< 2D texture"]
pub const filament_backend_SamplerType_SAMPLER_2D: filament_backend_SamplerType = 0;
#[doc = "!< 2D array texture"]
pub const filament_backend_SamplerType_SAMPLER_2D_ARRAY: filament_backend_SamplerType = 1;
#[doc = "!< Cube map texture"]
pub const filament_backend_SamplerType_SAMPLER_CUBEMAP: filament_backend_SamplerType = 2;
#[doc = "!< External texture"]
pub const filament_backend_SamplerType_SAMPLER_EXTERNAL: filament_backend_SamplerType = 3;
#[doc = "!< 3D texture"]
pub const filament_backend_SamplerType_SAMPLER_3D: filament_backend_SamplerType = 4;
#[doc = "! Texture sampler type"]
pub type filament_backend_SamplerType = u8;
pub const filament_backend_SubpassType_SUBPASS_INPUT: filament_backend_SubpassType = 0;
#[doc = "! Subpass type"]
pub type filament_backend_SubpassType = u8;
#[doc = "!< signed integer sampler"]
pub const filament_backend_SamplerFormat_INT: filament_backend_SamplerFormat = 0;
#[doc = "!< unsigned integer sampler"]
pub const filament_backend_SamplerFormat_UINT: filament_backend_SamplerFormat = 1;
#[doc = "!< float sampler"]
pub const filament_backend_SamplerFormat_FLOAT: filament_backend_SamplerFormat = 2;
#[doc = "!< shadow sampler (PCF)"]
pub const filament_backend_SamplerFormat_SHADOW: filament_backend_SamplerFormat = 3;
#[doc = "! Texture sampler format"]
pub type filament_backend_SamplerFormat = u8;
pub const filament_backend_ElementType_BYTE: filament_backend_ElementType = 0;
pub const filament_backend_ElementType_BYTE2: filament_backend_ElementType = 1;
pub const filament_backend_ElementType_BYTE3: filament_backend_ElementType = 2;
pub const filament_backend_ElementType_BYTE4: filament_backend_ElementType = 3;
pub const filament_backend_ElementType_UBYTE: filament_backend_ElementType = 4;
pub const filament_backend_ElementType_UBYTE2: filament_backend_ElementType = 5;
pub const filament_backend_ElementType_UBYTE3: filament_backend_ElementType = 6;
pub const filament_backend_ElementType_UBYTE4: filament_backend_ElementType = 7;
pub const filament_backend_ElementType_SHORT: filament_backend_ElementType = 8;
pub const filament_backend_ElementType_SHORT2: filament_backend_ElementType = 9;
pub const filament_backend_ElementType_SHORT3: filament_backend_ElementType = 10;
pub const filament_backend_ElementType_SHORT4: filament_backend_ElementType = 11;
pub const filament_backend_ElementType_USHORT: filament_backend_ElementType = 12;
pub const filament_backend_ElementType_USHORT2: filament_backend_ElementType = 13;
pub const filament_backend_ElementType_USHORT3: filament_backend_ElementType = 14;
pub const filament_backend_ElementType_USHORT4: filament_backend_ElementType = 15;
pub const filament_backend_ElementType_INT: filament_backend_ElementType = 16;
pub const filament_backend_ElementType_UINT: filament_backend_ElementType = 17;
pub const filament_backend_ElementType_FLOAT: filament_backend_ElementType = 18;
pub const filament_backend_ElementType_FLOAT2: filament_backend_ElementType = 19;
pub const filament_backend_ElementType_FLOAT3: filament_backend_ElementType = 20;
pub const filament_backend_ElementType_FLOAT4: filament_backend_ElementType = 21;
pub const filament_backend_ElementType_HALF: filament_backend_ElementType = 22;
pub const filament_backend_ElementType_HALF2: filament_backend_ElementType = 23;
pub const filament_backend_ElementType_HALF3: filament_backend_ElementType = 24;
pub const filament_backend_ElementType_HALF4: filament_backend_ElementType = 25;
#[doc = " Supported element types"]
pub type filament_backend_ElementType = u8;
pub const filament_backend_BufferObjectBinding_VERTEX: filament_backend_BufferObjectBinding = 0;
pub const filament_backend_BufferObjectBinding_UNIFORM: filament_backend_BufferObjectBinding = 1;
#[doc = "! Buffer object binding type"]
pub type filament_backend_BufferObjectBinding = u8;
#[doc = "!< No culling, front and back faces are visible"]
pub const filament_backend_CullingMode_NONE: filament_backend_CullingMode = 0;
#[doc = "!< Front face culling, only back faces are visible"]
pub const filament_backend_CullingMode_FRONT: filament_backend_CullingMode = 1;
#[doc = "!< Back face culling, only front faces are visible"]
pub const filament_backend_CullingMode_BACK: filament_backend_CullingMode = 2;
#[doc = "!< Front and Back, geometry is not visible"]
pub const filament_backend_CullingMode_FRONT_AND_BACK: filament_backend_CullingMode = 3;
#[doc = "! Face culling Mode"]
pub type filament_backend_CullingMode = u8;
#[doc = "!< One Red channel, float"]
pub const filament_backend_PixelDataFormat_R: filament_backend_PixelDataFormat = 0;
#[doc = "!< One Red channel, integer"]
pub const filament_backend_PixelDataFormat_R_INTEGER: filament_backend_PixelDataFormat = 1;
#[doc = "!< Two Red and Green channels, float"]
pub const filament_backend_PixelDataFormat_RG: filament_backend_PixelDataFormat = 2;
#[doc = "!< Two Red and Green channels, integer"]
pub const filament_backend_PixelDataFormat_RG_INTEGER: filament_backend_PixelDataFormat = 3;
#[doc = "!< Three Red, Green and Blue channels, float"]
pub const filament_backend_PixelDataFormat_RGB: filament_backend_PixelDataFormat = 4;
#[doc = "!< Three Red, Green and Blue channels, integer"]
pub const filament_backend_PixelDataFormat_RGB_INTEGER: filament_backend_PixelDataFormat = 5;
#[doc = "!< Four Red, Green, Blue and Alpha channels, float"]
pub const filament_backend_PixelDataFormat_RGBA: filament_backend_PixelDataFormat = 6;
#[doc = "!< Four Red, Green, Blue and Alpha channels, integer"]
pub const filament_backend_PixelDataFormat_RGBA_INTEGER: filament_backend_PixelDataFormat = 7;
pub const filament_backend_PixelDataFormat_UNUSED: filament_backend_PixelDataFormat = 8;
#[doc = "!< Depth, 16-bit or 24-bits usually"]
pub const filament_backend_PixelDataFormat_DEPTH_COMPONENT: filament_backend_PixelDataFormat = 9;
#[doc = "!< Two Depth (24-bits) + Stencil (8-bits) channels"]
pub const filament_backend_PixelDataFormat_DEPTH_STENCIL: filament_backend_PixelDataFormat = 10;
pub const filament_backend_PixelDataFormat_ALPHA: filament_backend_PixelDataFormat = 11;
#[doc = "! Pixel Data Format"]
pub type filament_backend_PixelDataFormat = u8;
#[doc = "!< unsigned byte"]
pub const filament_backend_PixelDataType_UBYTE: filament_backend_PixelDataType = 0;
#[doc = "!< signed byte"]
pub const filament_backend_PixelDataType_BYTE: filament_backend_PixelDataType = 1;
#[doc = "!< unsigned short (16-bit)"]
pub const filament_backend_PixelDataType_USHORT: filament_backend_PixelDataType = 2;
#[doc = "!< signed short (16-bit)"]
pub const filament_backend_PixelDataType_SHORT: filament_backend_PixelDataType = 3;
#[doc = "!< unsigned int (32-bit)"]
pub const filament_backend_PixelDataType_UINT: filament_backend_PixelDataType = 4;
#[doc = "!< signed int (32-bit)"]
pub const filament_backend_PixelDataType_INT: filament_backend_PixelDataType = 5;
#[doc = "!< half-float (16-bit float)"]
pub const filament_backend_PixelDataType_HALF: filament_backend_PixelDataType = 6;
#[doc = "!< float (32-bits float)"]
pub const filament_backend_PixelDataType_FLOAT: filament_backend_PixelDataType = 7;
#[doc = "!< compressed pixels, @see CompressedPixelDataType"]
pub const filament_backend_PixelDataType_COMPRESSED: filament_backend_PixelDataType = 8;
#[doc = "!< three low precision floating-point numbers"]
pub const filament_backend_PixelDataType_UINT_10F_11F_11F_REV: filament_backend_PixelDataType = 9;
#[doc = "!< unsigned int (16-bit), encodes 3 RGB channels"]
pub const filament_backend_PixelDataType_USHORT_565: filament_backend_PixelDataType = 10;
#[doc = "!< unsigned normalized 10 bits RGB, 2 bits alpha"]
pub const filament_backend_PixelDataType_UINT_2_10_10_10_REV: filament_backend_PixelDataType = 11;
#[doc = "! Pixel Data Type"]
pub type filament_backend_PixelDataType = u8;
pub const filament_backend_CompressedPixelDataType_EAC_R11:
    filament_backend_CompressedPixelDataType = 0;
pub const filament_backend_CompressedPixelDataType_EAC_R11_SIGNED:
    filament_backend_CompressedPixelDataType = 1;
pub const filament_backend_CompressedPixelDataType_EAC_RG11:
    filament_backend_CompressedPixelDataType = 2;
pub const filament_backend_CompressedPixelDataType_EAC_RG11_SIGNED:
    filament_backend_CompressedPixelDataType = 3;
pub const filament_backend_CompressedPixelDataType_ETC2_RGB8:
    filament_backend_CompressedPixelDataType = 4;
pub const filament_backend_CompressedPixelDataType_ETC2_SRGB8:
    filament_backend_CompressedPixelDataType = 5;
pub const filament_backend_CompressedPixelDataType_ETC2_RGB8_A1:
    filament_backend_CompressedPixelDataType = 6;
pub const filament_backend_CompressedPixelDataType_ETC2_SRGB8_A1:
    filament_backend_CompressedPixelDataType = 7;
pub const filament_backend_CompressedPixelDataType_ETC2_EAC_RGBA8:
    filament_backend_CompressedPixelDataType = 8;
pub const filament_backend_CompressedPixelDataType_ETC2_EAC_SRGBA8:
    filament_backend_CompressedPixelDataType = 9;
pub const filament_backend_CompressedPixelDataType_DXT1_RGB:
    filament_backend_CompressedPixelDataType = 10;
pub const filament_backend_CompressedPixelDataType_DXT1_RGBA:
    filament_backend_CompressedPixelDataType = 11;
pub const filament_backend_CompressedPixelDataType_DXT3_RGBA:
    filament_backend_CompressedPixelDataType = 12;
pub const filament_backend_CompressedPixelDataType_DXT5_RGBA:
    filament_backend_CompressedPixelDataType = 13;
pub const filament_backend_CompressedPixelDataType_DXT1_SRGB:
    filament_backend_CompressedPixelDataType = 14;
pub const filament_backend_CompressedPixelDataType_DXT1_SRGBA:
    filament_backend_CompressedPixelDataType = 15;
pub const filament_backend_CompressedPixelDataType_DXT3_SRGBA:
    filament_backend_CompressedPixelDataType = 16;
pub const filament_backend_CompressedPixelDataType_DXT5_SRGBA:
    filament_backend_CompressedPixelDataType = 17;
pub const filament_backend_CompressedPixelDataType_RGBA_ASTC_4x4:
    filament_backend_CompressedPixelDataType = 18;
pub const filament_backend_CompressedPixelDataType_RGBA_ASTC_5x4:
    filament_backend_CompressedPixelDataType = 19;
pub const filament_backend_CompressedPixelDataType_RGBA_ASTC_5x5:
    filament_backend_CompressedPixelDataType = 20;
pub const filament_backend_CompressedPixelDataType_RGBA_ASTC_6x5:
    filament_backend_CompressedPixelDataType = 21;
pub const filament_backend_CompressedPixelDataType_RGBA_ASTC_6x6:
    filament_backend_CompressedPixelDataType = 22;
pub const filament_backend_CompressedPixelDataType_RGBA_ASTC_8x5:
    filament_backend_CompressedPixelDataType = 23;
pub const filament_backend_CompressedPixelDataType_RGBA_ASTC_8x6:
    filament_backend_CompressedPixelDataType = 24;
pub const filament_backend_CompressedPixelDataType_RGBA_ASTC_8x8:
    filament_backend_CompressedPixelDataType = 25;
pub const filament_backend_CompressedPixelDataType_RGBA_ASTC_10x5:
    filament_backend_CompressedPixelDataType = 26;
pub const filament_backend_CompressedPixelDataType_RGBA_ASTC_10x6:
    filament_backend_CompressedPixelDataType = 27;
pub const filament_backend_CompressedPixelDataType_RGBA_ASTC_10x8:
    filament_backend_CompressedPixelDataType = 28;
pub const filament_backend_CompressedPixelDataType_RGBA_ASTC_10x10:
    filament_backend_CompressedPixelDataType = 29;
pub const filament_backend_CompressedPixelDataType_RGBA_ASTC_12x10:
    filament_backend_CompressedPixelDataType = 30;
pub const filament_backend_CompressedPixelDataType_RGBA_ASTC_12x12:
    filament_backend_CompressedPixelDataType = 31;
pub const filament_backend_CompressedPixelDataType_SRGB8_ALPHA8_ASTC_4x4:
    filament_backend_CompressedPixelDataType = 32;
pub const filament_backend_CompressedPixelDataType_SRGB8_ALPHA8_ASTC_5x4:
    filament_backend_CompressedPixelDataType = 33;
pub const filament_backend_CompressedPixelDataType_SRGB8_ALPHA8_ASTC_5x5:
    filament_backend_CompressedPixelDataType = 34;
pub const filament_backend_CompressedPixelDataType_SRGB8_ALPHA8_ASTC_6x5:
    filament_backend_CompressedPixelDataType = 35;
pub const filament_backend_CompressedPixelDataType_SRGB8_ALPHA8_ASTC_6x6:
    filament_backend_CompressedPixelDataType = 36;
pub const filament_backend_CompressedPixelDataType_SRGB8_ALPHA8_ASTC_8x5:
    filament_backend_CompressedPixelDataType = 37;
pub const filament_backend_CompressedPixelDataType_SRGB8_ALPHA8_ASTC_8x6:
    filament_backend_CompressedPixelDataType = 38;
pub const filament_backend_CompressedPixelDataType_SRGB8_ALPHA8_ASTC_8x8:
    filament_backend_CompressedPixelDataType = 39;
pub const filament_backend_CompressedPixelDataType_SRGB8_ALPHA8_ASTC_10x5:
    filament_backend_CompressedPixelDataType = 40;
pub const filament_backend_CompressedPixelDataType_SRGB8_ALPHA8_ASTC_10x6:
    filament_backend_CompressedPixelDataType = 41;
pub const filament_backend_CompressedPixelDataType_SRGB8_ALPHA8_ASTC_10x8:
    filament_backend_CompressedPixelDataType = 42;
pub const filament_backend_CompressedPixelDataType_SRGB8_ALPHA8_ASTC_10x10:
    filament_backend_CompressedPixelDataType = 43;
pub const filament_backend_CompressedPixelDataType_SRGB8_ALPHA8_ASTC_12x10:
    filament_backend_CompressedPixelDataType = 44;
pub const filament_backend_CompressedPixelDataType_SRGB8_ALPHA8_ASTC_12x12:
    filament_backend_CompressedPixelDataType = 45;
#[doc = "! Compressed pixel data types"]
pub type filament_backend_CompressedPixelDataType = u16;
pub const filament_backend_TextureFormat_R8: filament_backend_TextureFormat = 0;
pub const filament_backend_TextureFormat_R8_SNORM: filament_backend_TextureFormat = 1;
pub const filament_backend_TextureFormat_R8UI: filament_backend_TextureFormat = 2;
pub const filament_backend_TextureFormat_R8I: filament_backend_TextureFormat = 3;
pub const filament_backend_TextureFormat_STENCIL8: filament_backend_TextureFormat = 4;
pub const filament_backend_TextureFormat_R16F: filament_backend_TextureFormat = 5;
pub const filament_backend_TextureFormat_R16UI: filament_backend_TextureFormat = 6;
pub const filament_backend_TextureFormat_R16I: filament_backend_TextureFormat = 7;
pub const filament_backend_TextureFormat_RG8: filament_backend_TextureFormat = 8;
pub const filament_backend_TextureFormat_RG8_SNORM: filament_backend_TextureFormat = 9;
pub const filament_backend_TextureFormat_RG8UI: filament_backend_TextureFormat = 10;
pub const filament_backend_TextureFormat_RG8I: filament_backend_TextureFormat = 11;
pub const filament_backend_TextureFormat_RGB565: filament_backend_TextureFormat = 12;
pub const filament_backend_TextureFormat_RGB9_E5: filament_backend_TextureFormat = 13;
pub const filament_backend_TextureFormat_RGB5_A1: filament_backend_TextureFormat = 14;
pub const filament_backend_TextureFormat_RGBA4: filament_backend_TextureFormat = 15;
pub const filament_backend_TextureFormat_DEPTH16: filament_backend_TextureFormat = 16;
pub const filament_backend_TextureFormat_RGB8: filament_backend_TextureFormat = 17;
pub const filament_backend_TextureFormat_SRGB8: filament_backend_TextureFormat = 18;
pub const filament_backend_TextureFormat_RGB8_SNORM: filament_backend_TextureFormat = 19;
pub const filament_backend_TextureFormat_RGB8UI: filament_backend_TextureFormat = 20;
pub const filament_backend_TextureFormat_RGB8I: filament_backend_TextureFormat = 21;
pub const filament_backend_TextureFormat_DEPTH24: filament_backend_TextureFormat = 22;
pub const filament_backend_TextureFormat_R32F: filament_backend_TextureFormat = 23;
pub const filament_backend_TextureFormat_R32UI: filament_backend_TextureFormat = 24;
pub const filament_backend_TextureFormat_R32I: filament_backend_TextureFormat = 25;
pub const filament_backend_TextureFormat_RG16F: filament_backend_TextureFormat = 26;
pub const filament_backend_TextureFormat_RG16UI: filament_backend_TextureFormat = 27;
pub const filament_backend_TextureFormat_RG16I: filament_backend_TextureFormat = 28;
pub const filament_backend_TextureFormat_R11F_G11F_B10F: filament_backend_TextureFormat = 29;
pub const filament_backend_TextureFormat_RGBA8: filament_backend_TextureFormat = 30;
pub const filament_backend_TextureFormat_SRGB8_A8: filament_backend_TextureFormat = 31;
pub const filament_backend_TextureFormat_RGBA8_SNORM: filament_backend_TextureFormat = 32;
pub const filament_backend_TextureFormat_UNUSED: filament_backend_TextureFormat = 33;
pub const filament_backend_TextureFormat_RGB10_A2: filament_backend_TextureFormat = 34;
pub const filament_backend_TextureFormat_RGBA8UI: filament_backend_TextureFormat = 35;
pub const filament_backend_TextureFormat_RGBA8I: filament_backend_TextureFormat = 36;
pub const filament_backend_TextureFormat_DEPTH32F: filament_backend_TextureFormat = 37;
pub const filament_backend_TextureFormat_DEPTH24_STENCIL8: filament_backend_TextureFormat = 38;
pub const filament_backend_TextureFormat_DEPTH32F_STENCIL8: filament_backend_TextureFormat = 39;
pub const filament_backend_TextureFormat_RGB16F: filament_backend_TextureFormat = 40;
pub const filament_backend_TextureFormat_RGB16UI: filament_backend_TextureFormat = 41;
pub const filament_backend_TextureFormat_RGB16I: filament_backend_TextureFormat = 42;
pub const filament_backend_TextureFormat_RG32F: filament_backend_TextureFormat = 43;
pub const filament_backend_TextureFormat_RG32UI: filament_backend_TextureFormat = 44;
pub const filament_backend_TextureFormat_RG32I: filament_backend_TextureFormat = 45;
pub const filament_backend_TextureFormat_RGBA16F: filament_backend_TextureFormat = 46;
pub const filament_backend_TextureFormat_RGBA16UI: filament_backend_TextureFormat = 47;
pub const filament_backend_TextureFormat_RGBA16I: filament_backend_TextureFormat = 48;
pub const filament_backend_TextureFormat_RGB32F: filament_backend_TextureFormat = 49;
pub const filament_backend_TextureFormat_RGB32UI: filament_backend_TextureFormat = 50;
pub const filament_backend_TextureFormat_RGB32I: filament_backend_TextureFormat = 51;
pub const filament_backend_TextureFormat_RGBA32F: filament_backend_TextureFormat = 52;
pub const filament_backend_TextureFormat_RGBA32UI: filament_backend_TextureFormat = 53;
pub const filament_backend_TextureFormat_RGBA32I: filament_backend_TextureFormat = 54;
pub const filament_backend_TextureFormat_EAC_R11: filament_backend_TextureFormat = 55;
pub const filament_backend_TextureFormat_EAC_R11_SIGNED: filament_backend_TextureFormat = 56;
pub const filament_backend_TextureFormat_EAC_RG11: filament_backend_TextureFormat = 57;
pub const filament_backend_TextureFormat_EAC_RG11_SIGNED: filament_backend_TextureFormat = 58;
pub const filament_backend_TextureFormat_ETC2_RGB8: filament_backend_TextureFormat = 59;
pub const filament_backend_TextureFormat_ETC2_SRGB8: filament_backend_TextureFormat = 60;
pub const filament_backend_TextureFormat_ETC2_RGB8_A1: filament_backend_TextureFormat = 61;
pub const filament_backend_TextureFormat_ETC2_SRGB8_A1: filament_backend_TextureFormat = 62;
pub const filament_backend_TextureFormat_ETC2_EAC_RGBA8: filament_backend_TextureFormat = 63;
pub const filament_backend_TextureFormat_ETC2_EAC_SRGBA8: filament_backend_TextureFormat = 64;
pub const filament_backend_TextureFormat_DXT1_RGB: filament_backend_TextureFormat = 65;
pub const filament_backend_TextureFormat_DXT1_RGBA: filament_backend_TextureFormat = 66;
pub const filament_backend_TextureFormat_DXT3_RGBA: filament_backend_TextureFormat = 67;
pub const filament_backend_TextureFormat_DXT5_RGBA: filament_backend_TextureFormat = 68;
pub const filament_backend_TextureFormat_DXT1_SRGB: filament_backend_TextureFormat = 69;
pub const filament_backend_TextureFormat_DXT1_SRGBA: filament_backend_TextureFormat = 70;
pub const filament_backend_TextureFormat_DXT3_SRGBA: filament_backend_TextureFormat = 71;
pub const filament_backend_TextureFormat_DXT5_SRGBA: filament_backend_TextureFormat = 72;
pub const filament_backend_TextureFormat_RGBA_ASTC_4x4: filament_backend_TextureFormat = 73;
pub const filament_backend_TextureFormat_RGBA_ASTC_5x4: filament_backend_TextureFormat = 74;
pub const filament_backend_TextureFormat_RGBA_ASTC_5x5: filament_backend_TextureFormat = 75;
pub const filament_backend_TextureFormat_RGBA_ASTC_6x5: filament_backend_TextureFormat = 76;
pub const filament_backend_TextureFormat_RGBA_ASTC_6x6: filament_backend_TextureFormat = 77;
pub const filament_backend_TextureFormat_RGBA_ASTC_8x5: filament_backend_TextureFormat = 78;
pub const filament_backend_TextureFormat_RGBA_ASTC_8x6: filament_backend_TextureFormat = 79;
pub const filament_backend_TextureFormat_RGBA_ASTC_8x8: filament_backend_TextureFormat = 80;
pub const filament_backend_TextureFormat_RGBA_ASTC_10x5: filament_backend_TextureFormat = 81;
pub const filament_backend_TextureFormat_RGBA_ASTC_10x6: filament_backend_TextureFormat = 82;
pub const filament_backend_TextureFormat_RGBA_ASTC_10x8: filament_backend_TextureFormat = 83;
pub const filament_backend_TextureFormat_RGBA_ASTC_10x10: filament_backend_TextureFormat = 84;
pub const filament_backend_TextureFormat_RGBA_ASTC_12x10: filament_backend_TextureFormat = 85;
pub const filament_backend_TextureFormat_RGBA_ASTC_12x12: filament_backend_TextureFormat = 86;
pub const filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_4x4: filament_backend_TextureFormat = 87;
pub const filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_5x4: filament_backend_TextureFormat = 88;
pub const filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_5x5: filament_backend_TextureFormat = 89;
pub const filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_6x5: filament_backend_TextureFormat = 90;
pub const filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_6x6: filament_backend_TextureFormat = 91;
pub const filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_8x5: filament_backend_TextureFormat = 92;
pub const filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_8x6: filament_backend_TextureFormat = 93;
pub const filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_8x8: filament_backend_TextureFormat = 94;
pub const filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_10x5: filament_backend_TextureFormat =
    95;
pub const filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_10x6: filament_backend_TextureFormat =
    96;
pub const filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_10x8: filament_backend_TextureFormat =
    97;
pub const filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_10x10: filament_backend_TextureFormat =
    98;
pub const filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_12x10: filament_backend_TextureFormat =
    99;
pub const filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_12x12: filament_backend_TextureFormat =
    100;
#[doc = " Supported texel formats"]
#[doc = " These formats are typically used to specify a texture's internal storage format."]
#[doc = ""]
#[doc = " Enumerants syntax format"]
#[doc = " ========================"]
#[doc = ""]
#[doc = " `[components][size][type]`"]
#[doc = ""]
#[doc = " `components` : List of stored components by this format.\\n"]
#[doc = " `size`       : Size in bit of each component.\\n"]
#[doc = " `type`       : Type this format is stored as.\\n"]
#[doc = ""]
#[doc = ""]
#[doc = " Name     | Component"]
#[doc = " :--------|:-------------------------------"]
#[doc = " R        | Linear Red"]
#[doc = " RG       | Linear Red, Green"]
#[doc = " RGB      | Linear Red, Green, Blue"]
#[doc = " RGBA     | Linear Red, Green Blue, Alpha"]
#[doc = " SRGB     | sRGB encoded Red, Green, Blue"]
#[doc = " DEPTH    | Depth"]
#[doc = " STENCIL  | Stencil"]
#[doc = ""]
#[doc = " \\n"]
#[doc = " Name     | Type"]
#[doc = " :--------|:---------------------------------------------------"]
#[doc = " (none)   | Unsigned Normalized Integer [0, 1]"]
#[doc = " _SNORM   | Signed Normalized Integer [-1, 1]"]
#[doc = " UI       | Unsigned Integer @f$ [0, 2^{size}] @f$"]
#[doc = " I        | Signed Integer @f$ [-2^{size-1}, 2^{size-1}-1] @f$"]
#[doc = " F        | Floating-point"]
#[doc = ""]
#[doc = ""]
#[doc = " Special color formats"]
#[doc = " ---------------------"]
#[doc = ""]
#[doc = " There are a few special color formats that don't follow the convention above:"]
#[doc = ""]
#[doc = " Name             | Format"]
#[doc = " :----------------|:--------------------------------------------------------------------------"]
#[doc = " RGB565           |  5-bits for R and B, 6-bits for G."]
#[doc = " RGB5_A1          |  5-bits for R, G and B, 1-bit for A."]
#[doc = " RGB10_A2         | 10-bits for R, G and B, 2-bits for A."]
#[doc = " RGB9_E5          | **Unsigned** floating point. 9-bits mantissa for RGB, 5-bits shared exponent"]
#[doc = " R11F_G11F_B10F   | **Unsigned** floating point. 6-bits mantissa, for R and G, 5-bits for B. 5-bits exponent."]
#[doc = " SRGB8_A8         | sRGB 8-bits with linear 8-bits alpha."]
#[doc = " DEPTH24_STENCIL8 | 24-bits unsigned normalized integer depth, 8-bits stencil."]
#[doc = " DEPTH32F_STENCIL8| 32-bits floating-point depth, 8-bits stencil."]
#[doc = ""]
#[doc = ""]
#[doc = " Compressed texture formats"]
#[doc = " --------------------------"]
#[doc = ""]
#[doc = " Many compressed texture formats are supported as well, which include (but are not limited to)"]
#[doc = " the following list:"]
#[doc = ""]
#[doc = " Name             | Format"]
#[doc = " :----------------|:--------------------------------------------------------------------------"]
#[doc = " EAC_R11          | Compresses R11UI"]
#[doc = " EAC_R11_SIGNED   | Compresses R11I"]
#[doc = " EAC_RG11         | Compresses RG11UI"]
#[doc = " EAC_RG11_SIGNED  | Compresses RG11I"]
#[doc = " ETC2_RGB8        | Compresses RGB8"]
#[doc = " ETC2_SRGB8       | compresses SRGB8"]
#[doc = " ETC2_EAC_RGBA8   | Compresses RGBA8"]
#[doc = " ETC2_EAC_SRGBA8  | Compresses SRGB8_A8"]
#[doc = " ETC2_RGB8_A1     | Compresses RGB8 with 1-bit alpha"]
#[doc = " ETC2_SRGB8_A1    | Compresses sRGB8 with 1-bit alpha"]
#[doc = ""]
#[doc = ""]
#[doc = " @see Texture"]
pub type filament_backend_TextureFormat = u16;
pub const filament_backend_TextureUsage_NONE: filament_backend_TextureUsage = 0;
#[doc = "!< Texture can be used as a color attachment"]
pub const filament_backend_TextureUsage_COLOR_ATTACHMENT: filament_backend_TextureUsage = 1;
#[doc = "!< Texture can be used as a depth attachment"]
pub const filament_backend_TextureUsage_DEPTH_ATTACHMENT: filament_backend_TextureUsage = 2;
#[doc = "!< Texture can be used as a stencil attachment"]
pub const filament_backend_TextureUsage_STENCIL_ATTACHMENT: filament_backend_TextureUsage = 4;
#[doc = "!< Data can be uploaded into this texture (default)"]
pub const filament_backend_TextureUsage_UPLOADABLE: filament_backend_TextureUsage = 8;
#[doc = "!< Texture can be sampled (default)"]
pub const filament_backend_TextureUsage_SAMPLEABLE: filament_backend_TextureUsage = 16;
#[doc = "!< Texture can be used as a subpass input"]
pub const filament_backend_TextureUsage_SUBPASS_INPUT: filament_backend_TextureUsage = 32;
#[doc = "!< Default texture usage"]
pub const filament_backend_TextureUsage_DEFAULT: filament_backend_TextureUsage = 24;
#[doc = "! Bitmask describing the intended Texture Usage"]
pub type filament_backend_TextureUsage = u8;
pub const filament_backend_TextureSwizzle_SUBSTITUTE_ZERO: filament_backend_TextureSwizzle = 0;
pub const filament_backend_TextureSwizzle_SUBSTITUTE_ONE: filament_backend_TextureSwizzle = 1;
pub const filament_backend_TextureSwizzle_CHANNEL_0: filament_backend_TextureSwizzle = 2;
pub const filament_backend_TextureSwizzle_CHANNEL_1: filament_backend_TextureSwizzle = 3;
pub const filament_backend_TextureSwizzle_CHANNEL_2: filament_backend_TextureSwizzle = 4;
pub const filament_backend_TextureSwizzle_CHANNEL_3: filament_backend_TextureSwizzle = 5;
#[doc = "! Texture swizzle"]
pub type filament_backend_TextureSwizzle = u8;
#[doc = "!< +x face"]
pub const filament_backend_TextureCubemapFace_POSITIVE_X: filament_backend_TextureCubemapFace = 0;
#[doc = "!< -x face"]
pub const filament_backend_TextureCubemapFace_NEGATIVE_X: filament_backend_TextureCubemapFace = 1;
#[doc = "!< +y face"]
pub const filament_backend_TextureCubemapFace_POSITIVE_Y: filament_backend_TextureCubemapFace = 2;
#[doc = "!< -y face"]
pub const filament_backend_TextureCubemapFace_NEGATIVE_Y: filament_backend_TextureCubemapFace = 3;
#[doc = "!< +z face"]
pub const filament_backend_TextureCubemapFace_POSITIVE_Z: filament_backend_TextureCubemapFace = 4;
#[doc = "!< -z face"]
pub const filament_backend_TextureCubemapFace_NEGATIVE_Z: filament_backend_TextureCubemapFace = 5;
#[doc = "! Texture Cubemap Face"]
pub type filament_backend_TextureCubemapFace = u8;
#[doc = "! Face offsets for all faces of a cubemap"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct filament_backend_FaceOffsets {
    pub __bindgen_anon_1: filament_backend_FaceOffsets__bindgen_ty_1,
}
pub type filament_backend_FaceOffsets_size_type = size_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub union filament_backend_FaceOffsets__bindgen_ty_1 {
    pub __bindgen_anon_1: filament_backend_FaceOffsets__bindgen_ty_1__bindgen_ty_1,
    pub offsets: [filament_backend_FaceOffsets_size_type; 6usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_backend_FaceOffsets__bindgen_ty_1__bindgen_ty_1 {
    #[doc = "!< +x face offset in bytes"]
    pub px: filament_backend_FaceOffsets_size_type,
    #[doc = "!< -x face offset in bytes"]
    pub nx: filament_backend_FaceOffsets_size_type,
    #[doc = "!< +y face offset in bytes"]
    pub py: filament_backend_FaceOffsets_size_type,
    #[doc = "!< -y face offset in bytes"]
    pub ny: filament_backend_FaceOffsets_size_type,
    #[doc = "!< +z face offset in bytes"]
    pub pz: filament_backend_FaceOffsets_size_type,
    #[doc = "!< -z face offset in bytes"]
    pub nz: filament_backend_FaceOffsets_size_type,
}
#[test]
fn bindgen_test_layout_filament_backend_FaceOffsets__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<filament_backend_FaceOffsets__bindgen_ty_1__bindgen_ty_1>(),
        48usize,
        concat!(
            "Size of: ",
            stringify!(filament_backend_FaceOffsets__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_backend_FaceOffsets__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(filament_backend_FaceOffsets__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_backend_FaceOffsets__bindgen_ty_1__bindgen_ty_1>())).px
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_backend_FaceOffsets__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(px)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_backend_FaceOffsets__bindgen_ty_1__bindgen_ty_1>())).nx
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_backend_FaceOffsets__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(nx)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_backend_FaceOffsets__bindgen_ty_1__bindgen_ty_1>())).py
                as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_backend_FaceOffsets__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(py)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_backend_FaceOffsets__bindgen_ty_1__bindgen_ty_1>())).ny
                as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_backend_FaceOffsets__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(ny)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_backend_FaceOffsets__bindgen_ty_1__bindgen_ty_1>())).pz
                as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_backend_FaceOffsets__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(pz)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_backend_FaceOffsets__bindgen_ty_1__bindgen_ty_1>())).nz
                as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_backend_FaceOffsets__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(nz)
        )
    );
}
#[test]
fn bindgen_test_layout_filament_backend_FaceOffsets__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<filament_backend_FaceOffsets__bindgen_ty_1>(),
        48usize,
        concat!(
            "Size of: ",
            stringify!(filament_backend_FaceOffsets__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_backend_FaceOffsets__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(filament_backend_FaceOffsets__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_backend_FaceOffsets__bindgen_ty_1>())).offsets
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_backend_FaceOffsets__bindgen_ty_1),
            "::",
            stringify!(offsets)
        )
    );
}
impl Default for filament_backend_FaceOffsets__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn bindgen_test_layout_filament_backend_FaceOffsets() {
    assert_eq!(
        ::std::mem::size_of::<filament_backend_FaceOffsets>(),
        48usize,
        concat!("Size of: ", stringify!(filament_backend_FaceOffsets))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_backend_FaceOffsets>(),
        8usize,
        concat!("Alignment of ", stringify!(filament_backend_FaceOffsets))
    );
}
impl Default for filament_backend_FaceOffsets {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = "!< clamp-to-edge. The edge of the texture extends to infinity."]
pub const filament_backend_SamplerWrapMode_CLAMP_TO_EDGE: filament_backend_SamplerWrapMode = 0;
#[doc = "!< repeat. The texture infinitely repeats in the wrap direction."]
pub const filament_backend_SamplerWrapMode_REPEAT: filament_backend_SamplerWrapMode = 1;
#[doc = "!< mirrored-repeat. The texture infinitely repeats and mirrors in the wrap direction."]
pub const filament_backend_SamplerWrapMode_MIRRORED_REPEAT: filament_backend_SamplerWrapMode = 2;
#[doc = "! Sampler Wrap mode"]
pub type filament_backend_SamplerWrapMode = u8;
#[doc = "!< No filtering. Nearest neighbor is used."]
pub const filament_backend_SamplerMinFilter_NEAREST: filament_backend_SamplerMinFilter = 0;
#[doc = "!< Box filtering. Weighted average of 4 neighbors is used."]
pub const filament_backend_SamplerMinFilter_LINEAR: filament_backend_SamplerMinFilter = 1;
#[doc = "!< Mip-mapping is activated. But no filtering occurs."]
pub const filament_backend_SamplerMinFilter_NEAREST_MIPMAP_NEAREST:
    filament_backend_SamplerMinFilter = 2;
#[doc = "!< Box filtering within a mip-map level."]
pub const filament_backend_SamplerMinFilter_LINEAR_MIPMAP_NEAREST:
    filament_backend_SamplerMinFilter = 3;
#[doc = "!< Mip-map levels are interpolated, but no other filtering occurs."]
pub const filament_backend_SamplerMinFilter_NEAREST_MIPMAP_LINEAR:
    filament_backend_SamplerMinFilter = 4;
#[doc = "!< Both interpolated Mip-mapping and linear filtering are used."]
pub const filament_backend_SamplerMinFilter_LINEAR_MIPMAP_LINEAR:
    filament_backend_SamplerMinFilter = 5;
#[doc = "! Sampler minification filter"]
pub type filament_backend_SamplerMinFilter = u8;
#[doc = "!< No filtering. Nearest neighbor is used."]
pub const filament_backend_SamplerMagFilter_NEAREST: filament_backend_SamplerMagFilter = 0;
#[doc = "!< Box filtering. Weighted average of 4 neighbors is used."]
pub const filament_backend_SamplerMagFilter_LINEAR: filament_backend_SamplerMagFilter = 1;
#[doc = "! Sampler magnification filter"]
pub type filament_backend_SamplerMagFilter = u8;
pub const filament_backend_SamplerCompareMode_NONE: filament_backend_SamplerCompareMode = 0;
pub const filament_backend_SamplerCompareMode_COMPARE_TO_TEXTURE:
    filament_backend_SamplerCompareMode = 1;
#[doc = "! Sampler compare mode"]
pub type filament_backend_SamplerCompareMode = u8;
#[doc = "!< Less or equal"]
pub const filament_backend_SamplerCompareFunc_LE: filament_backend_SamplerCompareFunc = 0;
#[doc = "!< Greater or equal"]
pub const filament_backend_SamplerCompareFunc_GE: filament_backend_SamplerCompareFunc = 1;
#[doc = "!< Strictly less than"]
pub const filament_backend_SamplerCompareFunc_L: filament_backend_SamplerCompareFunc = 2;
#[doc = "!< Strictly greater than"]
pub const filament_backend_SamplerCompareFunc_G: filament_backend_SamplerCompareFunc = 3;
#[doc = "!< Equal"]
pub const filament_backend_SamplerCompareFunc_E: filament_backend_SamplerCompareFunc = 4;
#[doc = "!< Not equal"]
pub const filament_backend_SamplerCompareFunc_NE: filament_backend_SamplerCompareFunc = 5;
#[doc = "!< Always. Depth testing is deactivated."]
pub const filament_backend_SamplerCompareFunc_A: filament_backend_SamplerCompareFunc = 6;
#[doc = "!< Never. The depth test always fails."]
pub const filament_backend_SamplerCompareFunc_N: filament_backend_SamplerCompareFunc = 7;
#[doc = "! comparison function for the depth sampler"]
pub type filament_backend_SamplerCompareFunc = u8;
#[doc = "! Sampler paramters"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct filament_backend_SamplerParams {
    pub __bindgen_anon_1: filament_backend_SamplerParams__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union filament_backend_SamplerParams__bindgen_ty_1 {
    pub __bindgen_anon_1: filament_backend_SamplerParams__bindgen_ty_1__bindgen_ty_1,
    pub u: u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct filament_backend_SamplerParams__bindgen_ty_1__bindgen_ty_1 {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
#[test]
fn bindgen_test_layout_filament_backend_SamplerParams__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<filament_backend_SamplerParams__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Size of: ",
            stringify!(filament_backend_SamplerParams__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_backend_SamplerParams__bindgen_ty_1__bindgen_ty_1>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(filament_backend_SamplerParams__bindgen_ty_1__bindgen_ty_1)
        )
    );
}
impl Default for filament_backend_SamplerParams__bindgen_ty_1__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl filament_backend_SamplerParams__bindgen_ty_1__bindgen_ty_1 {
    #[inline]
    pub fn filterMag(&self) -> filament_backend_SamplerMagFilter {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_filterMag(&mut self, val: filament_backend_SamplerMagFilter) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn filterMin(&self) -> filament_backend_SamplerMinFilter {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 3u8) as u8) }
    }
    #[inline]
    pub fn set_filterMin(&mut self, val: filament_backend_SamplerMinFilter) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn wrapS(&self) -> filament_backend_SamplerWrapMode {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 2u8) as u8) }
    }
    #[inline]
    pub fn set_wrapS(&mut self, val: filament_backend_SamplerWrapMode) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn wrapT(&self) -> filament_backend_SamplerWrapMode {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 2u8) as u8) }
    }
    #[inline]
    pub fn set_wrapT(&mut self, val: filament_backend_SamplerWrapMode) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(6usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn wrapR(&self) -> filament_backend_SamplerWrapMode {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 2u8) as u8) }
    }
    #[inline]
    pub fn set_wrapR(&mut self, val: filament_backend_SamplerWrapMode) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn anisotropyLog2(&self) -> u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(10usize, 3u8) as u8) }
    }
    #[inline]
    pub fn set_anisotropyLog2(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(10usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn compareMode(&self) -> filament_backend_SamplerCompareMode {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_compareMode(&mut self, val: filament_backend_SamplerCompareMode) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn padding0(&self) -> u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(14usize, 2u8) as u8) }
    }
    #[inline]
    pub fn set_padding0(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(14usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn compareFunc(&self) -> filament_backend_SamplerCompareFunc {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 3u8) as u8) }
    }
    #[inline]
    pub fn set_compareFunc(&mut self, val: filament_backend_SamplerCompareFunc) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn padding1(&self) -> u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(19usize, 5u8) as u8) }
    }
    #[inline]
    pub fn set_padding1(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(19usize, 5u8, val as u64)
        }
    }
    #[inline]
    pub fn padding2(&self) -> u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(24usize, 8u8) as u8) }
    }
    #[inline]
    pub fn set_padding2(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(24usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        filterMag: filament_backend_SamplerMagFilter,
        filterMin: filament_backend_SamplerMinFilter,
        wrapS: filament_backend_SamplerWrapMode,
        wrapT: filament_backend_SamplerWrapMode,
        wrapR: filament_backend_SamplerWrapMode,
        anisotropyLog2: u8,
        compareMode: filament_backend_SamplerCompareMode,
        padding0: u8,
        compareFunc: filament_backend_SamplerCompareFunc,
        padding1: u8,
        padding2: u8,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let filterMag: u8 = unsafe { ::std::mem::transmute(filterMag) };
            filterMag as u64
        });
        __bindgen_bitfield_unit.set(1usize, 3u8, {
            let filterMin: u8 = unsafe { ::std::mem::transmute(filterMin) };
            filterMin as u64
        });
        __bindgen_bitfield_unit.set(4usize, 2u8, {
            let wrapS: u8 = unsafe { ::std::mem::transmute(wrapS) };
            wrapS as u64
        });
        __bindgen_bitfield_unit.set(6usize, 2u8, {
            let wrapT: u8 = unsafe { ::std::mem::transmute(wrapT) };
            wrapT as u64
        });
        __bindgen_bitfield_unit.set(8usize, 2u8, {
            let wrapR: u8 = unsafe { ::std::mem::transmute(wrapR) };
            wrapR as u64
        });
        __bindgen_bitfield_unit.set(10usize, 3u8, {
            let anisotropyLog2: u8 = unsafe { ::std::mem::transmute(anisotropyLog2) };
            anisotropyLog2 as u64
        });
        __bindgen_bitfield_unit.set(13usize, 1u8, {
            let compareMode: u8 = unsafe { ::std::mem::transmute(compareMode) };
            compareMode as u64
        });
        __bindgen_bitfield_unit.set(14usize, 2u8, {
            let padding0: u8 = unsafe { ::std::mem::transmute(padding0) };
            padding0 as u64
        });
        __bindgen_bitfield_unit.set(16usize, 3u8, {
            let compareFunc: u8 = unsafe { ::std::mem::transmute(compareFunc) };
            compareFunc as u64
        });
        __bindgen_bitfield_unit.set(19usize, 5u8, {
            let padding1: u8 = unsafe { ::std::mem::transmute(padding1) };
            padding1 as u64
        });
        __bindgen_bitfield_unit.set(24usize, 8u8, {
            let padding2: u8 = unsafe { ::std::mem::transmute(padding2) };
            padding2 as u64
        });
        __bindgen_bitfield_unit
    }
}
#[test]
fn bindgen_test_layout_filament_backend_SamplerParams__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<filament_backend_SamplerParams__bindgen_ty_1>(),
        4usize,
        concat!(
            "Size of: ",
            stringify!(filament_backend_SamplerParams__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_backend_SamplerParams__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(filament_backend_SamplerParams__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_backend_SamplerParams__bindgen_ty_1>())).u as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_backend_SamplerParams__bindgen_ty_1),
            "::",
            stringify!(u)
        )
    );
}
impl Default for filament_backend_SamplerParams__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn bindgen_test_layout_filament_backend_SamplerParams() {
    assert_eq!(
        ::std::mem::size_of::<filament_backend_SamplerParams>(),
        4usize,
        concat!("Size of: ", stringify!(filament_backend_SamplerParams))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_backend_SamplerParams>(),
        4usize,
        concat!("Alignment of ", stringify!(filament_backend_SamplerParams))
    );
}
impl Default for filament_backend_SamplerParams {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = "!< Not synchronized but copy-free. Good for video."]
pub const filament_backend_StreamType_NATIVE: filament_backend_StreamType = 0;
#[doc = "!< Synchronized, but GL-only and incurs copies. Good for AR on devices before API 26."]
pub const filament_backend_StreamType_TEXTURE_ID: filament_backend_StreamType = 1;
#[doc = "!< Synchronized, copy-free, and take a release callback. Good for AR but requires API 26+."]
pub const filament_backend_StreamType_ACQUIRED: filament_backend_StreamType = 2;
#[doc = "! Stream for external textures"]
pub type filament_backend_StreamType = ::std::os::raw::c_int;
#[doc = "! Releases an ACQUIRED external texture, guaranteed to be called on the application thread."]
pub type filament_backend_StreamCallback = ::std::option::Option<
    unsafe extern "C" fn(image: *mut ::std::os::raw::c_void, user: *mut ::std::os::raw::c_void),
>;
pub type filament_backend_FrameScheduledCallback = ::std::option::Option<
    unsafe extern "C" fn(
        callable: filament_backend_PresentCallable,
        user: *mut ::std::os::raw::c_void,
    ),
>;
pub type filament_backend_FrameCompletedCallback =
    ::std::option::Option<unsafe extern "C" fn(user: *mut ::std::os::raw::c_void)>;
#[doc = " A CPU memory-buffer descriptor, typically used to transfer data from the CPU to the GPU."]
#[doc = ""]
#[doc = " A BufferDescriptor owns the memory buffer it references, therefore BufferDescriptor cannot"]
#[doc = " be copied, but can be moved."]
#[doc = ""]
#[doc = " BufferDescriptor releases ownership of the memory-buffer when it's destroyed."]
#[repr(C)]
#[derive(Debug)]
pub struct filament_backend_BufferDescriptor {
    #[doc = "! CPU mempry-buffer virtual address"]
    pub buffer: *mut ::std::os::raw::c_void,
    #[doc = "! CPU memory-buffer size in bytes"]
    pub size: size_t,
    pub mCallback: filament_backend_BufferDescriptor_Callback,
    pub mUser: *mut ::std::os::raw::c_void,
    pub mHandler: *mut filament_backend_CallbackHandler,
}
#[doc = " Callback used to destroy the buffer data."]
#[doc = " Guarantees:"]
#[doc = "      Called on the main filament thread."]
#[doc = ""]
#[doc = " Limitations:"]
#[doc = "      Must be lightweight."]
#[doc = "      Must not call filament APIs."]
pub type filament_backend_BufferDescriptor_Callback = ::std::option::Option<
    unsafe extern "C" fn(
        buffer: *mut ::std::os::raw::c_void,
        size: size_t,
        user: *mut ::std::os::raw::c_void,
    ),
>;
#[test]
fn bindgen_test_layout_filament_backend_BufferDescriptor() {
    assert_eq!(
        ::std::mem::size_of::<filament_backend_BufferDescriptor>(),
        40usize,
        concat!("Size of: ", stringify!(filament_backend_BufferDescriptor))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_backend_BufferDescriptor>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(filament_backend_BufferDescriptor)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_backend_BufferDescriptor>())).buffer as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_backend_BufferDescriptor),
            "::",
            stringify!(buffer)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_backend_BufferDescriptor>())).size as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_backend_BufferDescriptor),
            "::",
            stringify!(size)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_backend_BufferDescriptor>())).mCallback as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_backend_BufferDescriptor),
            "::",
            stringify!(mCallback)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_backend_BufferDescriptor>())).mUser as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_backend_BufferDescriptor),
            "::",
            stringify!(mUser)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_backend_BufferDescriptor>())).mHandler as *const _
                as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_backend_BufferDescriptor),
            "::",
            stringify!(mHandler)
        )
    );
}
impl Default for filament_backend_BufferDescriptor {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct filament_backend_Driver {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct filament_backend_Platform__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug)]
pub struct filament_backend_Platform {
    pub vtable_: *const filament_backend_Platform__bindgen_vtable,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_backend_Platform_SwapChain {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_filament_backend_Platform_SwapChain() {
    assert_eq!(
        ::std::mem::size_of::<filament_backend_Platform_SwapChain>(),
        1usize,
        concat!("Size of: ", stringify!(filament_backend_Platform_SwapChain))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_backend_Platform_SwapChain>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(filament_backend_Platform_SwapChain)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_backend_Platform_Fence {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_filament_backend_Platform_Fence() {
    assert_eq!(
        ::std::mem::size_of::<filament_backend_Platform_Fence>(),
        1usize,
        concat!("Size of: ", stringify!(filament_backend_Platform_Fence))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_backend_Platform_Fence>(),
        1usize,
        concat!("Alignment of ", stringify!(filament_backend_Platform_Fence))
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_backend_Platform_Stream {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_filament_backend_Platform_Stream() {
    assert_eq!(
        ::std::mem::size_of::<filament_backend_Platform_Stream>(),
        1usize,
        concat!("Size of: ", stringify!(filament_backend_Platform_Stream))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_backend_Platform_Stream>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(filament_backend_Platform_Stream)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_backend_Platform_ExternalTexture {
    pub image: usize,
}
#[test]
fn bindgen_test_layout_filament_backend_Platform_ExternalTexture() {
    assert_eq!(
        ::std::mem::size_of::<filament_backend_Platform_ExternalTexture>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(filament_backend_Platform_ExternalTexture)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_backend_Platform_ExternalTexture>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(filament_backend_Platform_ExternalTexture)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_backend_Platform_ExternalTexture>())).image as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_backend_Platform_ExternalTexture),
            "::",
            stringify!(image)
        )
    );
}
#[test]
fn bindgen_test_layout_filament_backend_Platform() {
    assert_eq!(
        ::std::mem::size_of::<filament_backend_Platform>(),
        8usize,
        concat!("Size of: ", stringify!(filament_backend_Platform))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_backend_Platform>(),
        8usize,
        concat!("Alignment of ", stringify!(filament_backend_Platform))
    );
}
impl Default for filament_backend_Platform {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament7backend8PlatformD1Ev"]
    pub fn filament_backend_Platform_Platform_destructor(this: *mut filament_backend_Platform);
}
#[doc = " A descriptor to an image in main memory, typically used to transfer image data from the CPU"]
#[doc = " to the GPU."]
#[doc = ""]
#[doc = " A PixelBufferDescriptor owns the memory buffer it references, therefore PixelBufferDescriptor"]
#[doc = " cannot be copied, but can be moved."]
#[doc = ""]
#[doc = " PixelBufferDescriptor releases ownership of the memory-buffer when it's destroyed."]
#[repr(C)]
pub struct filament_backend_PixelBufferDescriptor {
    pub _base: filament_backend_BufferDescriptor,
    #[doc = "! left coordinate in pixels"]
    pub left: u32,
    #[doc = "! top coordinate in pixels"]
    pub top: u32,
    pub __bindgen_anon_1: filament_backend_PixelBufferDescriptor__bindgen_ty_1,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub __bindgen_padding_0: [u8; 7usize],
}
#[doc = "! Pixel Data Format"]
pub use self::filament_backend_PixelDataFormat as filament_backend_PixelBufferDescriptor_PixelDataFormat;
#[doc = "! Pixel Data Type"]
pub use self::filament_backend_PixelDataType as filament_backend_PixelBufferDescriptor_PixelDataType;
#[repr(C)]
#[derive(Copy, Clone)]
pub union filament_backend_PixelBufferDescriptor__bindgen_ty_1 {
    pub __bindgen_anon_1: filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_1,
    pub __bindgen_anon_2: filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_1 {
    #[doc = "! stride in pixels"]
    pub stride: u32,
    #[doc = "! Pixel data format"]
    pub format: filament_backend_PixelBufferDescriptor_PixelDataFormat,
}
#[test]
fn bindgen_test_layout_filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_1>(
        ),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<
                filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_1,
            >()))
            .stride as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(stride)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<
                filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_1,
            >()))
            .format as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(format)
        )
    );
}
impl Default for filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_2 {
    #[doc = "! compressed image size"]
    pub imageSize: u32,
    #[doc = "! compressed image format"]
    pub compressedFormat: filament_backend_CompressedPixelDataType,
}
#[test]
fn bindgen_test_layout_filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_2>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_2>(
        ),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<
                filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_2,
            >()))
            .imageSize as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(imageSize)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<
                filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_2,
            >()))
            .compressedFormat as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(compressedFormat)
        )
    );
}
impl Default for filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn bindgen_test_layout_filament_backend_PixelBufferDescriptor__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<filament_backend_PixelBufferDescriptor__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(filament_backend_PixelBufferDescriptor__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_backend_PixelBufferDescriptor__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(filament_backend_PixelBufferDescriptor__bindgen_ty_1)
        )
    );
}
impl Default for filament_backend_PixelBufferDescriptor__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn bindgen_test_layout_filament_backend_PixelBufferDescriptor() {
    assert_eq!(
        ::std::mem::size_of::<filament_backend_PixelBufferDescriptor>(),
        64usize,
        concat!(
            "Size of: ",
            stringify!(filament_backend_PixelBufferDescriptor)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_backend_PixelBufferDescriptor>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(filament_backend_PixelBufferDescriptor)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_backend_PixelBufferDescriptor>())).left as *const _
                as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_backend_PixelBufferDescriptor),
            "::",
            stringify!(left)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_backend_PixelBufferDescriptor>())).top as *const _
                as usize
        },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_backend_PixelBufferDescriptor),
            "::",
            stringify!(top)
        )
    );
}
impl Default for filament_backend_PixelBufferDescriptor {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl filament_backend_PixelBufferDescriptor {
    #[inline]
    pub fn type_(&self) -> filament_backend_PixelBufferDescriptor_PixelDataType {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u8) }
    }
    #[inline]
    pub fn set_type(&mut self, val: filament_backend_PixelBufferDescriptor_PixelDataType) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn alignment(&self) -> u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 4u8) as u8) }
    }
    #[inline]
    pub fn set_alignment(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        type_: filament_backend_PixelBufferDescriptor_PixelDataType,
        alignment: u8,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 4u8, {
            let type_: u8 = unsafe { ::std::mem::transmute(type_) };
            type_ as u64
        });
        __bindgen_bitfield_unit.set(4usize, 4u8, {
            let alignment: u8 = unsafe { ::std::mem::transmute(alignment) };
            alignment as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
pub struct filament_backend_CallbackHandler__bindgen_vtable(::std::os::raw::c_void);
#[doc = " A generic interface to dispatch callbacks."]
#[doc = ""]
#[doc = " All APIs that take a callback as argument also take a"]
#[doc = " CallbackHandler* which is used to dispatch the"]
#[doc = " callback: CallbackHandler::post() method is called from a service thread as soon"]
#[doc = " as possible (this will NEVER be the main thread), CallbackHandler::post()"]
#[doc = " is  responsible for scheduling the callback onto the thread the"]
#[doc = " user desires."]
#[doc = ""]
#[doc = " This is intended to make callbacks interoperate with"]
#[doc = " the platform/OS's own messaging system."]
#[doc = ""]
#[doc = " CallbackHandler* can always be nullptr in which case the default handler is used. The"]
#[doc = " default handler always dispatches callbacks on filament's main thread opportunistically."]
#[doc = ""]
#[doc = " Life time:"]
#[doc = " ---------"]
#[doc = ""]
#[doc = " Filament make no attempts to manage the life time of the CallbackHandler* and never takes"]
#[doc = " ownership."]
#[doc = " In particular, this means that the CallbackHandler instance must stay valid until all"]
#[doc = " pending callbacks are been dispatched."]
#[doc = ""]
#[doc = " Similarly, when shutting down filament, care must be taken to ensure that all pending callbacks"]
#[doc = " that might access filament's state have been dispatched. Filament can no longer ensure this"]
#[doc = " because callback execution is the responsibility of the CallbackHandler, which is external to"]
#[doc = " filament."]
#[doc = " Typically, the concrete CallbackHandler would have a mechanism to drain and/or wait for all"]
#[doc = " callbacks to be processed."]
#[doc = ""]
#[repr(C)]
#[derive(Debug)]
pub struct filament_backend_CallbackHandler {
    pub vtable_: *const filament_backend_CallbackHandler__bindgen_vtable,
}
pub type filament_backend_CallbackHandler_Callback =
    ::std::option::Option<unsafe extern "C" fn(user: *mut ::std::os::raw::c_void)>;
#[test]
fn bindgen_test_layout_filament_backend_CallbackHandler() {
    assert_eq!(
        ::std::mem::size_of::<filament_backend_CallbackHandler>(),
        8usize,
        concat!("Size of: ", stringify!(filament_backend_CallbackHandler))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_backend_CallbackHandler>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(filament_backend_CallbackHandler)
        )
    );
}
impl Default for filament_backend_CallbackHandler {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament7backend15CallbackHandlerD1Ev"]
    pub fn filament_backend_CallbackHandler_CallbackHandler_destructor(
        this: *mut filament_backend_CallbackHandler,
    );
}
#[doc = " A generic GPU buffer containing data."]
#[doc = ""]
#[doc = " Usage of this BufferObject is optional. For simple use cases it is not necessary. It is useful"]
#[doc = " only when you need to share data between multiple VertexBuffer instances. It also allows you to"]
#[doc = " efficiently swap-out the buffers in VertexBuffer."]
#[doc = ""]
#[doc = " NOTE: For now this is only used for vertex data, but in the future we may use it for other things"]
#[doc = " (e.g. compute)."]
#[doc = ""]
#[doc = " @see VertexBuffer"]
#[repr(C)]
#[derive(Debug, Default)]
pub struct filament_BufferObject {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct filament_BufferObject_BuilderDetails {
    _unused: [u8; 0],
}
#[doc = " A CPU memory-buffer descriptor, typically used to transfer data from the CPU to the GPU."]
#[doc = ""]
#[doc = " A BufferDescriptor owns the memory buffer it references, therefore BufferDescriptor cannot"]
#[doc = " be copied, but can be moved."]
#[doc = ""]
#[doc = " BufferDescriptor releases ownership of the memory-buffer when it's destroyed."]
pub type filament_BufferObject_BufferDescriptor = filament_backend_BufferDescriptor;
#[doc = "! Buffer object binding type"]
pub use self::filament_backend_BufferObjectBinding as filament_BufferObject_BindingType;
#[repr(C)]
#[derive(Debug)]
pub struct filament_BufferObject_Builder {
    pub _base: filament_BuilderBase<filament_BufferObject_BuilderDetails>,
}
#[test]
fn bindgen_test_layout_filament_BufferObject_Builder() {
    assert_eq!(
        ::std::mem::size_of::<filament_BufferObject_Builder>(),
        8usize,
        concat!("Size of: ", stringify!(filament_BufferObject_Builder))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_BufferObject_Builder>(),
        8usize,
        concat!("Alignment of ", stringify!(filament_BufferObject_Builder))
    );
}
extern "C" {
    #[doc = " Size of the buffer in bytes."]
    #[doc = " @param byteCount Maximum number of bytes the BufferObject can hold."]
    #[doc = " @return A reference to this Builder for chaining calls."]
    #[link_name = "\u{1}_ZN8filament12BufferObject7Builder4sizeEj"]
    pub fn filament_BufferObject_Builder_size(
        this: *mut filament_BufferObject_Builder,
        byteCount: u32,
    ) -> *mut filament_BufferObject_Builder;
}
extern "C" {
    #[doc = " The binding type for this buffer object. (defaults to VERTEX)"]
    #[doc = " @param BindingType Distinguishes between SSBO, VBO, etc. For now this must be VERTEX."]
    #[doc = " @return A reference to this Builder for chaining calls."]
    #[link_name = "\u{1}_ZN8filament12BufferObject7Builder11bindingTypeENS_7backend19BufferObjectBindingE"]
    pub fn filament_BufferObject_Builder_bindingType(
        this: *mut filament_BufferObject_Builder,
        bindingType: filament_BufferObject_BindingType,
    ) -> *mut filament_BufferObject_Builder;
}
extern "C" {
    #[doc = " Creates the BufferObject and returns a pointer to it. After creation, the buffer"]
    #[doc = " object is uninitialized. Use BufferObject::setBuffer() to initialize it."]
    #[doc = ""]
    #[doc = " @param engine Reference to the filament::Engine to associate this BufferObject with."]
    #[doc = ""]
    #[doc = " @return pointer to the newly created object or nullptr if exceptions are disabled and"]
    #[doc = "         an error occurred."]
    #[doc = ""]
    #[doc = " @exception utils::PostConditionPanic if a runtime error occurred, such as running out of"]
    #[doc = "            memory or other resources."]
    #[doc = " @exception utils::PreConditionPanic if a parameter to a builder function was invalid."]
    #[doc = ""]
    #[doc = " @see IndexBuffer::setBuffer"]
    #[link_name = "\u{1}_ZN8filament12BufferObject7Builder5buildERNS_6EngineE"]
    pub fn filament_BufferObject_Builder_build(
        this: *mut filament_BufferObject_Builder,
        engine: *mut filament_Engine,
    ) -> *mut filament_BufferObject;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament12BufferObject7BuilderC1Ev"]
    pub fn filament_BufferObject_Builder_Builder(this: *mut filament_BufferObject_Builder);
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament12BufferObject7BuilderC1ERKS1_"]
    pub fn filament_BufferObject_Builder_Builder1(
        this: *mut filament_BufferObject_Builder,
        rhs: *const filament_BufferObject_Builder,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament12BufferObject7BuilderC1EOS1_"]
    pub fn filament_BufferObject_Builder_Builder2(
        this: *mut filament_BufferObject_Builder,
        rhs: *mut filament_BufferObject_Builder,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament12BufferObject7BuilderD1Ev"]
    pub fn filament_BufferObject_Builder_Builder_destructor(
        this: *mut filament_BufferObject_Builder,
    );
}
impl Default for filament_BufferObject_Builder {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl filament_BufferObject_Builder {
    #[inline]
    pub unsafe fn size(&mut self, byteCount: u32) -> *mut filament_BufferObject_Builder {
        filament_BufferObject_Builder_size(self, byteCount)
    }
    #[inline]
    pub unsafe fn bindingType(
        &mut self,
        bindingType: filament_BufferObject_BindingType,
    ) -> *mut filament_BufferObject_Builder {
        filament_BufferObject_Builder_bindingType(self, bindingType)
    }
    #[inline]
    pub unsafe fn build(&mut self, engine: *mut filament_Engine) -> *mut filament_BufferObject {
        filament_BufferObject_Builder_build(self, engine)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_BufferObject_Builder_Builder(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(rhs: *const filament_BufferObject_Builder) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_BufferObject_Builder_Builder1(__bindgen_tmp.as_mut_ptr(), rhs);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new2(rhs: *mut filament_BufferObject_Builder) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_BufferObject_Builder_Builder2(__bindgen_tmp.as_mut_ptr(), rhs);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        filament_BufferObject_Builder_Builder_destructor(self)
    }
}
#[test]
fn bindgen_test_layout_filament_BufferObject() {
    assert_eq!(
        ::std::mem::size_of::<filament_BufferObject>(),
        1usize,
        concat!("Size of: ", stringify!(filament_BufferObject))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_BufferObject>(),
        1usize,
        concat!("Alignment of ", stringify!(filament_BufferObject))
    );
}
extern "C" {
    #[doc = " Asynchronously copy-initializes a region of this BufferObject from the data provided."]
    #[doc = ""]
    #[doc = " @param engine Reference to the filament::Engine associated with this BufferObject."]
    #[doc = " @param buffer A BufferDescriptor representing the data used to initialize the BufferObject."]
    #[doc = " @param byteOffset Offset in bytes into the BufferObject"]
    #[link_name = "\u{1}_ZN8filament12BufferObject9setBufferERNS_6EngineEONS_7backend16BufferDescriptorEj"]
    pub fn filament_BufferObject_setBuffer(
        this: *mut filament_BufferObject,
        engine: *mut filament_Engine,
        buffer: *mut filament_BufferObject_BufferDescriptor,
        byteOffset: u32,
    );
}
extern "C" {
    #[doc = " Returns the size of this BufferObject in elements."]
    #[doc = " @return The maximum capacity of the BufferObject."]
    #[link_name = "\u{1}_ZNK8filament12BufferObject12getByteCountEv"]
    pub fn filament_BufferObject_getByteCount(this: *const filament_BufferObject) -> size_t;
}
impl filament_BufferObject {
    #[inline]
    pub unsafe fn setBuffer(
        &mut self,
        engine: *mut filament_Engine,
        buffer: *mut filament_BufferObject_BufferDescriptor,
        byteOffset: u32,
    ) {
        filament_BufferObject_setBuffer(self, engine, buffer, byteOffset)
    }
    #[inline]
    pub unsafe fn getByteCount(&self) -> size_t {
        filament_BufferObject_getByteCount(self)
    }
}
#[doc = " Camera represents the eye through which the scene is viewed."]
#[doc = ""]
#[doc = " A Camera has a position and orientation and controls the projection and exposure parameters."]
#[doc = ""]
#[doc = " Creation and destruction"]
#[doc = " ========================"]
#[doc = ""]
#[doc = " In Filament, Camera is a component that must be associated with an entity. To do so,"]
#[doc = " use Engine::createCamera(Entity). A Camera component is destroyed using"]
#[doc = " Engine::destroyCameraComponent(Entity)."]
#[doc = ""]
#[doc = " ~~~~~~~~~~~{.cpp}"]
#[doc = "  filament::Engine* engine = filament::Engine::create();"]
#[doc = ""]
#[doc = "  utils::Entity myCameraEntity = utils::EntityManager::get().create();"]
#[doc = "  filament::Camera* myCamera = engine->createCamera(myCameraEntity);"]
#[doc = "  myCamera->setProjection(45, 16.0/9.0, 0.1, 1.0);"]
#[doc = "  myCamera->lookAt({0, 1.60, 1}, {0, 0, 0});"]
#[doc = "  engine->destroyCameraComponent(myCamera);"]
#[doc = " ~~~~~~~~~~~"]
#[doc = ""]
#[doc = ""]
#[doc = " Coordinate system"]
#[doc = " ================="]
#[doc = ""]
#[doc = " The camera coordinate system defines the *view space*. The camera points towards its -z axis"]
#[doc = " and is oriented such that its top side is in the direction of +y, and its right side in the"]
#[doc = " direction of +x."]
#[doc = ""]
#[doc = " @note"]
#[doc = " Since the *near* and *far* planes are defined by the distance from the camera,"]
#[doc = " their respective coordinates are -\\p distance(near) and -\\p distance(far)."]
#[doc = ""]
#[doc = " Clipping planes"]
#[doc = " ==============="]
#[doc = ""]
#[doc = " The camera defines six *clipping planes* which together create a *clipping volume*. The"]
#[doc = " geometry outside this volume is clipped."]
#[doc = ""]
#[doc = " The clipping volume can either be a box or a frustum depending on which projection is used,"]
#[doc = " respectively Projection.ORTHO or Projection.PERSPECTIVE. The six planes are specified either"]
#[doc = " directly or indirectly using setProjection()."]
#[doc = ""]
#[doc = " The six planes are:"]
#[doc = " - left"]
#[doc = " - right"]
#[doc = " - bottom"]
#[doc = " - top"]
#[doc = " - near"]
#[doc = " - far"]
#[doc = ""]
#[doc = " @note"]
#[doc = " To increase the depth-buffer precision, the *far* clipping plane is always assumed to be at"]
#[doc = " infinity for rendering. That is, it is not used to clip geometry during rendering."]
#[doc = " However, it is used during the culling phase (objects entirely behind the *far*"]
#[doc = " plane are culled)."]
#[doc = ""]
#[doc = ""]
#[doc = " Choosing the *near* plane distance"]
#[doc = " =================================="]
#[doc = ""]
#[doc = " The *near* plane distance greatly affects the depth-buffer resolution."]
#[doc = ""]
#[doc = " Example: Precision at 1m, 10m, 100m and 1Km for various near distances assuming a 32-bit float"]
#[doc = " depth-buffer"]
#[doc = ""]
#[doc = "    near (m)  |   1 m  |   10 m  |  100 m   |  1 Km"]
#[doc = "  -----------:|:------:|:-------:|:--------:|:--------:"]
#[doc = "      0.001   | 7.2e-5 |  0.0043 |  0.4624  |  48.58"]
#[doc = "      0.01    | 6.9e-6 |  0.0001 |  0.0430  |   4.62"]
#[doc = "      0.1     | 3.6e-7 |  7.0e-5 |  0.0072  |   0.43"]
#[doc = "      1.0     |    0   |  3.8e-6 |  0.0007  |   0.07"]
#[doc = ""]
#[doc = ""]
#[doc = "  As can be seen in the table above, the depth-buffer precision drops rapidly with the"]
#[doc = "  distance to the camera."]
#[doc = " Make sure to pick the highest *near* plane distance possible."]
#[doc = ""]
#[doc = ""]
#[doc = " Exposure"]
#[doc = " ========"]
#[doc = ""]
#[doc = " The Camera is also used to set the scene's exposure, just like with a real camera. The lights"]
#[doc = " intensity and the Camera exposure interact to produce the final scene's brightness."]
#[doc = ""]
#[doc = ""]
#[doc = ""]
#[doc = " \\see Frustum, View"]
#[repr(C)]
#[derive(Debug, Default)]
pub struct filament_Camera {
    pub _address: u8,
}
#[doc = "!< perspective projection, objects get smaller as they are farther"]
pub const filament_Camera_Projection_PERSPECTIVE: filament_Camera_Projection = 0;
#[doc = "!< orthonormal projection, preserves distances"]
pub const filament_Camera_Projection_ORTHO: filament_Camera_Projection = 1;
#[doc = "! Denotes the projection type used by this camera. \\see setProjection"]
pub type filament_Camera_Projection = ::std::os::raw::c_int;
#[doc = "!< the field-of-view angle is defined on the vertical axis"]
pub const filament_Camera_Fov_VERTICAL: filament_Camera_Fov = 0;
#[doc = "!< the field-of-view angle is defined on the horizontal axis"]
pub const filament_Camera_Fov_HORIZONTAL: filament_Camera_Fov = 1;
#[doc = "! Denotes a field-of-view direction. \\see setProjection"]
pub type filament_Camera_Fov = ::std::os::raw::c_int;
#[test]
fn bindgen_test_layout_filament_Camera() {
    assert_eq!(
        ::std::mem::size_of::<filament_Camera>(),
        1usize,
        concat!("Size of: ", stringify!(filament_Camera))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_Camera>(),
        1usize,
        concat!("Alignment of ", stringify!(filament_Camera))
    );
}
extern "C" {
    #[doc = " Sets the projection matrix from a frustum defined by six planes."]
    #[doc = ""]
    #[doc = " @param projection    type of #Projection to use."]
    #[doc = ""]
    #[doc = " @param left      distance in world units from the camera to the left plane,"]
    #[doc = "                  at the near plane."]
    #[doc = "                  Precondition: \\p left != \\p right."]
    #[doc = ""]
    #[doc = " @param right     distance in world units from the camera to the right plane,"]
    #[doc = "                  at the near plane."]
    #[doc = "                  Precondition: \\p left != \\p right."]
    #[doc = ""]
    #[doc = " @param bottom    distance in world units from the camera to the bottom plane,"]
    #[doc = "                  at the near plane."]
    #[doc = "                  Precondition: \\p bottom != \\p top."]
    #[doc = ""]
    #[doc = " @param top       distance in world units from the camera to the top plane,"]
    #[doc = "                  at the near plane."]
    #[doc = "                  Precondition: \\p left != \\p right."]
    #[doc = ""]
    #[doc = " @param near      distance in world units from the camera to the near plane. The near plane's"]
    #[doc = "                  position in view space is z = -\\p near."]
    #[doc = "                  Precondition: \\p near > 0 for PROJECTION::PERSPECTIVE or"]
    #[doc = "                                \\p near != far for PROJECTION::ORTHO"]
    #[doc = ""]
    #[doc = " @param far       distance in world units from the camera to the far plane. The far plane's"]
    #[doc = "                  position in view space is z = -\\p far."]
    #[doc = "                  Precondition: \\p far > near for PROJECTION::PERSPECTIVE or"]
    #[doc = "                                \\p far != near for PROJECTION::ORTHO"]
    #[doc = ""]
    #[doc = " @attention these parameters are silently modified to meet the preconditions above."]
    #[doc = ""]
    #[doc = " @see Projection, Frustum"]
    #[link_name = "\u{1}_ZN8filament6Camera13setProjectionENS0_10ProjectionEdddddd"]
    pub fn filament_Camera_setProjection(
        this: *mut filament_Camera,
        projection: filament_Camera_Projection,
        left: f64,
        right: f64,
        bottom: f64,
        top: f64,
        near: f64,
        far: f64,
    );
}
extern "C" {
    #[doc = " Sets the projection matrix from the field-of-view."]
    #[doc = ""]
    #[doc = " @param fovInDegrees full field-of-view in degrees. 0 < \\p fov < 180."]
    #[doc = " @param aspect       aspect ratio \\f$ \\frac{width}{height} \\f$. \\p aspect > 0."]
    #[doc = " @param near         distance in world units from the camera to the near plane. \\p near > 0."]
    #[doc = " @param far          distance in world units from the camera to the far plane. \\p far > \\p near."]
    #[doc = " @param direction    direction of the \\p fovInDegrees parameter."]
    #[doc = ""]
    #[doc = " @see Fov."]
    #[link_name = "\u{1}_ZN8filament6Camera13setProjectionEddddNS0_3FovE"]
    pub fn filament_Camera_setProjection1(
        this: *mut filament_Camera,
        fovInDegrees: f64,
        aspect: f64,
        near: f64,
        far: f64,
        direction: filament_Camera_Fov,
    );
}
extern "C" {
    #[doc = " Sets the projection matrix from the focal length."]
    #[doc = ""]
    #[doc = " @param focalLengthInMillimeters lens's focal length in millimeters. \\p focalLength > 0."]
    #[doc = " @param aspect      aspect ratio \\f$ \\frac{width}{height} \\f$. \\p aspect > 0."]
    #[doc = " @param near        distance in world units from the camera to the near plane. \\p near > 0."]
    #[doc = " @param far         distance in world units from the camera to the far plane. \\p far > \\p near."]
    #[link_name = "\u{1}_ZN8filament6Camera17setLensProjectionEdddd"]
    pub fn filament_Camera_setLensProjection(
        this: *mut filament_Camera,
        focalLengthInMillimeters: f64,
        aspect: f64,
        near: f64,
        far: f64,
    );
}
extern "C" {
    #[doc = " Sets a custom projection matrix."]
    #[doc = ""]
    #[doc = " The projection matrix must be of one of the following form:"]
    #[doc = "       a 0 tx 0              a 0 0 tx"]
    #[doc = "       0 b ty 0              0 b 0 ty"]
    #[doc = "       0 0 tz c              0 0 c tz"]
    #[doc = "       0 0 -1 0              0 0 0 1"]
    #[doc = ""]
    #[doc = " The projection matrix must define an NDC system that must match the OpenGL convention,"]
    #[doc = " that is all 3 axis are mapped to [-1, 1]."]
    #[doc = ""]
    #[doc = " @param projection  custom projection matrix used for rendering and culling"]
    #[doc = " @param near        distance in world units from the camera to the near plane. \\p near > 0."]
    #[doc = " @param far         distance in world units from the camera to the far plane. \\p far > \\p near."]
    #[link_name = "\u{1}_ZN8filament6Camera19setCustomProjectionERKNS_4math7details6TMat44IdEEdd"]
    pub fn filament_Camera_setCustomProjection(
        this: *mut filament_Camera,
        projection: *const filament_math_mat4,
        near: f64,
        far: f64,
    );
}
extern "C" {
    #[doc = " Sets the projection matrix."]
    #[doc = ""]
    #[doc = " The projection matrices must be of one of the following form:"]
    #[doc = "       a 0 tx 0              a 0 0 tx"]
    #[doc = "       0 b ty 0              0 b 0 ty"]
    #[doc = "       0 0 tz c              0 0 c tz"]
    #[doc = "       0 0 -1 0              0 0 0 1"]
    #[doc = ""]
    #[doc = " The projection matrices must define an NDC system that must match the OpenGL convention,"]
    #[doc = " that is all 3 axis are mapped to [-1, 1]."]
    #[doc = ""]
    #[doc = " @param projection  custom projection matrix used for rendering"]
    #[doc = " @param projectionForCulling  custom projection matrix used for culling"]
    #[doc = " @param near        distance in world units from the camera to the near plane. \\p near > 0."]
    #[doc = " @param far         distance in world units from the camera to the far plane. \\p far > \\p near."]
    #[link_name = "\u{1}_ZN8filament6Camera19setCustomProjectionERKNS_4math7details6TMat44IdEES6_dd"]
    pub fn filament_Camera_setCustomProjection1(
        this: *mut filament_Camera,
        projection: *const filament_math_mat4,
        projectionForCulling: *const filament_math_mat4,
        near: f64,
        far: f64,
    );
}
extern "C" {
    #[doc = " Sets an additional matrix that scales the projection matrix."]
    #[doc = ""]
    #[doc = " This is useful to adjust the aspect ratio of the camera independent from its projection."]
    #[doc = " First, pass an aspect of 1.0 to setProjection. Then set the scaling with the desired aspect"]
    #[doc = " ratio:"]
    #[doc = ""]
    #[doc = "     const double aspect = width / height;"]
    #[doc = ""]
    #[doc = "     // with Fov::HORIZONTAL passed to setProjection:"]
    #[doc = "     camera->setScaling(double4 {1.0, aspect});"]
    #[doc = ""]
    #[doc = "     // with Fov::VERTICAL passed to setProjection:"]
    #[doc = "     camera->setScaling(double4 {1.0 / aspect, 1.0});"]
    #[doc = ""]
    #[doc = ""]
    #[doc = " By default, this is an identity matrix."]
    #[doc = ""]
    #[doc = " @param scaling     diagonal of the 2x2 scaling matrix to be applied after the projection matrix."]
    #[doc = ""]
    #[doc = " @see setProjection, setLensProjection, setCustomProjection"]
    #[link_name = "\u{1}_ZN8filament6Camera10setScalingENS_4math7details5TVec2IdEE"]
    pub fn filament_Camera_setScaling(this: *mut filament_Camera, scaling: filament_math_double2);
}
extern "C" {
    #[doc = " Sets an additional matrix that shifts the projection matrix."]
    #[doc = " By default, this is an identity matrix."]
    #[doc = ""]
    #[doc = " @param shift     x and y translation added to the projection matrix, specified in NDC"]
    #[doc = "                  coordinates, that is, if the translation must be specified in pixels,"]
    #[doc = "                  shift must be scaled by 1.0 / { viewport.width, viewport.height }."]
    #[doc = ""]
    #[doc = " @see setProjection, setLensProjection, setCustomProjection"]
    #[link_name = "\u{1}_ZN8filament6Camera8setShiftENS_4math7details5TVec2IdEE"]
    pub fn filament_Camera_setShift(this: *mut filament_Camera, shift: filament_math_double2);
}
extern "C" {
    #[doc = " Returns the scaling amount used to scale the projection matrix."]
    #[doc = ""]
    #[doc = " @return the diagonal of the scaling matrix applied after the projection matrix."]
    #[doc = ""]
    #[doc = " @see setScaling"]
    #[link_name = "\u{1}_ZNK8filament6Camera10getScalingEv"]
    pub fn filament_Camera_getScaling(this: *const filament_Camera) -> filament_math_double4;
}
extern "C" {
    #[doc = " Returns the shift amount used to translate the projection matrix."]
    #[doc = ""]
    #[doc = " @return the 2D translation x and y offsets applied after the projection matrix."]
    #[doc = ""]
    #[doc = " @see setShift"]
    #[link_name = "\u{1}_ZNK8filament6Camera8getShiftEv"]
    pub fn filament_Camera_getShift(this: *const filament_Camera) -> filament_math_double2;
}
extern "C" {
    #[doc = " Returns the projection matrix used for rendering."]
    #[doc = ""]
    #[doc = " The projection matrix used for rendering always has its far plane set to infinity. This"]
    #[doc = " is why it may differ from the matrix set through setProjection() or setLensProjection()."]
    #[doc = ""]
    #[doc = " @return The projection matrix used for rendering"]
    #[doc = ""]
    #[doc = " @see setProjection, setLensProjection, setCustomProjection, getCullingProjectionMatrix"]
    #[link_name = "\u{1}_ZNK8filament6Camera19getProjectionMatrixEv"]
    pub fn filament_Camera_getProjectionMatrix(this: *const filament_Camera) -> filament_math_mat4;
}
extern "C" {
    #[doc = " Returns the projection matrix used for culling (far plane is finite)."]
    #[doc = ""]
    #[doc = " @return The projection matrix set by setProjection or setLensProjection."]
    #[doc = ""]
    #[doc = " @see setProjection, setLensProjection, getProjectionMatrix"]
    #[link_name = "\u{1}_ZNK8filament6Camera26getCullingProjectionMatrixEv"]
    pub fn filament_Camera_getCullingProjectionMatrix(
        this: *const filament_Camera,
    ) -> filament_math_mat4;
}
extern "C" {
    #[doc = "! Returns the frustum's near plane"]
    #[link_name = "\u{1}_ZNK8filament6Camera7getNearEv"]
    pub fn filament_Camera_getNear(this: *const filament_Camera) -> f32;
}
extern "C" {
    #[doc = "! Returns the frustum's far plane used for culling"]
    #[link_name = "\u{1}_ZNK8filament6Camera13getCullingFarEv"]
    pub fn filament_Camera_getCullingFar(this: *const filament_Camera) -> f32;
}
extern "C" {
    #[doc = " Sets the camera's view matrix."]
    #[doc = ""]
    #[doc = " Helper method to set the camera's entity transform component."]
    #[doc = " It has the same effect as calling:"]
    #[doc = ""]
    #[doc = " ~~~~~~~~~~~{.cpp}"]
    #[doc = "  engine.getTransformManager().setTransform("]
    #[doc = "          engine.getTransformManager().getInstance(camera->getEntity()), view);"]
    #[doc = " ~~~~~~~~~~~"]
    #[doc = ""]
    #[doc = " @param view The camera position and orientation provided as a rigid transform matrix."]
    #[doc = ""]
    #[doc = " @note The Camera \"looks\" towards its -z axis"]
    #[doc = ""]
    #[doc = " @warning \\p view must be a rigid transform"]
    #[link_name = "\u{1}_ZN8filament6Camera14setModelMatrixERKNS_4math7details6TMat44IdEE"]
    pub fn filament_Camera_setModelMatrix(
        this: *mut filament_Camera,
        view: *const filament_math_mat4,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament6Camera14setModelMatrixERKNS_4math7details6TMat44IfEE"]
    pub fn filament_Camera_setModelMatrix1(
        this: *mut filament_Camera,
        view: *const filament_math_mat4f,
    );
}
extern "C" {
    #[doc = " Sets the camera's view matrix"]
    #[doc = ""]
    #[doc = " @param eye       The position of the camera in world space."]
    #[doc = " @param center    The point in world space the camera is looking at."]
    #[doc = " @param up        A unit vector denoting the camera's \"up\" direction."]
    #[link_name = "\u{1}_ZN8filament6Camera6lookAtERKNS_4math7details5TVec3IfEES6_S6_"]
    pub fn filament_Camera_lookAt(
        this: *mut filament_Camera,
        eye: *const filament_math_float3,
        center: *const filament_math_float3,
        up: *const filament_math_float3,
    );
}
extern "C" {
    #[doc = " Sets the camera's view matrix, assuming up is along the y axis"]
    #[doc = ""]
    #[doc = " @param eye       The position of the camera in world space."]
    #[doc = " @param center    The point in world space the camera is looking at."]
    #[link_name = "\u{1}_ZN8filament6Camera6lookAtERKNS_4math7details5TVec3IfEES6_"]
    pub fn filament_Camera_lookAt1(
        this: *mut filament_Camera,
        eye: *const filament_math_float3,
        center: *const filament_math_float3,
    );
}
extern "C" {
    #[doc = " Returns the camera's model matrix"]
    #[doc = ""]
    #[doc = " Helper method to return the camera's entity transform component."]
    #[doc = " It has the same effect as calling:"]
    #[doc = ""]
    #[doc = " ~~~~~~~~~~~{.cpp}"]
    #[doc = "  engine.getTransformManager().getWorldTransform("]
    #[doc = "          engine.getTransformManager().getInstance(camera->getEntity()));"]
    #[doc = " ~~~~~~~~~~~"]
    #[doc = ""]
    #[doc = " @return The camera's pose in world space as a rigid transform. Parent transforms, if any,"]
    #[doc = " are taken into account."]
    #[link_name = "\u{1}_ZNK8filament6Camera14getModelMatrixEv"]
    pub fn filament_Camera_getModelMatrix(this: *const filament_Camera) -> filament_math_mat4;
}
extern "C" {
    #[doc = "! Returns the camera's view matrix (inverse of the model matrix)"]
    #[link_name = "\u{1}_ZNK8filament6Camera13getViewMatrixEv"]
    pub fn filament_Camera_getViewMatrix(this: *const filament_Camera) -> filament_math_mat4;
}
extern "C" {
    #[doc = "! Returns the camera's position in world space"]
    #[link_name = "\u{1}_ZNK8filament6Camera11getPositionEv"]
    pub fn filament_Camera_getPosition(this: *const filament_Camera) -> filament_math_float3;
}
extern "C" {
    #[doc = "! Returns the camera's normalized left vector"]
    #[link_name = "\u{1}_ZNK8filament6Camera13getLeftVectorEv"]
    pub fn filament_Camera_getLeftVector(this: *const filament_Camera) -> filament_math_float3;
}
extern "C" {
    #[doc = "! Returns the camera's normalized up vector"]
    #[link_name = "\u{1}_ZNK8filament6Camera11getUpVectorEv"]
    pub fn filament_Camera_getUpVector(this: *const filament_Camera) -> filament_math_float3;
}
extern "C" {
    #[doc = "! Returns the camera's forward vector"]
    #[link_name = "\u{1}_ZNK8filament6Camera16getForwardVectorEv"]
    pub fn filament_Camera_getForwardVector(this: *const filament_Camera) -> filament_math_float3;
}
extern "C" {
    #[doc = "! Returns the camera's field of view in degrees"]
    #[link_name = "\u{1}_ZNK8filament6Camera23getFieldOfViewInDegreesENS0_3FovE"]
    pub fn filament_Camera_getFieldOfViewInDegrees(
        this: *const filament_Camera,
        direction: filament_Camera_Fov,
    ) -> f32;
}
extern "C" {
    #[doc = "! Returns the camera's culling Frustum in world space"]
    #[link_name = "\u{1}_ZNK8filament6Camera10getFrustumEv"]
    pub fn filament_Camera_getFrustum(this: *const filament_Camera) -> filament_Frustum;
}
extern "C" {
    #[doc = "! Returns the entity representing this camera"]
    #[link_name = "\u{1}_ZNK8filament6Camera9getEntityEv"]
    pub fn filament_Camera_getEntity(this: *const filament_Camera) -> utils_Entity;
}
extern "C" {
    #[doc = " Sets this camera's exposure (default is f/16, 1/125s, 100 ISO)"]
    #[doc = ""]
    #[doc = " The exposure ultimately controls the scene's brightness, just like with a real camera."]
    #[doc = " The default values provide adequate exposure for a camera placed outdoors on a sunny day"]
    #[doc = " with the sun at the zenith."]
    #[doc = ""]
    #[doc = " @param aperture      Aperture in f-stops, clamped between 0.5 and 64."]
    #[doc = "                      A lower \\p aperture value *increases* the exposure, leading to"]
    #[doc = "                      a brighter scene. Realistic values are between 0.95 and 32."]
    #[doc = ""]
    #[doc = " @param shutterSpeed  Shutter speed in seconds, clamped between 1/25,000 and 60."]
    #[doc = "                      A lower shutter speed increases the exposure. Realistic values are"]
    #[doc = "                      between 1/8000 and 30."]
    #[doc = ""]
    #[doc = " @param sensitivity   Sensitivity in ISO, clamped between 10 and 204,800."]
    #[doc = "                      A higher \\p sensitivity increases the exposure. Realistic values are"]
    #[doc = "                      between 50 and 25600."]
    #[doc = ""]
    #[doc = " @note"]
    #[doc = " With the default parameters, the scene must contain at least one Light of intensity"]
    #[doc = " similar to the sun (e.g.: a 100,000 lux directional light)."]
    #[doc = ""]
    #[doc = " @see LightManager, Exposure"]
    #[link_name = "\u{1}_ZN8filament6Camera11setExposureEfff"]
    pub fn filament_Camera_setExposure(
        this: *mut filament_Camera,
        aperture: f32,
        shutterSpeed: f32,
        sensitivity: f32,
    );
}
extern "C" {
    #[doc = "! returns this camera's aperture in f-stops"]
    #[link_name = "\u{1}_ZNK8filament6Camera11getApertureEv"]
    pub fn filament_Camera_getAperture(this: *const filament_Camera) -> f32;
}
extern "C" {
    #[doc = "! returns this camera's shutter speed in seconds"]
    #[link_name = "\u{1}_ZNK8filament6Camera15getShutterSpeedEv"]
    pub fn filament_Camera_getShutterSpeed(this: *const filament_Camera) -> f32;
}
extern "C" {
    #[doc = "! returns this camera's sensitivity in ISO"]
    #[link_name = "\u{1}_ZNK8filament6Camera14getSensitivityEv"]
    pub fn filament_Camera_getSensitivity(this: *const filament_Camera) -> f32;
}
extern "C" {
    #[doc = "! returns the focal length in meters [m] for a 35mm camera"]
    #[link_name = "\u{1}_ZNK8filament6Camera14getFocalLengthEv"]
    pub fn filament_Camera_getFocalLength(this: *const filament_Camera) -> f64;
}
extern "C" {
    #[doc = " Sets the camera focus distance. This is used by the Depth-of-field PostProcessing effect."]
    #[doc = " @param distance Distance from the camera to the plane of focus in world units."]
    #[doc = "                 Must be positive and larger than the near clipping plane."]
    #[link_name = "\u{1}_ZN8filament6Camera16setFocusDistanceEf"]
    pub fn filament_Camera_setFocusDistance(this: *mut filament_Camera, distance: f32);
}
extern "C" {
    #[doc = "! Returns the focus distance in world units"]
    #[link_name = "\u{1}_ZNK8filament6Camera16getFocusDistanceEv"]
    pub fn filament_Camera_getFocusDistance(this: *const filament_Camera) -> f32;
}
extern "C" {
    #[doc = " Returns the inverse of a projection matrix."]
    #[doc = ""]
    #[doc = " \\param p the projection matrix to inverse"]
    #[doc = " \\returns the inverse of the projection matrix \\p p"]
    #[doc = ""]
    #[doc = " \\warning the projection matrix to invert must have one of the form below:"]
    #[doc = " - perspective projection"]
    #[doc = ""]
    #[doc = "      \\f$"]
    #[doc = "      \\left("]
    #[doc = "      \\begin{array}{cccc}"]
    #[doc = "      a & 0 & tx & 0 \\\\"]
    #[doc = "      0 & b & ty & 0 \\\\"]
    #[doc = "      0 & 0 & tz & c \\\\"]
    #[doc = "      0 & 0 & -1 & 0 \\\\"]
    #[doc = "      \\end{array}"]
    #[doc = "      \\right)"]
    #[doc = "      \\f$"]
    #[doc = ""]
    #[doc = " - orthographic projection"]
    #[doc = ""]
    #[doc = "      \\f$"]
    #[doc = "      \\left("]
    #[doc = "      \\begin{array}{cccc}"]
    #[doc = "      a & 0 & 0 & tx \\\\"]
    #[doc = "      0 & b & 0 & ty \\\\"]
    #[doc = "      0 & 0 & c & tz \\\\"]
    #[doc = "      0 & 0 & 0 & 1  \\\\"]
    #[doc = "      \\end{array}"]
    #[doc = "      \\right)"]
    #[doc = "      \\f$"]
    #[link_name = "\u{1}_ZN8filament6Camera17inverseProjectionERKNS_4math7details6TMat44IdEE"]
    pub fn filament_Camera_inverseProjection(p: *const filament_math_mat4) -> filament_math_mat4;
}
extern "C" {
    #[doc = " Returns the inverse of a projection matrix."]
    #[doc = " @see inverseProjection(const math::mat4&)"]
    #[link_name = "\u{1}_ZN8filament6Camera17inverseProjectionERKNS_4math7details6TMat44IfEE"]
    pub fn filament_Camera_inverseProjection1(p: *const filament_math_mat4f)
        -> filament_math_mat4f;
}
extern "C" {
    #[doc = " Helper to compute the effective focal length taking into account the focus distance"]
    #[doc = ""]
    #[doc = " @param focalLength       focal length in any unit (e.g. [m] or [mm])"]
    #[doc = " @param focusDistance     focus distance in same unit as focalLength"]
    #[doc = " @return                  the effective focal length in same unit as focalLength"]
    #[link_name = "\u{1}_ZN8filament6Camera27computeEffectiveFocalLengthEdd"]
    pub fn filament_Camera_computeEffectiveFocalLength(focalLength: f64, focusDistance: f64)
        -> f64;
}
extern "C" {
    #[doc = " Helper to compute the effective field-of-view taking into account the focus distance"]
    #[doc = ""]
    #[doc = " @param fovInDegrees      full field of view in degrees"]
    #[doc = " @param focusDistance     focus distance in meters [m]"]
    #[doc = " @return                  effective full field of view in degrees"]
    #[link_name = "\u{1}_ZN8filament6Camera19computeEffectiveFovEdd"]
    pub fn filament_Camera_computeEffectiveFov(fovInDegrees: f64, focusDistance: f64) -> f64;
}
impl filament_Camera {
    #[inline]
    pub unsafe fn setProjection(
        &mut self,
        projection: filament_Camera_Projection,
        left: f64,
        right: f64,
        bottom: f64,
        top: f64,
        near: f64,
        far: f64,
    ) {
        filament_Camera_setProjection(self, projection, left, right, bottom, top, near, far)
    }
    #[inline]
    pub unsafe fn setProjection1(
        &mut self,
        fovInDegrees: f64,
        aspect: f64,
        near: f64,
        far: f64,
        direction: filament_Camera_Fov,
    ) {
        filament_Camera_setProjection1(self, fovInDegrees, aspect, near, far, direction)
    }
    #[inline]
    pub unsafe fn setLensProjection(
        &mut self,
        focalLengthInMillimeters: f64,
        aspect: f64,
        near: f64,
        far: f64,
    ) {
        filament_Camera_setLensProjection(self, focalLengthInMillimeters, aspect, near, far)
    }
    #[inline]
    pub unsafe fn setCustomProjection(
        &mut self,
        projection: *const filament_math_mat4,
        near: f64,
        far: f64,
    ) {
        filament_Camera_setCustomProjection(self, projection, near, far)
    }
    #[inline]
    pub unsafe fn setCustomProjection1(
        &mut self,
        projection: *const filament_math_mat4,
        projectionForCulling: *const filament_math_mat4,
        near: f64,
        far: f64,
    ) {
        filament_Camera_setCustomProjection1(self, projection, projectionForCulling, near, far)
    }
    #[inline]
    pub unsafe fn setScaling(&mut self, scaling: filament_math_double2) {
        filament_Camera_setScaling(self, scaling)
    }
    #[inline]
    pub unsafe fn setShift(&mut self, shift: filament_math_double2) {
        filament_Camera_setShift(self, shift)
    }
    #[inline]
    pub unsafe fn getScaling(&self) -> filament_math_double4 {
        filament_Camera_getScaling(self)
    }
    #[inline]
    pub unsafe fn getShift(&self) -> filament_math_double2 {
        filament_Camera_getShift(self)
    }
    #[inline]
    pub unsafe fn getProjectionMatrix(&self) -> filament_math_mat4 {
        filament_Camera_getProjectionMatrix(self)
    }
    #[inline]
    pub unsafe fn getCullingProjectionMatrix(&self) -> filament_math_mat4 {
        filament_Camera_getCullingProjectionMatrix(self)
    }
    #[inline]
    pub unsafe fn getNear(&self) -> f32 {
        filament_Camera_getNear(self)
    }
    #[inline]
    pub unsafe fn getCullingFar(&self) -> f32 {
        filament_Camera_getCullingFar(self)
    }
    #[inline]
    pub unsafe fn setModelMatrix(&mut self, view: *const filament_math_mat4) {
        filament_Camera_setModelMatrix(self, view)
    }
    #[inline]
    pub unsafe fn setModelMatrix1(&mut self, view: *const filament_math_mat4f) {
        filament_Camera_setModelMatrix1(self, view)
    }
    #[inline]
    pub unsafe fn lookAt(
        &mut self,
        eye: *const filament_math_float3,
        center: *const filament_math_float3,
        up: *const filament_math_float3,
    ) {
        filament_Camera_lookAt(self, eye, center, up)
    }
    #[inline]
    pub unsafe fn lookAt1(
        &mut self,
        eye: *const filament_math_float3,
        center: *const filament_math_float3,
    ) {
        filament_Camera_lookAt1(self, eye, center)
    }
    #[inline]
    pub unsafe fn getModelMatrix(&self) -> filament_math_mat4 {
        filament_Camera_getModelMatrix(self)
    }
    #[inline]
    pub unsafe fn getViewMatrix(&self) -> filament_math_mat4 {
        filament_Camera_getViewMatrix(self)
    }
    #[inline]
    pub unsafe fn getPosition(&self) -> filament_math_float3 {
        filament_Camera_getPosition(self)
    }
    #[inline]
    pub unsafe fn getLeftVector(&self) -> filament_math_float3 {
        filament_Camera_getLeftVector(self)
    }
    #[inline]
    pub unsafe fn getUpVector(&self) -> filament_math_float3 {
        filament_Camera_getUpVector(self)
    }
    #[inline]
    pub unsafe fn getForwardVector(&self) -> filament_math_float3 {
        filament_Camera_getForwardVector(self)
    }
    #[inline]
    pub unsafe fn getFieldOfViewInDegrees(&self, direction: filament_Camera_Fov) -> f32 {
        filament_Camera_getFieldOfViewInDegrees(self, direction)
    }
    #[inline]
    pub unsafe fn getFrustum(&self) -> filament_Frustum {
        filament_Camera_getFrustum(self)
    }
    #[inline]
    pub unsafe fn getEntity(&self) -> utils_Entity {
        filament_Camera_getEntity(self)
    }
    #[inline]
    pub unsafe fn setExposure(&mut self, aperture: f32, shutterSpeed: f32, sensitivity: f32) {
        filament_Camera_setExposure(self, aperture, shutterSpeed, sensitivity)
    }
    #[inline]
    pub unsafe fn getAperture(&self) -> f32 {
        filament_Camera_getAperture(self)
    }
    #[inline]
    pub unsafe fn getShutterSpeed(&self) -> f32 {
        filament_Camera_getShutterSpeed(self)
    }
    #[inline]
    pub unsafe fn getSensitivity(&self) -> f32 {
        filament_Camera_getSensitivity(self)
    }
    #[inline]
    pub unsafe fn getFocalLength(&self) -> f64 {
        filament_Camera_getFocalLength(self)
    }
    #[inline]
    pub unsafe fn setFocusDistance(&mut self, distance: f32) {
        filament_Camera_setFocusDistance(self, distance)
    }
    #[inline]
    pub unsafe fn getFocusDistance(&self) -> f32 {
        filament_Camera_getFocusDistance(self)
    }
    #[inline]
    pub unsafe fn inverseProjection(p: *const filament_math_mat4) -> filament_math_mat4 {
        filament_Camera_inverseProjection(p)
    }
    #[inline]
    pub unsafe fn inverseProjection1(p: *const filament_math_mat4f) -> filament_math_mat4f {
        filament_Camera_inverseProjection1(p)
    }
    #[inline]
    pub unsafe fn computeEffectiveFocalLength(focalLength: f64, focusDistance: f64) -> f64 {
        filament_Camera_computeEffectiveFocalLength(focalLength, focusDistance)
    }
    #[inline]
    pub unsafe fn computeEffectiveFov(fovInDegrees: f64, focusDistance: f64) -> f64 {
        filament_Camera_computeEffectiveFov(fovInDegrees, focusDistance)
    }
}
#[repr(C)]
pub struct filament_ToneMapper__bindgen_vtable(::std::os::raw::c_void);
#[doc = " Interface for tone mapping operators. A tone mapping operator, or tone mapper,"]
#[doc = " is responsible for compressing the dynamic range of the rendered scene to a"]
#[doc = " dynamic range suitable for display."]
#[doc = ""]
#[doc = " In Filament, tone mapping is a color grading step. ToneMapper instances are"]
#[doc = " created and passed to the ColorGrading::Builder to produce a 3D LUT that will"]
#[doc = " be used during post-processing to prepare the final color buffer for display."]
#[doc = ""]
#[doc = " Filament provides several default tone mapping operators that fall into three"]
#[doc = " categories:"]
#[doc = ""]
#[doc = " - Configurable tone mapping operators"]
#[doc = "   - GenericToneMapper"]
#[doc = " - Fixed-aesthetic tone mapping operators"]
#[doc = "   - ACESToneMapper"]
#[doc = "   - ACESLegacyToneMapper"]
#[doc = "   - FilmicToneMapper"]
#[doc = " - Debug/validation tone mapping operators"]
#[doc = "   - LinearToneMapper"]
#[doc = "   - DisplayRangeToneMapper"]
#[doc = ""]
#[doc = " You can create custom tone mapping operators by subclassing ToneMapper."]
#[repr(C)]
#[derive(Debug)]
pub struct filament_ToneMapper {
    pub vtable_: *const filament_ToneMapper__bindgen_vtable,
}
#[test]
fn bindgen_test_layout_filament_ToneMapper() {
    assert_eq!(
        ::std::mem::size_of::<filament_ToneMapper>(),
        8usize,
        concat!("Size of: ", stringify!(filament_ToneMapper))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_ToneMapper>(),
        8usize,
        concat!("Alignment of ", stringify!(filament_ToneMapper))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament10ToneMapperC2Ev"]
    pub fn filament_ToneMapper_ToneMapper(this: *mut filament_ToneMapper);
}
impl Default for filament_ToneMapper {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl filament_ToneMapper {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_ToneMapper_ToneMapper(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament10ToneMapperD1Ev"]
    pub fn filament_ToneMapper_ToneMapper_destructor(this: *mut filament_ToneMapper);
}
#[doc = " ColorGrading is used to transform (either to modify or correct) the colors of the HDR buffer"]
#[doc = " rendered by Filament. Color grading transforms are applied after lighting, and after any lens"]
#[doc = " effects (bloom for instance), and include tone mapping."]
#[doc = ""]
#[doc = " Creation, usage and destruction"]
#[doc = " ==============================="]
#[doc = ""]
#[doc = " A ColorGrading object is created using the ColorGrading::Builder and destroyed by calling"]
#[doc = " Engine::destroy(const ColorGrading*). A ColorGrading object is meant to be set on a View."]
#[doc = ""]
#[doc = " ~~~~~~~~~~~{.cpp}"]
#[doc = "  filament::Engine* engine = filament::Engine::create();"]
#[doc = ""]
#[doc = "  filament::ColorGrading* colorGrading = filament::ColorGrading::Builder()"]
#[doc = "              .toneMapping(filament::ColorGrading::ToneMapping::ACES)"]
#[doc = "              .build(*engine);"]
#[doc = ""]
#[doc = "  myView->setColorGrading(colorGrading);"]
#[doc = ""]
#[doc = "  engine->destroy(colorGrading);"]
#[doc = " ~~~~~~~~~~~"]
#[doc = ""]
#[doc = " Performance"]
#[doc = " ==========="]
#[doc = ""]
#[doc = " Creating a new ColorGrading object may be more expensive than other Filament objects as a"]
#[doc = " 3D LUT may need to be generated. The generation of a 3D LUT, if necessary, may happen on"]
#[doc = " the CPU."]
#[doc = ""]
#[doc = " Ordering"]
#[doc = " ========"]
#[doc = ""]
#[doc = " The various transforms held by ColorGrading are applied in the following order:"]
#[doc = " - Exposure"]
#[doc = " - Night adaptation"]
#[doc = " - White balance"]
#[doc = " - Channel mixer"]
#[doc = " - Shadows/mid-tones/highlights"]
#[doc = " - Slope/offset/power (CDL)"]
#[doc = " - Contrast"]
#[doc = " - Vibrance"]
#[doc = " - Saturation"]
#[doc = " - Curves"]
#[doc = " - Tone mapping"]
#[doc = " - Luminance scaling"]
#[doc = " - Gamut mapping"]
#[doc = ""]
#[doc = " Defaults"]
#[doc = " ========"]
#[doc = ""]
#[doc = " Here are the default color grading options:"]
#[doc = " - Exposure: 0.0"]
#[doc = " - Night adaptation: 0.0"]
#[doc = " - White balance: temperature 0, and tint 0"]
#[doc = " - Channel mixer: red {1,0,0}, green {0,1,0}, blue {0,0,1}"]
#[doc = " - Shadows/mid-tones/highlights: shadows {1,1,1,0}, mid-tones {1,1,1,0}, highlights {1,1,1,0},"]
#[doc = "   ranges {0,0.333,0.550,1}"]
#[doc = " - Slope/offset/power: slope 1.0, offset 0.0, and power 1.0"]
#[doc = " - Contrast: 1.0"]
#[doc = " - Vibrance: 1.0"]
#[doc = " - Saturation: 1.0"]
#[doc = " - Curves: gamma {1,1,1}, midPoint {1,1,1}, and scale {1,1,1}"]
#[doc = " - Tone mapping: ACESLegacyToneMapper"]
#[doc = " - Luminance scaling: false"]
#[doc = " - Gamut mapping: false"]
#[doc = ""]
#[doc = " @see View"]
#[repr(C)]
#[derive(Debug, Default)]
pub struct filament_ColorGrading {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct filament_ColorGrading_BuilderDetails {
    _unused: [u8; 0],
}
pub const filament_ColorGrading_QualityLevel_LOW: filament_ColorGrading_QualityLevel = 0;
pub const filament_ColorGrading_QualityLevel_MEDIUM: filament_ColorGrading_QualityLevel = 1;
pub const filament_ColorGrading_QualityLevel_HIGH: filament_ColorGrading_QualityLevel = 2;
pub const filament_ColorGrading_QualityLevel_ULTRA: filament_ColorGrading_QualityLevel = 3;
pub type filament_ColorGrading_QualityLevel = u8;
#[doc = "!< 10 bits per component"]
pub const filament_ColorGrading_LutFormat_INTEGER: filament_ColorGrading_LutFormat = 0;
#[doc = "!< 16 bits per component (10 bits mantissa precision)"]
pub const filament_ColorGrading_LutFormat_FLOAT: filament_ColorGrading_LutFormat = 1;
pub type filament_ColorGrading_LutFormat = u8;
#[doc = "!< Linear tone mapping (i.e. no tone mapping)"]
pub const filament_ColorGrading_ToneMapping_LINEAR: filament_ColorGrading_ToneMapping = 0;
#[doc = "!< ACES tone mapping, with a brightness modifier to match Filament's legacy tone mapper"]
pub const filament_ColorGrading_ToneMapping_ACES_LEGACY: filament_ColorGrading_ToneMapping = 1;
#[doc = "!< ACES tone mapping"]
pub const filament_ColorGrading_ToneMapping_ACES: filament_ColorGrading_ToneMapping = 2;
#[doc = "!< Filmic tone mapping, modelled after ACES but applied in sRGB space"]
pub const filament_ColorGrading_ToneMapping_FILMIC: filament_ColorGrading_ToneMapping = 3;
#[doc = "!< Tone mapping used to validate/debug scene exposure"]
pub const filament_ColorGrading_ToneMapping_DISPLAY_RANGE: filament_ColorGrading_ToneMapping = 4;
#[doc = " List of available tone-mapping operators."]
#[doc = ""]
#[doc = " @deprecated Use Builder::toneMapper(ToneMapper*) instead"]
pub type filament_ColorGrading_ToneMapping = u8;
#[doc = "! Use Builder to construct a ColorGrading object instance"]
#[repr(C)]
#[derive(Debug)]
pub struct filament_ColorGrading_Builder {
    pub _base: filament_BuilderBase<filament_ColorGrading_BuilderDetails>,
}
#[test]
fn bindgen_test_layout_filament_ColorGrading_Builder() {
    assert_eq!(
        ::std::mem::size_of::<filament_ColorGrading_Builder>(),
        8usize,
        concat!("Size of: ", stringify!(filament_ColorGrading_Builder))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_ColorGrading_Builder>(),
        8usize,
        concat!("Alignment of ", stringify!(filament_ColorGrading_Builder))
    );
}
extern "C" {
    #[doc = " Sets the quality level of the color grading. When color grading is implemented using"]
    #[doc = " a 3D LUT, the quality level may impact the resolution and bit depth of the backing"]
    #[doc = " 3D texture. For instance, a low quality level will use a 16x16x16 10 bit LUT, a medium"]
    #[doc = " quality level will use a 32x32x32 10 bit LUT, a high quality will use a 32x32x32 16 bit"]
    #[doc = " LUT, and a ultra quality will use a 64x64x64 16 bit LUT."]
    #[doc = " This overrides the values set by format() and dimensions()."]
    #[doc = ""]
    #[doc = " The default quality is medium."]
    #[doc = ""]
    #[doc = " @param qualityLevel The desired quality of the color grading process"]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls"]
    #[link_name = "\u{1}_ZN8filament12ColorGrading7Builder7qualityENS0_12QualityLevelE"]
    pub fn filament_ColorGrading_Builder_quality(
        this: *mut filament_ColorGrading_Builder,
        qualityLevel: filament_ColorGrading_QualityLevel,
    ) -> *mut filament_ColorGrading_Builder;
}
extern "C" {
    #[doc = " When color grading is implemented using a 3D LUT, this sets the texture format of"]
    #[doc = " of the LUT. This overrides the value set by quality()."]
    #[doc = ""]
    #[doc = " The default is INTEGER"]
    #[doc = ""]
    #[doc = " @param format The desired format of the 3D LUT."]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls"]
    #[link_name = "\u{1}_ZN8filament12ColorGrading7Builder6formatENS0_9LutFormatE"]
    pub fn filament_ColorGrading_Builder_format(
        this: *mut filament_ColorGrading_Builder,
        format: filament_ColorGrading_LutFormat,
    ) -> *mut filament_ColorGrading_Builder;
}
extern "C" {
    #[doc = " When color grading is implemented using a 3D LUT, this sets the dimension of the LUT."]
    #[doc = " This overrides the value set by quality()."]
    #[doc = ""]
    #[doc = " The default is 32"]
    #[doc = ""]
    #[doc = " @param dim The desired dimension of the LUT. Between 16 and 64."]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls"]
    #[link_name = "\u{1}_ZN8filament12ColorGrading7Builder10dimensionsEh"]
    pub fn filament_ColorGrading_Builder_dimensions(
        this: *mut filament_ColorGrading_Builder,
        dim: u8,
    ) -> *mut filament_ColorGrading_Builder;
}
extern "C" {
    #[doc = " Selects the tone mapping operator to apply to the HDR color buffer as the last"]
    #[doc = " operation of the color grading post-processing step."]
    #[doc = ""]
    #[doc = " The default tone mapping operator is ACESLegacyToneMapper."]
    #[doc = ""]
    #[doc = " The specified tone mapper must have a lifecycle that exceeds the lifetime of"]
    #[doc = " this builder. Since the build(Engine&) method is synchronous, it is safe to"]
    #[doc = " delete the tone mapper object after that finishes executing."]
    #[doc = ""]
    #[doc = " @param toneMapper The tone mapping operator to apply to the HDR color buffer"]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls"]
    #[link_name = "\u{1}_ZN8filament12ColorGrading7Builder10toneMapperEPKNS_10ToneMapperE"]
    pub fn filament_ColorGrading_Builder_toneMapper(
        this: *mut filament_ColorGrading_Builder,
        toneMapper: *const filament_ToneMapper,
    ) -> *mut filament_ColorGrading_Builder;
}
extern "C" {
    #[doc = " Selects the tone mapping operator to apply to the HDR color buffer as the last"]
    #[doc = " operation of the color grading post-processing step."]
    #[doc = ""]
    #[doc = " The default tone mapping operator is ACES_LEGACY."]
    #[doc = ""]
    #[doc = " @param toneMapping The tone mapping operator to apply to the HDR color buffer"]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls"]
    #[doc = ""]
    #[doc = " @deprecated Use toneMapper(ToneMapper*) instead"]
    #[link_name = "\u{1}_ZN8filament12ColorGrading7Builder11toneMappingENS0_11ToneMappingE"]
    pub fn filament_ColorGrading_Builder_toneMapping(
        this: *mut filament_ColorGrading_Builder,
        toneMapping: filament_ColorGrading_ToneMapping,
    ) -> *mut filament_ColorGrading_Builder;
}
extern "C" {
    #[doc = " Enables or disables the luminance scaling component (LICH) from the exposure value"]
    #[doc = " invariant luminance system (EVILS). When this setting is enabled, pixels with high"]
    #[doc = " chromatic values will roll-off to white to offer a more natural rendering. This step"]
    #[doc = " also helps avoid undesirable hue skews caused by out of gamut colors clipped"]
    #[doc = " to the destination color gamut."]
    #[doc = ""]
    #[doc = " When luminance scaling is enabled, tone mapping is performed on the luminance of each"]
    #[doc = " pixel instead of per-channel."]
    #[doc = ""]
    #[doc = " @param luminanceScaling Enables or disables luminance scaling post-tone mapping"]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls"]
    #[link_name = "\u{1}_ZN8filament12ColorGrading7Builder16luminanceScalingEb"]
    pub fn filament_ColorGrading_Builder_luminanceScaling(
        this: *mut filament_ColorGrading_Builder,
        luminanceScaling: bool,
    ) -> *mut filament_ColorGrading_Builder;
}
extern "C" {
    #[doc = " Enables or disables gamut mapping to the destination color space's gamut. When gamut"]
    #[doc = " mapping is turned off, out-of-gamut colors are clipped to the destination's gamut,"]
    #[doc = " which may produce hue skews (blue skewing to purple, green to yellow, etc.). When"]
    #[doc = " gamut mapping is enabled, out-of-gamut colors are brought back in gamut by trying to"]
    #[doc = " preserve the perceived chroma and lightness of the original values."]
    #[doc = ""]
    #[doc = " @param gamutMapping Enables or disables gamut mapping"]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls"]
    #[link_name = "\u{1}_ZN8filament12ColorGrading7Builder12gamutMappingEb"]
    pub fn filament_ColorGrading_Builder_gamutMapping(
        this: *mut filament_ColorGrading_Builder,
        gamutMapping: bool,
    ) -> *mut filament_ColorGrading_Builder;
}
extern "C" {
    #[doc = " Adjusts the exposure of this image. The exposure is specified in stops:"]
    #[doc = " each stop brightens (positive values) or darkens (negative values) the image by"]
    #[doc = " a factor of 2. This means that an exposure of 3 will brighten the image 8 times"]
    #[doc = " more than an exposure of 0 (2^3 = 8 and 2^0 = 1). Contrary to the camera's exposure,"]
    #[doc = " this setting is applied after all post-processing (bloom, etc.) are applied."]
    #[doc = ""]
    #[doc = " @param exposure Value in EV stops. Can be negative, 0, or positive."]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls"]
    #[link_name = "\u{1}_ZN8filament12ColorGrading7Builder8exposureEf"]
    pub fn filament_ColorGrading_Builder_exposure(
        this: *mut filament_ColorGrading_Builder,
        exposure: f32,
    ) -> *mut filament_ColorGrading_Builder;
}
extern "C" {
    #[doc = " Controls the amount of night adaptation to replicate a more natural representation of"]
    #[doc = " low-light conditions as perceived by the human vision system. In low-light conditions,"]
    #[doc = " peak luminance sensitivity of the eye shifts toward the blue end of the color spectrum:"]
    #[doc = " darker tones appear brighter, reducing contrast, and colors are blue shifted (the darker"]
    #[doc = " the more intense the effect)."]
    #[doc = ""]
    #[doc = " @param adaptation Amount of adaptation, between 0 (no adaptation) and 1 (full adaptation)."]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls"]
    #[link_name = "\u{1}_ZN8filament12ColorGrading7Builder15nightAdaptationEf"]
    pub fn filament_ColorGrading_Builder_nightAdaptation(
        this: *mut filament_ColorGrading_Builder,
        adaptation: f32,
    ) -> *mut filament_ColorGrading_Builder;
}
extern "C" {
    #[doc = " Adjusts the while balance of the image. This can be used to remove color casts"]
    #[doc = " and correct the appearance of the white point in the scene, or to alter the"]
    #[doc = " overall chromaticity of the image for artistic reasons (to make the image appear"]
    #[doc = " cooler or warmer for instance)."]
    #[doc = ""]
    #[doc = " The while balance adjustment is defined with two values:"]
    #[doc = " - Temperature, to modify the color temperature. This value will modify the colors"]
    #[doc = "   on a blue/yellow axis. Lower values apply a cool color temperature, and higher"]
    #[doc = "   values apply a warm color temperature. The lowest value, -1.0f, is equivalent to"]
    #[doc = "   a temperature of 50,000K. The highest value, 1.0f, is equivalent to a temperature"]
    #[doc = "   of 2,000K."]
    #[doc = " - Tint, to modify the colors on a green/magenta axis. The lowest value, -1.0f, will"]
    #[doc = "   apply a strong green cast, and the highest value, 1.0f, will apply a strong magenta"]
    #[doc = "   cast."]
    #[doc = ""]
    #[doc = " Both values are expected to be in the range [-1.0..+1.0]. Values outside of that"]
    #[doc = " range will be clipped to that range."]
    #[doc = ""]
    #[doc = " @param temperature Modification on the blue/yellow axis, as a value between -1.0 and +1.0."]
    #[doc = " @param tint Modification on the green/magenta axis, as a value between -1.0 and +1.0."]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls"]
    #[link_name = "\u{1}_ZN8filament12ColorGrading7Builder12whiteBalanceEff"]
    pub fn filament_ColorGrading_Builder_whiteBalance(
        this: *mut filament_ColorGrading_Builder,
        temperature: f32,
        tint: f32,
    ) -> *mut filament_ColorGrading_Builder;
}
extern "C" {
    #[doc = " The channel mixer adjustment modifies each output color channel using the specified"]
    #[doc = " mix of the source color channels."]
    #[doc = ""]
    #[doc = " By default each output color channel is set to use 100% of the corresponding source"]
    #[doc = " channel and 0% of the other channels. For instance, the output red channel is set to"]
    #[doc = " {1.0, 0.0, 1.0} or 100% red, 0% green and 0% blue."]
    #[doc = ""]
    #[doc = " Each output channel can add or subtract data from the source channel by using values"]
    #[doc = " in the range [-2.0..+2.0]. Values outside of that range will be clipped to that range."]
    #[doc = ""]
    #[doc = " Using the channel mixer adjustment you can for instance create a monochrome output"]
    #[doc = " by setting all 3 output channels to the same mix. For instance: {0.4, 0.4, 0.2} for"]
    #[doc = " all 3 output channels(40% red, 40% green and 20% blue)."]
    #[doc = ""]
    #[doc = " More complex mixes can be used to create more complex effects. For instance, here is"]
    #[doc = " a mix that creates a sepia tone effect:"]
    #[doc = " - outRed   = {0.255, 0.858, 0.087}"]
    #[doc = " - outGreen = {0.213, 0.715, 0.072}"]
    #[doc = " - outBlue  = {0.170, 0.572, 0.058}"]
    #[doc = ""]
    #[doc = " @param outRed The mix of source RGB for the output red channel, between -2.0 and +2.0"]
    #[doc = " @param outGreen The mix of source RGB for the output green channel, between -2.0 and +2.0"]
    #[doc = " @param outBlue The mix of source RGB for the output blue channel, between -2.0 and +2.0"]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls"]
    #[link_name = "\u{1}_ZN8filament12ColorGrading7Builder12channelMixerENS_4math7details5TVec3IfEES5_S5_"]
    pub fn filament_ColorGrading_Builder_channelMixer(
        this: *mut filament_ColorGrading_Builder,
        outRed: filament_math_float3,
        outGreen: filament_math_float3,
        outBlue: filament_math_float3,
    ) -> *mut filament_ColorGrading_Builder;
}
extern "C" {
    #[doc = " Adjusts the colors separately in 3 distinct tonal ranges or zones: shadows, mid-tones,"]
    #[doc = " and highlights."]
    #[doc = ""]
    #[doc = " The tonal zones are by the ranges parameter: the x and y components define the beginning"]
    #[doc = " and end of the transition from shadows to mid-tones, and the z and w components define"]
    #[doc = " the beginning and end of the transition from mid-tones to highlights."]
    #[doc = ""]
    #[doc = " A smooth transition is applied between the zones which means for instance that the"]
    #[doc = " correction color of the shadows range will partially apply to the mid-tones, and the"]
    #[doc = " other way around. This ensure smooth visual transitions in the final image."]
    #[doc = ""]
    #[doc = " Each correction color is defined as a linear RGB color and a weight. The weight is a"]
    #[doc = " value (which may be positive or negative) that is added to the linear RGB color before"]
    #[doc = " mixing. This can be used to darken or brighten the selected tonal range."]
    #[doc = ""]
    #[doc = " Shadows/mid-tones/highlights adjustment are performed linear space."]
    #[doc = ""]
    #[doc = " @param shadows Linear RGB color (.rgb) and weight (.w) to apply to the shadows"]
    #[doc = " @param midtones Linear RGB color (.rgb) and weight (.w) to apply to the mid-tones"]
    #[doc = " @param highlights Linear RGB color (.rgb) and weight (.w) to apply to the highlights"]
    #[doc = " @param ranges Range of the shadows (x and y), and range of the highlights (z and w)"]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls"]
    #[link_name = "\u{1}_ZN8filament12ColorGrading7Builder25shadowsMidtonesHighlightsENS_4math7details5TVec4IfEES5_S5_S5_"]
    pub fn filament_ColorGrading_Builder_shadowsMidtonesHighlights(
        this: *mut filament_ColorGrading_Builder,
        shadows: filament_math_float4,
        midtones: filament_math_float4,
        highlights: filament_math_float4,
        ranges: filament_math_float4,
    ) -> *mut filament_ColorGrading_Builder;
}
extern "C" {
    #[doc = " Applies a slope, offset, and power, as defined by the ASC CDL (American Society of"]
    #[doc = " Cinematographers Color Decision List) to the image. The CDL can be used to adjust the"]
    #[doc = " colors of different tonal ranges in the image."]
    #[doc = ""]
    #[doc = " The ASC CDL is similar to the lift/gamma/gain controls found in many color grading tools."]
    #[doc = " Lift is equivalent to a combination of offset and slope, gain is equivalent to slope,"]
    #[doc = " and gamma is equivalent to power."]
    #[doc = ""]
    #[doc = " The slope and power values must be strictly positive. Values less than or equal to 0 will"]
    #[doc = " be clamped to a small positive value, offset can be any positive or negative value."]
    #[doc = ""]
    #[doc = " Version 1.2 of the ASC CDL adds saturation control, which is here provided as a separate"]
    #[doc = " API. See the saturation() method for more information."]
    #[doc = ""]
    #[doc = " Slope/offset/power adjustments are performed in log space."]
    #[doc = ""]
    #[doc = " @param slope Multiplier of the input color, must be a strictly positive number"]
    #[doc = " @param offset Added to the input color, can be a negative or positive number, including 0"]
    #[doc = " @param power Power exponent of the input color, must be a strictly positive number"]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls"]
    #[link_name = "\u{1}_ZN8filament12ColorGrading7Builder16slopeOffsetPowerENS_4math7details5TVec3IfEES5_S5_"]
    pub fn filament_ColorGrading_Builder_slopeOffsetPower(
        this: *mut filament_ColorGrading_Builder,
        slope: filament_math_float3,
        offset: filament_math_float3,
        power: filament_math_float3,
    ) -> *mut filament_ColorGrading_Builder;
}
extern "C" {
    #[doc = " Adjusts the contrast of the image. Lower values decrease the contrast of the image"]
    #[doc = " (the tonal range is narrowed), and higher values increase the contrast of the image"]
    #[doc = " (the tonal range is widened). A value of 1.0 has no effect."]
    #[doc = ""]
    #[doc = " The contrast is defined as a value in the range [0.0...2.0]. Values outside of that"]
    #[doc = " range will be clipped to that range."]
    #[doc = ""]
    #[doc = " Contrast adjustment is performed in log space."]
    #[doc = ""]
    #[doc = " @param contrast Contrast expansion, between 0.0 and 2.0. 1.0 leaves contrast unaffected"]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls"]
    #[link_name = "\u{1}_ZN8filament12ColorGrading7Builder8contrastEf"]
    pub fn filament_ColorGrading_Builder_contrast(
        this: *mut filament_ColorGrading_Builder,
        contrast: f32,
    ) -> *mut filament_ColorGrading_Builder;
}
extern "C" {
    #[doc = " Adjusts the saturation of the image based on the input color's saturation level."]
    #[doc = " Colors with a high level of saturation are less affected than colors with low saturation"]
    #[doc = " levels."]
    #[doc = ""]
    #[doc = " Lower vibrance values decrease intensity of the colors present in the image, and"]
    #[doc = " higher values increase the intensity of the colors in the image. A value of 1.0 has"]
    #[doc = " no effect."]
    #[doc = ""]
    #[doc = " The vibrance is defined as a value in the range [0.0...2.0]. Values outside of that"]
    #[doc = " range will be clipped to that range."]
    #[doc = ""]
    #[doc = " Vibrance adjustment is performed in linear space."]
    #[doc = ""]
    #[doc = " @param vibrance Vibrance, between 0.0 and 2.0. 1.0 leaves vibrance unaffected"]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls"]
    #[link_name = "\u{1}_ZN8filament12ColorGrading7Builder8vibranceEf"]
    pub fn filament_ColorGrading_Builder_vibrance(
        this: *mut filament_ColorGrading_Builder,
        vibrance: f32,
    ) -> *mut filament_ColorGrading_Builder;
}
extern "C" {
    #[doc = " Adjusts the saturation of the image. Lower values decrease intensity of the colors"]
    #[doc = " present in the image, and higher values increase the intensity of the colors in the"]
    #[doc = " image. A value of 1.0 has no effect."]
    #[doc = ""]
    #[doc = " The saturation is defined as a value in the range [0.0...2.0]. Values outside of that"]
    #[doc = " range will be clipped to that range."]
    #[doc = ""]
    #[doc = " Saturation adjustment is performed in linear space."]
    #[doc = ""]
    #[doc = " @param saturation Saturation, between 0.0 and 2.0. 1.0 leaves saturation unaffected"]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls"]
    #[link_name = "\u{1}_ZN8filament12ColorGrading7Builder10saturationEf"]
    pub fn filament_ColorGrading_Builder_saturation(
        this: *mut filament_ColorGrading_Builder,
        saturation: f32,
    ) -> *mut filament_ColorGrading_Builder;
}
extern "C" {
    #[doc = " Applies a curve to each RGB channel of the image. Each curve is defined by 3 values:"]
    #[doc = " a gamma value applied to the shadows only, a mid-point indicating where shadows stop"]
    #[doc = " and highlights start, and a scale factor for the highlights."]
    #[doc = ""]
    #[doc = " The gamma and mid-point must be strictly positive values. If they are not, they will be"]
    #[doc = " clamped to a small positive value. The scale can be any negative of positive value."]
    #[doc = ""]
    #[doc = " Curves are applied in linear space."]
    #[doc = ""]
    #[doc = " @param shadowGamma Power value to apply to the shadows, must be strictly positive"]
    #[doc = " @param midPoint Mid-point defining where shadows stop and highlights start, must be strictly positive"]
    #[doc = " @param highlightScale Scale factor for the highlights, can be any negative or positive value"]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls"]
    #[link_name = "\u{1}_ZN8filament12ColorGrading7Builder6curvesENS_4math7details5TVec3IfEES5_S5_"]
    pub fn filament_ColorGrading_Builder_curves(
        this: *mut filament_ColorGrading_Builder,
        shadowGamma: filament_math_float3,
        midPoint: filament_math_float3,
        highlightScale: filament_math_float3,
    ) -> *mut filament_ColorGrading_Builder;
}
extern "C" {
    #[doc = " Creates the ColorGrading object and returns a pointer to it."]
    #[doc = ""]
    #[doc = " @param engine Reference to the filament::Engine to associate this ColorGrading with."]
    #[doc = ""]
    #[doc = " @return pointer to the newly created object or nullptr if exceptions are disabled and"]
    #[doc = "         an error occurred."]
    #[link_name = "\u{1}_ZN8filament12ColorGrading7Builder5buildERNS_6EngineE"]
    pub fn filament_ColorGrading_Builder_build(
        this: *mut filament_ColorGrading_Builder,
        engine: *mut filament_Engine,
    ) -> *mut filament_ColorGrading;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament12ColorGrading7BuilderC1Ev"]
    pub fn filament_ColorGrading_Builder_Builder(this: *mut filament_ColorGrading_Builder);
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament12ColorGrading7BuilderC1ERKS1_"]
    pub fn filament_ColorGrading_Builder_Builder1(
        this: *mut filament_ColorGrading_Builder,
        rhs: *const filament_ColorGrading_Builder,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament12ColorGrading7BuilderC1EOS1_"]
    pub fn filament_ColorGrading_Builder_Builder2(
        this: *mut filament_ColorGrading_Builder,
        rhs: *mut filament_ColorGrading_Builder,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament12ColorGrading7BuilderD1Ev"]
    pub fn filament_ColorGrading_Builder_Builder_destructor(
        this: *mut filament_ColorGrading_Builder,
    );
}
impl Default for filament_ColorGrading_Builder {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl filament_ColorGrading_Builder {
    #[inline]
    pub unsafe fn quality(
        &mut self,
        qualityLevel: filament_ColorGrading_QualityLevel,
    ) -> *mut filament_ColorGrading_Builder {
        filament_ColorGrading_Builder_quality(self, qualityLevel)
    }
    #[inline]
    pub unsafe fn format(
        &mut self,
        format: filament_ColorGrading_LutFormat,
    ) -> *mut filament_ColorGrading_Builder {
        filament_ColorGrading_Builder_format(self, format)
    }
    #[inline]
    pub unsafe fn dimensions(&mut self, dim: u8) -> *mut filament_ColorGrading_Builder {
        filament_ColorGrading_Builder_dimensions(self, dim)
    }
    #[inline]
    pub unsafe fn toneMapper(
        &mut self,
        toneMapper: *const filament_ToneMapper,
    ) -> *mut filament_ColorGrading_Builder {
        filament_ColorGrading_Builder_toneMapper(self, toneMapper)
    }
    #[inline]
    pub unsafe fn toneMapping(
        &mut self,
        toneMapping: filament_ColorGrading_ToneMapping,
    ) -> *mut filament_ColorGrading_Builder {
        filament_ColorGrading_Builder_toneMapping(self, toneMapping)
    }
    #[inline]
    pub unsafe fn luminanceScaling(
        &mut self,
        luminanceScaling: bool,
    ) -> *mut filament_ColorGrading_Builder {
        filament_ColorGrading_Builder_luminanceScaling(self, luminanceScaling)
    }
    #[inline]
    pub unsafe fn gamutMapping(
        &mut self,
        gamutMapping: bool,
    ) -> *mut filament_ColorGrading_Builder {
        filament_ColorGrading_Builder_gamutMapping(self, gamutMapping)
    }
    #[inline]
    pub unsafe fn exposure(&mut self, exposure: f32) -> *mut filament_ColorGrading_Builder {
        filament_ColorGrading_Builder_exposure(self, exposure)
    }
    #[inline]
    pub unsafe fn nightAdaptation(
        &mut self,
        adaptation: f32,
    ) -> *mut filament_ColorGrading_Builder {
        filament_ColorGrading_Builder_nightAdaptation(self, adaptation)
    }
    #[inline]
    pub unsafe fn whiteBalance(
        &mut self,
        temperature: f32,
        tint: f32,
    ) -> *mut filament_ColorGrading_Builder {
        filament_ColorGrading_Builder_whiteBalance(self, temperature, tint)
    }
    #[inline]
    pub unsafe fn channelMixer(
        &mut self,
        outRed: filament_math_float3,
        outGreen: filament_math_float3,
        outBlue: filament_math_float3,
    ) -> *mut filament_ColorGrading_Builder {
        filament_ColorGrading_Builder_channelMixer(self, outRed, outGreen, outBlue)
    }
    #[inline]
    pub unsafe fn shadowsMidtonesHighlights(
        &mut self,
        shadows: filament_math_float4,
        midtones: filament_math_float4,
        highlights: filament_math_float4,
        ranges: filament_math_float4,
    ) -> *mut filament_ColorGrading_Builder {
        filament_ColorGrading_Builder_shadowsMidtonesHighlights(
            self, shadows, midtones, highlights, ranges,
        )
    }
    #[inline]
    pub unsafe fn slopeOffsetPower(
        &mut self,
        slope: filament_math_float3,
        offset: filament_math_float3,
        power: filament_math_float3,
    ) -> *mut filament_ColorGrading_Builder {
        filament_ColorGrading_Builder_slopeOffsetPower(self, slope, offset, power)
    }
    #[inline]
    pub unsafe fn contrast(&mut self, contrast: f32) -> *mut filament_ColorGrading_Builder {
        filament_ColorGrading_Builder_contrast(self, contrast)
    }
    #[inline]
    pub unsafe fn vibrance(&mut self, vibrance: f32) -> *mut filament_ColorGrading_Builder {
        filament_ColorGrading_Builder_vibrance(self, vibrance)
    }
    #[inline]
    pub unsafe fn saturation(&mut self, saturation: f32) -> *mut filament_ColorGrading_Builder {
        filament_ColorGrading_Builder_saturation(self, saturation)
    }
    #[inline]
    pub unsafe fn curves(
        &mut self,
        shadowGamma: filament_math_float3,
        midPoint: filament_math_float3,
        highlightScale: filament_math_float3,
    ) -> *mut filament_ColorGrading_Builder {
        filament_ColorGrading_Builder_curves(self, shadowGamma, midPoint, highlightScale)
    }
    #[inline]
    pub unsafe fn build(&mut self, engine: *mut filament_Engine) -> *mut filament_ColorGrading {
        filament_ColorGrading_Builder_build(self, engine)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_ColorGrading_Builder_Builder(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(rhs: *const filament_ColorGrading_Builder) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_ColorGrading_Builder_Builder1(__bindgen_tmp.as_mut_ptr(), rhs);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new2(rhs: *mut filament_ColorGrading_Builder) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_ColorGrading_Builder_Builder2(__bindgen_tmp.as_mut_ptr(), rhs);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        filament_ColorGrading_Builder_Builder_destructor(self)
    }
}
#[test]
fn bindgen_test_layout_filament_ColorGrading() {
    assert_eq!(
        ::std::mem::size_of::<filament_ColorGrading>(),
        1usize,
        concat!("Size of: ", stringify!(filament_ColorGrading))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_ColorGrading>(),
        1usize,
        concat!("Alignment of ", stringify!(filament_ColorGrading))
    );
}
#[doc = "! RGB color in linear space"]
pub type filament_LinearColor = filament_math_float3;
#[doc = "! RGBA color in linear space, with alpha"]
pub type filament_LinearColorA = filament_math_float4;
#[doc = "!< the color is defined in sRGB space"]
pub const filament_RgbType_sRGB: filament_RgbType = 0;
#[doc = "!< the color is defined in linear space"]
pub const filament_RgbType_LINEAR: filament_RgbType = 1;
#[doc = "! types of RGB colors"]
pub type filament_RgbType = u8;
#[doc = " the color is defined in sRGB space and the RGB values"]
#[doc = " have not been premultiplied by the alpha (for instance, a 50%"]
#[doc = " transparent red is <1,0,0,0.5>)"]
pub const filament_RgbaType_sRGB: filament_RgbaType = 0;
#[doc = " the color is defined in linear space and the RGB values"]
#[doc = " have not been premultiplied by the alpha (for instance, a 50%"]
#[doc = " transparent red is <1,0,0,0.5>)"]
pub const filament_RgbaType_LINEAR: filament_RgbaType = 1;
#[doc = " the color is defined in sRGB space and the RGB values"]
#[doc = " have been premultiplied by the alpha (for instance, a 50%"]
#[doc = " transparent red is <0.5,0,0,0.5>)"]
pub const filament_RgbaType_PREMULTIPLIED_sRGB: filament_RgbaType = 2;
#[doc = " the color is defined in linear space and the RGB values"]
#[doc = " have been premultiplied by the alpha (for instance, a 50%"]
#[doc = " transparent red is <0.5,0,0,0.5>)"]
pub const filament_RgbaType_PREMULTIPLIED_LINEAR: filament_RgbaType = 3;
#[doc = "! types of RGBA colors"]
pub type filament_RgbaType = u8;
#[doc = " A registry of runtime properties used exclusively for debugging"]
#[doc = ""]
#[doc = " Filament exposes a few properties that can be queried and set, which control certain debugging"]
#[doc = " features of the engine. These properties can be set at runtime at anytime."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Default)]
pub struct filament_DebugRegistry {
    pub _address: u8,
}
pub const filament_DebugRegistry_Type_BOOL: filament_DebugRegistry_Type = 0;
pub const filament_DebugRegistry_Type_INT: filament_DebugRegistry_Type = 1;
pub const filament_DebugRegistry_Type_FLOAT: filament_DebugRegistry_Type = 2;
pub const filament_DebugRegistry_Type_FLOAT2: filament_DebugRegistry_Type = 3;
pub const filament_DebugRegistry_Type_FLOAT3: filament_DebugRegistry_Type = 4;
pub const filament_DebugRegistry_Type_FLOAT4: filament_DebugRegistry_Type = 5;
#[doc = " Type of a property"]
pub type filament_DebugRegistry_Type = ::std::os::raw::c_uint;
#[doc = " Information about a property"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct filament_DebugRegistry_Property {
    #[doc = "!< property name"]
    pub name: *const ::std::os::raw::c_char,
    #[doc = "!< property type"]
    pub type_: filament_DebugRegistry_Type,
}
#[test]
fn bindgen_test_layout_filament_DebugRegistry_Property() {
    assert_eq!(
        ::std::mem::size_of::<filament_DebugRegistry_Property>(),
        16usize,
        concat!("Size of: ", stringify!(filament_DebugRegistry_Property))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_DebugRegistry_Property>(),
        8usize,
        concat!("Alignment of ", stringify!(filament_DebugRegistry_Property))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_DebugRegistry_Property>())).name as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_DebugRegistry_Property),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_DebugRegistry_Property>())).type_ as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_DebugRegistry_Property),
            "::",
            stringify!(type_)
        )
    );
}
impl Default for filament_DebugRegistry_Property {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @}"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct filament_DebugRegistry_DataSource {
    pub data: *const ::std::os::raw::c_void,
    pub count: size_t,
}
#[test]
fn bindgen_test_layout_filament_DebugRegistry_DataSource() {
    assert_eq!(
        ::std::mem::size_of::<filament_DebugRegistry_DataSource>(),
        16usize,
        concat!("Size of: ", stringify!(filament_DebugRegistry_DataSource))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_DebugRegistry_DataSource>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(filament_DebugRegistry_DataSource)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_DebugRegistry_DataSource>())).data as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_DebugRegistry_DataSource),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_DebugRegistry_DataSource>())).count as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_DebugRegistry_DataSource),
            "::",
            stringify!(count)
        )
    );
}
impl Default for filament_DebugRegistry_DataSource {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_DebugRegistry_FrameHistory {
    pub target: filament_DebugRegistry_FrameHistory_duration_ms,
    pub targetWithHeadroom: filament_DebugRegistry_FrameHistory_duration_ms,
    pub frameTime: filament_DebugRegistry_FrameHistory_duration_ms,
    pub frameTimeDenoised: filament_DebugRegistry_FrameHistory_duration_ms,
    pub scale: f32,
    pub pid_e: f32,
    pub pid_i: f32,
    pub pid_d: f32,
}
pub type filament_DebugRegistry_FrameHistory_duration_ms = f32;
#[test]
fn bindgen_test_layout_filament_DebugRegistry_FrameHistory() {
    assert_eq!(
        ::std::mem::size_of::<filament_DebugRegistry_FrameHistory>(),
        32usize,
        concat!("Size of: ", stringify!(filament_DebugRegistry_FrameHistory))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_DebugRegistry_FrameHistory>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(filament_DebugRegistry_FrameHistory)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_DebugRegistry_FrameHistory>())).target as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_DebugRegistry_FrameHistory),
            "::",
            stringify!(target)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_DebugRegistry_FrameHistory>())).targetWithHeadroom
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_DebugRegistry_FrameHistory),
            "::",
            stringify!(targetWithHeadroom)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_DebugRegistry_FrameHistory>())).frameTime as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_DebugRegistry_FrameHistory),
            "::",
            stringify!(frameTime)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_DebugRegistry_FrameHistory>())).frameTimeDenoised
                as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_DebugRegistry_FrameHistory),
            "::",
            stringify!(frameTimeDenoised)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_DebugRegistry_FrameHistory>())).scale as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_DebugRegistry_FrameHistory),
            "::",
            stringify!(scale)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_DebugRegistry_FrameHistory>())).pid_e as *const _
                as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_DebugRegistry_FrameHistory),
            "::",
            stringify!(pid_e)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_DebugRegistry_FrameHistory>())).pid_i as *const _
                as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_DebugRegistry_FrameHistory),
            "::",
            stringify!(pid_i)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_DebugRegistry_FrameHistory>())).pid_d as *const _
                as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_DebugRegistry_FrameHistory),
            "::",
            stringify!(pid_d)
        )
    );
}
#[test]
fn bindgen_test_layout_filament_DebugRegistry() {
    assert_eq!(
        ::std::mem::size_of::<filament_DebugRegistry>(),
        1usize,
        concat!("Size of: ", stringify!(filament_DebugRegistry))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_DebugRegistry>(),
        1usize,
        concat!("Alignment of ", stringify!(filament_DebugRegistry))
    );
}
extern "C" {
    #[doc = " Queries whether a property exists"]
    #[doc = " @param name The name of the property to query"]
    #[doc = " @return true if the property exists, false otherwise"]
    #[link_name = "\u{1}_ZNK8filament13DebugRegistry11hasPropertyEPKc"]
    pub fn filament_DebugRegistry_hasProperty(
        this: *const filament_DebugRegistry,
        name: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    #[doc = " Queries the address of a property's data from its name"]
    #[doc = " @param name Name of the property we want the data address of"]
    #[doc = " @return Address of the data of the \\p name property"]
    #[doc = " @{"]
    #[link_name = "\u{1}_ZN8filament13DebugRegistry18getPropertyAddressEPKc"]
    pub fn filament_DebugRegistry_getPropertyAddress(
        this: *mut filament_DebugRegistry,
        name: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[doc = " Set the value of a property"]
    #[doc = " @param name Name of the property to set the value of"]
    #[doc = " @param v Value to set"]
    #[doc = " @return true if the operation was successful, false otherwise."]
    #[doc = " @{"]
    #[link_name = "\u{1}_ZN8filament13DebugRegistry11setPropertyEPKcb"]
    pub fn filament_DebugRegistry_setProperty(
        this: *mut filament_DebugRegistry,
        name: *const ::std::os::raw::c_char,
        v: bool,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament13DebugRegistry11setPropertyEPKci"]
    pub fn filament_DebugRegistry_setProperty1(
        this: *mut filament_DebugRegistry,
        name: *const ::std::os::raw::c_char,
        v: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament13DebugRegistry11setPropertyEPKcf"]
    pub fn filament_DebugRegistry_setProperty2(
        this: *mut filament_DebugRegistry,
        name: *const ::std::os::raw::c_char,
        v: f32,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament13DebugRegistry11setPropertyEPKcNS_4math7details5TVec2IfEE"]
    pub fn filament_DebugRegistry_setProperty3(
        this: *mut filament_DebugRegistry,
        name: *const ::std::os::raw::c_char,
        v: filament_math_float2,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament13DebugRegistry11setPropertyEPKcNS_4math7details5TVec3IfEE"]
    pub fn filament_DebugRegistry_setProperty4(
        this: *mut filament_DebugRegistry,
        name: *const ::std::os::raw::c_char,
        v: filament_math_float3,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament13DebugRegistry11setPropertyEPKcNS_4math7details5TVec4IfEE"]
    pub fn filament_DebugRegistry_setProperty5(
        this: *mut filament_DebugRegistry,
        name: *const ::std::os::raw::c_char,
        v: filament_math_float4,
    ) -> bool;
}
extern "C" {
    #[doc = " Get the value of a property"]
    #[doc = " @param name Name of the property to get the value of"]
    #[doc = " @param v A pointer to a variable which will hold the result"]
    #[doc = " @return true if the call was successful and \\p v was updated"]
    #[doc = " @{"]
    #[link_name = "\u{1}_ZNK8filament13DebugRegistry11getPropertyEPKcPb"]
    pub fn filament_DebugRegistry_getProperty(
        this: *const filament_DebugRegistry,
        name: *const ::std::os::raw::c_char,
        v: *mut bool,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZNK8filament13DebugRegistry11getPropertyEPKcPi"]
    pub fn filament_DebugRegistry_getProperty1(
        this: *const filament_DebugRegistry,
        name: *const ::std::os::raw::c_char,
        v: *mut ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZNK8filament13DebugRegistry11getPropertyEPKcPf"]
    pub fn filament_DebugRegistry_getProperty2(
        this: *const filament_DebugRegistry,
        name: *const ::std::os::raw::c_char,
        v: *mut f32,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZNK8filament13DebugRegistry11getPropertyEPKcPNS_4math7details5TVec2IfEE"]
    pub fn filament_DebugRegistry_getProperty3(
        this: *const filament_DebugRegistry,
        name: *const ::std::os::raw::c_char,
        v: *mut filament_math_float2,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZNK8filament13DebugRegistry11getPropertyEPKcPNS_4math7details5TVec3IfEE"]
    pub fn filament_DebugRegistry_getProperty4(
        this: *const filament_DebugRegistry,
        name: *const ::std::os::raw::c_char,
        v: *mut filament_math_float3,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZNK8filament13DebugRegistry11getPropertyEPKcPNS_4math7details5TVec4IfEE"]
    pub fn filament_DebugRegistry_getProperty5(
        this: *const filament_DebugRegistry,
        name: *const ::std::os::raw::c_char,
        v: *mut filament_math_float4,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZNK8filament13DebugRegistry13getDataSourceEPKc"]
    pub fn filament_DebugRegistry_getDataSource(
        this: *const filament_DebugRegistry,
        name: *const ::std::os::raw::c_char,
    ) -> filament_DebugRegistry_DataSource;
}
impl filament_DebugRegistry {
    #[inline]
    pub unsafe fn hasProperty(&self, name: *const ::std::os::raw::c_char) -> bool {
        filament_DebugRegistry_hasProperty(self, name)
    }
    #[inline]
    pub unsafe fn getPropertyAddress(
        &mut self,
        name: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void {
        filament_DebugRegistry_getPropertyAddress(self, name)
    }
    #[inline]
    pub unsafe fn setProperty(&mut self, name: *const ::std::os::raw::c_char, v: bool) -> bool {
        filament_DebugRegistry_setProperty(self, name, v)
    }
    #[inline]
    pub unsafe fn setProperty1(
        &mut self,
        name: *const ::std::os::raw::c_char,
        v: ::std::os::raw::c_int,
    ) -> bool {
        filament_DebugRegistry_setProperty1(self, name, v)
    }
    #[inline]
    pub unsafe fn setProperty2(&mut self, name: *const ::std::os::raw::c_char, v: f32) -> bool {
        filament_DebugRegistry_setProperty2(self, name, v)
    }
    #[inline]
    pub unsafe fn setProperty3(
        &mut self,
        name: *const ::std::os::raw::c_char,
        v: filament_math_float2,
    ) -> bool {
        filament_DebugRegistry_setProperty3(self, name, v)
    }
    #[inline]
    pub unsafe fn setProperty4(
        &mut self,
        name: *const ::std::os::raw::c_char,
        v: filament_math_float3,
    ) -> bool {
        filament_DebugRegistry_setProperty4(self, name, v)
    }
    #[inline]
    pub unsafe fn setProperty5(
        &mut self,
        name: *const ::std::os::raw::c_char,
        v: filament_math_float4,
    ) -> bool {
        filament_DebugRegistry_setProperty5(self, name, v)
    }
    #[inline]
    pub unsafe fn getProperty(&self, name: *const ::std::os::raw::c_char, v: *mut bool) -> bool {
        filament_DebugRegistry_getProperty(self, name, v)
    }
    #[inline]
    pub unsafe fn getProperty1(
        &self,
        name: *const ::std::os::raw::c_char,
        v: *mut ::std::os::raw::c_int,
    ) -> bool {
        filament_DebugRegistry_getProperty1(self, name, v)
    }
    #[inline]
    pub unsafe fn getProperty2(&self, name: *const ::std::os::raw::c_char, v: *mut f32) -> bool {
        filament_DebugRegistry_getProperty2(self, name, v)
    }
    #[inline]
    pub unsafe fn getProperty3(
        &self,
        name: *const ::std::os::raw::c_char,
        v: *mut filament_math_float2,
    ) -> bool {
        filament_DebugRegistry_getProperty3(self, name, v)
    }
    #[inline]
    pub unsafe fn getProperty4(
        &self,
        name: *const ::std::os::raw::c_char,
        v: *mut filament_math_float3,
    ) -> bool {
        filament_DebugRegistry_getProperty4(self, name, v)
    }
    #[inline]
    pub unsafe fn getProperty5(
        &self,
        name: *const ::std::os::raw::c_char,
        v: *mut filament_math_float4,
    ) -> bool {
        filament_DebugRegistry_getProperty5(self, name, v)
    }
    #[inline]
    pub unsafe fn getDataSource(
        &self,
        name: *const ::std::os::raw::c_char,
    ) -> filament_DebugRegistry_DataSource {
        filament_DebugRegistry_getDataSource(self, name)
    }
}
#[doc = " Engine is filament's main entry-point."]
#[doc = ""]
#[doc = " An Engine instance main function is to keep track of all resources created by the user and"]
#[doc = " manage the rendering thread as well as the hardware renderer."]
#[doc = ""]
#[doc = " To use filament, an Engine instance must be created first:"]
#[doc = ""]
#[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
#[doc = " #include <filament/Engine.h>"]
#[doc = " using namespace filament;"]
#[doc = ""]
#[doc = " Engine* engine = Engine::create();"]
#[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
#[doc = ""]
#[doc = " Engine essentially represents (or is associated to) a hardware context"]
#[doc = " (e.g. an OpenGL ES context)."]
#[doc = ""]
#[doc = " Rendering typically happens in an operating system's window (which can be full screen), such"]
#[doc = " window is managed by a filament.Renderer."]
#[doc = ""]
#[doc = " A typical filament render loop looks like this:"]
#[doc = ""]
#[doc = ""]
#[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
#[doc = " #include <filament/Engine.h>"]
#[doc = " #include <filament/Renderer.h>"]
#[doc = " #include <filament/Scene.h>"]
#[doc = " #include <filament/View.h>"]
#[doc = " using namespace filament;"]
#[doc = ""]
#[doc = " Engine* engine       = Engine::create();"]
#[doc = " SwapChain* swapChain = engine->createSwapChain(nativeWindow);"]
#[doc = " Renderer* renderer   = engine->createRenderer();"]
#[doc = " Scene* scene         = engine->createScene();"]
#[doc = " View* view           = engine->createView();"]
#[doc = ""]
#[doc = " view->setScene(scene);"]
#[doc = ""]
#[doc = " do {"]
#[doc = "     // typically we wait for VSYNC and user input events"]
#[doc = "     if (renderer->beginFrame(swapChain)) {"]
#[doc = "         renderer->render(view);"]
#[doc = "         renderer->endFrame();"]
#[doc = "     }"]
#[doc = " } while (!quit);"]
#[doc = ""]
#[doc = " engine->destroy(view);"]
#[doc = " engine->destroy(scene);"]
#[doc = " engine->destroy(renderer);"]
#[doc = " engine->destroy(swapChain);"]
#[doc = " Engine::destroy(&engine); // clears engine*"]
#[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
#[doc = ""]
#[doc = " Resource Tracking"]
#[doc = " ================="]
#[doc = ""]
#[doc = "  Each Engine instance keeps track of all objects created by the user, such as vertex and index"]
#[doc = "  buffers, lights, cameras, etc..."]
#[doc = "  The user is expected to free those resources, however, leaked resources are freed when the"]
#[doc = "  engine instance is destroyed and a warning is emitted in the console."]
#[doc = ""]
#[doc = " Thread safety"]
#[doc = " ============="]
#[doc = ""]
#[doc = " An Engine instance is not thread-safe. The implementation makes no attempt to synchronize"]
#[doc = " calls to an Engine instance methods."]
#[doc = " If multi-threading is needed, synchronization must be external."]
#[doc = ""]
#[doc = " Multi-threading"]
#[doc = " ==============="]
#[doc = ""]
#[doc = " When created, the Engine instance starts a render thread as well as multiple worker threads,"]
#[doc = " these threads have an elevated priority appropriate for rendering, based on the platform's"]
#[doc = " best practices. The number of worker threads depends on the platform and is automatically"]
#[doc = " chosen for best performance."]
#[doc = ""]
#[doc = " On platforms with asymmetric cores (e.g. ARM's Big.Little), Engine makes some educated guesses"]
#[doc = " as to which cores to use for the render thread and worker threads. For example, it'll try to"]
#[doc = " keep an OpenGL ES thread on a Big core."]
#[doc = ""]
#[doc = " Swap Chains"]
#[doc = " ==========="]
#[doc = ""]
#[doc = " A swap chain represents an Operating System's *native* renderable surface. Typically it's a window"]
#[doc = " or a view. Because a SwapChain is initialized from a native object, it is given to filament"]
#[doc = " as a `void*`, which must be of the proper type for each platform filament is running on."]
#[doc = ""]
#[doc = " @see SwapChain"]
#[doc = ""]
#[doc = ""]
#[doc = " @see Renderer"]
#[repr(C)]
#[derive(Debug, Default)]
pub struct filament_Engine {
    pub _address: u8,
}
pub type filament_Engine_Platform = filament_backend_Platform;
#[doc = " Selects which driver a particular Engine should use."]
pub use self::filament_backend_Backend as filament_Engine_Backend;
#[doc = " A callback used with Engine::createAsync() called once the engine is initialized and it is"]
#[doc = " safe to call Engine::getEngine(token). This callback is invoked from an arbitrary worker"]
#[doc = " thread. Engine::getEngine() CANNOT be called from that thread, instead it must be called"]
#[doc = " from the same thread than Engine::createAsync() was called from."]
#[doc = ""]
#[doc = " @param user   User provided parameter given in createAsync()."]
#[doc = ""]
#[doc = " @param token  An opaque token used to call Engine::getEngine()."]
pub type filament_Engine_CreateCallback = ::std::option::Option<
    unsafe extern "C" fn(user: *mut ::std::os::raw::c_void, token: *mut ::std::os::raw::c_void),
>;
#[test]
fn bindgen_test_layout_filament_Engine() {
    assert_eq!(
        ::std::mem::size_of::<filament_Engine>(),
        1usize,
        concat!("Size of: ", stringify!(filament_Engine))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_Engine>(),
        1usize,
        concat!("Alignment of ", stringify!(filament_Engine))
    );
}
extern "C" {
    #[doc = " Creates an instance of Engine"]
    #[doc = ""]
    #[doc = " @param backend           Which driver backend to use."]
    #[doc = ""]
    #[doc = " @param platform          A pointer to an object that implements Platform. If this is"]
    #[doc = "                          provided, then this object is used to create the hardware context"]
    #[doc = "                          and expose platform features to it."]
    #[doc = ""]
    #[doc = "                          If not provided (or nullptr is used), an appropriate Platform"]
    #[doc = "                          is created automatically."]
    #[doc = ""]
    #[doc = "                          All methods of this interface are called from filament's"]
    #[doc = "                          render thread, which is different from the main thread."]
    #[doc = ""]
    #[doc = "                          The lifetime of \\p platform must exceed the lifetime of"]
    #[doc = "                          the Engine object."]
    #[doc = ""]
    #[doc = "  @param sharedGLContext  A platform-dependant OpenGL context used as a shared context"]
    #[doc = "                          when creating filament's internal context."]
    #[doc = "                          Setting this parameter will force filament to use the OpenGL"]
    #[doc = "                          implementation (instead of Vulkan for instance)."]
    #[doc = ""]
    #[doc = ""]
    #[doc = " @return A pointer to the newly created Engine, or nullptr if the Engine couldn't be created."]
    #[doc = ""]
    #[doc = " nullptr if the GPU driver couldn't be initialized, for instance if it doesn't"]
    #[doc = " support the right version of OpenGL or OpenGL ES."]
    #[doc = ""]
    #[doc = " @exception utils::PostConditionPanic can be thrown if there isn't enough memory to"]
    #[doc = " allocate the command buffer. If exceptions are disabled, this condition if fatal and"]
    #[doc = " this function will abort."]
    #[doc = ""]
    #[doc = " \\remark"]
    #[doc = " This method is thread-safe."]
    #[link_name = "\u{1}_ZN8filament6Engine6createENS_7backend7BackendEPNS1_8PlatformEPv"]
    pub fn filament_Engine_create(
        backend: filament_Engine_Backend,
        platform: *mut filament_Engine_Platform,
        sharedGLContext: *mut ::std::os::raw::c_void,
    ) -> *mut filament_Engine;
}
extern "C" {
    #[doc = " Creates an instance of Engine asynchronously"]
    #[doc = ""]
    #[doc = " @param callback          Callback called once the engine is initialized and it is safe to"]
    #[doc = "                          call Engine::getEngine."]
    #[doc = ""]
    #[doc = " @param user              A user provided pointer that is given back to callback unmodified."]
    #[doc = ""]
    #[doc = " @param backend           Which driver backend to use."]
    #[doc = ""]
    #[doc = " @param platform          A pointer to an object that implements Platform. If this is"]
    #[doc = "                          provided, then this object is used to create the hardware context"]
    #[doc = "                          and expose platform features to it."]
    #[doc = ""]
    #[doc = "                          If not provided (or nullptr is used), an appropriate Platform"]
    #[doc = "                          is created automatically."]
    #[doc = ""]
    #[doc = "                          All methods of this interface are called from filament's"]
    #[doc = "                          render thread, which is different from the main thread."]
    #[doc = ""]
    #[doc = "                          The lifetime of \\p platform must exceed the lifetime of"]
    #[doc = "                          the Engine object."]
    #[doc = ""]
    #[doc = "  @param sharedGLContext  A platform-dependant OpenGL context used as a shared context"]
    #[doc = "                          when creating filament's internal context."]
    #[doc = "                          Setting this parameter will force filament to use the OpenGL"]
    #[doc = "                          implementation (instead of Vulkan for instance)."]
    #[link_name = "\u{1}_ZN8filament6Engine11createAsyncEPFvPvS1_ES1_NS_7backend7BackendEPNS4_8PlatformES1_"]
    pub fn filament_Engine_createAsync(
        callback: filament_Engine_CreateCallback,
        user: *mut ::std::os::raw::c_void,
        backend: filament_Engine_Backend,
        platform: *mut filament_Engine_Platform,
        sharedGLContext: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    #[doc = " Retrieve an Engine* from createAsync(). This must be called from the same thread than"]
    #[doc = " Engine::createAsync() was called from."]
    #[doc = ""]
    #[doc = " @param token An opaque token given in the createAsync() callback function."]
    #[doc = ""]
    #[doc = " @return A pointer to the newly created Engine, or nullptr if the Engine couldn't be created."]
    #[doc = ""]
    #[doc = " @exception utils::PostConditionPanic can be thrown if there isn't enough memory to"]
    #[doc = " allocate the command buffer. If exceptions are disabled, this condition if fatal and"]
    #[doc = " this function will abort."]
    #[link_name = "\u{1}_ZN8filament6Engine9getEngineEPv"]
    pub fn filament_Engine_getEngine(token: *mut ::std::os::raw::c_void) -> *mut filament_Engine;
}
extern "C" {
    #[doc = " Destroy the Engine instance and all associated resources."]
    #[doc = ""]
    #[doc = " Engine.destroy() should be called last and after all other resources have been destroyed,"]
    #[doc = " it ensures all filament resources are freed."]
    #[doc = ""]
    #[doc = " Destroy performs the following tasks:"]
    #[doc = " 1. Destroy all internal software and hardware resources."]
    #[doc = " 2. Free all user allocated resources that are not already destroyed and logs a warning."]
    #[doc = "    This indicates a \"leak\" in the user's code."]
    #[doc = " 3. Terminate the rendering engine's thread."]
    #[doc = ""]
    #[doc = " @param engine A pointer to the filament.Engine* to be destroyed."]
    #[doc = "               \\p engine is cleared upon return."]
    #[doc = ""]
    #[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
    #[doc = " #include <filament/Engine.h>"]
    #[doc = " using namespace filament;"]
    #[doc = ""]
    #[doc = " Engine* engine = Engine::create();"]
    #[doc = " Engine::destroy(&engine); // clears engine*"]
    #[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
    #[doc = ""]
    #[doc = " \\remark"]
    #[doc = " This method is thread-safe."]
    #[link_name = "\u{1}_ZN8filament6Engine7destroyEPPS0_"]
    pub fn filament_Engine_destroy(engine: *mut *mut filament_Engine);
}
extern "C" {
    #[doc = " Destroy the Engine instance and all associated resources."]
    #[doc = ""]
    #[doc = " Engine.destroy() should be called last and after all other resources have been destroyed,"]
    #[doc = " it ensures all filament resources are freed."]
    #[doc = ""]
    #[doc = " Destroy performs the following tasks:"]
    #[doc = " 1. Destroy all internal software and hardware resources."]
    #[doc = " 2. Free all user allocated resources that are not already destroyed and logs a warning."]
    #[doc = "    This indicates a \"leak\" in the user's code."]
    #[doc = " 3. Terminate the rendering engine's thread."]
    #[doc = ""]
    #[doc = " @param engine A pointer to the filament.Engine to be destroyed."]
    #[doc = ""]
    #[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
    #[doc = " #include <filament/Engine.h>"]
    #[doc = " using namespace filament;"]
    #[doc = ""]
    #[doc = " Engine* engine = Engine::create();"]
    #[doc = " Engine::destroy(engine);"]
    #[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
    #[doc = ""]
    #[doc = " \\remark"]
    #[doc = " This method is thread-safe."]
    #[link_name = "\u{1}_ZN8filament6Engine7destroyEPS0_"]
    pub fn filament_Engine_destroy1(engine: *mut filament_Engine);
}
extern "C" {
    #[doc = " @return EntityManager used by filament"]
    #[link_name = "\u{1}_ZN8filament6Engine16getEntityManagerEv"]
    pub fn filament_Engine_getEntityManager(this: *mut filament_Engine)
        -> *mut utils_EntityManager;
}
extern "C" {
    #[doc = " @return RenderableManager reference"]
    #[link_name = "\u{1}_ZN8filament6Engine20getRenderableManagerEv"]
    pub fn filament_Engine_getRenderableManager(
        this: *mut filament_Engine,
    ) -> *mut filament_RenderableManager;
}
extern "C" {
    #[doc = " @return LightManager reference"]
    #[link_name = "\u{1}_ZN8filament6Engine15getLightManagerEv"]
    pub fn filament_Engine_getLightManager(
        this: *mut filament_Engine,
    ) -> *mut filament_LightManager;
}
extern "C" {
    #[doc = " @return TransformManager reference"]
    #[link_name = "\u{1}_ZN8filament6Engine19getTransformManagerEv"]
    pub fn filament_Engine_getTransformManager(
        this: *mut filament_Engine,
    ) -> *mut filament_TransformManager;
}
extern "C" {
    #[doc = " Helper to enable accurate translations."]
    #[doc = " If you need this Engine to handle a very large world space, one way to achieve this"]
    #[doc = " automatically is to enable accurate translations in the TransformManager. This helper"]
    #[doc = " provides a convenient way of doing that."]
    #[doc = " This is typically called once just after creating the Engine."]
    #[link_name = "\u{1}_ZN8filament6Engine26enableAccurateTranslationsEv"]
    pub fn filament_Engine_enableAccurateTranslations(this: *mut filament_Engine);
}
extern "C" {
    #[doc = " Creates a SwapChain from the given Operating System's native window handle."]
    #[doc = ""]
    #[doc = " @param nativeWindow An opaque native window handle. e.g.: on Android this is an"]
    #[doc = "                     `ANativeWindow*`."]
    #[doc = " @param flags One or more configuration flags as defined in `SwapChain`."]
    #[doc = ""]
    #[doc = " @return A pointer to the newly created SwapChain or nullptr if it couldn't be created."]
    #[doc = ""]
    #[doc = " @see Renderer.beginFrame()"]
    #[link_name = "\u{1}_ZN8filament6Engine15createSwapChainEPvm"]
    pub fn filament_Engine_createSwapChain(
        this: *mut filament_Engine,
        nativeWindow: *mut ::std::os::raw::c_void,
        flags: u64,
    ) -> *mut filament_SwapChain;
}
extern "C" {
    #[doc = " Creates a headless SwapChain."]
    #[doc = ""]
    #[doc = " @param width    Width of the drawing buffer in pixels."]
    #[doc = " @param height   Height of the drawing buffer in pixels."]
    #[doc = " @param flags     One or more configuration flags as defined in `SwapChain`."]
    #[doc = ""]
    #[doc = " @return A pointer to the newly created SwapChain or nullptr if it couldn't be created."]
    #[doc = ""]
    #[doc = " @see Renderer.beginFrame()"]
    #[link_name = "\u{1}_ZN8filament6Engine15createSwapChainEjjm"]
    pub fn filament_Engine_createSwapChain1(
        this: *mut filament_Engine,
        width: u32,
        height: u32,
        flags: u64,
    ) -> *mut filament_SwapChain;
}
extern "C" {
    #[doc = " Creates a renderer associated to this engine."]
    #[doc = ""]
    #[doc = " A Renderer is intended to map to a *window* on screen."]
    #[doc = ""]
    #[doc = " @return A pointer to the newly created Renderer or nullptr if it couldn't be created."]
    #[link_name = "\u{1}_ZN8filament6Engine14createRendererEv"]
    pub fn filament_Engine_createRenderer(this: *mut filament_Engine) -> *mut filament_Renderer;
}
extern "C" {
    #[doc = " Creates a View."]
    #[doc = ""]
    #[doc = " @return A pointer to the newly created View or nullptr if it couldn't be created."]
    #[link_name = "\u{1}_ZN8filament6Engine10createViewEv"]
    pub fn filament_Engine_createView(this: *mut filament_Engine) -> *mut filament_View;
}
extern "C" {
    #[doc = " Creates a Scene."]
    #[doc = ""]
    #[doc = " @return A pointer to the newly created Scene or nullptr if it couldn't be created."]
    #[link_name = "\u{1}_ZN8filament6Engine11createSceneEv"]
    pub fn filament_Engine_createScene(this: *mut filament_Engine) -> *mut filament_Scene;
}
extern "C" {
    #[doc = " Creates a Camera component."]
    #[doc = ""]
    #[doc = " @param entity Entity to add the camera component to."]
    #[doc = " @return A pointer to the newly created Camera or nullptr if it couldn't be created."]
    #[link_name = "\u{1}_ZN8filament6Engine12createCameraEN5utils6EntityE"]
    pub fn filament_Engine_createCamera(
        this: *mut filament_Engine,
        entity: utils_Entity,
    ) -> *mut filament_Camera;
}
extern "C" {
    #[doc = " Returns the Camera component of the given entity."]
    #[doc = ""]
    #[doc = " @param entity An entity."]
    #[doc = " @return A pointer to the Camera component for this entity or nullptr if the entity didn't"]
    #[doc = "         have a Camera component. The pointer is valid until destroyCameraComponent()"]
    #[doc = "         is called or the entity itself is destroyed."]
    #[link_name = "\u{1}_ZN8filament6Engine18getCameraComponentEN5utils6EntityE"]
    pub fn filament_Engine_getCameraComponent(
        this: *mut filament_Engine,
        entity: utils_Entity,
    ) -> *mut filament_Camera;
}
extern "C" {
    #[doc = " Destroys the Camera component associated with the given entity."]
    #[doc = ""]
    #[doc = " @param entity An entity."]
    #[link_name = "\u{1}_ZN8filament6Engine22destroyCameraComponentEN5utils6EntityE"]
    pub fn filament_Engine_destroyCameraComponent(this: *mut filament_Engine, entity: utils_Entity);
}
extern "C" {
    #[doc = " Creates a Fence."]
    #[doc = ""]
    #[doc = " @return A pointer to the newly created Fence or nullptr if it couldn't be created."]
    #[link_name = "\u{1}_ZN8filament6Engine11createFenceEv"]
    pub fn filament_Engine_createFence(this: *mut filament_Engine) -> *mut filament_Fence;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament6Engine7destroyEPKNS_12BufferObjectE"]
    pub fn filament_Engine_destroy2(
        this: *mut filament_Engine,
        p: *const filament_BufferObject,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament6Engine7destroyEPKNS_12VertexBufferE"]
    pub fn filament_Engine_destroy3(
        this: *mut filament_Engine,
        p: *const filament_VertexBuffer,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament6Engine7destroyEPKNS_5FenceE"]
    pub fn filament_Engine_destroy4(this: *mut filament_Engine, p: *const filament_Fence) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament6Engine7destroyEPKNS_11IndexBufferE"]
    pub fn filament_Engine_destroy5(
        this: *mut filament_Engine,
        p: *const filament_IndexBuffer,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament6Engine7destroyEPKNS_14SkinningBufferE"]
    pub fn filament_Engine_destroy6(
        this: *mut filament_Engine,
        p: *const filament_SkinningBuffer,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament6Engine7destroyEPKNS_17MorphTargetBufferE"]
    pub fn filament_Engine_destroy7(
        this: *mut filament_Engine,
        p: *const filament_MorphTargetBuffer,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament6Engine7destroyEPKNS_13IndirectLightE"]
    pub fn filament_Engine_destroy8(
        this: *mut filament_Engine,
        p: *const filament_IndirectLight,
    ) -> bool;
}
extern "C" {
    #[doc = " Destroys a Material object"]
    #[doc = " @param p the material object to destroy"]
    #[doc = " @attention All MaterialInstance of the specified material must be destroyed before"]
    #[doc = "            destroying it."]
    #[doc = " @exception utils::PreConditionPanic is thrown if some MaterialInstances remain."]
    #[doc = " no-op if exceptions are disabled and some MaterialInstances remain."]
    #[link_name = "\u{1}_ZN8filament6Engine7destroyEPKNS_8MaterialE"]
    pub fn filament_Engine_destroy9(
        this: *mut filament_Engine,
        p: *const filament_Material,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament6Engine7destroyEPKNS_16MaterialInstanceE"]
    pub fn filament_Engine_destroy10(
        this: *mut filament_Engine,
        p: *const filament_MaterialInstance,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament6Engine7destroyEPKNS_8RendererE"]
    pub fn filament_Engine_destroy11(
        this: *mut filament_Engine,
        p: *const filament_Renderer,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament6Engine7destroyEPKNS_5SceneE"]
    pub fn filament_Engine_destroy12(this: *mut filament_Engine, p: *const filament_Scene) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament6Engine7destroyEPKNS_6SkyboxE"]
    pub fn filament_Engine_destroy13(this: *mut filament_Engine, p: *const filament_Skybox)
        -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament6Engine7destroyEPKNS_12ColorGradingE"]
    pub fn filament_Engine_destroy14(
        this: *mut filament_Engine,
        p: *const filament_ColorGrading,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament6Engine7destroyEPKNS_9SwapChainE"]
    pub fn filament_Engine_destroy15(
        this: *mut filament_Engine,
        p: *const filament_SwapChain,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament6Engine7destroyEPKNS_6StreamE"]
    pub fn filament_Engine_destroy16(this: *mut filament_Engine, p: *const filament_Stream)
        -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament6Engine7destroyEPKNS_7TextureE"]
    pub fn filament_Engine_destroy17(
        this: *mut filament_Engine,
        p: *const filament_Texture,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament6Engine7destroyEPKNS_12RenderTargetE"]
    pub fn filament_Engine_destroy18(
        this: *mut filament_Engine,
        p: *const filament_RenderTarget,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament6Engine7destroyEPKNS_4ViewE"]
    pub fn filament_Engine_destroy19(this: *mut filament_Engine, p: *const filament_View) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament6Engine7destroyEN5utils6EntityE"]
    pub fn filament_Engine_destroy20(this: *mut filament_Engine, e: utils_Entity);
}
extern "C" {
    #[doc = " Kicks the hardware thread (e.g. the OpenGL, Vulkan or Metal thread) and blocks until"]
    #[doc = " all commands to this point are executed. Note that does guarantee that the"]
    #[doc = " hardware is actually finished."]
    #[doc = ""]
    #[doc = " <p>This is typically used right after destroying the <code>SwapChain</code>,"]
    #[doc = " in cases where a guarantee about the <code>SwapChain</code> destruction is needed in a"]
    #[doc = " timely fashion, such as when responding to Android's"]
    #[doc = " <code>android.view.SurfaceHolder.Callback.surfaceDestroyed</code></p>"]
    #[link_name = "\u{1}_ZN8filament6Engine12flushAndWaitEv"]
    pub fn filament_Engine_flushAndWait(this: *mut filament_Engine);
}
extern "C" {
    #[doc = " Kicks the hardware thread (e.g. the OpenGL, Vulkan or Metal thread) but does not wait"]
    #[doc = " for commands to be either executed or the hardware finished."]
    #[doc = ""]
    #[doc = " <p>This is typically used after creating a lot of objects to start draining the command"]
    #[doc = " queue which has a limited size.</p>"]
    #[link_name = "\u{1}_ZN8filament6Engine5flushEv"]
    pub fn filament_Engine_flush(this: *mut filament_Engine);
}
extern "C" {
    #[doc = " Drains the user callback message queue and immediately execute all pending callbacks."]
    #[doc = ""]
    #[doc = " <p> Typically this should be called once per frame right after the application's vsync tick,"]
    #[doc = " and typically just before computing parameters (e.g. object positions) for the next frame."]
    #[doc = " This is useful because otherwise callbacks will be executed by filament at a later time,"]
    #[doc = " which may increase latency in certain applications.</p>"]
    #[link_name = "\u{1}_ZN8filament6Engine17pumpMessageQueuesEv"]
    pub fn filament_Engine_pumpMessageQueues(this: *mut filament_Engine);
}
extern "C" {
    #[doc = " Returns the default Material."]
    #[doc = ""]
    #[doc = " The default material is 80% white and uses the Material.Shading.LIT shading."]
    #[doc = ""]
    #[doc = " @return A pointer to the default Material instance (a singleton)."]
    #[link_name = "\u{1}_ZNK8filament6Engine18getDefaultMaterialEv"]
    pub fn filament_Engine_getDefaultMaterial(
        this: *const filament_Engine,
    ) -> *const filament_Material;
}
extern "C" {
    #[doc = " Returns the resolved backend."]
    #[link_name = "\u{1}_ZNK8filament6Engine10getBackendEv"]
    pub fn filament_Engine_getBackend(this: *const filament_Engine) -> filament_Engine_Backend;
}
extern "C" {
    #[doc = " Returns the Platform object that belongs to this Engine."]
    #[doc = ""]
    #[doc = " When Engine::create is called with no platform argument, Filament creates an appropriate"]
    #[doc = " Platform subclass automatically. The specific subclass created depends on the backend and"]
    #[doc = " OS. For example, when the OpenGL backend is used, the Platform object will be a descendant of"]
    #[doc = " OpenGLPlatform."]
    #[doc = ""]
    #[doc = " dynamic_cast should be used to cast the returned Platform object into a specific subclass."]
    #[doc = " Note that RTTI must be available to use dynamic_cast."]
    #[doc = ""]
    #[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
    #[doc = " Platform* platform = engine->getPlatform();"]
    #[doc = " // static_cast also works, but more dangerous."]
    #[doc = " SpecificPlatform* specificPlatform = dynamic_cast<SpecificPlatform*>(platform);"]
    #[doc = " specificPlatform->platformSpecificMethod();"]
    #[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
    #[doc = ""]
    #[doc = " When a custom Platform is passed to Engine::create, Filament will use it instead, and this"]
    #[doc = " method will return it."]
    #[doc = ""]
    #[doc = " @return A pointer to the Platform object that was provided to Engine::create, or the"]
    #[doc = " Filament-created one."]
    #[link_name = "\u{1}_ZNK8filament6Engine11getPlatformEv"]
    pub fn filament_Engine_getPlatform(
        this: *const filament_Engine,
    ) -> *mut filament_Engine_Platform;
}
extern "C" {
    #[doc = " Allocate a small amount of memory directly in the command stream. The allocated memory is"]
    #[doc = " guaranteed to be preserved until the current command buffer is executed"]
    #[doc = ""]
    #[doc = " @param size       size to allocate in bytes. This should be small (e.g. < 1 KB)"]
    #[doc = " @param alignment  alignment requested"]
    #[doc = " @return           a pointer to the allocated buffer or nullptr if no memory was available."]
    #[doc = ""]
    #[doc = " @note there is no need to destroy this buffer, it will be freed automatically when"]
    #[doc = "       the current command buffer is executed."]
    #[link_name = "\u{1}_ZN8filament6Engine11streamAllocEmm"]
    pub fn filament_Engine_streamAlloc(
        this: *mut filament_Engine,
        size: size_t,
        alignment: size_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[doc = " Invokes one iteration of the render loop, used only on single-threaded platforms."]
    #[doc = ""]
    #[doc = " This should be called every time the windowing system needs to paint (e.g. at 60 Hz)."]
    #[link_name = "\u{1}_ZN8filament6Engine7executeEv"]
    pub fn filament_Engine_execute(this: *mut filament_Engine);
}
extern "C" {
    #[doc = " Retrieves the job system that the Engine has ownership over."]
    #[doc = ""]
    #[doc = " @return JobSystem used by filament"]
    #[link_name = "\u{1}_ZN8filament6Engine12getJobSystemEv"]
    pub fn filament_Engine_getJobSystem(this: *mut filament_Engine) -> *mut utils_JobSystem;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament6Engine16getDebugRegistryEv"]
    pub fn filament_Engine_getDebugRegistry(
        this: *mut filament_Engine,
    ) -> *mut filament_DebugRegistry;
}
impl filament_Engine {
    #[inline]
    pub unsafe fn create(
        backend: filament_Engine_Backend,
        platform: *mut filament_Engine_Platform,
        sharedGLContext: *mut ::std::os::raw::c_void,
    ) -> *mut filament_Engine {
        filament_Engine_create(backend, platform, sharedGLContext)
    }
    #[inline]
    pub unsafe fn createAsync(
        callback: filament_Engine_CreateCallback,
        user: *mut ::std::os::raw::c_void,
        backend: filament_Engine_Backend,
        platform: *mut filament_Engine_Platform,
        sharedGLContext: *mut ::std::os::raw::c_void,
    ) {
        filament_Engine_createAsync(callback, user, backend, platform, sharedGLContext)
    }
    #[inline]
    pub unsafe fn getEngine(token: *mut ::std::os::raw::c_void) -> *mut filament_Engine {
        filament_Engine_getEngine(token)
    }
    #[inline]
    pub unsafe fn destroy(engine: *mut *mut filament_Engine) {
        filament_Engine_destroy(engine)
    }
    #[inline]
    pub unsafe fn destroy1(engine: *mut filament_Engine) {
        filament_Engine_destroy1(engine)
    }
    #[inline]
    pub unsafe fn getEntityManager(&mut self) -> *mut utils_EntityManager {
        filament_Engine_getEntityManager(self)
    }
    #[inline]
    pub unsafe fn getRenderableManager(&mut self) -> *mut filament_RenderableManager {
        filament_Engine_getRenderableManager(self)
    }
    #[inline]
    pub unsafe fn getLightManager(&mut self) -> *mut filament_LightManager {
        filament_Engine_getLightManager(self)
    }
    #[inline]
    pub unsafe fn getTransformManager(&mut self) -> *mut filament_TransformManager {
        filament_Engine_getTransformManager(self)
    }
    #[inline]
    pub unsafe fn enableAccurateTranslations(&mut self) {
        filament_Engine_enableAccurateTranslations(self)
    }
    #[inline]
    pub unsafe fn createSwapChain(
        &mut self,
        nativeWindow: *mut ::std::os::raw::c_void,
        flags: u64,
    ) -> *mut filament_SwapChain {
        filament_Engine_createSwapChain(self, nativeWindow, flags)
    }
    #[inline]
    pub unsafe fn createSwapChain1(
        &mut self,
        width: u32,
        height: u32,
        flags: u64,
    ) -> *mut filament_SwapChain {
        filament_Engine_createSwapChain1(self, width, height, flags)
    }
    #[inline]
    pub unsafe fn createRenderer(&mut self) -> *mut filament_Renderer {
        filament_Engine_createRenderer(self)
    }
    #[inline]
    pub unsafe fn createView(&mut self) -> *mut filament_View {
        filament_Engine_createView(self)
    }
    #[inline]
    pub unsafe fn createScene(&mut self) -> *mut filament_Scene {
        filament_Engine_createScene(self)
    }
    #[inline]
    pub unsafe fn createCamera(&mut self, entity: utils_Entity) -> *mut filament_Camera {
        filament_Engine_createCamera(self, entity)
    }
    #[inline]
    pub unsafe fn getCameraComponent(&mut self, entity: utils_Entity) -> *mut filament_Camera {
        filament_Engine_getCameraComponent(self, entity)
    }
    #[inline]
    pub unsafe fn destroyCameraComponent(&mut self, entity: utils_Entity) {
        filament_Engine_destroyCameraComponent(self, entity)
    }
    #[inline]
    pub unsafe fn createFence(&mut self) -> *mut filament_Fence {
        filament_Engine_createFence(self)
    }
    #[inline]
    pub unsafe fn destroy2(&mut self, p: *const filament_BufferObject) -> bool {
        filament_Engine_destroy2(self, p)
    }
    #[inline]
    pub unsafe fn destroy3(&mut self, p: *const filament_VertexBuffer) -> bool {
        filament_Engine_destroy3(self, p)
    }
    #[inline]
    pub unsafe fn destroy4(&mut self, p: *const filament_Fence) -> bool {
        filament_Engine_destroy4(self, p)
    }
    #[inline]
    pub unsafe fn destroy5(&mut self, p: *const filament_IndexBuffer) -> bool {
        filament_Engine_destroy5(self, p)
    }
    #[inline]
    pub unsafe fn destroy6(&mut self, p: *const filament_SkinningBuffer) -> bool {
        filament_Engine_destroy6(self, p)
    }
    #[inline]
    pub unsafe fn destroy7(&mut self, p: *const filament_MorphTargetBuffer) -> bool {
        filament_Engine_destroy7(self, p)
    }
    #[inline]
    pub unsafe fn destroy8(&mut self, p: *const filament_IndirectLight) -> bool {
        filament_Engine_destroy8(self, p)
    }
    #[inline]
    pub unsafe fn destroy9(&mut self, p: *const filament_Material) -> bool {
        filament_Engine_destroy9(self, p)
    }
    #[inline]
    pub unsafe fn destroy10(&mut self, p: *const filament_MaterialInstance) -> bool {
        filament_Engine_destroy10(self, p)
    }
    #[inline]
    pub unsafe fn destroy11(&mut self, p: *const filament_Renderer) -> bool {
        filament_Engine_destroy11(self, p)
    }
    #[inline]
    pub unsafe fn destroy12(&mut self, p: *const filament_Scene) -> bool {
        filament_Engine_destroy12(self, p)
    }
    #[inline]
    pub unsafe fn destroy13(&mut self, p: *const filament_Skybox) -> bool {
        filament_Engine_destroy13(self, p)
    }
    #[inline]
    pub unsafe fn destroy14(&mut self, p: *const filament_ColorGrading) -> bool {
        filament_Engine_destroy14(self, p)
    }
    #[inline]
    pub unsafe fn destroy15(&mut self, p: *const filament_SwapChain) -> bool {
        filament_Engine_destroy15(self, p)
    }
    #[inline]
    pub unsafe fn destroy16(&mut self, p: *const filament_Stream) -> bool {
        filament_Engine_destroy16(self, p)
    }
    #[inline]
    pub unsafe fn destroy17(&mut self, p: *const filament_Texture) -> bool {
        filament_Engine_destroy17(self, p)
    }
    #[inline]
    pub unsafe fn destroy18(&mut self, p: *const filament_RenderTarget) -> bool {
        filament_Engine_destroy18(self, p)
    }
    #[inline]
    pub unsafe fn destroy19(&mut self, p: *const filament_View) -> bool {
        filament_Engine_destroy19(self, p)
    }
    #[inline]
    pub unsafe fn destroy20(&mut self, e: utils_Entity) {
        filament_Engine_destroy20(self, e)
    }
    #[inline]
    pub unsafe fn flushAndWait(&mut self) {
        filament_Engine_flushAndWait(self)
    }
    #[inline]
    pub unsafe fn flush(&mut self) {
        filament_Engine_flush(self)
    }
    #[inline]
    pub unsafe fn pumpMessageQueues(&mut self) {
        filament_Engine_pumpMessageQueues(self)
    }
    #[inline]
    pub unsafe fn getDefaultMaterial(&self) -> *const filament_Material {
        filament_Engine_getDefaultMaterial(self)
    }
    #[inline]
    pub unsafe fn getBackend(&self) -> filament_Engine_Backend {
        filament_Engine_getBackend(self)
    }
    #[inline]
    pub unsafe fn getPlatform(&self) -> *mut filament_Engine_Platform {
        filament_Engine_getPlatform(self)
    }
    #[inline]
    pub unsafe fn streamAlloc(
        &mut self,
        size: size_t,
        alignment: size_t,
    ) -> *mut ::std::os::raw::c_void {
        filament_Engine_streamAlloc(self, size, alignment)
    }
    #[inline]
    pub unsafe fn execute(&mut self) {
        filament_Engine_execute(self)
    }
    #[inline]
    pub unsafe fn getJobSystem(&mut self) -> *mut utils_JobSystem {
        filament_Engine_getJobSystem(self)
    }
    #[inline]
    pub unsafe fn getDebugRegistry(&mut self) -> *mut filament_DebugRegistry {
        filament_Engine_getDebugRegistry(self)
    }
}
#[doc = " Fence is used to synchronize rendering operations together, with the CPU or with compute."]
#[doc = ""]
#[doc = " \\note"]
#[doc = " Currently Fence only provide client-side synchronization."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Default)]
pub struct filament_Fence {
    pub _address: u8,
}
#[doc = " Error codes for Fence::wait()"]
#[doc = " @see Fence, Fence::wait()"]
pub use self::filament_backend_FenceStatus as filament_Fence_FenceStatus;
#[doc = "!< The command stream is flushed"]
pub const filament_Fence_Mode_FLUSH: filament_Fence_Mode = 0;
#[doc = "!< The command stream is not flushed"]
pub const filament_Fence_Mode_DONT_FLUSH: filament_Fence_Mode = 1;
#[doc = " Mode controls the behavior of the command stream when calling wait()"]
#[doc = ""]
#[doc = " @attention"]
#[doc = " It would be unwise to call `wait(..., Mode::DONT_FLUSH)` from the same thread"]
#[doc = " the Fence was created, as it would most certainly create a dead-lock."]
pub type filament_Fence_Mode = u8;
extern "C" {
    #[doc = "! Special \\p timeout value to disable wait()'s timeout."]
    #[link_name = "\u{1}_ZN8filament5Fence19FENCE_WAIT_FOR_EVERE"]
    pub static filament_Fence_FENCE_WAIT_FOR_EVER: u64;
}
#[test]
fn bindgen_test_layout_filament_Fence() {
    assert_eq!(
        ::std::mem::size_of::<filament_Fence>(),
        1usize,
        concat!("Size of: ", stringify!(filament_Fence))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_Fence>(),
        1usize,
        concat!("Alignment of ", stringify!(filament_Fence))
    );
}
extern "C" {
    #[doc = " Client-side wait on the Fence."]
    #[doc = ""]
    #[doc = " Blocks the current thread until the Fence signals."]
    #[doc = ""]
    #[doc = " @param mode      Whether the command stream is flushed before waiting or not."]
    #[doc = " @param timeout   Wait time out. Using a \\p timeout of 0 is a way to query the state of the fence."]
    #[doc = "                  A \\p timeout value of FENCE_WAIT_FOR_EVER is used to disable the timeout."]
    #[doc = " @return          FenceStatus::CONDITION_SATISFIED on success,"]
    #[doc = "                  FenceStatus::TIMEOUT_EXPIRED if the time out expired or"]
    #[doc = "                  FenceStatus::ERROR in other cases."]
    #[doc = " @see #Mode"]
    #[link_name = "\u{1}_ZN8filament5Fence4waitENS0_4ModeEm"]
    pub fn filament_Fence_wait(
        this: *mut filament_Fence,
        mode: filament_Fence_Mode,
        timeout: u64,
    ) -> filament_Fence_FenceStatus;
}
extern "C" {
    #[doc = " Client-side wait on a Fence and destroy the Fence."]
    #[doc = ""]
    #[doc = " @param fence Fence object to wait on."]
    #[doc = ""]
    #[doc = " @param mode  Whether the command stream is flushed before waiting or not."]
    #[doc = ""]
    #[doc = " @return  FenceStatus::CONDITION_SATISFIED on success,"]
    #[doc = "          FenceStatus::ERROR otherwise."]
    #[link_name = "\u{1}_ZN8filament5Fence14waitAndDestroyEPS0_NS0_4ModeE"]
    pub fn filament_Fence_waitAndDestroy(
        fence: *mut filament_Fence,
        mode: filament_Fence_Mode,
    ) -> filament_Fence_FenceStatus;
}
impl filament_Fence {
    #[inline]
    pub unsafe fn wait(
        &mut self,
        mode: filament_Fence_Mode,
        timeout: u64,
    ) -> filament_Fence_FenceStatus {
        filament_Fence_wait(self, mode, timeout)
    }
    #[inline]
    pub unsafe fn waitAndDestroy(
        fence: *mut filament_Fence,
        mode: filament_Fence_Mode,
    ) -> filament_Fence_FenceStatus {
        filament_Fence_waitAndDestroy(fence, mode)
    }
}
#[doc = " A frustum defined by six planes"]
#[repr(C)]
pub struct filament_Frustum {
    pub mPlanes: [filament_math_float4; 6usize],
}
pub const filament_Frustum_Plane_LEFT: filament_Frustum_Plane = 0;
pub const filament_Frustum_Plane_RIGHT: filament_Frustum_Plane = 1;
pub const filament_Frustum_Plane_BOTTOM: filament_Frustum_Plane = 2;
pub const filament_Frustum_Plane_TOP: filament_Frustum_Plane = 3;
pub const filament_Frustum_Plane_FAR: filament_Frustum_Plane = 4;
pub const filament_Frustum_Plane_NEAR: filament_Frustum_Plane = 5;
pub type filament_Frustum_Plane = u8;
#[test]
fn bindgen_test_layout_filament_Frustum() {
    assert_eq!(
        ::std::mem::size_of::<filament_Frustum>(),
        96usize,
        concat!("Size of: ", stringify!(filament_Frustum))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_Frustum>(),
        4usize,
        concat!("Alignment of ", stringify!(filament_Frustum))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<filament_Frustum>())).mPlanes as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_Frustum),
            "::",
            stringify!(mPlanes)
        )
    );
}
extern "C" {
    #[doc = " Sets the frustum from the given projection matrix"]
    #[doc = " @param pv a 4x4 projection matrix"]
    #[link_name = "\u{1}_ZN8filament7Frustum13setProjectionERKNS_4math7details6TMat44IfEE"]
    pub fn filament_Frustum_setProjection(
        this: *mut filament_Frustum,
        pv: *const filament_math_mat4f,
    );
}
extern "C" {
    #[doc = " Returns the plane equation parameters with normalized normals"]
    #[doc = " @param plane Identifier of the plane to retrieve the equation of"]
    #[doc = " @return A plane equation encoded a float4 R such as R.x*x + R.y*y + R.z*z + R.w = 0"]
    #[link_name = "\u{1}_ZNK8filament7Frustum18getNormalizedPlaneENS0_5PlaneE"]
    pub fn filament_Frustum_getNormalizedPlane(
        this: *const filament_Frustum,
        plane: filament_Frustum_Plane,
    ) -> filament_math_float4;
}
extern "C" {
    #[doc = " Returns a copy of all six frustum planes in left, right, bottom, top, far, near order"]
    #[doc = " @param planes six plane equations encoded as in getNormalizedPlane() in"]
    #[doc = "              left, right, bottom, top, far, near order"]
    #[link_name = "\u{1}_ZNK8filament7Frustum19getNormalizedPlanesEPNS_4math7details5TVec4IfEE"]
    pub fn filament_Frustum_getNormalizedPlanes(
        this: *const filament_Frustum,
        planes: *mut filament_math_float4,
    );
}
extern "C" {
    #[doc = " Returns whether a box intersects the frustum (i.e. is visible)"]
    #[doc = " @param box The box to test against the frustum"]
    #[doc = " @return true if the box may intersects the frustum, false otherwise. In some situations"]
    #[doc = " a box that doesn't intersect the frustum might be reported as though it does. However,"]
    #[doc = " a box that does intersect the frustum is always reported correctly (true)."]
    #[link_name = "\u{1}_ZNK8filament7Frustum10intersectsERKNS_3BoxE"]
    pub fn filament_Frustum_intersects(
        this: *const filament_Frustum,
        box_: *const filament_Box,
    ) -> bool;
}
extern "C" {
    #[doc = " Returns whether a sphere intersects the frustum (i.e. is visible)"]
    #[doc = " @param sphere A sphere encoded as a center + radius."]
    #[doc = " @return true if the sphere may intersects the frustum, false otherwise. In some situations"]
    #[doc = " a sphere that doesn't intersect the frustum might be reported as though it does. However,"]
    #[doc = " a sphere that does intersect the frustum is always reported correctly (true)."]
    #[link_name = "\u{1}_ZNK8filament7Frustum10intersectsERKNS_4math7details5TVec4IfEE"]
    pub fn filament_Frustum_intersects1(
        this: *const filament_Frustum,
        sphere: *const filament_math_float4,
    ) -> bool;
}
extern "C" {
    #[doc = " Returns whether the frustum contains a given point."]
    #[doc = " @param p the point to test"]
    #[doc = " @return the maximum signed distance to the frustum. Negative if p is inside."]
    #[link_name = "\u{1}_ZNK8filament7Frustum8containsENS_4math7details5TVec3IfEE"]
    pub fn filament_Frustum_contains(this: *const filament_Frustum, p: filament_math_float3)
        -> f32;
}
extern "C" {
    #[doc = " Creates a frustum from a projection matrix in GL convention"]
    #[doc = " (usually the projection * view matrix)"]
    #[doc = " @param pv a 4x4 projection matrix in GL convention"]
    #[link_name = "\u{1}_ZN8filament7FrustumC1ERKNS_4math7details6TMat44IfEE"]
    pub fn filament_Frustum_Frustum(this: *mut filament_Frustum, pv: *const filament_math_mat4f);
}
extern "C" {
    #[doc = " Creates a frustum from 8 corner coordinates."]
    #[doc = " @param corners the corners of the frustum"]
    #[doc = ""]
    #[doc = " The corners should be specified in this order:"]
    #[doc = " 0. far bottom left"]
    #[doc = " 1. far bottom right"]
    #[doc = " 2. far top left"]
    #[doc = " 3. far top right"]
    #[doc = " 4. near bottom left"]
    #[doc = " 5. near bottom right"]
    #[doc = " 6. near top left"]
    #[doc = " 7. near top right"]
    #[doc = ""]
    #[doc = "     2----3"]
    #[doc = "    /|   /|"]
    #[doc = "   6----7 |"]
    #[doc = "   | 0--|-1      far"]
    #[doc = "   |/   |/       /"]
    #[doc = "   4----5      near"]
    #[doc = ""]
    #[link_name = "\u{1}_ZN8filament7FrustumC1EPKNS_4math7details5TVec3IfEE"]
    pub fn filament_Frustum_Frustum1(
        this: *mut filament_Frustum,
        corners: *const filament_math_float3,
    );
}
impl Default for filament_Frustum {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl filament_Frustum {
    #[inline]
    pub unsafe fn setProjection(&mut self, pv: *const filament_math_mat4f) {
        filament_Frustum_setProjection(self, pv)
    }
    #[inline]
    pub unsafe fn getNormalizedPlane(&self, plane: filament_Frustum_Plane) -> filament_math_float4 {
        filament_Frustum_getNormalizedPlane(self, plane)
    }
    #[inline]
    pub unsafe fn getNormalizedPlanes(&self, planes: *mut filament_math_float4) {
        filament_Frustum_getNormalizedPlanes(self, planes)
    }
    #[inline]
    pub unsafe fn intersects(&self, box_: *const filament_Box) -> bool {
        filament_Frustum_intersects(self, box_)
    }
    #[inline]
    pub unsafe fn intersects1(&self, sphere: *const filament_math_float4) -> bool {
        filament_Frustum_intersects1(self, sphere)
    }
    #[inline]
    pub unsafe fn contains(&self, p: filament_math_float3) -> f32 {
        filament_Frustum_contains(self, p)
    }
    #[inline]
    pub unsafe fn new(pv: *const filament_math_mat4f) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_Frustum_Frustum(__bindgen_tmp.as_mut_ptr(), pv);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(corners: *const filament_math_float3) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_Frustum_Frustum1(__bindgen_tmp.as_mut_ptr(), corners);
        __bindgen_tmp.assume_init()
    }
}
#[doc = " A buffer containing vertex indices into a VertexBuffer. Indices can be 16 or 32 bit."]
#[doc = " The buffer itself is a GPU resource, therefore mutating the data can be relatively slow."]
#[doc = " Typically these buffers are constant."]
#[doc = ""]
#[doc = " It is possible, and even encouraged, to use a single index buffer for several Renderables."]
#[doc = ""]
#[doc = " @see VertexBuffer, RenderableManager"]
#[repr(C)]
#[derive(Debug, Default)]
pub struct filament_IndexBuffer {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct filament_IndexBuffer_BuilderDetails {
    _unused: [u8; 0],
}
#[doc = " A CPU memory-buffer descriptor, typically used to transfer data from the CPU to the GPU."]
#[doc = ""]
#[doc = " A BufferDescriptor owns the memory buffer it references, therefore BufferDescriptor cannot"]
#[doc = " be copied, but can be moved."]
#[doc = ""]
#[doc = " BufferDescriptor releases ownership of the memory-buffer when it's destroyed."]
pub type filament_IndexBuffer_BufferDescriptor = filament_backend_BufferDescriptor;
#[doc = "!< 16-bit indices"]
pub const filament_IndexBuffer_IndexType_USHORT: filament_IndexBuffer_IndexType = 12;
#[doc = "!< 32-bit indices"]
pub const filament_IndexBuffer_IndexType_UINT: filament_IndexBuffer_IndexType = 17;
#[doc = " Type of the index buffer"]
pub type filament_IndexBuffer_IndexType = u8;
#[repr(C)]
#[derive(Debug)]
pub struct filament_IndexBuffer_Builder {
    pub _base: filament_BuilderBase<filament_IndexBuffer_BuilderDetails>,
}
#[test]
fn bindgen_test_layout_filament_IndexBuffer_Builder() {
    assert_eq!(
        ::std::mem::size_of::<filament_IndexBuffer_Builder>(),
        8usize,
        concat!("Size of: ", stringify!(filament_IndexBuffer_Builder))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_IndexBuffer_Builder>(),
        8usize,
        concat!("Alignment of ", stringify!(filament_IndexBuffer_Builder))
    );
}
extern "C" {
    #[doc = " Size of the index buffer in elements."]
    #[doc = " @param indexCount Number of indices the IndexBuffer can hold."]
    #[doc = " @return A reference to this Builder for chaining calls."]
    #[link_name = "\u{1}_ZN8filament11IndexBuffer7Builder10indexCountEj"]
    pub fn filament_IndexBuffer_Builder_indexCount(
        this: *mut filament_IndexBuffer_Builder,
        indexCount: u32,
    ) -> *mut filament_IndexBuffer_Builder;
}
extern "C" {
    #[doc = " Type of the index buffer, 16-bit or 32-bit."]
    #[doc = " @param indexType Type of indices stored in the IndexBuffer."]
    #[doc = " @return A reference to this Builder for chaining calls."]
    #[link_name = "\u{1}_ZN8filament11IndexBuffer7Builder10bufferTypeENS0_9IndexTypeE"]
    pub fn filament_IndexBuffer_Builder_bufferType(
        this: *mut filament_IndexBuffer_Builder,
        indexType: filament_IndexBuffer_IndexType,
    ) -> *mut filament_IndexBuffer_Builder;
}
extern "C" {
    #[doc = " Creates the IndexBuffer object and returns a pointer to it. After creation, the index"]
    #[doc = " buffer is uninitialized. Use IndexBuffer::setBuffer() to initialize the IndexBuffer."]
    #[doc = ""]
    #[doc = " @param engine Reference to the filament::Engine to associate this IndexBuffer with."]
    #[doc = ""]
    #[doc = " @return pointer to the newly created object or nullptr if exceptions are disabled and"]
    #[doc = "         an error occurred."]
    #[doc = ""]
    #[doc = " @exception utils::PostConditionPanic if a runtime error occurred, such as running out of"]
    #[doc = "            memory or other resources."]
    #[doc = " @exception utils::PreConditionPanic if a parameter to a builder function was invalid."]
    #[doc = ""]
    #[doc = " @see IndexBuffer::setBuffer"]
    #[link_name = "\u{1}_ZN8filament11IndexBuffer7Builder5buildERNS_6EngineE"]
    pub fn filament_IndexBuffer_Builder_build(
        this: *mut filament_IndexBuffer_Builder,
        engine: *mut filament_Engine,
    ) -> *mut filament_IndexBuffer;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament11IndexBuffer7BuilderC1Ev"]
    pub fn filament_IndexBuffer_Builder_Builder(this: *mut filament_IndexBuffer_Builder);
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament11IndexBuffer7BuilderC1ERKS1_"]
    pub fn filament_IndexBuffer_Builder_Builder1(
        this: *mut filament_IndexBuffer_Builder,
        rhs: *const filament_IndexBuffer_Builder,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament11IndexBuffer7BuilderC1EOS1_"]
    pub fn filament_IndexBuffer_Builder_Builder2(
        this: *mut filament_IndexBuffer_Builder,
        rhs: *mut filament_IndexBuffer_Builder,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament11IndexBuffer7BuilderD1Ev"]
    pub fn filament_IndexBuffer_Builder_Builder_destructor(this: *mut filament_IndexBuffer_Builder);
}
impl Default for filament_IndexBuffer_Builder {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl filament_IndexBuffer_Builder {
    #[inline]
    pub unsafe fn indexCount(&mut self, indexCount: u32) -> *mut filament_IndexBuffer_Builder {
        filament_IndexBuffer_Builder_indexCount(self, indexCount)
    }
    #[inline]
    pub unsafe fn bufferType(
        &mut self,
        indexType: filament_IndexBuffer_IndexType,
    ) -> *mut filament_IndexBuffer_Builder {
        filament_IndexBuffer_Builder_bufferType(self, indexType)
    }
    #[inline]
    pub unsafe fn build(&mut self, engine: *mut filament_Engine) -> *mut filament_IndexBuffer {
        filament_IndexBuffer_Builder_build(self, engine)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_IndexBuffer_Builder_Builder(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(rhs: *const filament_IndexBuffer_Builder) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_IndexBuffer_Builder_Builder1(__bindgen_tmp.as_mut_ptr(), rhs);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new2(rhs: *mut filament_IndexBuffer_Builder) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_IndexBuffer_Builder_Builder2(__bindgen_tmp.as_mut_ptr(), rhs);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        filament_IndexBuffer_Builder_Builder_destructor(self)
    }
}
#[test]
fn bindgen_test_layout_filament_IndexBuffer() {
    assert_eq!(
        ::std::mem::size_of::<filament_IndexBuffer>(),
        1usize,
        concat!("Size of: ", stringify!(filament_IndexBuffer))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_IndexBuffer>(),
        1usize,
        concat!("Alignment of ", stringify!(filament_IndexBuffer))
    );
}
extern "C" {
    #[doc = " Asynchronously copy-initializes a region of this IndexBuffer from the data provided."]
    #[doc = ""]
    #[doc = " @param engine Reference to the filament::Engine to associate this IndexBuffer with."]
    #[doc = " @param buffer A BufferDescriptor representing the data used to initialize the IndexBuffer."]
    #[doc = "               BufferDescriptor points to raw, untyped data that will be interpreted as"]
    #[doc = "               either 16-bit or 32-bits indices based on the Type of this IndexBuffer."]
    #[doc = " @param byteOffset Offset in *bytes* into the IndexBuffer"]
    #[link_name = "\u{1}_ZN8filament11IndexBuffer9setBufferERNS_6EngineEONS_7backend16BufferDescriptorEj"]
    pub fn filament_IndexBuffer_setBuffer(
        this: *mut filament_IndexBuffer,
        engine: *mut filament_Engine,
        buffer: *mut filament_IndexBuffer_BufferDescriptor,
        byteOffset: u32,
    );
}
extern "C" {
    #[doc = " Returns the size of this IndexBuffer in elements."]
    #[doc = " @return The number of indices the IndexBuffer holds."]
    #[link_name = "\u{1}_ZNK8filament11IndexBuffer13getIndexCountEv"]
    pub fn filament_IndexBuffer_getIndexCount(this: *const filament_IndexBuffer) -> size_t;
}
impl filament_IndexBuffer {
    #[inline]
    pub unsafe fn setBuffer(
        &mut self,
        engine: *mut filament_Engine,
        buffer: *mut filament_IndexBuffer_BufferDescriptor,
        byteOffset: u32,
    ) {
        filament_IndexBuffer_setBuffer(self, engine, buffer, byteOffset)
    }
    #[inline]
    pub unsafe fn getIndexCount(&self) -> size_t {
        filament_IndexBuffer_getIndexCount(self)
    }
}
#[doc = " IndirectLight is used to simulate environment lighting, a form of global illumination."]
#[doc = ""]
#[doc = " Environment lighting has a two components:"]
#[doc = "  1. irradiance"]
#[doc = "  2. reflections (specular component)"]
#[doc = ""]
#[doc = " Environments are usually captured as high-resolution HDR equirectangular images and processed"]
#[doc = " by the **cmgen** tool to generate the data needed by IndirectLight."]
#[doc = ""]
#[doc = " @note"]
#[doc = " Currently IndirectLight is intended to be used for \"distant probes\", that is, to represent"]
#[doc = " global illumination from a distant (i.e. at infinity) environment, such as the sky or distant"]
#[doc = " mountains. Only a single IndirectLight can be used in a Scene. This limitation will be lifted"]
#[doc = " in the future."]
#[doc = ""]
#[doc = " Creation and destruction"]
#[doc = " ========================"]
#[doc = ""]
#[doc = " An IndirectLight object is created using the IndirectLight::Builder and destroyed by calling"]
#[doc = " Engine::destroy(const IndirectLight*)."]
#[doc = ""]
#[doc = " ~~~~~~~~~~~{.cpp}"]
#[doc = "  filament::Engine* engine = filament::Engine::create();"]
#[doc = ""]
#[doc = "  filament::IndirectLight* environment = filament::IndirectLight::Builder()"]
#[doc = "              .reflections(cubemap)"]
#[doc = "              .build(*engine);"]
#[doc = ""]
#[doc = "  engine->destroy(environment);"]
#[doc = " ~~~~~~~~~~~"]
#[doc = ""]
#[doc = ""]
#[doc = " Irradiance"]
#[doc = " =========="]
#[doc = ""]
#[doc = " The irradiance represents the light that comes from the environment and shines an"]
#[doc = " object's surface."]
#[doc = ""]
#[doc = " The irradiance is calculated automatically from the Reflections (see below), and generally"]
#[doc = " doesn't need to be provided explicitly.  However, it can be provided separately from the"]
#[doc = " Reflections as"]
#[doc = " [spherical harmonics](https://en.wikipedia.org/wiki/Spherical_harmonics) (SH) of 1, 2 or"]
#[doc = " 3 bands, respectively 1, 4 or 9 coefficients."]
#[doc = ""]
#[doc = " @note"]
#[doc = " Use the **cmgen** tool to generate the `SH` for a given environment."]
#[doc = ""]
#[doc = " Reflections"]
#[doc = " ==========="]
#[doc = ""]
#[doc = " The reflections on object surfaces (specular component) is calculated from a specially"]
#[doc = " filtered cubemap pyramid generated by the **cmgen** tool."]
#[doc = ""]
#[doc = ""]
#[doc = " @see Scene, Light, Texture, Skybox"]
#[repr(C)]
#[derive(Debug, Default)]
pub struct filament_IndirectLight {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct filament_IndirectLight_BuilderDetails {
    _unused: [u8; 0],
}
#[doc = "! Use Builder to construct an IndirectLight object instance"]
#[repr(C)]
#[derive(Debug)]
pub struct filament_IndirectLight_Builder {
    pub _base: filament_BuilderBase<filament_IndirectLight_BuilderDetails>,
}
#[test]
fn bindgen_test_layout_filament_IndirectLight_Builder() {
    assert_eq!(
        ::std::mem::size_of::<filament_IndirectLight_Builder>(),
        8usize,
        concat!("Size of: ", stringify!(filament_IndirectLight_Builder))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_IndirectLight_Builder>(),
        8usize,
        concat!("Alignment of ", stringify!(filament_IndirectLight_Builder))
    );
}
extern "C" {
    #[doc = " Set the reflections cubemap mipmap chain."]
    #[doc = ""]
    #[doc = " @param cubemap   A mip-mapped cubemap generated by **cmgen**. Each cubemap level"]
    #[doc = "                  encodes a the irradiance for a roughness level."]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls."]
    #[doc = ""]
    #[link_name = "\u{1}_ZN8filament13IndirectLight7Builder11reflectionsEPKNS_7TextureE"]
    pub fn filament_IndirectLight_Builder_reflections(
        this: *mut filament_IndirectLight_Builder,
        cubemap: *const filament_Texture,
    ) -> *mut filament_IndirectLight_Builder;
}
extern "C" {
    #[doc = " Sets the irradiance as Spherical Harmonics."]
    #[doc = ""]
    #[doc = " The irradiance must be pre-convolved by \\f$ \\langle n \\cdot l \\rangle \\f$ and"]
    #[doc = " pre-multiplied by the Lambertian diffuse BRDF \\f$ \\frac{1}{\\pi} \\f$ and"]
    #[doc = " specified as Spherical Harmonics coefficients."]
    #[doc = ""]
    #[doc = " Additionally, these Spherical Harmonics coefficients must be pre-scaled by the"]
    #[doc = " reconstruction factors \\f$ A_{l}^{m} \\f$ below."]
    #[doc = ""]
    #[doc = " The final coefficients can be generated using the `cmgen` tool."]
    #[doc = ""]
    #[doc = " The index in the \\p sh array is given by:"]
    #[doc = ""]
    #[doc = "  `index(l, m) = l * (l + 1) + m`"]
    #[doc = ""]
    #[doc = "  \\f$ sh[index(l,m)] = L_{l}^{m} \\frac{1}{\\pi} A_{l}^{m} \\hat{C_{l}} \\f$"]
    #[doc = ""]
    #[doc = "   index |  l  |  m  |  \\f$ A_{l}^{m} \\f$ |  \\f$ \\hat{C_{l}} \\f$  |  \\f$ \\frac{1}{\\pi} A_{l}^{m}\\hat{C_{l}} \\f$ |"]
    #[doc = "  :-----:|:---:|:---:|:------------------:|:---------------------:|:--------------------------------------------:"]
    #[doc = "     0   |  0  |  0  |      0.282095      |       3.1415926       |   0.282095"]
    #[doc = "     1   |  1  | -1  |     -0.488602      |       2.0943951       |  -0.325735"]
    #[doc = "     2   |  ^  |  0  |      0.488602      |       ^               |   0.325735"]
    #[doc = "     3   |  ^  |  1  |     -0.488602      |       ^               |  -0.325735"]
    #[doc = "     4   |  2  | -2  |      1.092548      |       0.785398        |   0.273137"]
    #[doc = "     5   |  ^  | -1  |     -1.092548      |       ^               |  -0.273137"]
    #[doc = "     6   |  ^  |  0  |      0.315392      |       ^               |   0.078848"]
    #[doc = "     7   |  ^  |  1  |     -1.092548      |       ^               |  -0.273137"]
    #[doc = "     8   |  ^  |  2  |      0.546274      |       ^               |   0.136569"]
    #[doc = ""]
    #[doc = ""]
    #[doc = " Only 1, 2 or 3 bands are allowed."]
    #[doc = ""]
    #[doc = " @param bands     Number of spherical harmonics bands. Must be 1, 2 or 3."]
    #[doc = " @param sh        Array containing the spherical harmonics coefficients."]
    #[doc = "                  The size of the array must be \\f$ bands^{2} \\f$."]
    #[doc = "                  (i.e. 1, 4 or 9 coefficients respectively)."]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls."]
    #[doc = ""]
    #[doc = " @note"]
    #[doc = " Because the coefficients are pre-scaled, `sh[0]` is the environment's"]
    #[doc = " average irradiance."]
    #[link_name = "\u{1}_ZN8filament13IndirectLight7Builder10irradianceEhPKNS_4math7details5TVec3IfEE"]
    pub fn filament_IndirectLight_Builder_irradiance(
        this: *mut filament_IndirectLight_Builder,
        bands: u8,
        sh: *const filament_math_float3,
    ) -> *mut filament_IndirectLight_Builder;
}
extern "C" {
    #[doc = " Sets the irradiance from the radiance expressed as Spherical Harmonics."]
    #[doc = ""]
    #[doc = " The radiance must be specified as Spherical Harmonics coefficients \\f$ L_{l}^{m} \\f$"]
    #[doc = ""]
    #[doc = " The index in the \\p sh array is given by:"]
    #[doc = ""]
    #[doc = "  `index(l, m) = l * (l + 1) + m`"]
    #[doc = ""]
    #[doc = "  \\f$ sh[index(l,m)] = L_{l}^{m} \\f$"]
    #[doc = ""]
    #[doc = "   index |  l  |  m"]
    #[doc = "  :-----:|:---:|:---:"]
    #[doc = "     0   |  0  |  0"]
    #[doc = "     1   |  1  | -1"]
    #[doc = "     2   |  ^  |  0"]
    #[doc = "     3   |  ^  |  1"]
    #[doc = "     4   |  2  | -2"]
    #[doc = "     5   |  ^  | -1"]
    #[doc = "     6   |  ^  |  0"]
    #[doc = "     7   |  ^  |  1"]
    #[doc = "     8   |  ^  |  2"]
    #[doc = ""]
    #[doc = " @param bands     Number of spherical harmonics bands. Must be 1, 2 or 3."]
    #[doc = " @param sh        Array containing the spherical harmonics coefficients."]
    #[doc = "                  The size of the array must be \\f$ bands^{2} \\f$."]
    #[doc = "                  (i.e. 1, 4 or 9 coefficients respectively)."]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls."]
    #[link_name = "\u{1}_ZN8filament13IndirectLight7Builder8radianceEhPKNS_4math7details5TVec3IfEE"]
    pub fn filament_IndirectLight_Builder_radiance(
        this: *mut filament_IndirectLight_Builder,
        bands: u8,
        sh: *const filament_math_float3,
    ) -> *mut filament_IndirectLight_Builder;
}
extern "C" {
    #[doc = " Sets the irradiance as a cubemap."]
    #[doc = ""]
    #[doc = " The irradiance can alternatively be specified as a cubemap instead of Spherical"]
    #[doc = " Harmonics coefficients. It may or may not be more efficient, depending on your"]
    #[doc = " hardware (essentially, it's trading ALU for bandwidth)."]
    #[doc = ""]
    #[doc = " @param cubemap   Cubemap representing the Irradiance pre-convolved by"]
    #[doc = "                  \\f$ \\langle n \\cdot l \\rangle \\f$."]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls."]
    #[doc = ""]
    #[doc = " @note"]
    #[doc = " This irradiance cubemap can be generated with the **cmgen** tool."]
    #[doc = ""]
    #[doc = " @see irradiance(uint8_t bands, math::float3 const* sh)"]
    #[link_name = "\u{1}_ZN8filament13IndirectLight7Builder10irradianceEPKNS_7TextureE"]
    pub fn filament_IndirectLight_Builder_irradiance1(
        this: *mut filament_IndirectLight_Builder,
        cubemap: *const filament_Texture,
    ) -> *mut filament_IndirectLight_Builder;
}
extern "C" {
    #[doc = " (optional) Environment intensity."]
    #[doc = ""]
    #[doc = " Because the environment is encoded usually relative to some reference, the"]
    #[doc = " range can be adjusted with this method."]
    #[doc = ""]
    #[doc = " @param envIntensity  Scale factor applied to the environment and irradiance such that"]
    #[doc = "                      the result is in lux, or lumen/m^2 (default = 30000)"]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls."]
    #[link_name = "\u{1}_ZN8filament13IndirectLight7Builder9intensityEf"]
    pub fn filament_IndirectLight_Builder_intensity(
        this: *mut filament_IndirectLight_Builder,
        envIntensity: f32,
    ) -> *mut filament_IndirectLight_Builder;
}
extern "C" {
    #[doc = " Specifies the rigid-body transformation to apply to the IBL."]
    #[doc = ""]
    #[doc = " @param rotation 3x3 rotation matrix. Must be a rigid-body transform."]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls."]
    #[link_name = "\u{1}_ZN8filament13IndirectLight7Builder8rotationERKNS_4math7details6TMat33IfEE"]
    pub fn filament_IndirectLight_Builder_rotation(
        this: *mut filament_IndirectLight_Builder,
        rotation: *const filament_math_mat3f,
    ) -> *mut filament_IndirectLight_Builder;
}
extern "C" {
    #[doc = " Creates the IndirectLight object and returns a pointer to it."]
    #[doc = ""]
    #[doc = " @param engine Reference to the filament::Engine to associate this IndirectLight with."]
    #[doc = ""]
    #[doc = " @return pointer to the newly created object or nullptr if exceptions are disabled and"]
    #[doc = "         an error occurred."]
    #[doc = ""]
    #[doc = " @exception utils::PostConditionPanic if a runtime error occurred, such as running out of"]
    #[doc = "            memory or other resources."]
    #[doc = " @exception utils::PreConditionPanic if a parameter to a builder function was invalid."]
    #[link_name = "\u{1}_ZN8filament13IndirectLight7Builder5buildERNS_6EngineE"]
    pub fn filament_IndirectLight_Builder_build(
        this: *mut filament_IndirectLight_Builder,
        engine: *mut filament_Engine,
    ) -> *mut filament_IndirectLight;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament13IndirectLight7BuilderC1Ev"]
    pub fn filament_IndirectLight_Builder_Builder(this: *mut filament_IndirectLight_Builder);
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament13IndirectLight7BuilderC1ERKS1_"]
    pub fn filament_IndirectLight_Builder_Builder1(
        this: *mut filament_IndirectLight_Builder,
        rhs: *const filament_IndirectLight_Builder,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament13IndirectLight7BuilderC1EOS1_"]
    pub fn filament_IndirectLight_Builder_Builder2(
        this: *mut filament_IndirectLight_Builder,
        rhs: *mut filament_IndirectLight_Builder,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament13IndirectLight7BuilderD1Ev"]
    pub fn filament_IndirectLight_Builder_Builder_destructor(
        this: *mut filament_IndirectLight_Builder,
    );
}
impl Default for filament_IndirectLight_Builder {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl filament_IndirectLight_Builder {
    #[inline]
    pub unsafe fn reflections(
        &mut self,
        cubemap: *const filament_Texture,
    ) -> *mut filament_IndirectLight_Builder {
        filament_IndirectLight_Builder_reflections(self, cubemap)
    }
    #[inline]
    pub unsafe fn irradiance(
        &mut self,
        bands: u8,
        sh: *const filament_math_float3,
    ) -> *mut filament_IndirectLight_Builder {
        filament_IndirectLight_Builder_irradiance(self, bands, sh)
    }
    #[inline]
    pub unsafe fn radiance(
        &mut self,
        bands: u8,
        sh: *const filament_math_float3,
    ) -> *mut filament_IndirectLight_Builder {
        filament_IndirectLight_Builder_radiance(self, bands, sh)
    }
    #[inline]
    pub unsafe fn irradiance1(
        &mut self,
        cubemap: *const filament_Texture,
    ) -> *mut filament_IndirectLight_Builder {
        filament_IndirectLight_Builder_irradiance1(self, cubemap)
    }
    #[inline]
    pub unsafe fn intensity(&mut self, envIntensity: f32) -> *mut filament_IndirectLight_Builder {
        filament_IndirectLight_Builder_intensity(self, envIntensity)
    }
    #[inline]
    pub unsafe fn rotation(
        &mut self,
        rotation: *const filament_math_mat3f,
    ) -> *mut filament_IndirectLight_Builder {
        filament_IndirectLight_Builder_rotation(self, rotation)
    }
    #[inline]
    pub unsafe fn build(&mut self, engine: *mut filament_Engine) -> *mut filament_IndirectLight {
        filament_IndirectLight_Builder_build(self, engine)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_IndirectLight_Builder_Builder(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(rhs: *const filament_IndirectLight_Builder) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_IndirectLight_Builder_Builder1(__bindgen_tmp.as_mut_ptr(), rhs);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new2(rhs: *mut filament_IndirectLight_Builder) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_IndirectLight_Builder_Builder2(__bindgen_tmp.as_mut_ptr(), rhs);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        filament_IndirectLight_Builder_Builder_destructor(self)
    }
}
#[test]
fn bindgen_test_layout_filament_IndirectLight() {
    assert_eq!(
        ::std::mem::size_of::<filament_IndirectLight>(),
        1usize,
        concat!("Size of: ", stringify!(filament_IndirectLight))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_IndirectLight>(),
        1usize,
        concat!("Alignment of ", stringify!(filament_IndirectLight))
    );
}
extern "C" {
    #[doc = " Sets the environment's intensity."]
    #[doc = ""]
    #[doc = " Because the environment is encoded usually relative to some reference, the"]
    #[doc = " range can be adjusted with this method."]
    #[doc = ""]
    #[doc = " @param intensity  Scale factor applied to the environment and irradiance such that"]
    #[doc = "                   the result is in lux, or <i>lumen/m^2(default = 30000)"]
    #[link_name = "\u{1}_ZN8filament13IndirectLight12setIntensityEf"]
    pub fn filament_IndirectLight_setIntensity(this: *mut filament_IndirectLight, intensity: f32);
}
extern "C" {
    #[doc = " Returns the environment's intensity in <i>lux</i>, or <i>lumen/m^2</i>."]
    #[link_name = "\u{1}_ZNK8filament13IndirectLight12getIntensityEv"]
    pub fn filament_IndirectLight_getIntensity(this: *const filament_IndirectLight) -> f32;
}
extern "C" {
    #[doc = " Sets the rigid-body transformation to apply to the IBL."]
    #[doc = ""]
    #[doc = " @param rotation 3x3 rotation matrix. Must be a rigid-body transform."]
    #[link_name = "\u{1}_ZN8filament13IndirectLight11setRotationERKNS_4math7details6TMat33IfEE"]
    pub fn filament_IndirectLight_setRotation(
        this: *mut filament_IndirectLight,
        rotation: *const filament_math_mat3f,
    );
}
extern "C" {
    #[doc = " Returns the rigid-body transformation applied to the IBL."]
    #[link_name = "\u{1}_ZNK8filament13IndirectLight11getRotationEv"]
    pub fn filament_IndirectLight_getRotation(
        this: *const filament_IndirectLight,
    ) -> *const filament_math_mat3f;
}
extern "C" {
    #[doc = " Returns the associated reflection map, or null if it does not exist."]
    #[link_name = "\u{1}_ZNK8filament13IndirectLight21getReflectionsTextureEv"]
    pub fn filament_IndirectLight_getReflectionsTexture(
        this: *const filament_IndirectLight,
    ) -> *const filament_Texture;
}
extern "C" {
    #[doc = " Returns the associated irradiance map, or null if it does not exist."]
    #[link_name = "\u{1}_ZNK8filament13IndirectLight20getIrradianceTextureEv"]
    pub fn filament_IndirectLight_getIrradianceTexture(
        this: *const filament_IndirectLight,
    ) -> *const filament_Texture;
}
extern "C" {
    #[doc = " Helper to estimate the direction of the dominant light in the environment represented by"]
    #[doc = " spherical harmonics."]
    #[doc = ""]
    #[doc = " This assumes that there is only a single dominant light (such as the sun in outdoors"]
    #[doc = " environments), if it's not the case the direction returned will be an average of the"]
    #[doc = " various lights based on their intensity."]
    #[doc = ""]
    #[doc = " If there are no clear dominant light, as is often the case with low dynamic range (LDR)"]
    #[doc = " environments, this method may return a wrong or unexpected direction."]
    #[doc = ""]
    #[doc = " The dominant light direction can be used to set a directional light's direction,"]
    #[doc = " for instance to produce shadows that match the environment."]
    #[doc = ""]
    #[doc = " @param sh        3-band spherical harmonics"]
    #[doc = ""]
    #[doc = " @return A unit vector representing the direction of the dominant light"]
    #[doc = ""]
    #[doc = " @see LightManager::Builder::direction()"]
    #[doc = " @see getColorEstimate()"]
    #[link_name = "\u{1}_ZN8filament13IndirectLight20getDirectionEstimateEPKNS_4math7details5TVec3IfEE"]
    pub fn filament_IndirectLight_getDirectionEstimate(
        sh: *const filament_math_float3,
    ) -> filament_math_float3;
}
extern "C" {
    #[doc = " Helper to estimate the color and relative intensity of the environment represented by"]
    #[doc = " spherical harmonics in a given direction."]
    #[doc = ""]
    #[doc = " This can be used to set the color and intensity of a directional light. In this case"]
    #[doc = " make sure to multiply this relative intensity by the the intensity of this indirect light."]
    #[doc = ""]
    #[doc = " @param sh        3-band spherical harmonics"]
    #[doc = " @param direction a unit vector representing the direction of the light to estimate the"]
    #[doc = "                  color of. Typically this the value returned by getDirectionEstimate()."]
    #[doc = ""]
    #[doc = " @return A vector of 4 floats where the first 3 components represent the linear color and"]
    #[doc = "         the 4th component represents the intensity of the dominant light"]
    #[doc = ""]
    #[doc = " @see LightManager::Builder::color()"]
    #[doc = " @see LightManager::Builder::intensity()"]
    #[doc = " @see getDirectionEstimate, getIntensity, setIntensity"]
    #[link_name = "\u{1}_ZN8filament13IndirectLight16getColorEstimateEPKNS_4math7details5TVec3IfEES4_"]
    pub fn filament_IndirectLight_getColorEstimate(
        sh: *const filament_math_float3,
        direction: filament_math_float3,
    ) -> filament_math_float4;
}
extern "C" {
    #[doc = " @deprecated use static versions instead"]
    #[link_name = "\u{1}_ZNK8filament13IndirectLight20getDirectionEstimateEv"]
    pub fn filament_IndirectLight_getDirectionEstimate1(
        this: *const filament_IndirectLight,
    ) -> filament_math_float3;
}
extern "C" {
    #[doc = " @deprecated use static versions instead"]
    #[link_name = "\u{1}_ZNK8filament13IndirectLight16getColorEstimateENS_4math7details5TVec3IfEE"]
    pub fn filament_IndirectLight_getColorEstimate1(
        this: *const filament_IndirectLight,
        direction: filament_math_float3,
    ) -> filament_math_float4;
}
impl filament_IndirectLight {
    #[inline]
    pub unsafe fn setIntensity(&mut self, intensity: f32) {
        filament_IndirectLight_setIntensity(self, intensity)
    }
    #[inline]
    pub unsafe fn getIntensity(&self) -> f32 {
        filament_IndirectLight_getIntensity(self)
    }
    #[inline]
    pub unsafe fn setRotation(&mut self, rotation: *const filament_math_mat3f) {
        filament_IndirectLight_setRotation(self, rotation)
    }
    #[inline]
    pub unsafe fn getRotation(&self) -> *const filament_math_mat3f {
        filament_IndirectLight_getRotation(self)
    }
    #[inline]
    pub unsafe fn getReflectionsTexture(&self) -> *const filament_Texture {
        filament_IndirectLight_getReflectionsTexture(self)
    }
    #[inline]
    pub unsafe fn getIrradianceTexture(&self) -> *const filament_Texture {
        filament_IndirectLight_getIrradianceTexture(self)
    }
    #[inline]
    pub unsafe fn getDirectionEstimate(sh: *const filament_math_float3) -> filament_math_float3 {
        filament_IndirectLight_getDirectionEstimate(sh)
    }
    #[inline]
    pub unsafe fn getColorEstimate(
        sh: *const filament_math_float3,
        direction: filament_math_float3,
    ) -> filament_math_float4 {
        filament_IndirectLight_getColorEstimate(sh, direction)
    }
    #[inline]
    pub unsafe fn getDirectionEstimate1(&self) -> filament_math_float3 {
        filament_IndirectLight_getDirectionEstimate1(self)
    }
    #[inline]
    pub unsafe fn getColorEstimate1(
        &self,
        direction: filament_math_float3,
    ) -> filament_math_float4 {
        filament_IndirectLight_getColorEstimate1(self, direction)
    }
}
#[doc = " LightManager allows to create a light source in the scene, such as a sun or street lights."]
#[doc = ""]
#[doc = " At least one light must be added to a scene in order to see anything"]
#[doc = " (unless the Material.Shading.UNLIT is used)."]
#[doc = ""]
#[doc = ""]
#[doc = " Creation and destruction"]
#[doc = " ========================"]
#[doc = ""]
#[doc = " A Light component is created using the LightManager::Builder and destroyed by calling"]
#[doc = " LightManager::destroy(utils::Entity)."]
#[doc = ""]
#[doc = " ~~~~~~~~~~~{.cpp}"]
#[doc = "  filament::Engine* engine = filament::Engine::create();"]
#[doc = "  utils::Entity sun = utils::EntityManager.get().create();"]
#[doc = ""]
#[doc = "  filament::LightManager::Builder(Type::SUN)"]
#[doc = "              .castShadows(true)"]
#[doc = "              .build(*engine, sun);"]
#[doc = ""]
#[doc = "  engine->getLightManager().destroy(sun);"]
#[doc = " ~~~~~~~~~~~"]
#[doc = ""]
#[doc = ""]
#[doc = " Light types"]
#[doc = " ==========="]
#[doc = ""]
#[doc = " Lights come in three flavors:"]
#[doc = " - directional lights"]
#[doc = " - point lights"]
#[doc = " - spot lights"]
#[doc = ""]
#[doc = ""]
#[doc = " Directional lights"]
#[doc = " ------------------"]
#[doc = ""]
#[doc = " Directional lights have a direction, but don't have a position. All light rays are"]
#[doc = " parallel and come from infinitely far away and from everywhere. Typically a directional light"]
#[doc = " is used to simulate the sun."]
#[doc = ""]
#[doc = " Directional lights and spot lights are able to cast shadows."]
#[doc = ""]
#[doc = " To create a directional light use Type.DIRECTIONAL or Type.SUN, both are similar, but the later"]
#[doc = " also draws a sun's disk in the sky and its reflection on glossy objects."]
#[doc = ""]
#[doc = " @warning Currently, only a single directional light is supported. If several directional lights"]
#[doc = " are added to the scene, the dominant one will be used."]
#[doc = ""]
#[doc = " @see Builder.direction(), Builder.sunAngularRadius()"]
#[doc = ""]
#[doc = " Point lights"]
#[doc = " ------------"]
#[doc = ""]
#[doc = " Unlike directional lights, point lights have a position but emit light in all directions."]
#[doc = " The intensity of the light diminishes with the inverse square of the distance to the light."]
#[doc = " Builder.falloff() controls distance beyond which the light has no more influence."]
#[doc = ""]
#[doc = " A scene can have multiple point lights."]
#[doc = ""]
#[doc = " @see Builder.position(), Builder.falloff()"]
#[doc = ""]
#[doc = " Spot lights"]
#[doc = " -----------"]
#[doc = ""]
#[doc = " Spot lights are similar to point lights but the light it emits is limited to a cone defined by"]
#[doc = " Builder.spotLightCone() and the light's direction."]
#[doc = ""]
#[doc = " A spot light is therefore defined by a position, a direction and inner and outer cones. The"]
#[doc = " spot light's influence is limited to inside the outer cone. The inner cone defines the light's"]
#[doc = " falloff attenuation."]
#[doc = ""]
#[doc = " A physically correct spot light is a little difficult to use because changing the outer angle"]
#[doc = " of the cone changes the illumination levels, as the same amount of light is spread over a"]
#[doc = " changing volume. The coupling of illumination and the outer cone means that an artist cannot"]
#[doc = " tweak the influence cone of a spot light without also changing the perceived illumination."]
#[doc = " It therefore makes sense to provide artists with a parameter to disable this coupling. This"]
#[doc = " is the difference between Type.FOCUSED_SPOT and Type.SPOT."]
#[doc = ""]
#[doc = " @see Builder.position(), Builder.direction(), Builder.falloff(), Builder.spotLightCone()"]
#[doc = ""]
#[doc = " Performance considerations"]
#[doc = " =========================="]
#[doc = ""]
#[doc = " Generally, adding lights to the scene hurts performance, however filament is designed to be"]
#[doc = " able to handle hundreds of lights in a scene under certain conditions. Here are some tips"]
#[doc = " to keep performances high."]
#[doc = ""]
#[doc = " 1. Prefer spot lights to point lights and use the smallest outer cone angle possible."]
#[doc = ""]
#[doc = " 2. Use the smallest possible falloff distance for point and spot lights."]
#[doc = "    Performance is very sensitive to overlapping lights. The falloff distance essentially"]
#[doc = "    defines a sphere of influence for the light, so try to position point and spot lights"]
#[doc = "    such that they don't overlap too much."]
#[doc = ""]
#[doc = "    On the other hand, a scene can contain hundreds of non overlapping lights without"]
#[doc = "    incurring a significant overhead."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Default)]
pub struct filament_LightManager {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct filament_LightManager_BuilderDetails {
    _unused: [u8; 0],
}
pub type filament_LightManager_Instance = u32;
#[doc = "!< Directional light that also draws a sun's disk in the sky."]
pub const filament_LightManager_Type_SUN: filament_LightManager_Type = 0;
#[doc = "!< Directional light, emits light in a given direction."]
pub const filament_LightManager_Type_DIRECTIONAL: filament_LightManager_Type = 1;
#[doc = "!< Point light, emits light from a position, in all directions."]
pub const filament_LightManager_Type_POINT: filament_LightManager_Type = 2;
#[doc = "!< Physically correct spot light."]
pub const filament_LightManager_Type_FOCUSED_SPOT: filament_LightManager_Type = 3;
#[doc = "!< Spot light with coupling of outer cone and illumination disabled."]
pub const filament_LightManager_Type_SPOT: filament_LightManager_Type = 4;
#[doc = "! Denotes the type of the light being created."]
pub type filament_LightManager_Type = u8;
#[doc = " Control the quality / performance of the shadow map associated to this light"]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_LightManager_ShadowOptions {
    #[doc = " Size of the shadow map in texels. Must be a power-of-two and larger or equal to 8."]
    pub mapSize: u32,
    #[doc = " Number of shadow cascades to use for this light. Must be between 1 and 4 (inclusive)."]
    #[doc = " A value greater than 1 turns on cascaded shadow mapping (CSM)."]
    #[doc = " Only applicable to Type.SUN or Type.DIRECTIONAL lights."]
    #[doc = ""]
    #[doc = " When using shadow cascades, cascadeSplitPositions must also be set."]
    #[doc = ""]
    #[doc = " @see ShadowOptions::cascadeSplitPositions"]
    pub shadowCascades: u8,
    #[doc = " The split positions for shadow cascades."]
    #[doc = ""]
    #[doc = " Cascaded shadow mapping (CSM) partitions the camera frustum into cascades. These values"]
    #[doc = " determine the planes along the camera's Z axis to split the frustum. The camera near"]
    #[doc = " plane is represented by 0.0f and the far plane represented by 1.0f."]
    #[doc = ""]
    #[doc = " For example, if using 4 cascades, these values would set a uniform split scheme:"]
    #[doc = " { 0.25f, 0.50f, 0.75f }"]
    #[doc = ""]
    #[doc = " For N cascades, N - 1 split positions will be read from this array."]
    #[doc = ""]
    #[doc = " Filament provides utility methods inside LightManager::ShadowCascades to help set these"]
    #[doc = " values. For example, to use a uniform split scheme:"]
    #[doc = ""]
    #[doc = " ~~~~~~~~~~~{.cpp}"]
    #[doc = "   LightManager::ShadowCascades::computeUniformSplits(options.splitPositions, 4);"]
    #[doc = " ~~~~~~~~~~~"]
    #[doc = ""]
    #[doc = " @see ShadowCascades::computeUniformSplits"]
    #[doc = " @see ShadowCascades::computeLogSplits"]
    #[doc = " @see ShadowCascades::computePracticalSplits"]
    pub cascadeSplitPositions: [f32; 3usize],
    #[doc = " Constant bias in world units (e.g. meters) by which shadows are moved away from the"]
    #[doc = " light. 1mm by default."]
    #[doc = " This is ignored when the View's ShadowType is set to VSM."]
    pub constantBias: f32,
    #[doc = " Amount by which the maximum sampling error is scaled. The resulting value is used"]
    #[doc = " to move the shadow away from the fragment normal. Should be 1.0."]
    #[doc = " This is ignored when the View's ShadowType is set to VSM."]
    pub normalBias: f32,
    #[doc = " Distance from the camera after which shadows are clipped. This is used to clip"]
    #[doc = " shadows that are too far and wouldn't contribute to the scene much, improving"]
    #[doc = " performance and quality. This value is always positive."]
    #[doc = " Use 0.0f to use the camera far distance."]
    pub shadowFar: f32,
    #[doc = " Optimize the quality of shadows from this distance from the camera. Shadows will"]
    #[doc = " be rendered in front of this distance, but the quality may not be optimal."]
    #[doc = " This value is always positive. Use 0.0f to use the camera near distance."]
    #[doc = " The default of 1m works well with many scenes. The quality of shadows may drop"]
    #[doc = " rapidly when this value decreases."]
    pub shadowNearHint: f32,
    #[doc = " Optimize the quality of shadows in front of this distance from the camera. Shadows"]
    #[doc = " will be rendered behind this distance, but the quality may not be optimal."]
    #[doc = " This value is always positive. Use std::numerical_limits<float>::infinity() to"]
    #[doc = " use the camera far distance."]
    pub shadowFarHint: f32,
    #[doc = " Controls whether the shadow map should be optimized for resolution or stability."]
    #[doc = " When set to true, all resolution enhancing features that can affect stability are"]
    #[doc = " disabling, resulting in significantly lower resolution shadows, albeit stable ones."]
    pub stable: bool,
    #[doc = " Constant bias in depth-resolution units by which shadows are moved away from the"]
    #[doc = " light. The default value of 0.5 is used to round depth values up."]
    #[doc = " Generally this value shouldn't be changed or at least be small and positive."]
    #[doc = " This is ignored when the View's ShadowType is set to VSM."]
    pub polygonOffsetConstant: f32,
    #[doc = " Bias based on the change in depth in depth-resolution units by which shadows are moved"]
    #[doc = " away from the light. The default value of 2.0 works well with SHADOW_SAMPLING_PCF_LOW."]
    #[doc = " Generally this value is between 0.5 and the size in texel of the PCF filter."]
    #[doc = " Setting this value correctly is essential for LISPSM shadow-maps."]
    #[doc = " This is ignored when the View's ShadowType is set to VSM."]
    pub polygonOffsetSlope: f32,
    #[doc = " Whether screen-space contact shadows are used. This applies regardless of whether a"]
    #[doc = " Renderable is a shadow caster."]
    #[doc = " Screen-space contact shadows are typically useful in large scenes."]
    #[doc = " (off by default)"]
    pub screenSpaceContactShadows: bool,
    #[doc = " Number of ray-marching steps for screen-space contact shadows (8 by default)."]
    #[doc = ""]
    #[doc = " CAUTION: this parameter is ignored for all lights except the directional/sun light,"]
    #[doc = "          all other lights use the same value set for the directional/sun light."]
    #[doc = ""]
    pub stepCount: u8,
    #[doc = " Maximum shadow-occluder distance for screen-space contact shadows (world units)."]
    #[doc = " (30 cm by default)"]
    #[doc = ""]
    #[doc = " CAUTION: this parameter is ignored for all lights except the directional/sun light,"]
    #[doc = "          all other lights use the same value set for the directional/sun light."]
    #[doc = ""]
    pub maxShadowDistance: f32,
    pub vsm: filament_LightManager_ShadowOptions_Vsm,
    #[doc = " Light bulb radius used for soft shadows. Currently this is only used when DPCF or PCSS is"]
    #[doc = " enabled. (2cm by default)."]
    pub shadowBulbRadius: f32,
}
#[doc = " Options available when the View's ShadowType is set to VSM."]
#[doc = ""]
#[doc = " @warning This API is still experimental and subject to change."]
#[doc = " @see View::setShadowType"]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_LightManager_ShadowOptions_Vsm {
    #[doc = " The number of MSAA samples to use when rendering VSM shadow maps."]
    #[doc = " Must be a power-of-two and greater than or equal to 1. A value of 1 effectively turns"]
    #[doc = " off MSAA."]
    #[doc = " Higher values may not be available depending on the underlying hardware."]
    pub msaaSamples: u8,
    #[doc = " Blur width for the VSM blur. Zero do disable."]
    #[doc = " The maximum value is 125."]
    pub blurWidth: f32,
}
#[test]
fn bindgen_test_layout_filament_LightManager_ShadowOptions_Vsm() {
    assert_eq!(
        ::std::mem::size_of::<filament_LightManager_ShadowOptions_Vsm>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(filament_LightManager_ShadowOptions_Vsm)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_LightManager_ShadowOptions_Vsm>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(filament_LightManager_ShadowOptions_Vsm)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_LightManager_ShadowOptions_Vsm>())).msaaSamples
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_LightManager_ShadowOptions_Vsm),
            "::",
            stringify!(msaaSamples)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_LightManager_ShadowOptions_Vsm>())).blurWidth
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_LightManager_ShadowOptions_Vsm),
            "::",
            stringify!(blurWidth)
        )
    );
}
#[test]
fn bindgen_test_layout_filament_LightManager_ShadowOptions() {
    assert_eq!(
        ::std::mem::size_of::<filament_LightManager_ShadowOptions>(),
        72usize,
        concat!("Size of: ", stringify!(filament_LightManager_ShadowOptions))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_LightManager_ShadowOptions>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(filament_LightManager_ShadowOptions)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_LightManager_ShadowOptions>())).mapSize as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_LightManager_ShadowOptions),
            "::",
            stringify!(mapSize)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_LightManager_ShadowOptions>())).shadowCascades
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_LightManager_ShadowOptions),
            "::",
            stringify!(shadowCascades)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_LightManager_ShadowOptions>())).cascadeSplitPositions
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_LightManager_ShadowOptions),
            "::",
            stringify!(cascadeSplitPositions)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_LightManager_ShadowOptions>())).constantBias as *const _
                as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_LightManager_ShadowOptions),
            "::",
            stringify!(constantBias)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_LightManager_ShadowOptions>())).normalBias as *const _
                as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_LightManager_ShadowOptions),
            "::",
            stringify!(normalBias)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_LightManager_ShadowOptions>())).shadowFar as *const _
                as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_LightManager_ShadowOptions),
            "::",
            stringify!(shadowFar)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_LightManager_ShadowOptions>())).shadowNearHint
                as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_LightManager_ShadowOptions),
            "::",
            stringify!(shadowNearHint)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_LightManager_ShadowOptions>())).shadowFarHint
                as *const _ as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_LightManager_ShadowOptions),
            "::",
            stringify!(shadowFarHint)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_LightManager_ShadowOptions>())).stable as *const _
                as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_LightManager_ShadowOptions),
            "::",
            stringify!(stable)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_LightManager_ShadowOptions>())).polygonOffsetConstant
                as *const _ as usize
        },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_LightManager_ShadowOptions),
            "::",
            stringify!(polygonOffsetConstant)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_LightManager_ShadowOptions>())).polygonOffsetSlope
                as *const _ as usize
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_LightManager_ShadowOptions),
            "::",
            stringify!(polygonOffsetSlope)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_LightManager_ShadowOptions>()))
                .screenSpaceContactShadows as *const _ as usize
        },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_LightManager_ShadowOptions),
            "::",
            stringify!(screenSpaceContactShadows)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_LightManager_ShadowOptions>())).stepCount as *const _
                as usize
        },
        53usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_LightManager_ShadowOptions),
            "::",
            stringify!(stepCount)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_LightManager_ShadowOptions>())).maxShadowDistance
                as *const _ as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_LightManager_ShadowOptions),
            "::",
            stringify!(maxShadowDistance)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_LightManager_ShadowOptions>())).vsm as *const _ as usize
        },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_LightManager_ShadowOptions),
            "::",
            stringify!(vsm)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_LightManager_ShadowOptions>())).shadowBulbRadius
                as *const _ as usize
        },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_LightManager_ShadowOptions),
            "::",
            stringify!(shadowBulbRadius)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_LightManager_ShadowCascades {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_filament_LightManager_ShadowCascades() {
    assert_eq!(
        ::std::mem::size_of::<filament_LightManager_ShadowCascades>(),
        1usize,
        concat!(
            "Size of: ",
            stringify!(filament_LightManager_ShadowCascades)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_LightManager_ShadowCascades>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(filament_LightManager_ShadowCascades)
        )
    );
}
extern "C" {
    #[doc = " Utility method to compute ShadowOptions::cascadeSplitPositions according to a uniform"]
    #[doc = " split scheme."]
    #[doc = ""]
    #[doc = " @param splitPositions    a float array of at least size (cascades - 1) to write the split"]
    #[doc = "                          positions into"]
    #[doc = " @param cascades          the number of shadow cascades, at most 4"]
    #[link_name = "\u{1}_ZN8filament12LightManager14ShadowCascades20computeUniformSplitsEPfh"]
    pub fn filament_LightManager_ShadowCascades_computeUniformSplits(
        splitPositions: *mut f32,
        cascades: u8,
    );
}
extern "C" {
    #[doc = " Utility method to compute ShadowOptions::cascadeSplitPositions according to a logarithmic"]
    #[doc = " split scheme."]
    #[doc = ""]
    #[doc = " @param splitPositions    a float array of at least size (cascades - 1) to write the split"]
    #[doc = "                          positions into"]
    #[doc = " @param cascades          the number of shadow cascades, at most 4"]
    #[doc = " @param near              the camera near plane"]
    #[doc = " @param far               the camera far plane"]
    #[link_name = "\u{1}_ZN8filament12LightManager14ShadowCascades16computeLogSplitsEPfhff"]
    pub fn filament_LightManager_ShadowCascades_computeLogSplits(
        splitPositions: *mut f32,
        cascades: u8,
        near: f32,
        far: f32,
    );
}
extern "C" {
    #[doc = " Utility method to compute ShadowOptions::cascadeSplitPositions according to a practical"]
    #[doc = " split scheme."]
    #[doc = ""]
    #[doc = " The practical split scheme uses uses a lambda value to interpolate between the logarithmic"]
    #[doc = " and uniform split schemes. Start with a lambda value of 0.5f and adjust for your scene."]
    #[doc = ""]
    #[doc = " See: Zhang et al 2006, \"Parallel-split shadow maps for large-scale virtual environments\""]
    #[doc = ""]
    #[doc = " @param splitPositions    a float array of at least size (cascades - 1) to write the split"]
    #[doc = "                          positions into"]
    #[doc = " @param cascades          the number of shadow cascades, at most 4"]
    #[doc = " @param near              the camera near plane"]
    #[doc = " @param far               the camera far plane"]
    #[doc = " @param lambda            a float in the range [0, 1] that interpolates between log and"]
    #[doc = "                          uniform split schemes"]
    #[link_name = "\u{1}_ZN8filament12LightManager14ShadowCascades22computePracticalSplitsEPfhfff"]
    pub fn filament_LightManager_ShadowCascades_computePracticalSplits(
        splitPositions: *mut f32,
        cascades: u8,
        near: f32,
        far: f32,
        lambda: f32,
    );
}
impl filament_LightManager_ShadowCascades {
    #[inline]
    pub unsafe fn computeUniformSplits(splitPositions: *mut f32, cascades: u8) {
        filament_LightManager_ShadowCascades_computeUniformSplits(splitPositions, cascades)
    }
    #[inline]
    pub unsafe fn computeLogSplits(splitPositions: *mut f32, cascades: u8, near: f32, far: f32) {
        filament_LightManager_ShadowCascades_computeLogSplits(splitPositions, cascades, near, far)
    }
    #[inline]
    pub unsafe fn computePracticalSplits(
        splitPositions: *mut f32,
        cascades: u8,
        near: f32,
        far: f32,
        lambda: f32,
    ) {
        filament_LightManager_ShadowCascades_computePracticalSplits(
            splitPositions,
            cascades,
            near,
            far,
            lambda,
        )
    }
}
#[doc = "! Use Builder to construct a Light object instance"]
#[repr(C)]
#[derive(Debug)]
pub struct filament_LightManager_Builder {
    pub _base: filament_BuilderBase<filament_LightManager_BuilderDetails>,
}
pub const filament_LightManager_Builder_Result_Error: filament_LightManager_Builder_Result = -1;
pub const filament_LightManager_Builder_Result_Success: filament_LightManager_Builder_Result = 0;
pub type filament_LightManager_Builder_Result = ::std::os::raw::c_int;
#[test]
fn bindgen_test_layout_filament_LightManager_Builder() {
    assert_eq!(
        ::std::mem::size_of::<filament_LightManager_Builder>(),
        8usize,
        concat!("Size of: ", stringify!(filament_LightManager_Builder))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_LightManager_Builder>(),
        8usize,
        concat!("Alignment of ", stringify!(filament_LightManager_Builder))
    );
}
extern "C" {
    #[doc = " Enables or disables a light channel. Light channel 0 is enabled by default."]
    #[doc = ""]
    #[doc = " @param channel Light channel to enable or disable, between 0 and 7."]
    #[doc = " @param enable Whether to enable or disable the light channel."]
    #[doc = " @return This Builder, for chaining calls."]
    #[link_name = "\u{1}_ZN8filament12LightManager7Builder12lightChannelEjb"]
    pub fn filament_LightManager_Builder_lightChannel(
        this: *mut filament_LightManager_Builder,
        channel: ::std::os::raw::c_uint,
        enable: bool,
    ) -> *mut filament_LightManager_Builder;
}
extern "C" {
    #[doc = " Whether this Light casts shadows (disabled by default)"]
    #[doc = ""]
    #[doc = " @param enable Enables or disables casting shadows from this Light."]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls."]
    #[doc = ""]
    #[doc = " @warning"]
    #[doc = " - Only a Type.DIRECTIONAL, Type.SUN, Type.SPOT, or Type.FOCUSED_SPOT light can cast shadows"]
    #[link_name = "\u{1}_ZN8filament12LightManager7Builder11castShadowsEb"]
    pub fn filament_LightManager_Builder_castShadows(
        this: *mut filament_LightManager_Builder,
        enable: bool,
    ) -> *mut filament_LightManager_Builder;
}
extern "C" {
    #[doc = " Sets the shadow-map options for this light."]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls."]
    #[link_name = "\u{1}_ZN8filament12LightManager7Builder13shadowOptionsERKNS0_13ShadowOptionsE"]
    pub fn filament_LightManager_Builder_shadowOptions(
        this: *mut filament_LightManager_Builder,
        options: *const filament_LightManager_ShadowOptions,
    ) -> *mut filament_LightManager_Builder;
}
extern "C" {
    #[doc = " Whether this light casts light (enabled by default)"]
    #[doc = ""]
    #[doc = " @param enable Enables or disables lighting from this Light."]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls."]
    #[doc = ""]
    #[doc = " @note"]
    #[doc = " In some situations it can be useful to have a light in the scene that doesn't"]
    #[doc = " actually emit light, but does cast shadows."]
    #[link_name = "\u{1}_ZN8filament12LightManager7Builder9castLightEb"]
    pub fn filament_LightManager_Builder_castLight(
        this: *mut filament_LightManager_Builder,
        enable: bool,
    ) -> *mut filament_LightManager_Builder;
}
extern "C" {
    #[doc = " Sets the initial position of the light in world space."]
    #[doc = ""]
    #[doc = " @param position Light's position in world space. The default is at the origin."]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls."]
    #[doc = ""]
    #[doc = " @note"]
    #[doc = " The Light's position is ignored for directional lights (Type.DIRECTIONAL or Type.SUN)"]
    #[link_name = "\u{1}_ZN8filament12LightManager7Builder8positionERKNS_4math7details5TVec3IfEE"]
    pub fn filament_LightManager_Builder_position(
        this: *mut filament_LightManager_Builder,
        position: *const filament_math_float3,
    ) -> *mut filament_LightManager_Builder;
}
extern "C" {
    #[doc = " Sets the initial direction of a light in world space."]
    #[doc = ""]
    #[doc = " @param direction Light's direction in world space. Should be a unit vector."]
    #[doc = "                  The default is {0,-1,0}."]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls."]
    #[doc = ""]
    #[doc = " @note"]
    #[doc = " The Light's direction is ignored for Type.POINT lights."]
    #[link_name = "\u{1}_ZN8filament12LightManager7Builder9directionERKNS_4math7details5TVec3IfEE"]
    pub fn filament_LightManager_Builder_direction(
        this: *mut filament_LightManager_Builder,
        direction: *const filament_math_float3,
    ) -> *mut filament_LightManager_Builder;
}
extern "C" {
    #[doc = " Sets the initial color of a light."]
    #[doc = ""]
    #[doc = " @param color Color of the light specified in the linear sRGB color-space."]
    #[doc = "              The default is white {1,1,1}."]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls."]
    #[link_name = "\u{1}_ZN8filament12LightManager7Builder5colorERKNS_4math7details5TVec3IfEE"]
    pub fn filament_LightManager_Builder_color(
        this: *mut filament_LightManager_Builder,
        color: *const filament_LinearColor,
    ) -> *mut filament_LightManager_Builder;
}
extern "C" {
    #[doc = " Sets the initial intensity of a light."]
    #[doc = " @param intensity This parameter depends on the Light.Type:"]
    #[doc = "                  - For directional lights, it specifies the illuminance in *lux*"]
    #[doc = "                  (or *lumen/m^2*)."]
    #[doc = "                  - For point lights and spot lights, it specifies the luminous power"]
    #[doc = "                  in *lumen*."]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls."]
    #[doc = ""]
    #[doc = " For example, the sun's illuminance is about 100,000 lux."]
    #[doc = ""]
    #[doc = " This method overrides any prior calls to intensity or intensityCandela."]
    #[doc = ""]
    #[link_name = "\u{1}_ZN8filament12LightManager7Builder9intensityEf"]
    pub fn filament_LightManager_Builder_intensity(
        this: *mut filament_LightManager_Builder,
        intensity: f32,
    ) -> *mut filament_LightManager_Builder;
}
extern "C" {
    #[doc = " Sets the initial intensity of a spot or point light in candela."]
    #[doc = ""]
    #[doc = " @param intensity Luminous intensity in *candela*."]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls."]
    #[doc = ""]
    #[doc = " @note"]
    #[doc = " This method is equivalent to calling intensity(float intensity) for directional lights"]
    #[doc = " (Type.DIRECTIONAL or Type.SUN)."]
    #[doc = ""]
    #[doc = " This method overrides any prior calls to intensity or intensityCandela."]
    #[link_name = "\u{1}_ZN8filament12LightManager7Builder16intensityCandelaEf"]
    pub fn filament_LightManager_Builder_intensityCandela(
        this: *mut filament_LightManager_Builder,
        intensity: f32,
    ) -> *mut filament_LightManager_Builder;
}
extern "C" {
    #[doc = " Sets the initial intensity of a light in watts."]
    #[doc = ""]
    #[doc = " @param watts         Energy consumed by a lightbulb. It is related to the energy produced"]
    #[doc = "                      and ultimately the brightness by the \\p efficiency parameter."]
    #[doc = "                      This value is often available on the packaging of commercial"]
    #[doc = "                      lightbulbs."]
    #[doc = ""]
    #[doc = " @param efficiency    Efficiency in percent. This depends on the type of lightbulb used."]
    #[doc = ""]
    #[doc = "  Lightbulb type  | Efficiency"]
    #[doc = " ----------------:|-----------:"]
    #[doc = "     Incandescent |  2.2%"]
    #[doc = "         Halogen  |  7.0%"]
    #[doc = "             LED  |  8.7%"]
    #[doc = "     Fluorescent  | 10.7%"]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls."]
    #[doc = ""]
    #[doc = ""]
    #[doc = " @note"]
    #[doc = " This call is equivalent to `Builder::intensity(efficiency * 683 * watts);`"]
    #[doc = ""]
    #[doc = " This method overrides any prior calls to intensity or intensityCandela."]
    #[link_name = "\u{1}_ZN8filament12LightManager7Builder9intensityEff"]
    pub fn filament_LightManager_Builder_intensity1(
        this: *mut filament_LightManager_Builder,
        watts: f32,
        efficiency: f32,
    ) -> *mut filament_LightManager_Builder;
}
extern "C" {
    #[doc = " Set the falloff distance for point lights and spot lights."]
    #[doc = ""]
    #[doc = " At the falloff distance, the light has no more effect on objects."]
    #[doc = ""]
    #[doc = " The falloff distance essentially defines a *sphere of influence* around the light, and"]
    #[doc = " therefore has an impact on performance. Larger falloffs might reduce performance"]
    #[doc = " significantly, especially when many lights are used."]
    #[doc = ""]
    #[doc = " Try to avoid having a large number of light's spheres of influence overlap."]
    #[doc = ""]
    #[doc = " @param radius Falloff distance in world units. Default is 1 meter."]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls."]
    #[doc = ""]
    #[doc = " @note"]
    #[doc = " The Light's falloff is ignored for directional lights (Type.DIRECTIONAL or Type.SUN)"]
    #[link_name = "\u{1}_ZN8filament12LightManager7Builder7falloffEf"]
    pub fn filament_LightManager_Builder_falloff(
        this: *mut filament_LightManager_Builder,
        radius: f32,
    ) -> *mut filament_LightManager_Builder;
}
extern "C" {
    #[doc = " Defines a spot light'st angular falloff attenuation."]
    #[doc = ""]
    #[doc = " A spot light is defined by a position, a direction and two cones, \\p inner and \\p outer."]
    #[doc = " These two cones are used to define the angular falloff attenuation of the spot light"]
    #[doc = " and are defined by the angle from the center axis to where the falloff begins (i.e."]
    #[doc = " cones are defined by their half-angle)."]
    #[doc = ""]
    #[doc = " Both inner and outer are silently clamped to a minimum value of 0.5 degrees"]
    #[doc = " (~0.00873 radians) to avoid floating-point precision issues during rendering."]
    #[doc = ""]
    #[doc = " @param inner inner cone angle in *radians* between 0.00873 and \\p outer"]
    #[doc = " @param outer outer cone angle in *radians* between 0.00873 inner and @f$ \\pi/2 @f$"]
    #[doc = " @return This Builder, for chaining calls."]
    #[doc = ""]
    #[doc = " @note"]
    #[doc = " The spot light cone is ignored for directional and point lights."]
    #[doc = ""]
    #[doc = " @see Type.SPOT, Type.FOCUSED_SPOT"]
    #[link_name = "\u{1}_ZN8filament12LightManager7Builder13spotLightConeEff"]
    pub fn filament_LightManager_Builder_spotLightCone(
        this: *mut filament_LightManager_Builder,
        inner: f32,
        outer: f32,
    ) -> *mut filament_LightManager_Builder;
}
extern "C" {
    #[doc = " Defines the angular radius of the sun, in degrees, between 0.25° and 20.0°"]
    #[doc = ""]
    #[doc = " The Sun as seen from Earth has an angular size of 0.526° to 0.545°"]
    #[doc = ""]
    #[doc = " @param angularRadius sun's radius in degree. Default is 0.545°."]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls."]
    #[link_name = "\u{1}_ZN8filament12LightManager7Builder16sunAngularRadiusEf"]
    pub fn filament_LightManager_Builder_sunAngularRadius(
        this: *mut filament_LightManager_Builder,
        angularRadius: f32,
    ) -> *mut filament_LightManager_Builder;
}
extern "C" {
    #[doc = " Defines the halo radius of the sun. The radius of the halo is defined as a"]
    #[doc = " multiplier of the sun angular radius."]
    #[doc = ""]
    #[doc = " @param haloSize radius multiplier. Default is 10.0."]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls."]
    #[link_name = "\u{1}_ZN8filament12LightManager7Builder11sunHaloSizeEf"]
    pub fn filament_LightManager_Builder_sunHaloSize(
        this: *mut filament_LightManager_Builder,
        haloSize: f32,
    ) -> *mut filament_LightManager_Builder;
}
extern "C" {
    #[doc = " Defines the halo falloff of the sun. The falloff is a dimensionless number"]
    #[doc = " used as an exponent."]
    #[doc = ""]
    #[doc = " @param haloFalloff halo falloff. Default is 80.0."]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls."]
    #[link_name = "\u{1}_ZN8filament12LightManager7Builder14sunHaloFalloffEf"]
    pub fn filament_LightManager_Builder_sunHaloFalloff(
        this: *mut filament_LightManager_Builder,
        haloFalloff: f32,
    ) -> *mut filament_LightManager_Builder;
}
extern "C" {
    #[doc = " Adds the Light component to an entity."]
    #[doc = ""]
    #[doc = " @param engine Reference to the filament::Engine to associate this light with."]
    #[doc = " @param entity Entity to add the light component to."]
    #[doc = " @return Success if the component was created successfully, Error otherwise."]
    #[doc = ""]
    #[doc = " If exceptions are disabled and an error occurs, this function is a no-op."]
    #[doc = "        Success can be checked by looking at the return value."]
    #[doc = ""]
    #[doc = " If this component already exists on the given entity, it is first destroyed as if"]
    #[doc = " destroy(utils::Entity e) was called."]
    #[doc = ""]
    #[doc = " @warning"]
    #[doc = " Currently, only 2048 lights can be created on a given Engine."]
    #[doc = ""]
    #[doc = " @exception utils::PostConditionPanic if a runtime error occurred, such as running out of"]
    #[doc = "            memory or other resources."]
    #[doc = " @exception utils::PreConditionPanic if a parameter to a builder function was invalid."]
    #[link_name = "\u{1}_ZN8filament12LightManager7Builder5buildERNS_6EngineEN5utils6EntityE"]
    pub fn filament_LightManager_Builder_build(
        this: *mut filament_LightManager_Builder,
        engine: *mut filament_Engine,
        entity: utils_Entity,
    ) -> filament_LightManager_Builder_Result;
}
extern "C" {
    #[doc = " Creates a light builder and set the light's #Type."]
    #[doc = ""]
    #[doc = " @param type #Type of Light object to create."]
    #[link_name = "\u{1}_ZN8filament12LightManager7BuilderC1ENS0_4TypeE"]
    pub fn filament_LightManager_Builder_Builder(
        this: *mut filament_LightManager_Builder,
        type_: filament_LightManager_Type,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament12LightManager7BuilderC1ERKS1_"]
    pub fn filament_LightManager_Builder_Builder1(
        this: *mut filament_LightManager_Builder,
        rhs: *const filament_LightManager_Builder,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament12LightManager7BuilderC1EOS1_"]
    pub fn filament_LightManager_Builder_Builder2(
        this: *mut filament_LightManager_Builder,
        rhs: *mut filament_LightManager_Builder,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament12LightManager7BuilderD1Ev"]
    pub fn filament_LightManager_Builder_Builder_destructor(
        this: *mut filament_LightManager_Builder,
    );
}
impl Default for filament_LightManager_Builder {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl filament_LightManager_Builder {
    #[inline]
    pub unsafe fn lightChannel(
        &mut self,
        channel: ::std::os::raw::c_uint,
        enable: bool,
    ) -> *mut filament_LightManager_Builder {
        filament_LightManager_Builder_lightChannel(self, channel, enable)
    }
    #[inline]
    pub unsafe fn castShadows(&mut self, enable: bool) -> *mut filament_LightManager_Builder {
        filament_LightManager_Builder_castShadows(self, enable)
    }
    #[inline]
    pub unsafe fn shadowOptions(
        &mut self,
        options: *const filament_LightManager_ShadowOptions,
    ) -> *mut filament_LightManager_Builder {
        filament_LightManager_Builder_shadowOptions(self, options)
    }
    #[inline]
    pub unsafe fn castLight(&mut self, enable: bool) -> *mut filament_LightManager_Builder {
        filament_LightManager_Builder_castLight(self, enable)
    }
    #[inline]
    pub unsafe fn position(
        &mut self,
        position: *const filament_math_float3,
    ) -> *mut filament_LightManager_Builder {
        filament_LightManager_Builder_position(self, position)
    }
    #[inline]
    pub unsafe fn direction(
        &mut self,
        direction: *const filament_math_float3,
    ) -> *mut filament_LightManager_Builder {
        filament_LightManager_Builder_direction(self, direction)
    }
    #[inline]
    pub unsafe fn color(
        &mut self,
        color: *const filament_LinearColor,
    ) -> *mut filament_LightManager_Builder {
        filament_LightManager_Builder_color(self, color)
    }
    #[inline]
    pub unsafe fn intensity(&mut self, intensity: f32) -> *mut filament_LightManager_Builder {
        filament_LightManager_Builder_intensity(self, intensity)
    }
    #[inline]
    pub unsafe fn intensityCandela(
        &mut self,
        intensity: f32,
    ) -> *mut filament_LightManager_Builder {
        filament_LightManager_Builder_intensityCandela(self, intensity)
    }
    #[inline]
    pub unsafe fn intensity1(
        &mut self,
        watts: f32,
        efficiency: f32,
    ) -> *mut filament_LightManager_Builder {
        filament_LightManager_Builder_intensity1(self, watts, efficiency)
    }
    #[inline]
    pub unsafe fn falloff(&mut self, radius: f32) -> *mut filament_LightManager_Builder {
        filament_LightManager_Builder_falloff(self, radius)
    }
    #[inline]
    pub unsafe fn spotLightCone(
        &mut self,
        inner: f32,
        outer: f32,
    ) -> *mut filament_LightManager_Builder {
        filament_LightManager_Builder_spotLightCone(self, inner, outer)
    }
    #[inline]
    pub unsafe fn sunAngularRadius(
        &mut self,
        angularRadius: f32,
    ) -> *mut filament_LightManager_Builder {
        filament_LightManager_Builder_sunAngularRadius(self, angularRadius)
    }
    #[inline]
    pub unsafe fn sunHaloSize(&mut self, haloSize: f32) -> *mut filament_LightManager_Builder {
        filament_LightManager_Builder_sunHaloSize(self, haloSize)
    }
    #[inline]
    pub unsafe fn sunHaloFalloff(
        &mut self,
        haloFalloff: f32,
    ) -> *mut filament_LightManager_Builder {
        filament_LightManager_Builder_sunHaloFalloff(self, haloFalloff)
    }
    #[inline]
    pub unsafe fn build(
        &mut self,
        engine: *mut filament_Engine,
        entity: utils_Entity,
    ) -> filament_LightManager_Builder_Result {
        filament_LightManager_Builder_build(self, engine, entity)
    }
    #[inline]
    pub unsafe fn new(type_: filament_LightManager_Type) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_LightManager_Builder_Builder(__bindgen_tmp.as_mut_ptr(), type_);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(rhs: *const filament_LightManager_Builder) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_LightManager_Builder_Builder1(__bindgen_tmp.as_mut_ptr(), rhs);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new2(rhs: *mut filament_LightManager_Builder) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_LightManager_Builder_Builder2(__bindgen_tmp.as_mut_ptr(), rhs);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        filament_LightManager_Builder_Builder_destructor(self)
    }
}
#[doc = "!< Typical efficiency of an incandescent light bulb (2.2%)"]
pub const filament_LightManager_EFFICIENCY_INCANDESCENT: f32 = 0.02199999988079071;
#[doc = "!< Typical efficiency of an halogen light bulb (7.0%)"]
pub const filament_LightManager_EFFICIENCY_HALOGEN: f32 = 0.07069999724626541;
#[doc = "!< Typical efficiency of a fluorescent light bulb (8.7%)"]
pub const filament_LightManager_EFFICIENCY_FLUORESCENT: f32 = 0.08780000358819962;
#[doc = "!< Typical efficiency of a LED light bulb (11.7%)"]
pub const filament_LightManager_EFFICIENCY_LED: f32 = 0.11710000038146973;
#[test]
fn bindgen_test_layout_filament_LightManager() {
    assert_eq!(
        ::std::mem::size_of::<filament_LightManager>(),
        1usize,
        concat!("Size of: ", stringify!(filament_LightManager))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_LightManager>(),
        1usize,
        concat!("Alignment of ", stringify!(filament_LightManager))
    );
}
extern "C" {
    #[doc = " Returns the number of component in the LightManager, not that component are not"]
    #[doc = " guaranteed to be active. Use the EntityManager::isAlive() before use if needed."]
    #[doc = ""]
    #[doc = " @return number of component in the LightManager"]
    #[link_name = "\u{1}_ZNK8filament12LightManager17getComponentCountEv"]
    pub fn filament_LightManager_getComponentCount(this: *const filament_LightManager) -> size_t;
}
extern "C" {
    #[doc = " Returns the list of Entity for all components. Use getComponentCount() to know the size"]
    #[doc = " of the list."]
    #[doc = " @return a pointer to Entity"]
    #[link_name = "\u{1}_ZNK8filament12LightManager11getEntitiesEv"]
    pub fn filament_LightManager_getEntities(
        this: *const filament_LightManager,
    ) -> *const utils_Entity;
}
extern "C" {
    #[doc = " Returns whether a particular Entity is associated with a component of this LightManager"]
    #[doc = " @param e An Entity."]
    #[doc = " @return true if this Entity has a component associated with this manager."]
    #[link_name = "\u{1}_ZNK8filament12LightManager12hasComponentEN5utils6EntityE"]
    pub fn filament_LightManager_hasComponent(
        this: *const filament_LightManager,
        e: utils_Entity,
    ) -> bool;
}
extern "C" {
    #[doc = " Gets an Instance representing the Light component associated with the given Entity."]
    #[doc = " @param e An Entity."]
    #[doc = " @return An Instance object, which represents the Light component associated with the Entity e."]
    #[doc = " @note Use Instance::isValid() to make sure the component exists."]
    #[doc = " @see hasComponent()"]
    #[link_name = "\u{1}_ZNK8filament12LightManager11getInstanceEN5utils6EntityE"]
    pub fn filament_LightManager_getInstance(
        this: *const filament_LightManager,
        e: utils_Entity,
    ) -> filament_LightManager_Instance;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament12LightManager7destroyEN5utils6EntityE"]
    pub fn filament_LightManager_destroy(this: *mut filament_LightManager, e: utils_Entity);
}
extern "C" {
    #[link_name = "\u{1}_ZNK8filament12LightManager7getTypeEN5utils14EntityInstanceIS0_Lb0EEE"]
    pub fn filament_LightManager_getType(
        this: *const filament_LightManager,
        i: filament_LightManager_Instance,
    ) -> filament_LightManager_Type;
}
extern "C" {
    #[doc = " Enables or disables a light channel. Light channel 0 is enabled by default."]
    #[doc = " @param channel light channel to enable or disable, between 0 and 7."]
    #[doc = " @param enable whether to enable (true) or disable (false) the specified light channel."]
    #[link_name = "\u{1}_ZN8filament12LightManager15setLightChannelEN5utils14EntityInstanceIS0_Lb0EEEjb"]
    pub fn filament_LightManager_setLightChannel(
        this: *mut filament_LightManager,
        i: filament_LightManager_Instance,
        channel: ::std::os::raw::c_uint,
        enable: bool,
    );
}
extern "C" {
    #[doc = " Returns whether a light channel is enabled on a specified light."]
    #[doc = " @param i        Instance of the component obtained from getInstance()."]
    #[doc = " @param channel  Light channel to query"]
    #[doc = " @return         true if the light channel is enabled, false otherwise"]
    #[link_name = "\u{1}_ZNK8filament12LightManager15getLightChannelEN5utils14EntityInstanceIS0_Lb0EEEj"]
    pub fn filament_LightManager_getLightChannel(
        this: *const filament_LightManager,
        i: filament_LightManager_Instance,
        channel: ::std::os::raw::c_uint,
    ) -> bool;
}
extern "C" {
    #[doc = " Dynamically updates the light's position."]
    #[doc = ""]
    #[doc = " @param i        Instance of the component obtained from getInstance()."]
    #[doc = " @param position Light's position in world space. The default is at the origin."]
    #[doc = ""]
    #[doc = " @see Builder.position()"]
    #[link_name = "\u{1}_ZN8filament12LightManager11setPositionEN5utils14EntityInstanceIS0_Lb0EEERKNS_4math7details5TVec3IfEE"]
    pub fn filament_LightManager_setPosition(
        this: *mut filament_LightManager,
        i: filament_LightManager_Instance,
        position: *const filament_math_float3,
    );
}
extern "C" {
    #[doc = "! returns the light's position in world space"]
    #[link_name = "\u{1}_ZNK8filament12LightManager11getPositionEN5utils14EntityInstanceIS0_Lb0EEE"]
    pub fn filament_LightManager_getPosition(
        this: *const filament_LightManager,
        i: filament_LightManager_Instance,
    ) -> *const filament_math_float3;
}
extern "C" {
    #[doc = " Dynamically updates the light's direction"]
    #[doc = ""]
    #[doc = " @param i         Instance of the component obtained from getInstance()."]
    #[doc = " @param direction Light's direction in world space. Should be a unit vector."]
    #[doc = "                  The default is {0,-1,0}."]
    #[doc = ""]
    #[doc = " @see Builder.direction()"]
    #[link_name = "\u{1}_ZN8filament12LightManager12setDirectionEN5utils14EntityInstanceIS0_Lb0EEERKNS_4math7details5TVec3IfEE"]
    pub fn filament_LightManager_setDirection(
        this: *mut filament_LightManager,
        i: filament_LightManager_Instance,
        direction: *const filament_math_float3,
    );
}
extern "C" {
    #[doc = "! returns the light's direction in world space"]
    #[link_name = "\u{1}_ZNK8filament12LightManager12getDirectionEN5utils14EntityInstanceIS0_Lb0EEE"]
    pub fn filament_LightManager_getDirection(
        this: *const filament_LightManager,
        i: filament_LightManager_Instance,
    ) -> *const filament_math_float3;
}
extern "C" {
    #[doc = " Dynamically updates the light's hue as linear sRGB"]
    #[doc = ""]
    #[doc = " @param i     Instance of the component obtained from getInstance()."]
    #[doc = " @param color Color of the light specified in the linear sRGB color-space."]
    #[doc = "              The default is white {1,1,1}."]
    #[doc = ""]
    #[doc = " @see Builder.color(), getInstance()"]
    #[link_name = "\u{1}_ZN8filament12LightManager8setColorEN5utils14EntityInstanceIS0_Lb0EEERKNS_4math7details5TVec3IfEE"]
    pub fn filament_LightManager_setColor(
        this: *mut filament_LightManager,
        i: filament_LightManager_Instance,
        color: *const filament_LinearColor,
    );
}
extern "C" {
    #[doc = " @param i     Instance of the component obtained from getInstance()."]
    #[doc = " @return the light's color in linear sRGB"]
    #[link_name = "\u{1}_ZNK8filament12LightManager8getColorEN5utils14EntityInstanceIS0_Lb0EEE"]
    pub fn filament_LightManager_getColor(
        this: *const filament_LightManager,
        i: filament_LightManager_Instance,
    ) -> *const filament_math_float3;
}
extern "C" {
    #[doc = " Dynamically updates the light's intensity. The intensity can be negative."]
    #[doc = ""]
    #[doc = " @param i         Instance of the component obtained from getInstance()."]
    #[doc = " @param intensity This parameter depends on the Light.Type:"]
    #[doc = "                  - For directional lights, it specifies the illuminance in *lux*"]
    #[doc = "                  (or *lumen/m^2*)."]
    #[doc = "                  - For point lights and spot lights, it specifies the luminous power"]
    #[doc = "                  in *lumen*."]
    #[doc = ""]
    #[doc = " @see Builder.intensity()"]
    #[link_name = "\u{1}_ZN8filament12LightManager12setIntensityEN5utils14EntityInstanceIS0_Lb0EEEf"]
    pub fn filament_LightManager_setIntensity(
        this: *mut filament_LightManager,
        i: filament_LightManager_Instance,
        intensity: f32,
    );
}
extern "C" {
    #[doc = " Dynamically updates the light's intensity in candela. The intensity can be negative."]
    #[doc = ""]
    #[doc = " @param i         Instance of the component obtained from getInstance()."]
    #[doc = " @param intensity Luminous intensity in *candela*."]
    #[doc = ""]
    #[doc = " @note"]
    #[doc = " This method is equivalent to calling setIntensity(float intensity) for directional lights"]
    #[doc = " (Type.DIRECTIONAL or Type.SUN)."]
    #[doc = ""]
    #[doc = " @see Builder.intensityCandela(float intensity)"]
    #[link_name = "\u{1}_ZN8filament12LightManager19setIntensityCandelaEN5utils14EntityInstanceIS0_Lb0EEEf"]
    pub fn filament_LightManager_setIntensityCandela(
        this: *mut filament_LightManager,
        i: filament_LightManager_Instance,
        intensity: f32,
    );
}
extern "C" {
    #[doc = " returns the light's luminous intensity in candela."]
    #[doc = ""]
    #[doc = " @param i     Instance of the component obtained from getInstance()."]
    #[doc = ""]
    #[doc = " @note for Type.FOCUSED_SPOT lights, the returned value depends on the \\p outer cone angle."]
    #[doc = ""]
    #[doc = " @return luminous intensity in candela."]
    #[link_name = "\u{1}_ZNK8filament12LightManager12getIntensityEN5utils14EntityInstanceIS0_Lb0EEE"]
    pub fn filament_LightManager_getIntensity(
        this: *const filament_LightManager,
        i: filament_LightManager_Instance,
    ) -> f32;
}
extern "C" {
    #[doc = " Set the falloff distance for point lights and spot lights."]
    #[doc = ""]
    #[doc = " @param i      Instance of the component obtained from getInstance()."]
    #[doc = " @param radius falloff distance in world units. Default is 1 meter."]
    #[doc = ""]
    #[doc = " @see Builder.falloff()"]
    #[link_name = "\u{1}_ZN8filament12LightManager10setFalloffEN5utils14EntityInstanceIS0_Lb0EEEf"]
    pub fn filament_LightManager_setFalloff(
        this: *mut filament_LightManager,
        i: filament_LightManager_Instance,
        radius: f32,
    );
}
extern "C" {
    #[doc = " returns the falloff distance of this light."]
    #[doc = " @param i     Instance of the component obtained from getInstance()."]
    #[doc = " @return the falloff distance of this light."]
    #[link_name = "\u{1}_ZNK8filament12LightManager10getFalloffEN5utils14EntityInstanceIS0_Lb0EEE"]
    pub fn filament_LightManager_getFalloff(
        this: *const filament_LightManager,
        i: filament_LightManager_Instance,
    ) -> f32;
}
extern "C" {
    #[doc = " Dynamically updates a spot light's cone as angles"]
    #[doc = ""]
    #[doc = " @param i     Instance of the component obtained from getInstance()."]
    #[doc = " @param inner inner cone angle in *radians* between 0.00873 and outer"]
    #[doc = " @param outer outer cone angle in *radians* between 0.00873 and pi/2"]
    #[doc = ""]
    #[doc = " @see Builder.spotLightCone()"]
    #[link_name = "\u{1}_ZN8filament12LightManager16setSpotLightConeEN5utils14EntityInstanceIS0_Lb0EEEff"]
    pub fn filament_LightManager_setSpotLightCone(
        this: *mut filament_LightManager,
        i: filament_LightManager_Instance,
        inner: f32,
        outer: f32,
    );
}
extern "C" {
    #[doc = " returns the outer cone angle in *radians* between inner and pi/2."]
    #[doc = " @param i     Instance of the component obtained from getInstance()."]
    #[doc = " @return the outer cone angle of this light."]
    #[link_name = "\u{1}_ZNK8filament12LightManager21getSpotLightOuterConeEN5utils14EntityInstanceIS0_Lb0EEE"]
    pub fn filament_LightManager_getSpotLightOuterCone(
        this: *const filament_LightManager,
        i: filament_LightManager_Instance,
    ) -> f32;
}
extern "C" {
    #[doc = " returns the inner cone angle in *radians* between 0 and pi/2."]
    #[doc = ""]
    #[doc = " The value is recomputed from the initial values, thus is not precisely"]
    #[doc = " the same as the one passed to setSpotLightCone() or Builder.spotLightCone()."]
    #[doc = ""]
    #[doc = " @param i     Instance of the component obtained from getInstance()."]
    #[doc = " @return the inner cone angle of this light."]
    #[link_name = "\u{1}_ZNK8filament12LightManager21getSpotLightInnerConeEN5utils14EntityInstanceIS0_Lb0EEE"]
    pub fn filament_LightManager_getSpotLightInnerCone(
        this: *const filament_LightManager,
        i: filament_LightManager_Instance,
    ) -> f32;
}
extern "C" {
    #[doc = " Dynamically updates the angular radius of a Type.SUN light"]
    #[doc = ""]
    #[doc = " The Sun as seen from Earth has an angular size of 0.526° to 0.545°"]
    #[doc = ""]
    #[doc = " @param i     Instance of the component obtained from getInstance()."]
    #[doc = " @param angularRadius sun's radius in degrees. Default is 0.545°."]
    #[link_name = "\u{1}_ZN8filament12LightManager19setSunAngularRadiusEN5utils14EntityInstanceIS0_Lb0EEEf"]
    pub fn filament_LightManager_setSunAngularRadius(
        this: *mut filament_LightManager,
        i: filament_LightManager_Instance,
        angularRadius: f32,
    );
}
extern "C" {
    #[doc = " returns the angular radius if the sun in degrees."]
    #[doc = " @param i     Instance of the component obtained from getInstance()."]
    #[doc = " @return the angular radius if the sun in degrees."]
    #[link_name = "\u{1}_ZNK8filament12LightManager19getSunAngularRadiusEN5utils14EntityInstanceIS0_Lb0EEE"]
    pub fn filament_LightManager_getSunAngularRadius(
        this: *const filament_LightManager,
        i: filament_LightManager_Instance,
    ) -> f32;
}
extern "C" {
    #[doc = " Dynamically updates the halo radius of a Type.SUN light. The radius"]
    #[doc = " of the halo is defined as a multiplier of the sun angular radius."]
    #[doc = ""]
    #[doc = " @param i     Instance of the component obtained from getInstance()."]
    #[doc = " @param haloSize radius multiplier. Default is 10.0."]
    #[link_name = "\u{1}_ZN8filament12LightManager14setSunHaloSizeEN5utils14EntityInstanceIS0_Lb0EEEf"]
    pub fn filament_LightManager_setSunHaloSize(
        this: *mut filament_LightManager,
        i: filament_LightManager_Instance,
        haloSize: f32,
    );
}
extern "C" {
    #[doc = " returns the halo size of a Type.SUN light as a multiplier of the"]
    #[doc = " sun angular radius."]
    #[doc = " @param i     Instance of the component obtained from getInstance()."]
    #[doc = " @return the halo size"]
    #[link_name = "\u{1}_ZNK8filament12LightManager14getSunHaloSizeEN5utils14EntityInstanceIS0_Lb0EEE"]
    pub fn filament_LightManager_getSunHaloSize(
        this: *const filament_LightManager,
        i: filament_LightManager_Instance,
    ) -> f32;
}
extern "C" {
    #[doc = " Dynamically updates the halo falloff of a Type.SUN light. The falloff"]
    #[doc = " is a dimensionless number used as an exponent."]
    #[doc = ""]
    #[doc = " @param i     Instance of the component obtained from getInstance()."]
    #[doc = " @param haloFalloff halo falloff. Default is 80.0."]
    #[link_name = "\u{1}_ZN8filament12LightManager17setSunHaloFalloffEN5utils14EntityInstanceIS0_Lb0EEEf"]
    pub fn filament_LightManager_setSunHaloFalloff(
        this: *mut filament_LightManager,
        i: filament_LightManager_Instance,
        haloFalloff: f32,
    );
}
extern "C" {
    #[doc = " returns the halo falloff of a Type.SUN light as a dimensionless value."]
    #[doc = " @param i     Instance of the component obtained from getInstance()."]
    #[doc = " @return the halo falloff"]
    #[link_name = "\u{1}_ZNK8filament12LightManager17getSunHaloFalloffEN5utils14EntityInstanceIS0_Lb0EEE"]
    pub fn filament_LightManager_getSunHaloFalloff(
        this: *const filament_LightManager,
        i: filament_LightManager_Instance,
    ) -> f32;
}
extern "C" {
    #[doc = " returns the shadow-map options for a given light"]
    #[doc = " @param i     Instance of the component obtained from getInstance()."]
    #[doc = " @return      A ShadowOption structure"]
    #[link_name = "\u{1}_ZNK8filament12LightManager16getShadowOptionsEN5utils14EntityInstanceIS0_Lb0EEE"]
    pub fn filament_LightManager_getShadowOptions(
        this: *const filament_LightManager,
        i: filament_LightManager_Instance,
    ) -> *const filament_LightManager_ShadowOptions;
}
extern "C" {
    #[doc = " sets the shadow-map options for a given light"]
    #[doc = " @param i     Instance of the component obtained from getInstance()."]
    #[doc = " @param options  A ShadowOption structure"]
    #[link_name = "\u{1}_ZN8filament12LightManager16setShadowOptionsEN5utils14EntityInstanceIS0_Lb0EEERKNS0_13ShadowOptionsE"]
    pub fn filament_LightManager_setShadowOptions(
        this: *mut filament_LightManager,
        i: filament_LightManager_Instance,
        options: *const filament_LightManager_ShadowOptions,
    );
}
extern "C" {
    #[doc = " Whether this Light casts shadows (disabled by default)"]
    #[doc = ""]
    #[doc = " @param i     Instance of the component obtained from getInstance()."]
    #[doc = " @param shadowCaster Enables or disables casting shadows from this Light."]
    #[doc = ""]
    #[doc = " @warning"]
    #[doc = " - Only a Type.DIRECTIONAL, Type.SUN, Type.SPOT, or Type.FOCUSED_SPOT light can cast shadows"]
    #[link_name = "\u{1}_ZN8filament12LightManager15setShadowCasterEN5utils14EntityInstanceIS0_Lb0EEEb"]
    pub fn filament_LightManager_setShadowCaster(
        this: *mut filament_LightManager,
        i: filament_LightManager_Instance,
        shadowCaster: bool,
    );
}
extern "C" {
    #[doc = " returns whether this light casts shadows."]
    #[doc = " @param i     Instance of the component obtained from getInstance()."]
    #[link_name = "\u{1}_ZNK8filament12LightManager14isShadowCasterEN5utils14EntityInstanceIS0_Lb0EEE"]
    pub fn filament_LightManager_isShadowCaster(
        this: *const filament_LightManager,
        i: filament_LightManager_Instance,
    ) -> bool;
}
impl filament_LightManager {
    #[inline]
    pub unsafe fn getComponentCount(&self) -> size_t {
        filament_LightManager_getComponentCount(self)
    }
    #[inline]
    pub unsafe fn getEntities(&self) -> *const utils_Entity {
        filament_LightManager_getEntities(self)
    }
    #[inline]
    pub unsafe fn hasComponent(&self, e: utils_Entity) -> bool {
        filament_LightManager_hasComponent(self, e)
    }
    #[inline]
    pub unsafe fn getInstance(&self, e: utils_Entity) -> filament_LightManager_Instance {
        filament_LightManager_getInstance(self, e)
    }
    #[inline]
    pub unsafe fn destroy(&mut self, e: utils_Entity) {
        filament_LightManager_destroy(self, e)
    }
    #[inline]
    pub unsafe fn getType(&self, i: filament_LightManager_Instance) -> filament_LightManager_Type {
        filament_LightManager_getType(self, i)
    }
    #[inline]
    pub unsafe fn setLightChannel(
        &mut self,
        i: filament_LightManager_Instance,
        channel: ::std::os::raw::c_uint,
        enable: bool,
    ) {
        filament_LightManager_setLightChannel(self, i, channel, enable)
    }
    #[inline]
    pub unsafe fn getLightChannel(
        &self,
        i: filament_LightManager_Instance,
        channel: ::std::os::raw::c_uint,
    ) -> bool {
        filament_LightManager_getLightChannel(self, i, channel)
    }
    #[inline]
    pub unsafe fn setPosition(
        &mut self,
        i: filament_LightManager_Instance,
        position: *const filament_math_float3,
    ) {
        filament_LightManager_setPosition(self, i, position)
    }
    #[inline]
    pub unsafe fn getPosition(
        &self,
        i: filament_LightManager_Instance,
    ) -> *const filament_math_float3 {
        filament_LightManager_getPosition(self, i)
    }
    #[inline]
    pub unsafe fn setDirection(
        &mut self,
        i: filament_LightManager_Instance,
        direction: *const filament_math_float3,
    ) {
        filament_LightManager_setDirection(self, i, direction)
    }
    #[inline]
    pub unsafe fn getDirection(
        &self,
        i: filament_LightManager_Instance,
    ) -> *const filament_math_float3 {
        filament_LightManager_getDirection(self, i)
    }
    #[inline]
    pub unsafe fn setColor(
        &mut self,
        i: filament_LightManager_Instance,
        color: *const filament_LinearColor,
    ) {
        filament_LightManager_setColor(self, i, color)
    }
    #[inline]
    pub unsafe fn getColor(
        &self,
        i: filament_LightManager_Instance,
    ) -> *const filament_math_float3 {
        filament_LightManager_getColor(self, i)
    }
    #[inline]
    pub unsafe fn setIntensity(&mut self, i: filament_LightManager_Instance, intensity: f32) {
        filament_LightManager_setIntensity(self, i, intensity)
    }
    #[inline]
    pub unsafe fn setIntensityCandela(
        &mut self,
        i: filament_LightManager_Instance,
        intensity: f32,
    ) {
        filament_LightManager_setIntensityCandela(self, i, intensity)
    }
    #[inline]
    pub unsafe fn getIntensity(&self, i: filament_LightManager_Instance) -> f32 {
        filament_LightManager_getIntensity(self, i)
    }
    #[inline]
    pub unsafe fn setFalloff(&mut self, i: filament_LightManager_Instance, radius: f32) {
        filament_LightManager_setFalloff(self, i, radius)
    }
    #[inline]
    pub unsafe fn getFalloff(&self, i: filament_LightManager_Instance) -> f32 {
        filament_LightManager_getFalloff(self, i)
    }
    #[inline]
    pub unsafe fn setSpotLightCone(
        &mut self,
        i: filament_LightManager_Instance,
        inner: f32,
        outer: f32,
    ) {
        filament_LightManager_setSpotLightCone(self, i, inner, outer)
    }
    #[inline]
    pub unsafe fn getSpotLightOuterCone(&self, i: filament_LightManager_Instance) -> f32 {
        filament_LightManager_getSpotLightOuterCone(self, i)
    }
    #[inline]
    pub unsafe fn getSpotLightInnerCone(&self, i: filament_LightManager_Instance) -> f32 {
        filament_LightManager_getSpotLightInnerCone(self, i)
    }
    #[inline]
    pub unsafe fn setSunAngularRadius(
        &mut self,
        i: filament_LightManager_Instance,
        angularRadius: f32,
    ) {
        filament_LightManager_setSunAngularRadius(self, i, angularRadius)
    }
    #[inline]
    pub unsafe fn getSunAngularRadius(&self, i: filament_LightManager_Instance) -> f32 {
        filament_LightManager_getSunAngularRadius(self, i)
    }
    #[inline]
    pub unsafe fn setSunHaloSize(&mut self, i: filament_LightManager_Instance, haloSize: f32) {
        filament_LightManager_setSunHaloSize(self, i, haloSize)
    }
    #[inline]
    pub unsafe fn getSunHaloSize(&self, i: filament_LightManager_Instance) -> f32 {
        filament_LightManager_getSunHaloSize(self, i)
    }
    #[inline]
    pub unsafe fn setSunHaloFalloff(
        &mut self,
        i: filament_LightManager_Instance,
        haloFalloff: f32,
    ) {
        filament_LightManager_setSunHaloFalloff(self, i, haloFalloff)
    }
    #[inline]
    pub unsafe fn getSunHaloFalloff(&self, i: filament_LightManager_Instance) -> f32 {
        filament_LightManager_getSunHaloFalloff(self, i)
    }
    #[inline]
    pub unsafe fn getShadowOptions(
        &self,
        i: filament_LightManager_Instance,
    ) -> *const filament_LightManager_ShadowOptions {
        filament_LightManager_getShadowOptions(self, i)
    }
    #[inline]
    pub unsafe fn setShadowOptions(
        &mut self,
        i: filament_LightManager_Instance,
        options: *const filament_LightManager_ShadowOptions,
    ) {
        filament_LightManager_setShadowOptions(self, i, options)
    }
    #[inline]
    pub unsafe fn setShadowCaster(
        &mut self,
        i: filament_LightManager_Instance,
        shadowCaster: bool,
    ) {
        filament_LightManager_setShadowCaster(self, i, shadowCaster)
    }
    #[inline]
    pub unsafe fn isShadowCaster(&self, i: filament_LightManager_Instance) -> bool {
        filament_LightManager_isShadowCaster(self, i)
    }
}
#[doc = "!< no lighting applied, emissive possible"]
pub const filament_Shading_UNLIT: filament_Shading = 0;
#[doc = "!< default, standard lighting"]
pub const filament_Shading_LIT: filament_Shading = 1;
#[doc = "!< subsurface lighting model"]
pub const filament_Shading_SUBSURFACE: filament_Shading = 2;
#[doc = "!< cloth lighting model"]
pub const filament_Shading_CLOTH: filament_Shading = 3;
#[doc = "!< legacy lighting model"]
pub const filament_Shading_SPECULAR_GLOSSINESS: filament_Shading = 4;
#[doc = " Supported shading models"]
pub type filament_Shading = u8;
#[doc = "!< default, smooth interpolation"]
pub const filament_Interpolation_SMOOTH: filament_Interpolation = 0;
#[doc = "!< flat interpolation"]
pub const filament_Interpolation_FLAT: filament_Interpolation = 1;
#[doc = " Attribute interpolation types in the fragment shader"]
pub type filament_Interpolation = u8;
#[doc = "! material is opaque"]
pub const filament_BlendingMode_OPAQUE: filament_BlendingMode = 0;
#[doc = "! material is transparent and color is alpha-pre-multiplied, affects diffuse lighting only"]
pub const filament_BlendingMode_TRANSPARENT: filament_BlendingMode = 1;
#[doc = "! material is additive (e.g.: hologram)"]
pub const filament_BlendingMode_ADD: filament_BlendingMode = 2;
#[doc = "! material is masked (i.e. alpha tested)"]
pub const filament_BlendingMode_MASKED: filament_BlendingMode = 3;
#[doc = " material is transparent and color is alpha-pre-multiplied, affects specular lighting"]
#[doc = " when adding more entries, change the size of FRenderer::CommandKey::blending"]
pub const filament_BlendingMode_FADE: filament_BlendingMode = 4;
#[doc = "! material darkens what's behind it"]
pub const filament_BlendingMode_MULTIPLY: filament_BlendingMode = 5;
#[doc = "! material brightens what's behind it"]
pub const filament_BlendingMode_SCREEN: filament_BlendingMode = 6;
#[doc = " Supported blending modes"]
pub type filament_BlendingMode = u8;
#[doc = "! the transparent object is drawn honoring the raster state"]
pub const filament_TransparencyMode_DEFAULT: filament_TransparencyMode = 0;
#[doc = " the transparent object is first drawn in the depth buffer,"]
#[doc = " then in the color buffer, honoring the culling mode, but ignoring the depth test function"]
pub const filament_TransparencyMode_TWO_PASSES_ONE_SIDE: filament_TransparencyMode = 1;
#[doc = " the transparent object is drawn twice in the color buffer,"]
#[doc = " first with back faces only, then with front faces; the culling"]
#[doc = " mode is ignored. Can be combined with two-sided lighting"]
pub const filament_TransparencyMode_TWO_PASSES_TWO_SIDES: filament_TransparencyMode = 2;
#[doc = " How transparent objects are handled"]
pub type filament_TransparencyMode = u8;
#[doc = "!< vertices are in object space, default"]
pub const filament_VertexDomain_OBJECT: filament_VertexDomain = 0;
#[doc = "!< vertices are in world space"]
pub const filament_VertexDomain_WORLD: filament_VertexDomain = 1;
#[doc = "!< vertices are in view space"]
pub const filament_VertexDomain_VIEW: filament_VertexDomain = 2;
#[doc = "!< vertices are in normalized device space"]
pub const filament_VertexDomain_DEVICE: filament_VertexDomain = 3;
#[doc = " Supported types of vertex domains."]
pub type filament_VertexDomain = u8;
#[doc = "!< XYZ position (float3)"]
pub const filament_VertexAttribute_POSITION: filament_VertexAttribute = 0;
#[doc = "!< tangent, bitangent and normal, encoded as a quaternion (float4)"]
pub const filament_VertexAttribute_TANGENTS: filament_VertexAttribute = 1;
#[doc = "!< vertex color (float4)"]
pub const filament_VertexAttribute_COLOR: filament_VertexAttribute = 2;
#[doc = "!< texture coordinates (float2)"]
pub const filament_VertexAttribute_UV0: filament_VertexAttribute = 3;
#[doc = "!< texture coordinates (float2)"]
pub const filament_VertexAttribute_UV1: filament_VertexAttribute = 4;
#[doc = "!< indices of 4 bones, as unsigned integers (uvec4)"]
pub const filament_VertexAttribute_BONE_INDICES: filament_VertexAttribute = 5;
#[doc = "!< weights of the 4 bones (normalized float4)"]
pub const filament_VertexAttribute_BONE_WEIGHTS: filament_VertexAttribute = 6;
pub const filament_VertexAttribute_CUSTOM0: filament_VertexAttribute = 8;
pub const filament_VertexAttribute_CUSTOM1: filament_VertexAttribute = 9;
pub const filament_VertexAttribute_CUSTOM2: filament_VertexAttribute = 10;
pub const filament_VertexAttribute_CUSTOM3: filament_VertexAttribute = 11;
pub const filament_VertexAttribute_CUSTOM4: filament_VertexAttribute = 12;
pub const filament_VertexAttribute_CUSTOM5: filament_VertexAttribute = 13;
pub const filament_VertexAttribute_CUSTOM6: filament_VertexAttribute = 14;
pub const filament_VertexAttribute_CUSTOM7: filament_VertexAttribute = 15;
pub const filament_VertexAttribute_MORPH_POSITION_0: filament_VertexAttribute = 8;
pub const filament_VertexAttribute_MORPH_POSITION_1: filament_VertexAttribute = 9;
pub const filament_VertexAttribute_MORPH_POSITION_2: filament_VertexAttribute = 10;
pub const filament_VertexAttribute_MORPH_POSITION_3: filament_VertexAttribute = 11;
pub const filament_VertexAttribute_MORPH_TANGENTS_0: filament_VertexAttribute = 12;
pub const filament_VertexAttribute_MORPH_TANGENTS_1: filament_VertexAttribute = 13;
pub const filament_VertexAttribute_MORPH_TANGENTS_2: filament_VertexAttribute = 14;
pub const filament_VertexAttribute_MORPH_TANGENTS_3: filament_VertexAttribute = 15;
#[doc = " Vertex attribute types"]
pub type filament_VertexAttribute = u8;
#[doc = "!< shaders applied to renderables"]
pub const filament_MaterialDomain_SURFACE: filament_MaterialDomain = 0;
#[doc = "!< shaders applied to rendered buffers"]
pub const filament_MaterialDomain_POST_PROCESS: filament_MaterialDomain = 1;
#[doc = " Material domains"]
pub type filament_MaterialDomain = u8;
#[doc = "!< no refraction"]
pub const filament_RefractionMode_NONE: filament_RefractionMode = 0;
#[doc = "!< refracted rays go to the ibl cubemap"]
pub const filament_RefractionMode_CUBEMAP: filament_RefractionMode = 1;
#[doc = "!< refracted rays go to screen space"]
pub const filament_RefractionMode_SCREEN_SPACE: filament_RefractionMode = 2;
#[doc = " Refraction"]
pub type filament_RefractionMode = u8;
#[doc = "!< refraction through solid objects (e.g. a sphere)"]
pub const filament_RefractionType_SOLID: filament_RefractionType = 0;
#[doc = "!< refraction through thin objects (e.g. window)"]
pub const filament_RefractionType_THIN: filament_RefractionType = 1;
#[doc = " Refraction type"]
pub type filament_RefractionType = u8;
pub const filament_ReflectionMode_DEFAULT: filament_ReflectionMode = 0;
#[doc = "! reflections sample from the scene's IBL only"]
pub const filament_ReflectionMode_SCREEN_SPACE: filament_ReflectionMode = 1;
#[doc = " Reflection mode"]
pub type filament_ReflectionMode = u8;
pub type filament_AttributeBitset = utils_bitset32;
#[repr(C)]
#[derive(Debug, Default)]
pub struct filament_MaterialInstance {
    pub _address: u8,
}
#[doc = " How transparent objects are handled"]
pub use self::filament_TransparencyMode as filament_MaterialInstance_TransparencyMode;
#[doc = "! Face culling Mode"]
pub use self::filament_backend_CullingMode as filament_MaterialInstance_CullingMode;
pub type filament_MaterialInstance_is_supported_parameter_t = u8;
#[test]
fn bindgen_test_layout_filament_MaterialInstance() {
    assert_eq!(
        ::std::mem::size_of::<filament_MaterialInstance>(),
        1usize,
        concat!("Size of: ", stringify!(filament_MaterialInstance))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_MaterialInstance>(),
        1usize,
        concat!("Alignment of ", stringify!(filament_MaterialInstance))
    );
}
extern "C" {
    #[doc = " Creates a new MaterialInstance using another MaterialInstance as a template for initialization."]
    #[doc = " The new MaterialInstance is an instance of the same Material of the template instance and"]
    #[doc = " must be destroyed just like any other MaterialInstance."]
    #[doc = ""]
    #[doc = " @param other A MaterialInstance to use as a template for initializing a new instance"]
    #[doc = " @param name  A name for the new MaterialInstance or nullptr to use the template's name"]
    #[doc = " @return      A new MaterialInstance"]
    #[link_name = "\u{1}_ZN8filament16MaterialInstance9duplicateEPKS0_PKc"]
    pub fn filament_MaterialInstance_duplicate(
        other: *const filament_MaterialInstance,
        name: *const ::std::os::raw::c_char,
    ) -> *mut filament_MaterialInstance;
}
extern "C" {
    #[doc = " @return the Material associated with this instance"]
    #[link_name = "\u{1}_ZNK8filament16MaterialInstance11getMaterialEv"]
    pub fn filament_MaterialInstance_getMaterial(
        this: *const filament_MaterialInstance,
    ) -> *const filament_Material;
}
extern "C" {
    #[doc = " @return the name associated with this instance"]
    #[link_name = "\u{1}_ZNK8filament16MaterialInstance7getNameEv"]
    pub fn filament_MaterialInstance_getName(
        this: *const filament_MaterialInstance,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Set a texture as the named parameter"]
    #[doc = ""]
    #[doc = " Note: Depth textures can't be sampled with a linear filter unless the comparison mode is set"]
    #[doc = "       to COMPARE_TO_TEXTURE."]
    #[doc = ""]
    #[doc = " @param name      Name of the parameter as defined by Material. Cannot be nullptr."]
    #[doc = " @param texture   Non nullptr Texture object pointer."]
    #[doc = " @param sampler   Sampler parameters."]
    #[doc = " @throws utils::PreConditionPanic if name doesn't exist or no-op if exceptions are disabled."]
    #[link_name = "\u{1}_ZN8filament16MaterialInstance12setParameterEPKcPKNS_7TextureERKNS_14TextureSamplerE"]
    pub fn filament_MaterialInstance_setParameter(
        this: *mut filament_MaterialInstance,
        name: *const ::std::os::raw::c_char,
        texture: *const filament_Texture,
        sampler: *const filament_TextureSampler,
    );
}
extern "C" {
    #[doc = " Set an RGB color as the named parameter."]
    #[doc = " A conversion might occur depending on the specified type"]
    #[doc = ""]
    #[doc = " @param name      Name of the parameter as defined by Material. Cannot be nullptr."]
    #[doc = " @param type      Whether the color value is encoded as Linear or sRGB."]
    #[doc = " @param color     Array of read, green, blue channels values."]
    #[doc = " @throws utils::PreConditionPanic if name doesn't exist or no-op if exceptions are disabled."]
    #[link_name = "\u{1}_ZN8filament16MaterialInstance12setParameterEPKcNS_7RgbTypeENS_4math7details5TVec3IfEE"]
    pub fn filament_MaterialInstance_setParameter1(
        this: *mut filament_MaterialInstance,
        name: *const ::std::os::raw::c_char,
        type_: filament_RgbType,
        color: filament_math_float3,
    );
}
extern "C" {
    #[doc = " Set an RGBA color as the named parameter."]
    #[doc = " A conversion might occur depending on the specified type"]
    #[doc = ""]
    #[doc = " @param name      Name of the parameter as defined by Material. Cannot be nullptr."]
    #[doc = " @param type      Whether the color value is encoded as Linear or sRGB/A."]
    #[doc = " @param color     Array of read, green, blue and alpha channels values."]
    #[doc = " @throws utils::PreConditionPanic if name doesn't exist or no-op if exceptions are disabled."]
    #[link_name = "\u{1}_ZN8filament16MaterialInstance12setParameterEPKcNS_8RgbaTypeENS_4math7details5TVec4IfEE"]
    pub fn filament_MaterialInstance_setParameter2(
        this: *mut filament_MaterialInstance,
        name: *const ::std::os::raw::c_char,
        type_: filament_RgbaType,
        color: filament_math_float4,
    );
}
extern "C" {
    #[doc = " Set up a custom scissor rectangle; by default this encompasses the View."]
    #[doc = ""]
    #[doc = " @param left      left coordinate of the scissor box"]
    #[doc = " @param bottom    bottom coordinate of the scissor box"]
    #[doc = " @param width     width of the scissor box"]
    #[doc = " @param height    height of the scissor box"]
    #[link_name = "\u{1}_ZN8filament16MaterialInstance10setScissorEjjjj"]
    pub fn filament_MaterialInstance_setScissor(
        this: *mut filament_MaterialInstance,
        left: u32,
        bottom: u32,
        width: u32,
        height: u32,
    );
}
extern "C" {
    #[doc = " Returns the scissor rectangle to its default setting, which encompasses the View."]
    #[link_name = "\u{1}_ZN8filament16MaterialInstance12unsetScissorEv"]
    pub fn filament_MaterialInstance_unsetScissor(this: *mut filament_MaterialInstance);
}
extern "C" {
    #[doc = " Sets a polygon offset that will be applied to all renderables drawn with this material"]
    #[doc = " instance."]
    #[doc = ""]
    #[doc = "  The value of the offset is scale * dz + r * constant, where dz is the change in depth"]
    #[doc = "  relative to the screen area of the triangle, and r is the smallest value that is guaranteed"]
    #[doc = "  to produce a resolvable offset for a given implementation. This offset is added before the"]
    #[doc = "  depth test."]
    #[doc = ""]
    #[doc = "  @warning using a polygon offset other than zero has a significant negative performance"]
    #[doc = "  impact, as most implementations have to disable early depth culling. DO NOT USE unless"]
    #[doc = "  absolutely necessary."]
    #[doc = ""]
    #[doc = " @param scale scale factor used to create a variable depth offset for each triangle"]
    #[doc = " @param constant scale factor used to create a constant depth offset for each triangle"]
    #[link_name = "\u{1}_ZN8filament16MaterialInstance16setPolygonOffsetEff"]
    pub fn filament_MaterialInstance_setPolygonOffset(
        this: *mut filament_MaterialInstance,
        scale: f32,
        constant: f32,
    );
}
extern "C" {
    #[doc = " Overrides the minimum alpha value a fragment must have to not be discarded when the blend"]
    #[doc = " mode is MASKED. Defaults to 0.4 if it has not been set in the parent Material. The specified"]
    #[doc = " value should be between 0 and 1 and will be clamped if necessary."]
    #[link_name = "\u{1}_ZN8filament16MaterialInstance16setMaskThresholdEf"]
    pub fn filament_MaterialInstance_setMaskThreshold(
        this: *mut filament_MaterialInstance,
        threshold: f32,
    );
}
extern "C" {
    #[doc = " Sets the screen space variance of the filter kernel used when applying specular"]
    #[doc = " anti-aliasing. The default value is set to 0.15. The specified value should be between"]
    #[doc = " 0 and 1 and will be clamped if necessary."]
    #[link_name = "\u{1}_ZN8filament16MaterialInstance31setSpecularAntiAliasingVarianceEf"]
    pub fn filament_MaterialInstance_setSpecularAntiAliasingVariance(
        this: *mut filament_MaterialInstance,
        variance: f32,
    );
}
extern "C" {
    #[doc = " Sets the clamping threshold used to suppress estimation errors when applying specular"]
    #[doc = " anti-aliasing. The default value is set to 0.2. The specified value should be between 0"]
    #[doc = " and 1 and will be clamped if necessary."]
    #[link_name = "\u{1}_ZN8filament16MaterialInstance32setSpecularAntiAliasingThresholdEf"]
    pub fn filament_MaterialInstance_setSpecularAntiAliasingThreshold(
        this: *mut filament_MaterialInstance,
        threshold: f32,
    );
}
extern "C" {
    #[doc = " Enables or disables double-sided lighting if the parent Material has double-sided capability,"]
    #[doc = " otherwise prints a warning. If double-sided lighting is enabled, backface culling is"]
    #[doc = " automatically disabled."]
    #[link_name = "\u{1}_ZN8filament16MaterialInstance14setDoubleSidedEb"]
    pub fn filament_MaterialInstance_setDoubleSided(
        this: *mut filament_MaterialInstance,
        doubleSided: bool,
    );
}
extern "C" {
    #[doc = " Specifies how transparent objects should be rendered (default is DEFAULT)."]
    #[link_name = "\u{1}_ZN8filament16MaterialInstance19setTransparencyModeENS_16TransparencyModeE"]
    pub fn filament_MaterialInstance_setTransparencyMode(
        this: *mut filament_MaterialInstance,
        mode: filament_MaterialInstance_TransparencyMode,
    );
}
extern "C" {
    #[doc = " Overrides the default triangle culling state that was set on the material."]
    #[link_name = "\u{1}_ZN8filament16MaterialInstance14setCullingModeENS_7backend11CullingModeE"]
    pub fn filament_MaterialInstance_setCullingMode(
        this: *mut filament_MaterialInstance,
        culling: filament_MaterialInstance_CullingMode,
    );
}
extern "C" {
    #[doc = " Overrides the default color-buffer write state that was set on the material."]
    #[link_name = "\u{1}_ZN8filament16MaterialInstance13setColorWriteEb"]
    pub fn filament_MaterialInstance_setColorWrite(
        this: *mut filament_MaterialInstance,
        enable: bool,
    );
}
extern "C" {
    #[doc = " Overrides the default depth-buffer write state that was set on the material."]
    #[link_name = "\u{1}_ZN8filament16MaterialInstance13setDepthWriteEb"]
    pub fn filament_MaterialInstance_setDepthWrite(
        this: *mut filament_MaterialInstance,
        enable: bool,
    );
}
extern "C" {
    #[doc = " Overrides the default depth testing state that was set on the material."]
    #[link_name = "\u{1}_ZN8filament16MaterialInstance15setDepthCullingEb"]
    pub fn filament_MaterialInstance_setDepthCulling(
        this: *mut filament_MaterialInstance,
        enable: bool,
    );
}
impl filament_MaterialInstance {
    #[inline]
    pub unsafe fn duplicate(
        other: *const filament_MaterialInstance,
        name: *const ::std::os::raw::c_char,
    ) -> *mut filament_MaterialInstance {
        filament_MaterialInstance_duplicate(other, name)
    }
    #[inline]
    pub unsafe fn getMaterial(&self) -> *const filament_Material {
        filament_MaterialInstance_getMaterial(self)
    }
    #[inline]
    pub unsafe fn getName(&self) -> *const ::std::os::raw::c_char {
        filament_MaterialInstance_getName(self)
    }
    #[inline]
    pub unsafe fn setParameter(
        &mut self,
        name: *const ::std::os::raw::c_char,
        texture: *const filament_Texture,
        sampler: *const filament_TextureSampler,
    ) {
        filament_MaterialInstance_setParameter(self, name, texture, sampler)
    }
    #[inline]
    pub unsafe fn setParameter1(
        &mut self,
        name: *const ::std::os::raw::c_char,
        type_: filament_RgbType,
        color: filament_math_float3,
    ) {
        filament_MaterialInstance_setParameter1(self, name, type_, color)
    }
    #[inline]
    pub unsafe fn setParameter2(
        &mut self,
        name: *const ::std::os::raw::c_char,
        type_: filament_RgbaType,
        color: filament_math_float4,
    ) {
        filament_MaterialInstance_setParameter2(self, name, type_, color)
    }
    #[inline]
    pub unsafe fn setScissor(&mut self, left: u32, bottom: u32, width: u32, height: u32) {
        filament_MaterialInstance_setScissor(self, left, bottom, width, height)
    }
    #[inline]
    pub unsafe fn unsetScissor(&mut self) {
        filament_MaterialInstance_unsetScissor(self)
    }
    #[inline]
    pub unsafe fn setPolygonOffset(&mut self, scale: f32, constant: f32) {
        filament_MaterialInstance_setPolygonOffset(self, scale, constant)
    }
    #[inline]
    pub unsafe fn setMaskThreshold(&mut self, threshold: f32) {
        filament_MaterialInstance_setMaskThreshold(self, threshold)
    }
    #[inline]
    pub unsafe fn setSpecularAntiAliasingVariance(&mut self, variance: f32) {
        filament_MaterialInstance_setSpecularAntiAliasingVariance(self, variance)
    }
    #[inline]
    pub unsafe fn setSpecularAntiAliasingThreshold(&mut self, threshold: f32) {
        filament_MaterialInstance_setSpecularAntiAliasingThreshold(self, threshold)
    }
    #[inline]
    pub unsafe fn setDoubleSided(&mut self, doubleSided: bool) {
        filament_MaterialInstance_setDoubleSided(self, doubleSided)
    }
    #[inline]
    pub unsafe fn setTransparencyMode(&mut self, mode: filament_MaterialInstance_TransparencyMode) {
        filament_MaterialInstance_setTransparencyMode(self, mode)
    }
    #[inline]
    pub unsafe fn setCullingMode(&mut self, culling: filament_MaterialInstance_CullingMode) {
        filament_MaterialInstance_setCullingMode(self, culling)
    }
    #[inline]
    pub unsafe fn setColorWrite(&mut self, enable: bool) {
        filament_MaterialInstance_setColorWrite(self, enable)
    }
    #[inline]
    pub unsafe fn setDepthWrite(&mut self, enable: bool) {
        filament_MaterialInstance_setDepthWrite(self, enable)
    }
    #[inline]
    pub unsafe fn setDepthCulling(&mut self, enable: bool) {
        filament_MaterialInstance_setDepthCulling(self, enable)
    }
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct filament_Material {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct filament_Material_BuilderDetails {
    _unused: [u8; 0],
}
#[doc = " Supported blending modes"]
pub use self::filament_BlendingMode as filament_Material_BlendingMode;
#[doc = " Attribute interpolation types in the fragment shader"]
pub use self::filament_Interpolation as filament_Material_Interpolation;
#[doc = " Supported shading models"]
pub use self::filament_Shading as filament_Material_Shading;
#[doc = " How transparent objects are handled"]
pub use self::filament_TransparencyMode as filament_Material_TransparencyMode;
#[doc = " Supported types of vertex domains."]
pub use self::filament_VertexDomain as filament_Material_VertexDomain;
#[doc = "! Face culling Mode"]
pub use self::filament_backend_CullingMode as filament_Material_CullingMode;
pub use self::filament_backend_Precision as filament_Material_Precision;
#[doc = "! Texture sampler format"]
pub use self::filament_backend_SamplerFormat as filament_Material_SamplerFormat;
#[doc = "! Texture sampler type"]
pub use self::filament_backend_SamplerType as filament_Material_SamplerType;
#[doc = " Shader model."]
#[doc = ""]
#[doc = " These enumerants are used across all backends and refer to a level of functionality, rather"]
#[doc = " than to an OpenGL specific shader model."]
pub use self::filament_backend_ShaderModel as filament_Material_ShaderModel;
#[doc = "! Subpass type"]
pub use self::filament_backend_SubpassType as filament_Material_SubpassType;
#[doc = " Supported uniform types"]
pub use self::filament_backend_UniformType as filament_Material_ParameterType;
#[doc = " Holds information about a material parameter."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct filament_Material_ParameterInfo {
    #[doc = "! Name of the parameter."]
    pub name: *const ::std::os::raw::c_char,
    #[doc = "! Whether the parameter is a sampler (texture)."]
    pub isSampler: bool,
    #[doc = "! Whether the parameter is a subpass type."]
    pub isSubpass: bool,
    pub __bindgen_anon_1: filament_Material_ParameterInfo__bindgen_ty_1,
    #[doc = "! Size of the parameter when the parameter is an array."]
    pub count: u32,
    #[doc = "! Requested precision of the parameter."]
    pub precision: filament_Material_Precision,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union filament_Material_ParameterInfo__bindgen_ty_1 {
    #[doc = "! Type of the parameter if the parameter is not a sampler."]
    pub type_: filament_Material_ParameterType,
    #[doc = "! Type of the parameter if the parameter is a sampler."]
    pub samplerType: filament_Material_SamplerType,
    #[doc = "! Type of the parameter if the parameter is a subpass."]
    pub subpassType: filament_Material_SubpassType,
}
#[test]
fn bindgen_test_layout_filament_Material_ParameterInfo__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<filament_Material_ParameterInfo__bindgen_ty_1>(),
        1usize,
        concat!(
            "Size of: ",
            stringify!(filament_Material_ParameterInfo__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_Material_ParameterInfo__bindgen_ty_1>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(filament_Material_ParameterInfo__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_Material_ParameterInfo__bindgen_ty_1>())).type_
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_Material_ParameterInfo__bindgen_ty_1),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_Material_ParameterInfo__bindgen_ty_1>())).samplerType
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_Material_ParameterInfo__bindgen_ty_1),
            "::",
            stringify!(samplerType)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_Material_ParameterInfo__bindgen_ty_1>())).subpassType
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_Material_ParameterInfo__bindgen_ty_1),
            "::",
            stringify!(subpassType)
        )
    );
}
impl Default for filament_Material_ParameterInfo__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn bindgen_test_layout_filament_Material_ParameterInfo() {
    assert_eq!(
        ::std::mem::size_of::<filament_Material_ParameterInfo>(),
        24usize,
        concat!("Size of: ", stringify!(filament_Material_ParameterInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_Material_ParameterInfo>(),
        8usize,
        concat!("Alignment of ", stringify!(filament_Material_ParameterInfo))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_Material_ParameterInfo>())).name as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_Material_ParameterInfo),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_Material_ParameterInfo>())).isSampler as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_Material_ParameterInfo),
            "::",
            stringify!(isSampler)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_Material_ParameterInfo>())).isSubpass as *const _
                as usize
        },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_Material_ParameterInfo),
            "::",
            stringify!(isSubpass)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_Material_ParameterInfo>())).count as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_Material_ParameterInfo),
            "::",
            stringify!(count)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_Material_ParameterInfo>())).precision as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_Material_ParameterInfo),
            "::",
            stringify!(precision)
        )
    );
}
impl Default for filament_Material_ParameterInfo {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct filament_Material_Builder {
    pub _base: filament_BuilderBase<filament_Material_BuilderDetails>,
}
#[test]
fn bindgen_test_layout_filament_Material_Builder() {
    assert_eq!(
        ::std::mem::size_of::<filament_Material_Builder>(),
        8usize,
        concat!("Size of: ", stringify!(filament_Material_Builder))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_Material_Builder>(),
        8usize,
        concat!("Alignment of ", stringify!(filament_Material_Builder))
    );
}
extern "C" {
    #[doc = " Specifies the material data. The material data is a binary blob produced by"]
    #[doc = " libfilamat or by matc."]
    #[doc = ""]
    #[doc = " @param payload Pointer to the material data, must stay valid until build() is called."]
    #[doc = " @param size Size of the material data pointed to by \"payload\" in bytes."]
    #[link_name = "\u{1}_ZN8filament8Material7Builder7packageEPKvm"]
    pub fn filament_Material_Builder_package(
        this: *mut filament_Material_Builder,
        payload: *const ::std::os::raw::c_void,
        size: size_t,
    ) -> *mut filament_Material_Builder;
}
extern "C" {
    #[doc = " Creates the Material object and returns a pointer to it."]
    #[doc = ""]
    #[doc = " @param engine Reference to the filament::Engine to associate this Material with."]
    #[doc = ""]
    #[doc = " @return pointer to the newly created object or nullptr if exceptions are disabled and"]
    #[doc = "         an error occurred."]
    #[doc = ""]
    #[doc = " @exception utils::PostConditionPanic if a runtime error occurred, such as running out of"]
    #[doc = "            memory or other resources."]
    #[doc = " @exception utils::PreConditionPanic if a parameter to a builder function was invalid."]
    #[link_name = "\u{1}_ZN8filament8Material7Builder5buildERNS_6EngineE"]
    pub fn filament_Material_Builder_build(
        this: *mut filament_Material_Builder,
        engine: *mut filament_Engine,
    ) -> *mut filament_Material;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament8Material7BuilderC1Ev"]
    pub fn filament_Material_Builder_Builder(this: *mut filament_Material_Builder);
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament8Material7BuilderC1ERKS1_"]
    pub fn filament_Material_Builder_Builder1(
        this: *mut filament_Material_Builder,
        rhs: *const filament_Material_Builder,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament8Material7BuilderC1EOS1_"]
    pub fn filament_Material_Builder_Builder2(
        this: *mut filament_Material_Builder,
        rhs: *mut filament_Material_Builder,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament8Material7BuilderD1Ev"]
    pub fn filament_Material_Builder_Builder_destructor(this: *mut filament_Material_Builder);
}
impl Default for filament_Material_Builder {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl filament_Material_Builder {
    #[inline]
    pub unsafe fn package(
        &mut self,
        payload: *const ::std::os::raw::c_void,
        size: size_t,
    ) -> *mut filament_Material_Builder {
        filament_Material_Builder_package(self, payload, size)
    }
    #[inline]
    pub unsafe fn build(&mut self, engine: *mut filament_Engine) -> *mut filament_Material {
        filament_Material_Builder_build(self, engine)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_Material_Builder_Builder(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(rhs: *const filament_Material_Builder) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_Material_Builder_Builder1(__bindgen_tmp.as_mut_ptr(), rhs);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new2(rhs: *mut filament_Material_Builder) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_Material_Builder_Builder2(__bindgen_tmp.as_mut_ptr(), rhs);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        filament_Material_Builder_Builder_destructor(self)
    }
}
#[test]
fn bindgen_test_layout_filament_Material() {
    assert_eq!(
        ::std::mem::size_of::<filament_Material>(),
        1usize,
        concat!("Size of: ", stringify!(filament_Material))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_Material>(),
        1usize,
        concat!("Alignment of ", stringify!(filament_Material))
    );
}
extern "C" {
    #[doc = " Creates a new instance of this material. Material instances should be freed using"]
    #[doc = " Engine::destroy(const MaterialInstance*)."]
    #[doc = ""]
    #[doc = " @param name Optional name to associate with the given material instance. If this is null,"]
    #[doc = " then the instance inherits the material's name."]
    #[doc = ""]
    #[doc = " @return A pointer to the new instance."]
    #[link_name = "\u{1}_ZNK8filament8Material14createInstanceEPKc"]
    pub fn filament_Material_createInstance(
        this: *const filament_Material,
        name: *const ::std::os::raw::c_char,
    ) -> *mut filament_MaterialInstance;
}
extern "C" {
    #[doc = "! Returns the name of this material as a null-terminated string."]
    #[link_name = "\u{1}_ZNK8filament8Material7getNameEv"]
    pub fn filament_Material_getName(
        this: *const filament_Material,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "! Returns the shading model of this material."]
    #[link_name = "\u{1}_ZNK8filament8Material10getShadingEv"]
    pub fn filament_Material_getShading(
        this: *const filament_Material,
    ) -> filament_Material_Shading;
}
extern "C" {
    #[doc = "! Returns the interpolation mode of this material. This affects how variables are interpolated."]
    #[link_name = "\u{1}_ZNK8filament8Material16getInterpolationEv"]
    pub fn filament_Material_getInterpolation(
        this: *const filament_Material,
    ) -> filament_Material_Interpolation;
}
extern "C" {
    #[doc = "! Returns the blending mode of this material."]
    #[link_name = "\u{1}_ZNK8filament8Material15getBlendingModeEv"]
    pub fn filament_Material_getBlendingMode(
        this: *const filament_Material,
    ) -> filament_Material_BlendingMode;
}
extern "C" {
    #[doc = "! Returns the vertex domain of this material."]
    #[link_name = "\u{1}_ZNK8filament8Material15getVertexDomainEv"]
    pub fn filament_Material_getVertexDomain(
        this: *const filament_Material,
    ) -> filament_Material_VertexDomain;
}
extern "C" {
    #[doc = "! Returns the material domain of this material."]
    #[doc = "! The material domain determines how the material is used."]
    #[link_name = "\u{1}_ZNK8filament8Material17getMaterialDomainEv"]
    pub fn filament_Material_getMaterialDomain(
        this: *const filament_Material,
    ) -> filament_MaterialDomain;
}
extern "C" {
    #[doc = "! Returns the default culling mode of this material."]
    #[link_name = "\u{1}_ZNK8filament8Material14getCullingModeEv"]
    pub fn filament_Material_getCullingMode(
        this: *const filament_Material,
    ) -> filament_Material_CullingMode;
}
extern "C" {
    #[doc = "! Returns the transparency mode of this material."]
    #[doc = "! This value only makes sense when the blending mode is transparent or fade."]
    #[link_name = "\u{1}_ZNK8filament8Material19getTransparencyModeEv"]
    pub fn filament_Material_getTransparencyMode(
        this: *const filament_Material,
    ) -> filament_Material_TransparencyMode;
}
extern "C" {
    #[doc = "! Indicates whether instances of this material will, by default, write to the color buffer."]
    #[link_name = "\u{1}_ZNK8filament8Material19isColorWriteEnabledEv"]
    pub fn filament_Material_isColorWriteEnabled(this: *const filament_Material) -> bool;
}
extern "C" {
    #[doc = "! Indicates whether instances of this material will, by default, write to the depth buffer."]
    #[link_name = "\u{1}_ZNK8filament8Material19isDepthWriteEnabledEv"]
    pub fn filament_Material_isDepthWriteEnabled(this: *const filament_Material) -> bool;
}
extern "C" {
    #[doc = "! Indicates whether instances of this material will, by default, use depth testing."]
    #[link_name = "\u{1}_ZNK8filament8Material21isDepthCullingEnabledEv"]
    pub fn filament_Material_isDepthCullingEnabled(this: *const filament_Material) -> bool;
}
extern "C" {
    #[doc = "! Indicates whether this material is double-sided."]
    #[link_name = "\u{1}_ZNK8filament8Material13isDoubleSidedEv"]
    pub fn filament_Material_isDoubleSided(this: *const filament_Material) -> bool;
}
extern "C" {
    #[doc = "! Returns the alpha mask threshold used when the blending mode is set to masked."]
    #[link_name = "\u{1}_ZNK8filament8Material16getMaskThresholdEv"]
    pub fn filament_Material_getMaskThreshold(this: *const filament_Material) -> f32;
}
extern "C" {
    #[doc = "! Indicates whether this material uses the shadowing factor as a color multiplier."]
    #[doc = "! This values only makes sense when the shading mode is unlit."]
    #[link_name = "\u{1}_ZNK8filament8Material19hasShadowMultiplierEv"]
    pub fn filament_Material_hasShadowMultiplier(this: *const filament_Material) -> bool;
}
extern "C" {
    #[doc = "! Indicates whether this material has specular anti-aliasing enabled"]
    #[link_name = "\u{1}_ZNK8filament8Material23hasSpecularAntiAliasingEv"]
    pub fn filament_Material_hasSpecularAntiAliasing(this: *const filament_Material) -> bool;
}
extern "C" {
    #[doc = "! Returns the screen-space variance for specular-antialiasing, this value is between 0 and 1."]
    #[link_name = "\u{1}_ZNK8filament8Material31getSpecularAntiAliasingVarianceEv"]
    pub fn filament_Material_getSpecularAntiAliasingVariance(this: *const filament_Material)
        -> f32;
}
extern "C" {
    #[doc = "! Returns the clamping threshold for specular-antialiasing, this value is between 0 and 1."]
    #[link_name = "\u{1}_ZNK8filament8Material32getSpecularAntiAliasingThresholdEv"]
    pub fn filament_Material_getSpecularAntiAliasingThreshold(
        this: *const filament_Material,
    ) -> f32;
}
extern "C" {
    #[doc = "! Returns the list of vertex attributes required by this material."]
    #[link_name = "\u{1}_ZNK8filament8Material21getRequiredAttributesEv"]
    pub fn filament_Material_getRequiredAttributes(
        this: *const filament_Material,
    ) -> filament_AttributeBitset;
}
extern "C" {
    #[doc = "! Returns the refraction mode used by this material."]
    #[link_name = "\u{1}_ZNK8filament8Material17getRefractionModeEv"]
    pub fn filament_Material_getRefractionMode(
        this: *const filament_Material,
    ) -> filament_RefractionMode;
}
extern "C" {
    #[doc = "! Return the refraction type used by this material."]
    #[link_name = "\u{1}_ZNK8filament8Material17getRefractionTypeEv"]
    pub fn filament_Material_getRefractionType(
        this: *const filament_Material,
    ) -> filament_RefractionType;
}
extern "C" {
    #[doc = "! Returns the reflection mode used by this material."]
    #[link_name = "\u{1}_ZNK8filament8Material17getReflectionModeEv"]
    pub fn filament_Material_getReflectionMode(
        this: *const filament_Material,
    ) -> filament_ReflectionMode;
}
extern "C" {
    #[doc = " Returns the number of parameters declared by this material."]
    #[doc = " The returned value can be 0."]
    #[link_name = "\u{1}_ZNK8filament8Material17getParameterCountEv"]
    pub fn filament_Material_getParameterCount(this: *const filament_Material) -> size_t;
}
extern "C" {
    #[doc = " Gets information about this material's parameters."]
    #[doc = ""]
    #[doc = " @param parameters A pointer to a list of ParameterInfo."]
    #[doc = "                   The list must be at least \"count\" large"]
    #[doc = " @param count The number of parameters to retrieve. Must be >= 0 and can be > count."]
    #[doc = ""]
    #[doc = " @return The number of parameters written to the parameters pointer."]
    #[link_name = "\u{1}_ZNK8filament8Material13getParametersEPNS0_13ParameterInfoEm"]
    pub fn filament_Material_getParameters(
        this: *const filament_Material,
        parameters: *mut filament_Material_ParameterInfo,
        count: size_t,
    ) -> size_t;
}
extern "C" {
    #[doc = "! Indicates whether a parameter of the given name exists on this material."]
    #[link_name = "\u{1}_ZNK8filament8Material12hasParameterEPKc"]
    pub fn filament_Material_hasParameter(
        this: *const filament_Material,
        name: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    #[doc = "! Indicates whether an existing parameter is a sampler or not."]
    #[link_name = "\u{1}_ZNK8filament8Material9isSamplerEPKc"]
    pub fn filament_Material_isSampler(
        this: *const filament_Material,
        name: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    #[doc = "! Returns this material's default instance."]
    #[link_name = "\u{1}_ZN8filament8Material18getDefaultInstanceEv"]
    pub fn filament_Material_getDefaultInstance(
        this: *mut filament_Material,
    ) -> *mut filament_MaterialInstance;
}
extern "C" {
    #[doc = "! Returns this material's default instance."]
    #[link_name = "\u{1}_ZNK8filament8Material18getDefaultInstanceEv"]
    pub fn filament_Material_getDefaultInstance1(
        this: *const filament_Material,
    ) -> *const filament_MaterialInstance;
}
impl filament_Material {
    #[inline]
    pub unsafe fn createInstance(
        &self,
        name: *const ::std::os::raw::c_char,
    ) -> *mut filament_MaterialInstance {
        filament_Material_createInstance(self, name)
    }
    #[inline]
    pub unsafe fn getName(&self) -> *const ::std::os::raw::c_char {
        filament_Material_getName(self)
    }
    #[inline]
    pub unsafe fn getShading(&self) -> filament_Material_Shading {
        filament_Material_getShading(self)
    }
    #[inline]
    pub unsafe fn getInterpolation(&self) -> filament_Material_Interpolation {
        filament_Material_getInterpolation(self)
    }
    #[inline]
    pub unsafe fn getBlendingMode(&self) -> filament_Material_BlendingMode {
        filament_Material_getBlendingMode(self)
    }
    #[inline]
    pub unsafe fn getVertexDomain(&self) -> filament_Material_VertexDomain {
        filament_Material_getVertexDomain(self)
    }
    #[inline]
    pub unsafe fn getMaterialDomain(&self) -> filament_MaterialDomain {
        filament_Material_getMaterialDomain(self)
    }
    #[inline]
    pub unsafe fn getCullingMode(&self) -> filament_Material_CullingMode {
        filament_Material_getCullingMode(self)
    }
    #[inline]
    pub unsafe fn getTransparencyMode(&self) -> filament_Material_TransparencyMode {
        filament_Material_getTransparencyMode(self)
    }
    #[inline]
    pub unsafe fn isColorWriteEnabled(&self) -> bool {
        filament_Material_isColorWriteEnabled(self)
    }
    #[inline]
    pub unsafe fn isDepthWriteEnabled(&self) -> bool {
        filament_Material_isDepthWriteEnabled(self)
    }
    #[inline]
    pub unsafe fn isDepthCullingEnabled(&self) -> bool {
        filament_Material_isDepthCullingEnabled(self)
    }
    #[inline]
    pub unsafe fn isDoubleSided(&self) -> bool {
        filament_Material_isDoubleSided(self)
    }
    #[inline]
    pub unsafe fn getMaskThreshold(&self) -> f32 {
        filament_Material_getMaskThreshold(self)
    }
    #[inline]
    pub unsafe fn hasShadowMultiplier(&self) -> bool {
        filament_Material_hasShadowMultiplier(self)
    }
    #[inline]
    pub unsafe fn hasSpecularAntiAliasing(&self) -> bool {
        filament_Material_hasSpecularAntiAliasing(self)
    }
    #[inline]
    pub unsafe fn getSpecularAntiAliasingVariance(&self) -> f32 {
        filament_Material_getSpecularAntiAliasingVariance(self)
    }
    #[inline]
    pub unsafe fn getSpecularAntiAliasingThreshold(&self) -> f32 {
        filament_Material_getSpecularAntiAliasingThreshold(self)
    }
    #[inline]
    pub unsafe fn getRequiredAttributes(&self) -> filament_AttributeBitset {
        filament_Material_getRequiredAttributes(self)
    }
    #[inline]
    pub unsafe fn getRefractionMode(&self) -> filament_RefractionMode {
        filament_Material_getRefractionMode(self)
    }
    #[inline]
    pub unsafe fn getRefractionType(&self) -> filament_RefractionType {
        filament_Material_getRefractionType(self)
    }
    #[inline]
    pub unsafe fn getReflectionMode(&self) -> filament_ReflectionMode {
        filament_Material_getReflectionMode(self)
    }
    #[inline]
    pub unsafe fn getParameterCount(&self) -> size_t {
        filament_Material_getParameterCount(self)
    }
    #[inline]
    pub unsafe fn getParameters(
        &self,
        parameters: *mut filament_Material_ParameterInfo,
        count: size_t,
    ) -> size_t {
        filament_Material_getParameters(self, parameters, count)
    }
    #[inline]
    pub unsafe fn hasParameter(&self, name: *const ::std::os::raw::c_char) -> bool {
        filament_Material_hasParameter(self, name)
    }
    #[inline]
    pub unsafe fn isSampler(&self, name: *const ::std::os::raw::c_char) -> bool {
        filament_Material_isSampler(self, name)
    }
    #[inline]
    pub unsafe fn getDefaultInstance(&mut self) -> *mut filament_MaterialInstance {
        filament_Material_getDefaultInstance(self)
    }
    #[inline]
    pub unsafe fn getDefaultInstance1(&self) -> *const filament_MaterialInstance {
        filament_Material_getDefaultInstance1(self)
    }
}
#[doc = " MorphTargetBuffer is used to hold morphing data (positions and tangents)."]
#[doc = ""]
#[doc = " Both positions and tangents are required."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Default)]
pub struct filament_MorphTargetBuffer {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct filament_MorphTargetBuffer_BuilderDetails {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
pub struct filament_MorphTargetBuffer_Builder {
    pub _base: filament_BuilderBase<filament_MorphTargetBuffer_BuilderDetails>,
}
#[test]
fn bindgen_test_layout_filament_MorphTargetBuffer_Builder() {
    assert_eq!(
        ::std::mem::size_of::<filament_MorphTargetBuffer_Builder>(),
        8usize,
        concat!("Size of: ", stringify!(filament_MorphTargetBuffer_Builder))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_MorphTargetBuffer_Builder>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(filament_MorphTargetBuffer_Builder)
        )
    );
}
extern "C" {
    #[doc = " Size of the morph targets in vertex counts."]
    #[doc = " @param vertexCount Number of vertex counts the morph targets can hold."]
    #[doc = " @return A reference to this Builder for chaining calls."]
    #[link_name = "\u{1}_ZN8filament17MorphTargetBuffer7Builder11vertexCountEm"]
    pub fn filament_MorphTargetBuffer_Builder_vertexCount(
        this: *mut filament_MorphTargetBuffer_Builder,
        vertexCount: size_t,
    ) -> *mut filament_MorphTargetBuffer_Builder;
}
extern "C" {
    #[doc = " Size of the morph targets in targets."]
    #[doc = " @param count Number of targets the morph targets can hold."]
    #[doc = " @return A reference to this Builder for chaining calls."]
    #[link_name = "\u{1}_ZN8filament17MorphTargetBuffer7Builder5countEm"]
    pub fn filament_MorphTargetBuffer_Builder_count(
        this: *mut filament_MorphTargetBuffer_Builder,
        count: size_t,
    ) -> *mut filament_MorphTargetBuffer_Builder;
}
extern "C" {
    #[doc = " Creates the MorphTargetBuffer object and returns a pointer to it."]
    #[doc = ""]
    #[doc = " @param engine Reference to the filament::Engine to associate this MorphTargetBuffer with."]
    #[doc = ""]
    #[doc = " @return pointer to the newly created object or nullptr if exceptions are disabled and"]
    #[doc = "         an error occurred."]
    #[doc = ""]
    #[doc = " @exception utils::PostConditionPanic if a runtime error occurred, such as running out of"]
    #[doc = "            memory or other resources."]
    #[doc = " @exception utils::PreConditionPanic if a parameter to a builder function was invalid."]
    #[link_name = "\u{1}_ZN8filament17MorphTargetBuffer7Builder5buildERNS_6EngineE"]
    pub fn filament_MorphTargetBuffer_Builder_build(
        this: *mut filament_MorphTargetBuffer_Builder,
        engine: *mut filament_Engine,
    ) -> *mut filament_MorphTargetBuffer;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament17MorphTargetBuffer7BuilderC1Ev"]
    pub fn filament_MorphTargetBuffer_Builder_Builder(
        this: *mut filament_MorphTargetBuffer_Builder,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament17MorphTargetBuffer7BuilderC1ERKS1_"]
    pub fn filament_MorphTargetBuffer_Builder_Builder1(
        this: *mut filament_MorphTargetBuffer_Builder,
        rhs: *const filament_MorphTargetBuffer_Builder,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament17MorphTargetBuffer7BuilderC1EOS1_"]
    pub fn filament_MorphTargetBuffer_Builder_Builder2(
        this: *mut filament_MorphTargetBuffer_Builder,
        rhs: *mut filament_MorphTargetBuffer_Builder,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament17MorphTargetBuffer7BuilderD1Ev"]
    pub fn filament_MorphTargetBuffer_Builder_Builder_destructor(
        this: *mut filament_MorphTargetBuffer_Builder,
    );
}
impl Default for filament_MorphTargetBuffer_Builder {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl filament_MorphTargetBuffer_Builder {
    #[inline]
    pub unsafe fn vertexCount(
        &mut self,
        vertexCount: size_t,
    ) -> *mut filament_MorphTargetBuffer_Builder {
        filament_MorphTargetBuffer_Builder_vertexCount(self, vertexCount)
    }
    #[inline]
    pub unsafe fn count(&mut self, count: size_t) -> *mut filament_MorphTargetBuffer_Builder {
        filament_MorphTargetBuffer_Builder_count(self, count)
    }
    #[inline]
    pub unsafe fn build(
        &mut self,
        engine: *mut filament_Engine,
    ) -> *mut filament_MorphTargetBuffer {
        filament_MorphTargetBuffer_Builder_build(self, engine)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_MorphTargetBuffer_Builder_Builder(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(rhs: *const filament_MorphTargetBuffer_Builder) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_MorphTargetBuffer_Builder_Builder1(__bindgen_tmp.as_mut_ptr(), rhs);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new2(rhs: *mut filament_MorphTargetBuffer_Builder) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_MorphTargetBuffer_Builder_Builder2(__bindgen_tmp.as_mut_ptr(), rhs);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        filament_MorphTargetBuffer_Builder_Builder_destructor(self)
    }
}
#[test]
fn bindgen_test_layout_filament_MorphTargetBuffer() {
    assert_eq!(
        ::std::mem::size_of::<filament_MorphTargetBuffer>(),
        1usize,
        concat!("Size of: ", stringify!(filament_MorphTargetBuffer))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_MorphTargetBuffer>(),
        1usize,
        concat!("Alignment of ", stringify!(filament_MorphTargetBuffer))
    );
}
extern "C" {
    #[doc = " Updates the position of morph target at the index."]
    #[doc = ""]
    #[doc = " Both positions and tangents must be provided."]
    #[doc = ""]
    #[doc = " @param engine Reference to the filament::Engine associated with this MorphTargetBuffer."]
    #[doc = " @param targetIndex the index of morph target to be updated."]
    #[doc = " @param weights pointer to at least count positions"]
    #[doc = " @param count number of position elements in positions"]
    #[doc = " @see setTangentsAt"]
    #[link_name = "\u{1}_ZN8filament17MorphTargetBuffer14setPositionsAtERNS_6EngineEmPKNS_4math7details5TVec3IfEEmm"]
    pub fn filament_MorphTargetBuffer_setPositionsAt(
        this: *mut filament_MorphTargetBuffer,
        engine: *mut filament_Engine,
        targetIndex: size_t,
        positions: *const filament_math_float3,
        count: size_t,
        offset: size_t,
    );
}
extern "C" {
    #[doc = " Updates the position of morph target at the index."]
    #[doc = ""]
    #[doc = " Both positions and tangents must be provided."]
    #[doc = ""]
    #[doc = " @param engine Reference to the filament::Engine associated with this MorphTargetBuffer."]
    #[doc = " @param targetIndex the index of morph target to be updated."]
    #[doc = " @param weights pointer to at least count positions"]
    #[doc = " @param count number of position elements in positions"]
    #[doc = " @see setPositionsAt"]
    #[link_name = "\u{1}_ZN8filament17MorphTargetBuffer14setPositionsAtERNS_6EngineEmPKNS_4math7details5TVec4IfEEmm"]
    pub fn filament_MorphTargetBuffer_setPositionsAt1(
        this: *mut filament_MorphTargetBuffer,
        engine: *mut filament_Engine,
        targetIndex: size_t,
        positions: *const filament_math_float4,
        count: size_t,
        offset: size_t,
    );
}
extern "C" {
    #[doc = " Updates the position of morph target at the index."]
    #[doc = ""]
    #[doc = " Both positions and tangents must be provided."]
    #[doc = ""]
    #[doc = " @param engine Reference to the filament::Engine associated with this MorphTargetBuffer."]
    #[doc = " @param targetIndex the index of morph target to be updated."]
    #[doc = " @param tangents pointer to at least count tangents"]
    #[doc = " @param count number of tangent elements in tangents"]
    #[doc = " @see setTangentsAt"]
    #[link_name = "\u{1}_ZN8filament17MorphTargetBuffer13setTangentsAtERNS_6EngineEmPKNS_4math7details5TVec4IsEEmm"]
    pub fn filament_MorphTargetBuffer_setTangentsAt(
        this: *mut filament_MorphTargetBuffer,
        engine: *mut filament_Engine,
        targetIndex: size_t,
        tangents: *const filament_math_short4,
        count: size_t,
        offset: size_t,
    );
}
extern "C" {
    #[doc = " Returns the vertex count of this MorphTargetBuffer."]
    #[doc = " @return The number of vertices the MorphTargetBuffer holds."]
    #[link_name = "\u{1}_ZNK8filament17MorphTargetBuffer14getVertexCountEv"]
    pub fn filament_MorphTargetBuffer_getVertexCount(
        this: *const filament_MorphTargetBuffer,
    ) -> size_t;
}
extern "C" {
    #[doc = " Returns the target count of this MorphTargetBuffer."]
    #[doc = " @return The number of targets the MorphTargetBuffer holds."]
    #[link_name = "\u{1}_ZNK8filament17MorphTargetBuffer8getCountEv"]
    pub fn filament_MorphTargetBuffer_getCount(this: *const filament_MorphTargetBuffer) -> size_t;
}
impl filament_MorphTargetBuffer {
    #[inline]
    pub unsafe fn setPositionsAt(
        &mut self,
        engine: *mut filament_Engine,
        targetIndex: size_t,
        positions: *const filament_math_float3,
        count: size_t,
        offset: size_t,
    ) {
        filament_MorphTargetBuffer_setPositionsAt(
            self,
            engine,
            targetIndex,
            positions,
            count,
            offset,
        )
    }
    #[inline]
    pub unsafe fn setPositionsAt1(
        &mut self,
        engine: *mut filament_Engine,
        targetIndex: size_t,
        positions: *const filament_math_float4,
        count: size_t,
        offset: size_t,
    ) {
        filament_MorphTargetBuffer_setPositionsAt1(
            self,
            engine,
            targetIndex,
            positions,
            count,
            offset,
        )
    }
    #[inline]
    pub unsafe fn setTangentsAt(
        &mut self,
        engine: *mut filament_Engine,
        targetIndex: size_t,
        tangents: *const filament_math_short4,
        count: size_t,
        offset: size_t,
    ) {
        filament_MorphTargetBuffer_setTangentsAt(self, engine, targetIndex, tangents, count, offset)
    }
    #[inline]
    pub unsafe fn getVertexCount(&self) -> size_t {
        filament_MorphTargetBuffer_getVertexCount(self)
    }
    #[inline]
    pub unsafe fn getCount(&self) -> size_t {
        filament_MorphTargetBuffer_getCount(self)
    }
}
pub const filament_QualityLevel_LOW: filament_QualityLevel = 0;
pub const filament_QualityLevel_MEDIUM: filament_QualityLevel = 1;
pub const filament_QualityLevel_HIGH: filament_QualityLevel = 2;
pub const filament_QualityLevel_ULTRA: filament_QualityLevel = 3;
pub type filament_QualityLevel = u8;
pub const filament_BlendMode_OPAQUE: filament_BlendMode = 0;
pub const filament_BlendMode_TRANSLUCENT: filament_BlendMode = 1;
pub type filament_BlendMode = u8;
#[doc = " Dynamic resolution can be used to either reach a desired target frame rate"]
#[doc = " by lowering the resolution of a View, or to increase the quality when the"]
#[doc = " rendering is faster than the target frame rate."]
#[doc = ""]
#[doc = " This structure can be used to specify the minimum scale factor used when"]
#[doc = " lowering the resolution of a View, and the maximum scale factor used when"]
#[doc = " increasing the resolution for higher quality rendering. The scale factors"]
#[doc = " can be controlled on each X and Y axis independently. By default, all scale"]
#[doc = " factors are set to 1.0."]
#[doc = ""]
#[doc = " enabled:   enable or disables dynamic resolution on a View"]
#[doc = ""]
#[doc = " homogeneousScaling: by default the system scales the major axis first. Set this to true"]
#[doc = "                     to force homogeneous scaling."]
#[doc = ""]
#[doc = " minScale:  the minimum scale in X and Y this View should use"]
#[doc = ""]
#[doc = " maxScale:  the maximum scale in X and Y this View should use"]
#[doc = ""]
#[doc = " quality:   upscaling quality."]
#[doc = "            LOW: 1 bilinear tap, Medium: 4 bilinear taps, High: 9 bilinear taps (tent)"]
#[doc = ""]
#[doc = " \\note"]
#[doc = " Dynamic resolution is only supported on platforms where the time to render"]
#[doc = " a frame can be measured accurately. Dynamic resolution is currently only"]
#[doc = " supported on Android."]
#[doc = ""]
#[doc = " @see Renderer::FrameRateOptions"]
#[doc = ""]
#[repr(C)]
pub struct filament_DynamicResolutionOptions {
    #[doc = "!< minimum scale factors in x and y"]
    pub minScale: filament_math_float2,
    #[doc = "!< maximum scale factors in x and y"]
    pub maxScale: filament_math_float2,
    #[doc = "!< sharpness when QualityLevel::MEDIUM or higher is used [0 (disabled), 1 (sharpest)]"]
    pub sharpness: f32,
    #[doc = "!< enable or disable dynamic resolution"]
    pub enabled: bool,
    #[doc = "!< set to true to force homogeneous scaling"]
    pub homogeneousScaling: bool,
    #[doc = " Upscaling quality"]
    #[doc = " LOW:    bilinear filtered blit. Fastest, poor quality"]
    #[doc = " MEDIUM: AMD FidelityFX FSR1 w/ mobile optimizations"]
    #[doc = " HIGH:   AMD FidelityFX FSR1 w/ mobile optimizations"]
    #[doc = " ULTRA:  AMD FidelityFX FSR1"]
    #[doc = "      FSR1 require a well anti-aliased (MSAA or TAA), noise free scene."]
    #[doc = ""]
    #[doc = " The default upscaling quality is set to LOW."]
    pub quality: filament_QualityLevel,
}
#[test]
fn bindgen_test_layout_filament_DynamicResolutionOptions() {
    assert_eq!(
        ::std::mem::size_of::<filament_DynamicResolutionOptions>(),
        24usize,
        concat!("Size of: ", stringify!(filament_DynamicResolutionOptions))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_DynamicResolutionOptions>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(filament_DynamicResolutionOptions)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_DynamicResolutionOptions>())).minScale as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_DynamicResolutionOptions),
            "::",
            stringify!(minScale)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_DynamicResolutionOptions>())).maxScale as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_DynamicResolutionOptions),
            "::",
            stringify!(maxScale)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_DynamicResolutionOptions>())).sharpness as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_DynamicResolutionOptions),
            "::",
            stringify!(sharpness)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_DynamicResolutionOptions>())).enabled as *const _
                as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_DynamicResolutionOptions),
            "::",
            stringify!(enabled)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_DynamicResolutionOptions>())).homogeneousScaling
                as *const _ as usize
        },
        21usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_DynamicResolutionOptions),
            "::",
            stringify!(homogeneousScaling)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_DynamicResolutionOptions>())).quality as *const _
                as usize
        },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_DynamicResolutionOptions),
            "::",
            stringify!(quality)
        )
    );
}
impl Default for filament_DynamicResolutionOptions {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " Options to control the bloom effect"]
#[doc = ""]
#[doc = " enabled:     Enable or disable the bloom post-processing effect. Disabled by default."]
#[doc = ""]
#[doc = " levels:      Number of successive blurs to achieve the blur effect, the minimum is 3 and the"]
#[doc = "              maximum is 12. This value together with resolution influences the spread of the"]
#[doc = "              blur effect. This value can be silently reduced to accommodate the original"]
#[doc = "              image size."]
#[doc = ""]
#[doc = " resolution:  Resolution of bloom's minor axis. The minimum value is 2^levels and the"]
#[doc = "              the maximum is lower of the original resolution and 4096. This parameter is"]
#[doc = "              silently clamped to the minimum and maximum."]
#[doc = "              It is highly recommended that this value be smaller than the target resolution"]
#[doc = "              after dynamic resolution is applied (horizontally and vertically)."]
#[doc = ""]
#[doc = " strength:    how much of the bloom is added to the original image. Between 0 and 1."]
#[doc = ""]
#[doc = " blendMode:   Whether the bloom effect is purely additive (false) or mixed with the original"]
#[doc = "              image (true)."]
#[doc = ""]
#[doc = " anamorphism: Bloom's aspect ratio (x/y), for artistic purposes."]
#[doc = ""]
#[doc = " threshold:   When enabled, a threshold at 1.0 is applied on the source image, this is"]
#[doc = "              useful for artistic reasons and is usually needed when a dirt texture is used."]
#[doc = ""]
#[doc = " dirt:        A dirt/scratch/smudges texture (that can be RGB), which gets added to the"]
#[doc = "              bloom effect. Smudges are visible where bloom occurs. Threshold must be"]
#[doc = "              enabled for the dirt effect to work properly."]
#[doc = ""]
#[doc = " dirtStrength: Strength of the dirt texture."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct filament_BloomOptions {
    #[doc = "!< user provided dirt texture"]
    pub dirt: *mut filament_Texture,
    #[doc = "!< strength of the dirt texture"]
    pub dirtStrength: f32,
    #[doc = "!< bloom's strength between 0.0 and 1.0"]
    pub strength: f32,
    #[doc = "!< resolution of vertical axis (2^levels to 2048)"]
    pub resolution: u32,
    #[doc = "!< bloom x/y aspect-ratio (1/32 to 32)"]
    pub anamorphism: f32,
    #[doc = "!< number of blur levels (3 to 11)"]
    pub levels: u8,
    #[doc = "!< how the bloom effect is applied"]
    pub blendMode: filament_BloomOptions_BlendMode,
    #[doc = "!< whether to threshold the source"]
    pub threshold: bool,
    #[doc = "!< enable or disable bloom"]
    pub enabled: bool,
    #[doc = "!< limit highlights to this value before bloom [10, +inf]"]
    pub highlight: f32,
    #[doc = "!< enable screen-space lens flare"]
    pub lensFlare: bool,
    #[doc = "!< enable starburst effect on lens flare"]
    pub starburst: bool,
    #[doc = "!< amount of chromatic aberration"]
    pub chromaticAberration: f32,
    #[doc = "!< number of flare \"ghosts\""]
    pub ghostCount: u8,
    #[doc = "!< spacing of the ghost in screen units [0, 1["]
    pub ghostSpacing: f32,
    #[doc = "!< hdr threshold for the ghosts"]
    pub ghostThreshold: f32,
    #[doc = "!< thickness of halo in vertical screen units, 0 to disable"]
    pub haloThickness: f32,
    #[doc = "!< radius of halo in vertical screen units [0, 0.5]"]
    pub haloRadius: f32,
    #[doc = "!< hdr threshold for the halo"]
    pub haloThreshold: f32,
}
#[doc = "!< Bloom is modulated by the strength parameter and added to the scene"]
pub const filament_BloomOptions_BlendMode_ADD: filament_BloomOptions_BlendMode = 0;
#[doc = "!< Bloom is interpolated with the scene using the strength parameter"]
pub const filament_BloomOptions_BlendMode_INTERPOLATE: filament_BloomOptions_BlendMode = 1;
pub type filament_BloomOptions_BlendMode = u8;
#[test]
fn bindgen_test_layout_filament_BloomOptions() {
    assert_eq!(
        ::std::mem::size_of::<filament_BloomOptions>(),
        64usize,
        concat!("Size of: ", stringify!(filament_BloomOptions))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_BloomOptions>(),
        8usize,
        concat!("Alignment of ", stringify!(filament_BloomOptions))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<filament_BloomOptions>())).dirt as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_BloomOptions),
            "::",
            stringify!(dirt)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_BloomOptions>())).dirtStrength as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_BloomOptions),
            "::",
            stringify!(dirtStrength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<filament_BloomOptions>())).strength as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_BloomOptions),
            "::",
            stringify!(strength)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_BloomOptions>())).resolution as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_BloomOptions),
            "::",
            stringify!(resolution)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_BloomOptions>())).anamorphism as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_BloomOptions),
            "::",
            stringify!(anamorphism)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<filament_BloomOptions>())).levels as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_BloomOptions),
            "::",
            stringify!(levels)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<filament_BloomOptions>())).blendMode as *const _ as usize },
        25usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_BloomOptions),
            "::",
            stringify!(blendMode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<filament_BloomOptions>())).threshold as *const _ as usize },
        26usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_BloomOptions),
            "::",
            stringify!(threshold)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<filament_BloomOptions>())).enabled as *const _ as usize },
        27usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_BloomOptions),
            "::",
            stringify!(enabled)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<filament_BloomOptions>())).highlight as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_BloomOptions),
            "::",
            stringify!(highlight)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<filament_BloomOptions>())).lensFlare as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_BloomOptions),
            "::",
            stringify!(lensFlare)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<filament_BloomOptions>())).starburst as *const _ as usize },
        33usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_BloomOptions),
            "::",
            stringify!(starburst)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_BloomOptions>())).chromaticAberration as *const _
                as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_BloomOptions),
            "::",
            stringify!(chromaticAberration)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_BloomOptions>())).ghostCount as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_BloomOptions),
            "::",
            stringify!(ghostCount)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_BloomOptions>())).ghostSpacing as *const _ as usize
        },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_BloomOptions),
            "::",
            stringify!(ghostSpacing)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_BloomOptions>())).ghostThreshold as *const _ as usize
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_BloomOptions),
            "::",
            stringify!(ghostThreshold)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_BloomOptions>())).haloThickness as *const _ as usize
        },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_BloomOptions),
            "::",
            stringify!(haloThickness)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_BloomOptions>())).haloRadius as *const _ as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_BloomOptions),
            "::",
            stringify!(haloRadius)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_BloomOptions>())).haloThreshold as *const _ as usize
        },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_BloomOptions),
            "::",
            stringify!(haloThreshold)
        )
    );
}
impl Default for filament_BloomOptions {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " Options to control fog in the scene"]
#[repr(C)]
pub struct filament_FogOptions {
    #[doc = "!< distance in world units from the camera where the fog starts ( >= 0.0 )"]
    pub distance: f32,
    #[doc = "!< fog's maximum opacity between 0 and 1"]
    pub maximumOpacity: f32,
    #[doc = "!< fog's floor in world units"]
    pub height: f32,
    #[doc = "!< how fast fog dissipates with altitude"]
    pub heightFalloff: f32,
    #[doc = "!< fog's color (linear), see fogColorFromIbl"]
    pub color: filament_LinearColor,
    #[doc = "!< fog's density at altitude given by 'height'"]
    pub density: f32,
    #[doc = "!< distance in world units from the camera where in-scattering starts"]
    pub inScatteringStart: f32,
    #[doc = "!< size of in-scattering (>0 to activate). Good values are >> 1 (e.g. ~10 - 100)."]
    pub inScatteringSize: f32,
    #[doc = "!< Fog color will be modulated by the IBL color in the view direction."]
    pub fogColorFromIbl: bool,
    #[doc = "!< enable or disable fog"]
    pub enabled: bool,
}
#[test]
fn bindgen_test_layout_filament_FogOptions() {
    assert_eq!(
        ::std::mem::size_of::<filament_FogOptions>(),
        44usize,
        concat!("Size of: ", stringify!(filament_FogOptions))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_FogOptions>(),
        4usize,
        concat!("Alignment of ", stringify!(filament_FogOptions))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<filament_FogOptions>())).distance as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_FogOptions),
            "::",
            stringify!(distance)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_FogOptions>())).maximumOpacity as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_FogOptions),
            "::",
            stringify!(maximumOpacity)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<filament_FogOptions>())).height as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_FogOptions),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_FogOptions>())).heightFalloff as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_FogOptions),
            "::",
            stringify!(heightFalloff)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<filament_FogOptions>())).color as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_FogOptions),
            "::",
            stringify!(color)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<filament_FogOptions>())).density as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_FogOptions),
            "::",
            stringify!(density)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_FogOptions>())).inScatteringStart as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_FogOptions),
            "::",
            stringify!(inScatteringStart)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_FogOptions>())).inScatteringSize as *const _ as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_FogOptions),
            "::",
            stringify!(inScatteringSize)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_FogOptions>())).fogColorFromIbl as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_FogOptions),
            "::",
            stringify!(fogColorFromIbl)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<filament_FogOptions>())).enabled as *const _ as usize },
        41usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_FogOptions),
            "::",
            stringify!(enabled)
        )
    );
}
impl Default for filament_FogOptions {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " Options to control Depth of Field (DoF) effect in the scene."]
#[doc = ""]
#[doc = " cocScale can be used to set the depth of field blur independently from the camera"]
#[doc = " aperture, e.g. for artistic reasons. This can be achieved by setting:"]
#[doc = "      cocScale = cameraAperture / desiredDoFAperture"]
#[doc = ""]
#[doc = " @see Camera"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct filament_DepthOfFieldOptions {
    #[doc = "!< circle of confusion scale factor (amount of blur)"]
    pub cocScale: f32,
    #[doc = "!< maximum aperture diameter in meters (zero to disable rotation)"]
    pub maxApertureDiameter: f32,
    #[doc = "!< enable or disable depth of field effect"]
    pub enabled: bool,
    #[doc = "!< filter to use for filling gaps in the kernel"]
    pub filter: filament_DepthOfFieldOptions_Filter,
    #[doc = "!< perform DoF processing at native resolution"]
    pub nativeResolution: bool,
    #[doc = "!< number of kernel rings for foreground tiles"]
    pub foregroundRingCount: u8,
    #[doc = "!< number of kernel rings for background tiles"]
    pub backgroundRingCount: u8,
    #[doc = "!< number of kernel rings for fast tiles"]
    pub fastGatherRingCount: u8,
    #[doc = " maximum circle-of-confusion in pixels for the foreground, must be in [0, 32] range."]
    #[doc = " A value of 0 means default, which is 32 on desktop and 24 on mobile."]
    pub maxForegroundCOC: u16,
    #[doc = " maximum circle-of-confusion in pixels for the background, must be in [0, 32] range."]
    #[doc = " A value of 0 means default, which is 32 on desktop and 24 on mobile."]
    pub maxBackgroundCOC: u16,
}
pub const filament_DepthOfFieldOptions_Filter_NONE: filament_DepthOfFieldOptions_Filter = 0;
pub const filament_DepthOfFieldOptions_Filter_MEDIAN: filament_DepthOfFieldOptions_Filter = 2;
pub type filament_DepthOfFieldOptions_Filter = u8;
#[test]
fn bindgen_test_layout_filament_DepthOfFieldOptions() {
    assert_eq!(
        ::std::mem::size_of::<filament_DepthOfFieldOptions>(),
        20usize,
        concat!("Size of: ", stringify!(filament_DepthOfFieldOptions))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_DepthOfFieldOptions>(),
        4usize,
        concat!("Alignment of ", stringify!(filament_DepthOfFieldOptions))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_DepthOfFieldOptions>())).cocScale as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_DepthOfFieldOptions),
            "::",
            stringify!(cocScale)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_DepthOfFieldOptions>())).maxApertureDiameter as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_DepthOfFieldOptions),
            "::",
            stringify!(maxApertureDiameter)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_DepthOfFieldOptions>())).enabled as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_DepthOfFieldOptions),
            "::",
            stringify!(enabled)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_DepthOfFieldOptions>())).filter as *const _ as usize
        },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_DepthOfFieldOptions),
            "::",
            stringify!(filter)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_DepthOfFieldOptions>())).nativeResolution as *const _
                as usize
        },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_DepthOfFieldOptions),
            "::",
            stringify!(nativeResolution)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_DepthOfFieldOptions>())).foregroundRingCount as *const _
                as usize
        },
        11usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_DepthOfFieldOptions),
            "::",
            stringify!(foregroundRingCount)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_DepthOfFieldOptions>())).backgroundRingCount as *const _
                as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_DepthOfFieldOptions),
            "::",
            stringify!(backgroundRingCount)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_DepthOfFieldOptions>())).fastGatherRingCount as *const _
                as usize
        },
        13usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_DepthOfFieldOptions),
            "::",
            stringify!(fastGatherRingCount)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_DepthOfFieldOptions>())).maxForegroundCOC as *const _
                as usize
        },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_DepthOfFieldOptions),
            "::",
            stringify!(maxForegroundCOC)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_DepthOfFieldOptions>())).maxBackgroundCOC as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_DepthOfFieldOptions),
            "::",
            stringify!(maxBackgroundCOC)
        )
    );
}
impl Default for filament_DepthOfFieldOptions {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " Options to control the vignetting effect."]
#[repr(C)]
pub struct filament_VignetteOptions {
    #[doc = "!< high values restrict the vignette closer to the corners, between 0 and 1"]
    pub midPoint: f32,
    #[doc = "!< controls the shape of the vignette, from a rounded rectangle (0.0), to an oval (0.5), to a circle (1.0)"]
    pub roundness: f32,
    #[doc = "!< softening amount of the vignette effect, between 0 and 1"]
    pub feather: f32,
    #[doc = "!< color of the vignette effect, alpha is currently ignored"]
    pub color: filament_LinearColorA,
    #[doc = "!< enables or disables the vignette effect"]
    pub enabled: bool,
}
#[test]
fn bindgen_test_layout_filament_VignetteOptions() {
    assert_eq!(
        ::std::mem::size_of::<filament_VignetteOptions>(),
        32usize,
        concat!("Size of: ", stringify!(filament_VignetteOptions))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_VignetteOptions>(),
        4usize,
        concat!("Alignment of ", stringify!(filament_VignetteOptions))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_VignetteOptions>())).midPoint as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_VignetteOptions),
            "::",
            stringify!(midPoint)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_VignetteOptions>())).roundness as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_VignetteOptions),
            "::",
            stringify!(roundness)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_VignetteOptions>())).feather as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_VignetteOptions),
            "::",
            stringify!(feather)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<filament_VignetteOptions>())).color as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_VignetteOptions),
            "::",
            stringify!(color)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_VignetteOptions>())).enabled as *const _ as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_VignetteOptions),
            "::",
            stringify!(enabled)
        )
    );
}
impl Default for filament_VignetteOptions {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " Structure used to set the precision of the color buffer and related quality settings."]
#[doc = ""]
#[doc = " @see setRenderQuality, getRenderQuality"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct filament_RenderQuality {
    #[doc = " Sets the quality of the HDR color buffer."]
    #[doc = ""]
    #[doc = " A quality of HIGH or ULTRA means using an RGB16F or RGBA16F color buffer. This means"]
    #[doc = " colors in the LDR range (0..1) have a 10 bit precision. A quality of LOW or MEDIUM means"]
    #[doc = " using an R11G11B10F opaque color buffer or an RGBA16F transparent color buffer. With"]
    #[doc = " R11G11B10F colors in the LDR range have a precision of either 6 bits (red and green"]
    #[doc = " channels) or 5 bits (blue channel)."]
    pub hdrColorBuffer: filament_QualityLevel,
}
#[test]
fn bindgen_test_layout_filament_RenderQuality() {
    assert_eq!(
        ::std::mem::size_of::<filament_RenderQuality>(),
        1usize,
        concat!("Size of: ", stringify!(filament_RenderQuality))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_RenderQuality>(),
        1usize,
        concat!("Alignment of ", stringify!(filament_RenderQuality))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_RenderQuality>())).hdrColorBuffer as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_RenderQuality),
            "::",
            stringify!(hdrColorBuffer)
        )
    );
}
impl Default for filament_RenderQuality {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " Options for screen space Ambient Occlusion (SSAO) and Screen Space Cone Tracing (SSCT)"]
#[doc = " @see setAmbientOcclusionOptions()"]
#[repr(C)]
pub struct filament_AmbientOcclusionOptions {
    #[doc = "!< Ambient Occlusion radius in meters, between 0 and ~10."]
    pub radius: f32,
    #[doc = "!< Controls ambient occlusion's contrast. Must be positive."]
    pub power: f32,
    #[doc = "!< Self-occlusion bias in meters. Use to avoid self-occlusion. Between 0 and a few mm."]
    pub bias: f32,
    #[doc = "!< How each dimension of the AO buffer is scaled. Must be either 0.5 or 1.0."]
    pub resolution: f32,
    #[doc = "!< Strength of the Ambient Occlusion effect."]
    pub intensity: f32,
    #[doc = "!< depth distance that constitute an edge for filtering"]
    pub bilateralThreshold: f32,
    #[doc = "!< affects # of samples used for AO."]
    pub quality: filament_QualityLevel,
    #[doc = "!< affects AO smoothness"]
    pub lowPassFilter: filament_QualityLevel,
    #[doc = "!< affects AO buffer upsampling quality"]
    pub upsampling: filament_QualityLevel,
    #[doc = "!< enables or disables screen-space ambient occlusion"]
    pub enabled: bool,
    #[doc = "!< enables bent normals computation from AO, and specular AO"]
    pub bentNormals: bool,
    #[doc = "!< min angle in radian to consider"]
    pub minHorizonAngleRad: f32,
    pub ssct: filament_AmbientOcclusionOptions_Ssct,
}
#[doc = " Screen Space Cone Tracing (SSCT) options"]
#[doc = " Ambient shadows from dominant light"]
#[repr(C)]
pub struct filament_AmbientOcclusionOptions_Ssct {
    #[doc = "!< full cone angle in radian, between 0 and pi/2"]
    pub lightConeRad: f32,
    #[doc = "!< how far shadows can be cast"]
    pub shadowDistance: f32,
    #[doc = "!< max distance for contact"]
    pub contactDistanceMax: f32,
    #[doc = "!< intensity"]
    pub intensity: f32,
    #[doc = "!< light direction"]
    pub lightDirection: filament_math_float3,
    #[doc = "!< depth bias in world units (mitigate self shadowing)"]
    pub depthBias: f32,
    #[doc = "!< depth slope bias (mitigate self shadowing)"]
    pub depthSlopeBias: f32,
    #[doc = "!< tracing sample count, between 1 and 255"]
    pub sampleCount: u8,
    #[doc = "!< # of rays to trace, between 1 and 255"]
    pub rayCount: u8,
    #[doc = "!< enables or disables SSCT"]
    pub enabled: bool,
}
#[test]
fn bindgen_test_layout_filament_AmbientOcclusionOptions_Ssct() {
    assert_eq!(
        ::std::mem::size_of::<filament_AmbientOcclusionOptions_Ssct>(),
        40usize,
        concat!(
            "Size of: ",
            stringify!(filament_AmbientOcclusionOptions_Ssct)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_AmbientOcclusionOptions_Ssct>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(filament_AmbientOcclusionOptions_Ssct)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_AmbientOcclusionOptions_Ssct>())).lightConeRad
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_AmbientOcclusionOptions_Ssct),
            "::",
            stringify!(lightConeRad)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_AmbientOcclusionOptions_Ssct>())).shadowDistance
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_AmbientOcclusionOptions_Ssct),
            "::",
            stringify!(shadowDistance)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_AmbientOcclusionOptions_Ssct>())).contactDistanceMax
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_AmbientOcclusionOptions_Ssct),
            "::",
            stringify!(contactDistanceMax)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_AmbientOcclusionOptions_Ssct>())).intensity as *const _
                as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_AmbientOcclusionOptions_Ssct),
            "::",
            stringify!(intensity)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_AmbientOcclusionOptions_Ssct>())).lightDirection
                as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_AmbientOcclusionOptions_Ssct),
            "::",
            stringify!(lightDirection)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_AmbientOcclusionOptions_Ssct>())).depthBias as *const _
                as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_AmbientOcclusionOptions_Ssct),
            "::",
            stringify!(depthBias)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_AmbientOcclusionOptions_Ssct>())).depthSlopeBias
                as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_AmbientOcclusionOptions_Ssct),
            "::",
            stringify!(depthSlopeBias)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_AmbientOcclusionOptions_Ssct>())).sampleCount
                as *const _ as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_AmbientOcclusionOptions_Ssct),
            "::",
            stringify!(sampleCount)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_AmbientOcclusionOptions_Ssct>())).rayCount as *const _
                as usize
        },
        37usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_AmbientOcclusionOptions_Ssct),
            "::",
            stringify!(rayCount)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_AmbientOcclusionOptions_Ssct>())).enabled as *const _
                as usize
        },
        38usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_AmbientOcclusionOptions_Ssct),
            "::",
            stringify!(enabled)
        )
    );
}
impl Default for filament_AmbientOcclusionOptions_Ssct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn bindgen_test_layout_filament_AmbientOcclusionOptions() {
    assert_eq!(
        ::std::mem::size_of::<filament_AmbientOcclusionOptions>(),
        76usize,
        concat!("Size of: ", stringify!(filament_AmbientOcclusionOptions))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_AmbientOcclusionOptions>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(filament_AmbientOcclusionOptions)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_AmbientOcclusionOptions>())).radius as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_AmbientOcclusionOptions),
            "::",
            stringify!(radius)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_AmbientOcclusionOptions>())).power as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_AmbientOcclusionOptions),
            "::",
            stringify!(power)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_AmbientOcclusionOptions>())).bias as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_AmbientOcclusionOptions),
            "::",
            stringify!(bias)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_AmbientOcclusionOptions>())).resolution as *const _
                as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_AmbientOcclusionOptions),
            "::",
            stringify!(resolution)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_AmbientOcclusionOptions>())).intensity as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_AmbientOcclusionOptions),
            "::",
            stringify!(intensity)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_AmbientOcclusionOptions>())).bilateralThreshold
                as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_AmbientOcclusionOptions),
            "::",
            stringify!(bilateralThreshold)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_AmbientOcclusionOptions>())).quality as *const _
                as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_AmbientOcclusionOptions),
            "::",
            stringify!(quality)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_AmbientOcclusionOptions>())).lowPassFilter as *const _
                as usize
        },
        25usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_AmbientOcclusionOptions),
            "::",
            stringify!(lowPassFilter)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_AmbientOcclusionOptions>())).upsampling as *const _
                as usize
        },
        26usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_AmbientOcclusionOptions),
            "::",
            stringify!(upsampling)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_AmbientOcclusionOptions>())).enabled as *const _
                as usize
        },
        27usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_AmbientOcclusionOptions),
            "::",
            stringify!(enabled)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_AmbientOcclusionOptions>())).bentNormals as *const _
                as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_AmbientOcclusionOptions),
            "::",
            stringify!(bentNormals)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_AmbientOcclusionOptions>())).minHorizonAngleRad
                as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_AmbientOcclusionOptions),
            "::",
            stringify!(minHorizonAngleRad)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_AmbientOcclusionOptions>())).ssct as *const _ as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_AmbientOcclusionOptions),
            "::",
            stringify!(ssct)
        )
    );
}
impl Default for filament_AmbientOcclusionOptions {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " Options for Temporal Multi-Sample Anti-aliasing (MSAA)"]
#[doc = " @see setMultiSampleAntiAliasingOptions()"]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_MultiSampleAntiAliasingOptions {
    #[doc = "!< enables or disables msaa"]
    pub enabled: bool,
    #[doc = " sampleCount number of samples to use for multi-sampled anti-aliasing.\\n"]
    #[doc = "              0: treated as 1"]
    #[doc = "              1: no anti-aliasing"]
    #[doc = "              n: sample count. Effective sample could be different depending on the"]
    #[doc = "                 GPU capabilities."]
    pub sampleCount: u8,
    #[doc = " custom resolve improves quality for HDR scenes, but may impact performance."]
    pub customResolve: bool,
}
#[test]
fn bindgen_test_layout_filament_MultiSampleAntiAliasingOptions() {
    assert_eq!(
        ::std::mem::size_of::<filament_MultiSampleAntiAliasingOptions>(),
        3usize,
        concat!(
            "Size of: ",
            stringify!(filament_MultiSampleAntiAliasingOptions)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_MultiSampleAntiAliasingOptions>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(filament_MultiSampleAntiAliasingOptions)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_MultiSampleAntiAliasingOptions>())).enabled as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_MultiSampleAntiAliasingOptions),
            "::",
            stringify!(enabled)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_MultiSampleAntiAliasingOptions>())).sampleCount
                as *const _ as usize
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_MultiSampleAntiAliasingOptions),
            "::",
            stringify!(sampleCount)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_MultiSampleAntiAliasingOptions>())).customResolve
                as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_MultiSampleAntiAliasingOptions),
            "::",
            stringify!(customResolve)
        )
    );
}
#[doc = " Options for Temporal Anti-aliasing (TAA)"]
#[doc = " @see setTemporalAntiAliasingOptions()"]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_TemporalAntiAliasingOptions {
    #[doc = "!< reconstruction filter width typically between 0 (sharper, aliased) and 1 (smoother)"]
    pub filterWidth: f32,
    #[doc = "!< history feedback, between 0 (maximum temporal AA) and 1 (no temporal AA)."]
    pub feedback: f32,
    #[doc = "!< enables or disables temporal anti-aliasing"]
    pub enabled: bool,
}
#[test]
fn bindgen_test_layout_filament_TemporalAntiAliasingOptions() {
    assert_eq!(
        ::std::mem::size_of::<filament_TemporalAntiAliasingOptions>(),
        12usize,
        concat!(
            "Size of: ",
            stringify!(filament_TemporalAntiAliasingOptions)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_TemporalAntiAliasingOptions>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(filament_TemporalAntiAliasingOptions)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_TemporalAntiAliasingOptions>())).filterWidth as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_TemporalAntiAliasingOptions),
            "::",
            stringify!(filterWidth)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_TemporalAntiAliasingOptions>())).feedback as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_TemporalAntiAliasingOptions),
            "::",
            stringify!(feedback)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_TemporalAntiAliasingOptions>())).enabled as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_TemporalAntiAliasingOptions),
            "::",
            stringify!(enabled)
        )
    );
}
#[doc = " Options for Screen-space Reflections."]
#[doc = " @see setScreenSpaceReflectionsOptions()"]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_ScreenSpaceReflectionsOptions {
    #[doc = "!< ray thickness, in world units"]
    pub thickness: f32,
    #[doc = "!< bias, in world units, to prevent self-intersections"]
    pub bias: f32,
    #[doc = "!< maximum distance, in world units, to raycast"]
    pub maxDistance: f32,
    #[doc = "!< stride, in texels, for samples along the ray."]
    pub stride: f32,
    pub enabled: bool,
}
#[test]
fn bindgen_test_layout_filament_ScreenSpaceReflectionsOptions() {
    assert_eq!(
        ::std::mem::size_of::<filament_ScreenSpaceReflectionsOptions>(),
        20usize,
        concat!(
            "Size of: ",
            stringify!(filament_ScreenSpaceReflectionsOptions)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_ScreenSpaceReflectionsOptions>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(filament_ScreenSpaceReflectionsOptions)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_ScreenSpaceReflectionsOptions>())).thickness as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_ScreenSpaceReflectionsOptions),
            "::",
            stringify!(thickness)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_ScreenSpaceReflectionsOptions>())).bias as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_ScreenSpaceReflectionsOptions),
            "::",
            stringify!(bias)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_ScreenSpaceReflectionsOptions>())).maxDistance
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_ScreenSpaceReflectionsOptions),
            "::",
            stringify!(maxDistance)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_ScreenSpaceReflectionsOptions>())).stride as *const _
                as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_ScreenSpaceReflectionsOptions),
            "::",
            stringify!(stride)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_ScreenSpaceReflectionsOptions>())).enabled as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_ScreenSpaceReflectionsOptions),
            "::",
            stringify!(enabled)
        )
    );
}
#[doc = "!< no anti aliasing performed as part of post-processing"]
pub const filament_AntiAliasing_NONE: filament_AntiAliasing = 0;
#[doc = "!< FXAA is a low-quality but very efficient type of anti-aliasing. (default)."]
pub const filament_AntiAliasing_FXAA: filament_AntiAliasing = 1;
#[doc = " List of available post-processing anti-aliasing techniques."]
#[doc = " @see setAntiAliasing, getAntiAliasing, setSampleCount"]
pub type filament_AntiAliasing = u8;
#[doc = "!< No dithering"]
pub const filament_Dithering_NONE: filament_Dithering = 0;
#[doc = "!< Temporal dithering (default)"]
pub const filament_Dithering_TEMPORAL: filament_Dithering = 1;
#[doc = " List of available post-processing dithering techniques."]
pub type filament_Dithering = u8;
#[doc = "!< percentage-closer filtered shadows (default)"]
pub const filament_ShadowType_PCF: filament_ShadowType = 0;
#[doc = "!< variance shadows"]
pub const filament_ShadowType_VSM: filament_ShadowType = 1;
#[doc = "!< PCF with contact hardening simulation"]
pub const filament_ShadowType_DPCF: filament_ShadowType = 2;
#[doc = "!< PCF with soft shadows and contact hardening"]
pub const filament_ShadowType_PCSS: filament_ShadowType = 3;
#[doc = " List of available shadow mapping techniques."]
#[doc = " @see setShadowType"]
pub type filament_ShadowType = u8;
#[doc = " View-level options for VSM Shadowing."]
#[doc = " @see setVsmShadowOptions()"]
#[doc = " @warning This API is still experimental and subject to change."]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_VsmShadowOptions {
    #[doc = " Sets the number of anisotropic samples to use when sampling a VSM shadow map. If greater"]
    #[doc = " than 0, mipmaps will automatically be generated each frame for all lights."]
    #[doc = ""]
    #[doc = " The number of anisotropic samples = 2 ^ vsmAnisotropy."]
    pub anisotropy: u8,
    #[doc = " Whether to generate mipmaps for all VSM shadow maps."]
    pub mipmapping: bool,
    #[doc = " VSM minimum variance scale, must be positive."]
    pub minVarianceScale: f32,
    #[doc = " VSM light bleeding reduction amount, between 0 and 1."]
    pub lightBleedReduction: f32,
}
#[test]
fn bindgen_test_layout_filament_VsmShadowOptions() {
    assert_eq!(
        ::std::mem::size_of::<filament_VsmShadowOptions>(),
        12usize,
        concat!("Size of: ", stringify!(filament_VsmShadowOptions))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_VsmShadowOptions>(),
        4usize,
        concat!("Alignment of ", stringify!(filament_VsmShadowOptions))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_VsmShadowOptions>())).anisotropy as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_VsmShadowOptions),
            "::",
            stringify!(anisotropy)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_VsmShadowOptions>())).mipmapping as *const _ as usize
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_VsmShadowOptions),
            "::",
            stringify!(mipmapping)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_VsmShadowOptions>())).minVarianceScale as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_VsmShadowOptions),
            "::",
            stringify!(minVarianceScale)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_VsmShadowOptions>())).lightBleedReduction as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_VsmShadowOptions),
            "::",
            stringify!(lightBleedReduction)
        )
    );
}
#[doc = " View-level options for DPCF and PCSS Shadowing."]
#[doc = " @see setSoftShadowOptions()"]
#[doc = " @warning This API is still experimental and subject to change."]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_SoftShadowOptions {
    #[doc = " Globally scales the penumbra of all DPCF and PCSS shadows"]
    #[doc = " Acceptable values are greater than 0"]
    pub penumbraScale: f32,
    #[doc = " Globally scales the computed penumbra ratio of all DPCF and PCSS shadows."]
    #[doc = " This effectively controls the strength of contact hardening effect and is useful for"]
    #[doc = " artistic purposes. Higher values make the shadows become softer faster."]
    #[doc = " Acceptable values are equal to or greater than 1."]
    pub penumbraRatioScale: f32,
}
#[test]
fn bindgen_test_layout_filament_SoftShadowOptions() {
    assert_eq!(
        ::std::mem::size_of::<filament_SoftShadowOptions>(),
        8usize,
        concat!("Size of: ", stringify!(filament_SoftShadowOptions))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_SoftShadowOptions>(),
        4usize,
        concat!("Alignment of ", stringify!(filament_SoftShadowOptions))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_SoftShadowOptions>())).penumbraScale as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_SoftShadowOptions),
            "::",
            stringify!(penumbraScale)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_SoftShadowOptions>())).penumbraRatioScale as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_SoftShadowOptions),
            "::",
            stringify!(penumbraRatioScale)
        )
    );
}
#[doc = " Factory and manager for \\em renderables, which are entities that can be drawn."]
#[doc = ""]
#[doc = " Renderables are bundles of \\em primitives, each of which has its own geometry and material. All"]
#[doc = " primitives in a particular renderable share a set of rendering attributes, such as whether they"]
#[doc = " cast shadows or use vertex skinning."]
#[doc = ""]
#[doc = " Usage example:"]
#[doc = ""]
#[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
#[doc = " auto renderable = utils::EntityManager::get().create();"]
#[doc = ""]
#[doc = " RenderableManager::Builder(1)"]
#[doc = "         .boundingBox({{ -1, -1, -1 }, { 1, 1, 1 }})"]
#[doc = "         .material(0, matInstance)"]
#[doc = "         .geometry(0, RenderableManager::PrimitiveType::TRIANGLES, vertBuffer, indBuffer, 0, 3)"]
#[doc = "         .receiveShadows(false)"]
#[doc = "         .build(engine, renderable);"]
#[doc = ""]
#[doc = " scene->addEntity(renderable);"]
#[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
#[doc = ""]
#[doc = " To modify the state of an existing renderable, clients should first use RenderableManager"]
#[doc = " to get a temporary handle called an \\em instance. The instance can then be used to get or set"]
#[doc = " the renderable's state. Please note that instances are ephemeral; clients should store entities,"]
#[doc = " not instances."]
#[doc = ""]
#[doc = " - For details about constructing renderables, see RenderableManager::Builder."]
#[doc = " - To associate a 4x4 transform with an entity, see TransformManager."]
#[doc = " - To associate a human-readable label with an entity, see utils::NameComponentManager."]
#[repr(C)]
#[derive(Debug, Default)]
pub struct filament_RenderableManager {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct filament_RenderableManager_BuilderDetails {
    _unused: [u8; 0],
}
pub type filament_RenderableManager_Instance = u8;
#[doc = " Primitive types"]
pub use self::filament_backend_PrimitiveType as filament_RenderableManager_PrimitiveType;
#[doc = " The transformation associated with a skinning joint."]
#[doc = ""]
#[doc = " Clients can specify bones either using this quat-vec3 pair, or by using 4x4 matrices."]
#[repr(C)]
pub struct filament_RenderableManager_Bone {
    pub unitQuaternion: filament_math_quatf,
    pub translation: filament_math_float3,
    pub reserved: f32,
}
#[test]
fn bindgen_test_layout_filament_RenderableManager_Bone() {
    assert_eq!(
        ::std::mem::size_of::<filament_RenderableManager_Bone>(),
        32usize,
        concat!("Size of: ", stringify!(filament_RenderableManager_Bone))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_RenderableManager_Bone>(),
        4usize,
        concat!("Alignment of ", stringify!(filament_RenderableManager_Bone))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_RenderableManager_Bone>())).unitQuaternion as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_RenderableManager_Bone),
            "::",
            stringify!(unitQuaternion)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_RenderableManager_Bone>())).translation as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_RenderableManager_Bone),
            "::",
            stringify!(translation)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_RenderableManager_Bone>())).reserved as *const _
                as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_RenderableManager_Bone),
            "::",
            stringify!(reserved)
        )
    );
}
impl Default for filament_RenderableManager_Bone {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " Adds renderable components to entities using a builder pattern."]
#[repr(C)]
#[derive(Debug)]
pub struct filament_RenderableManager_Builder {
    pub _base: filament_BuilderBase<filament_RenderableManager_BuilderDetails>,
}
pub const filament_RenderableManager_Builder_Result_Error:
    filament_RenderableManager_Builder_Result = -1;
pub const filament_RenderableManager_Builder_Result_Success:
    filament_RenderableManager_Builder_Result = 0;
pub type filament_RenderableManager_Builder_Result = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct filament_RenderableManager_Builder_Entry {
    pub vertices: *mut filament_VertexBuffer,
    pub indices: *mut filament_IndexBuffer,
    pub offset: size_t,
    pub minIndex: size_t,
    pub maxIndex: size_t,
    pub count: size_t,
    pub materialInstance: *const filament_MaterialInstance,
    pub type_: filament_RenderableManager_PrimitiveType,
    pub blendOrder: u16,
}
#[test]
fn bindgen_test_layout_filament_RenderableManager_Builder_Entry() {
    assert_eq!(
        ::std::mem::size_of::<filament_RenderableManager_Builder_Entry>(),
        64usize,
        concat!(
            "Size of: ",
            stringify!(filament_RenderableManager_Builder_Entry)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_RenderableManager_Builder_Entry>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(filament_RenderableManager_Builder_Entry)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_RenderableManager_Builder_Entry>())).vertices
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_RenderableManager_Builder_Entry),
            "::",
            stringify!(vertices)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_RenderableManager_Builder_Entry>())).indices as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_RenderableManager_Builder_Entry),
            "::",
            stringify!(indices)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_RenderableManager_Builder_Entry>())).offset as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_RenderableManager_Builder_Entry),
            "::",
            stringify!(offset)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_RenderableManager_Builder_Entry>())).minIndex
                as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_RenderableManager_Builder_Entry),
            "::",
            stringify!(minIndex)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_RenderableManager_Builder_Entry>())).maxIndex
                as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_RenderableManager_Builder_Entry),
            "::",
            stringify!(maxIndex)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_RenderableManager_Builder_Entry>())).count as *const _
                as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_RenderableManager_Builder_Entry),
            "::",
            stringify!(count)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_RenderableManager_Builder_Entry>())).materialInstance
                as *const _ as usize
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_RenderableManager_Builder_Entry),
            "::",
            stringify!(materialInstance)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_RenderableManager_Builder_Entry>())).type_ as *const _
                as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_RenderableManager_Builder_Entry),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_RenderableManager_Builder_Entry>())).blendOrder
                as *const _ as usize
        },
        58usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_RenderableManager_Builder_Entry),
            "::",
            stringify!(blendOrder)
        )
    );
}
impl Default for filament_RenderableManager_Builder_Entry {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn bindgen_test_layout_filament_RenderableManager_Builder() {
    assert_eq!(
        ::std::mem::size_of::<filament_RenderableManager_Builder>(),
        8usize,
        concat!("Size of: ", stringify!(filament_RenderableManager_Builder))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_RenderableManager_Builder>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(filament_RenderableManager_Builder)
        )
    );
}
extern "C" {
    #[doc = " Specifies the geometry data for a primitive."]
    #[doc = ""]
    #[doc = " Filament primitives must have an associated VertexBuffer and IndexBuffer. Typically, each"]
    #[doc = " primitive is specified with a pair of daisy-chained calls: \\c geometry(...) and \\c"]
    #[doc = " material(...)."]
    #[doc = ""]
    #[doc = " @param index zero-based index of the primitive, must be less than the count passed to Builder constructor"]
    #[doc = " @param type specifies the topology of the primitive (e.g., \\c RenderableManager::PrimitiveType::TRIANGLES)"]
    #[doc = " @param vertices specifies the vertex buffer, which in turn specifies a set of attributes"]
    #[doc = " @param indices specifies the index buffer (either u16 or u32)"]
    #[doc = " @param offset specifies where in the index buffer to start reading (expressed as a number of indices)"]
    #[doc = " @param minIndex specifies the minimum index contained in the index buffer"]
    #[doc = " @param maxIndex specifies the maximum index contained in the index buffer"]
    #[doc = " @param count number of indices to read (for triangles, this should be a multiple of 3)"]
    #[link_name = "\u{1}_ZN8filament17RenderableManager7Builder8geometryEmNS_7backend13PrimitiveTypeEPNS_12VertexBufferEPNS_11IndexBufferEmmmm"]
    pub fn filament_RenderableManager_Builder_geometry(
        this: *mut filament_RenderableManager_Builder,
        index: size_t,
        type_: filament_RenderableManager_PrimitiveType,
        vertices: *mut filament_VertexBuffer,
        indices: *mut filament_IndexBuffer,
        offset: size_t,
        minIndex: size_t,
        maxIndex: size_t,
        count: size_t,
    ) -> *mut filament_RenderableManager_Builder;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament17RenderableManager7Builder8geometryEmNS_7backend13PrimitiveTypeEPNS_12VertexBufferEPNS_11IndexBufferEmm"]
    pub fn filament_RenderableManager_Builder_geometry1(
        this: *mut filament_RenderableManager_Builder,
        index: size_t,
        type_: filament_RenderableManager_PrimitiveType,
        vertices: *mut filament_VertexBuffer,
        indices: *mut filament_IndexBuffer,
        offset: size_t,
        count: size_t,
    ) -> *mut filament_RenderableManager_Builder;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament17RenderableManager7Builder8geometryEmNS_7backend13PrimitiveTypeEPNS_12VertexBufferEPNS_11IndexBufferE"]
    pub fn filament_RenderableManager_Builder_geometry2(
        this: *mut filament_RenderableManager_Builder,
        index: size_t,
        type_: filament_RenderableManager_PrimitiveType,
        vertices: *mut filament_VertexBuffer,
        indices: *mut filament_IndexBuffer,
    ) -> *mut filament_RenderableManager_Builder;
}
extern "C" {
    #[doc = " Binds a material instance to the specified primitive."]
    #[doc = ""]
    #[doc = " If no material is specified for a given primitive, Filament will fall back to a basic default material."]
    #[doc = ""]
    #[doc = " @param index zero-based index of the primitive, must be less than the count passed to Builder constructor"]
    #[doc = " @param materialInstance the material to bind"]
    #[link_name = "\u{1}_ZN8filament17RenderableManager7Builder8materialEmPKNS_16MaterialInstanceE"]
    pub fn filament_RenderableManager_Builder_material(
        this: *mut filament_RenderableManager_Builder,
        index: size_t,
        materialInstance: *const filament_MaterialInstance,
    ) -> *mut filament_RenderableManager_Builder;
}
extern "C" {
    #[doc = " The axis-aligned bounding box of the renderable."]
    #[doc = ""]
    #[doc = " This is an object-space AABB used for frustum culling. For skinning and morphing, this"]
    #[doc = " should encompass all possible vertex positions. It is mandatory unless culling is"]
    #[doc = " disabled for the renderable."]
    #[doc = ""]
    #[doc = " \\see computeAABB()"]
    #[link_name = "\u{1}_ZN8filament17RenderableManager7Builder11boundingBoxERKNS_3BoxE"]
    pub fn filament_RenderableManager_Builder_boundingBox(
        this: *mut filament_RenderableManager_Builder,
        axisAlignedBoundingBox: *const filament_Box,
    ) -> *mut filament_RenderableManager_Builder;
}
extern "C" {
    #[doc = " Sets bits in a visibility mask. By default, this is 0x1."]
    #[doc = ""]
    #[doc = " This feature provides a simple mechanism for hiding and showing groups of renderables"]
    #[doc = " in a Scene. See View::setVisibleLayers()."]
    #[doc = ""]
    #[doc = " For example, to set bit 1 and reset bits 0 and 2 while leaving all other bits unaffected,"]
    #[doc = " do: `builder.layerMask(7, 2)`."]
    #[doc = ""]
    #[doc = " To change this at run time, see RenderableManager::setLayerMask."]
    #[doc = ""]
    #[doc = " @param select the set of bits to affect"]
    #[doc = " @param values the replacement values for the affected bits"]
    #[link_name = "\u{1}_ZN8filament17RenderableManager7Builder9layerMaskEhh"]
    pub fn filament_RenderableManager_Builder_layerMask(
        this: *mut filament_RenderableManager_Builder,
        select: u8,
        values: u8,
    ) -> *mut filament_RenderableManager_Builder;
}
extern "C" {
    #[doc = " Provides coarse-grained control over draw order."]
    #[doc = ""]
    #[doc = " In general Filament reserves the right to re-order renderables to allow for efficient"]
    #[doc = " rendering. However clients can control ordering at a coarse level using \\em priority."]
    #[doc = ""]
    #[doc = " For example, this could be used to draw a semitransparent HUD, if a client wishes to"]
    #[doc = " avoid using a separate View for the HUD. Note that priority is completely orthogonal to"]
    #[doc = " Builder::layerMask, which merely controls visibility."]
    #[doc = ""]
    #[doc = " \\see Builder::blendOrder()"]
    #[doc = ""]
    #[doc = " The priority is clamped to the range [0..7], defaults to 4; 7 is lowest priority"]
    #[doc = " (rendered last)."]
    #[link_name = "\u{1}_ZN8filament17RenderableManager7Builder8priorityEh"]
    pub fn filament_RenderableManager_Builder_priority(
        this: *mut filament_RenderableManager_Builder,
        priority: u8,
    ) -> *mut filament_RenderableManager_Builder;
}
extern "C" {
    #[doc = " Controls frustum culling, true by default."]
    #[doc = ""]
    #[doc = " \\note Do not confuse frustum culling with backface culling. The latter is controlled via"]
    #[doc = " the material."]
    #[link_name = "\u{1}_ZN8filament17RenderableManager7Builder7cullingEb"]
    pub fn filament_RenderableManager_Builder_culling(
        this: *mut filament_RenderableManager_Builder,
        enable: bool,
    ) -> *mut filament_RenderableManager_Builder;
}
extern "C" {
    #[doc = " Enables or disables a light channel. Light channel 0 is enabled by default."]
    #[doc = ""]
    #[doc = " @param channel Light channel to enable or disable, between 0 and 7."]
    #[doc = " @param enable Whether to enable or disable the light channel."]
    #[link_name = "\u{1}_ZN8filament17RenderableManager7Builder12lightChannelEjb"]
    pub fn filament_RenderableManager_Builder_lightChannel(
        this: *mut filament_RenderableManager_Builder,
        channel: ::std::os::raw::c_uint,
        enable: bool,
    ) -> *mut filament_RenderableManager_Builder;
}
extern "C" {
    #[doc = " Controls if this renderable casts shadows, false by default."]
    #[doc = ""]
    #[doc = " If the View's shadow type is set to ShadowType::VSM, castShadows should only be disabled"]
    #[doc = " if either is true:"]
    #[doc = "   - receiveShadows is also disabled"]
    #[doc = "   - the object is guaranteed to not cast shadows on itself or other objects (for example,"]
    #[doc = "     a ground plane)"]
    #[link_name = "\u{1}_ZN8filament17RenderableManager7Builder11castShadowsEb"]
    pub fn filament_RenderableManager_Builder_castShadows(
        this: *mut filament_RenderableManager_Builder,
        enable: bool,
    ) -> *mut filament_RenderableManager_Builder;
}
extern "C" {
    #[doc = " Controls if this renderable receives shadows, true by default."]
    #[link_name = "\u{1}_ZN8filament17RenderableManager7Builder14receiveShadowsEb"]
    pub fn filament_RenderableManager_Builder_receiveShadows(
        this: *mut filament_RenderableManager_Builder,
        enable: bool,
    ) -> *mut filament_RenderableManager_Builder;
}
extern "C" {
    #[doc = " Controls if this renderable uses screen-space contact shadows. This is more"]
    #[doc = " expensive but can improve the quality of shadows, especially in large scenes."]
    #[doc = " (off by default)."]
    #[link_name = "\u{1}_ZN8filament17RenderableManager7Builder25screenSpaceContactShadowsEb"]
    pub fn filament_RenderableManager_Builder_screenSpaceContactShadows(
        this: *mut filament_RenderableManager_Builder,
        enable: bool,
    ) -> *mut filament_RenderableManager_Builder;
}
extern "C" {
    #[doc = " Allows bones to be swapped out and shared using SkinningBuffer."]
    #[doc = ""]
    #[doc = " If skinning buffer mode is enabled, clients must call setSkinningBuffer() rather than"]
    #[doc = " setBones(). This allows sharing of data between renderables."]
    #[doc = ""]
    #[doc = " @param enabled If true, enables buffer object mode.  False by default."]
    #[link_name = "\u{1}_ZN8filament17RenderableManager7Builder21enableSkinningBuffersEb"]
    pub fn filament_RenderableManager_Builder_enableSkinningBuffers(
        this: *mut filament_RenderableManager_Builder,
        enabled: bool,
    ) -> *mut filament_RenderableManager_Builder;
}
extern "C" {
    #[doc = " Enables GPU vertex skinning for up to 255 bones, 0 by default."]
    #[doc = ""]
    #[doc = " Skinning Buffer mode must be enabled."]
    #[doc = ""]
    #[doc = " Each vertex can be affected by up to 4 bones simultaneously. The attached"]
    #[doc = " VertexBuffer must provide data in the \\c BONE_INDICES slot (uvec4) and the"]
    #[doc = " \\c BONE_WEIGHTS slot (float4)."]
    #[doc = ""]
    #[doc = " See also RenderableManager::setSkinningBuffer() or SkinningBuffer::setBones(),"]
    #[doc = " which can be called on a per-frame basis to advance the animation."]
    #[doc = ""]
    #[doc = " @param skinningBuffer nullptr to disable, otherwise the SkinningBuffer to use"]
    #[doc = " @param count 0 to disable, otherwise the number of bone transforms (up to 255)"]
    #[doc = " @param offset offset in the SkinningBuffer"]
    #[link_name = "\u{1}_ZN8filament17RenderableManager7Builder8skinningEPNS_14SkinningBufferEmm"]
    pub fn filament_RenderableManager_Builder_skinning(
        this: *mut filament_RenderableManager_Builder,
        skinningBuffer: *mut filament_SkinningBuffer,
        count: size_t,
        offset: size_t,
    ) -> *mut filament_RenderableManager_Builder;
}
extern "C" {
    #[doc = " Enables GPU vertex skinning for up to 255 bones, 0 by default."]
    #[doc = ""]
    #[doc = " Skinning Buffer mode must be disabled."]
    #[doc = ""]
    #[doc = " Each vertex can be affected by up to 4 bones simultaneously. The attached"]
    #[doc = " VertexBuffer must provide data in the \\c BONE_INDICES slot (uvec4) and the"]
    #[doc = " \\c BONE_WEIGHTS slot (float4)."]
    #[doc = ""]
    #[doc = " See also RenderableManager::setBones(), which can be called on a per-frame basis"]
    #[doc = " to advance the animation."]
    #[doc = ""]
    #[doc = " @param boneCount 0 to disable, otherwise the number of bone transforms (up to 255)"]
    #[doc = " @param transforms the initial set of transforms (one for each bone)"]
    #[link_name = "\u{1}_ZN8filament17RenderableManager7Builder8skinningEmPKNS_4math7details6TMat44IfEE"]
    pub fn filament_RenderableManager_Builder_skinning1(
        this: *mut filament_RenderableManager_Builder,
        boneCount: size_t,
        transforms: *const filament_math_mat4f,
    ) -> *mut filament_RenderableManager_Builder;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament17RenderableManager7Builder8skinningEmPKNS0_4BoneE"]
    pub fn filament_RenderableManager_Builder_skinning2(
        this: *mut filament_RenderableManager_Builder,
        boneCount: size_t,
        bones: *const filament_RenderableManager_Bone,
    ) -> *mut filament_RenderableManager_Builder;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament17RenderableManager7Builder8skinningEm"]
    pub fn filament_RenderableManager_Builder_skinning3(
        this: *mut filament_RenderableManager_Builder,
        boneCount: size_t,
    ) -> *mut filament_RenderableManager_Builder;
}
extern "C" {
    #[doc = " Controls if the renderable has vertex morphing targets, zero by default. This is"]
    #[doc = " required to enable GPU morphing."]
    #[doc = ""]
    #[doc = " Filament supports two morphing modes: standard (default) and legacy."]
    #[doc = ""]
    #[doc = " For standard morphing, A MorphTargetBuffer must be created and provided via"]
    #[doc = " RenderableManager::setMorphTargetBufferAt(). Standard morphing supports up to"]
    #[doc = " \\c CONFIG_MAX_MORPH_TARGET_COUNT morph targets."]
    #[doc = ""]
    #[doc = " For legacy morphing, the attached VertexBuffer must provide data in the"]
    #[doc = " appropriate VertexAttribute slots (\\c MORPH_POSITION_0 etc). Legacy morphing only"]
    #[doc = " supports up to 4 morph targets and will be deprecated in the future. Legacy morphing must"]
    #[doc = " be enabled on the material definition: either via the legacyMorphing material attribute"]
    #[doc = " or by calling filamat::MaterialBuilder::useLegacyMorphing()."]
    #[doc = ""]
    #[doc = " See also RenderableManager::setMorphWeights(), which can be called on a per-frame basis"]
    #[doc = " to advance the animation."]
    #[link_name = "\u{1}_ZN8filament17RenderableManager7Builder8morphingEm"]
    pub fn filament_RenderableManager_Builder_morphing(
        this: *mut filament_RenderableManager_Builder,
        targetCount: size_t,
    ) -> *mut filament_RenderableManager_Builder;
}
extern "C" {
    #[doc = " Sets an ordering index for blended primitives that all live at the same Z value."]
    #[doc = ""]
    #[doc = " @param primitiveIndex the primitive of interest"]
    #[doc = " @param order draw order number (0 by default). Only the lowest 15 bits are used."]
    #[link_name = "\u{1}_ZN8filament17RenderableManager7Builder10blendOrderEmt"]
    pub fn filament_RenderableManager_Builder_blendOrder(
        this: *mut filament_RenderableManager_Builder,
        primitiveIndex: size_t,
        order: u16,
    ) -> *mut filament_RenderableManager_Builder;
}
extern "C" {
    #[doc = " Adds the Renderable component to an entity."]
    #[doc = ""]
    #[doc = " @param engine Reference to the filament::Engine to associate this Renderable with."]
    #[doc = " @param entity Entity to add the Renderable component to."]
    #[doc = " @return Success if the component was created successfully, Error otherwise."]
    #[doc = ""]
    #[doc = " If exceptions are disabled and an error occurs, this function is a no-op."]
    #[doc = "        Success can be checked by looking at the return value."]
    #[doc = ""]
    #[doc = " If this component already exists on the given entity and the construction is successful,"]
    #[doc = " it is first destroyed as if destroy(utils::Entity e) was called. In case of error,"]
    #[doc = " the existing component is unmodified."]
    #[doc = ""]
    #[doc = " @exception utils::PostConditionPanic if a runtime error occurred, such as running out of"]
    #[doc = "            memory or other resources."]
    #[doc = " @exception utils::PreConditionPanic if a parameter to a builder function was invalid."]
    #[link_name = "\u{1}_ZN8filament17RenderableManager7Builder5buildERNS_6EngineEN5utils6EntityE"]
    pub fn filament_RenderableManager_Builder_build(
        this: *mut filament_RenderableManager_Builder,
        engine: *mut filament_Engine,
        entity: utils_Entity,
    ) -> filament_RenderableManager_Builder_Result;
}
extern "C" {
    #[doc = " Creates a builder for renderable components."]
    #[doc = ""]
    #[doc = " @param count the number of primitives that will be supplied to the builder"]
    #[doc = ""]
    #[doc = " Note that builders typically do not have a long lifetime since clients should discard"]
    #[doc = " them after calling build(). For a usage example, see RenderableManager."]
    #[link_name = "\u{1}_ZN8filament17RenderableManager7BuilderC1Em"]
    pub fn filament_RenderableManager_Builder_Builder(
        this: *mut filament_RenderableManager_Builder,
        count: size_t,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament17RenderableManager7BuilderC1EOS1_"]
    pub fn filament_RenderableManager_Builder_Builder1(
        this: *mut filament_RenderableManager_Builder,
        rhs: *mut filament_RenderableManager_Builder,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament17RenderableManager7BuilderD1Ev"]
    pub fn filament_RenderableManager_Builder_Builder_destructor(
        this: *mut filament_RenderableManager_Builder,
    );
}
impl Default for filament_RenderableManager_Builder {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl filament_RenderableManager_Builder {
    #[inline]
    pub unsafe fn geometry(
        &mut self,
        index: size_t,
        type_: filament_RenderableManager_PrimitiveType,
        vertices: *mut filament_VertexBuffer,
        indices: *mut filament_IndexBuffer,
        offset: size_t,
        minIndex: size_t,
        maxIndex: size_t,
        count: size_t,
    ) -> *mut filament_RenderableManager_Builder {
        filament_RenderableManager_Builder_geometry(
            self, index, type_, vertices, indices, offset, minIndex, maxIndex, count,
        )
    }
    #[inline]
    pub unsafe fn geometry1(
        &mut self,
        index: size_t,
        type_: filament_RenderableManager_PrimitiveType,
        vertices: *mut filament_VertexBuffer,
        indices: *mut filament_IndexBuffer,
        offset: size_t,
        count: size_t,
    ) -> *mut filament_RenderableManager_Builder {
        filament_RenderableManager_Builder_geometry1(
            self, index, type_, vertices, indices, offset, count,
        )
    }
    #[inline]
    pub unsafe fn geometry2(
        &mut self,
        index: size_t,
        type_: filament_RenderableManager_PrimitiveType,
        vertices: *mut filament_VertexBuffer,
        indices: *mut filament_IndexBuffer,
    ) -> *mut filament_RenderableManager_Builder {
        filament_RenderableManager_Builder_geometry2(self, index, type_, vertices, indices)
    }
    #[inline]
    pub unsafe fn material(
        &mut self,
        index: size_t,
        materialInstance: *const filament_MaterialInstance,
    ) -> *mut filament_RenderableManager_Builder {
        filament_RenderableManager_Builder_material(self, index, materialInstance)
    }
    #[inline]
    pub unsafe fn boundingBox(
        &mut self,
        axisAlignedBoundingBox: *const filament_Box,
    ) -> *mut filament_RenderableManager_Builder {
        filament_RenderableManager_Builder_boundingBox(self, axisAlignedBoundingBox)
    }
    #[inline]
    pub unsafe fn layerMask(
        &mut self,
        select: u8,
        values: u8,
    ) -> *mut filament_RenderableManager_Builder {
        filament_RenderableManager_Builder_layerMask(self, select, values)
    }
    #[inline]
    pub unsafe fn priority(&mut self, priority: u8) -> *mut filament_RenderableManager_Builder {
        filament_RenderableManager_Builder_priority(self, priority)
    }
    #[inline]
    pub unsafe fn culling(&mut self, enable: bool) -> *mut filament_RenderableManager_Builder {
        filament_RenderableManager_Builder_culling(self, enable)
    }
    #[inline]
    pub unsafe fn lightChannel(
        &mut self,
        channel: ::std::os::raw::c_uint,
        enable: bool,
    ) -> *mut filament_RenderableManager_Builder {
        filament_RenderableManager_Builder_lightChannel(self, channel, enable)
    }
    #[inline]
    pub unsafe fn castShadows(&mut self, enable: bool) -> *mut filament_RenderableManager_Builder {
        filament_RenderableManager_Builder_castShadows(self, enable)
    }
    #[inline]
    pub unsafe fn receiveShadows(
        &mut self,
        enable: bool,
    ) -> *mut filament_RenderableManager_Builder {
        filament_RenderableManager_Builder_receiveShadows(self, enable)
    }
    #[inline]
    pub unsafe fn screenSpaceContactShadows(
        &mut self,
        enable: bool,
    ) -> *mut filament_RenderableManager_Builder {
        filament_RenderableManager_Builder_screenSpaceContactShadows(self, enable)
    }
    #[inline]
    pub unsafe fn enableSkinningBuffers(
        &mut self,
        enabled: bool,
    ) -> *mut filament_RenderableManager_Builder {
        filament_RenderableManager_Builder_enableSkinningBuffers(self, enabled)
    }
    #[inline]
    pub unsafe fn skinning(
        &mut self,
        skinningBuffer: *mut filament_SkinningBuffer,
        count: size_t,
        offset: size_t,
    ) -> *mut filament_RenderableManager_Builder {
        filament_RenderableManager_Builder_skinning(self, skinningBuffer, count, offset)
    }
    #[inline]
    pub unsafe fn skinning1(
        &mut self,
        boneCount: size_t,
        transforms: *const filament_math_mat4f,
    ) -> *mut filament_RenderableManager_Builder {
        filament_RenderableManager_Builder_skinning1(self, boneCount, transforms)
    }
    #[inline]
    pub unsafe fn skinning2(
        &mut self,
        boneCount: size_t,
        bones: *const filament_RenderableManager_Bone,
    ) -> *mut filament_RenderableManager_Builder {
        filament_RenderableManager_Builder_skinning2(self, boneCount, bones)
    }
    #[inline]
    pub unsafe fn skinning3(
        &mut self,
        boneCount: size_t,
    ) -> *mut filament_RenderableManager_Builder {
        filament_RenderableManager_Builder_skinning3(self, boneCount)
    }
    #[inline]
    pub unsafe fn morphing(
        &mut self,
        targetCount: size_t,
    ) -> *mut filament_RenderableManager_Builder {
        filament_RenderableManager_Builder_morphing(self, targetCount)
    }
    #[inline]
    pub unsafe fn blendOrder(
        &mut self,
        primitiveIndex: size_t,
        order: u16,
    ) -> *mut filament_RenderableManager_Builder {
        filament_RenderableManager_Builder_blendOrder(self, primitiveIndex, order)
    }
    #[inline]
    pub unsafe fn build(
        &mut self,
        engine: *mut filament_Engine,
        entity: utils_Entity,
    ) -> filament_RenderableManager_Builder_Result {
        filament_RenderableManager_Builder_build(self, engine, entity)
    }
    #[inline]
    pub unsafe fn new(count: size_t) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_RenderableManager_Builder_Builder(__bindgen_tmp.as_mut_ptr(), count);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(rhs: *mut filament_RenderableManager_Builder) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_RenderableManager_Builder_Builder1(__bindgen_tmp.as_mut_ptr(), rhs);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        filament_RenderableManager_Builder_Builder_destructor(self)
    }
}
#[doc = " \\cond PRIVATE"]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_RenderableManager_is_supported_vector_type {
    pub _address: u8,
}
pub type filament_RenderableManager_is_supported_vector_type_type = u8;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_RenderableManager_is_supported_index_type {
    pub _address: u8,
}
pub type filament_RenderableManager_is_supported_index_type_type = u8;
#[test]
fn bindgen_test_layout_filament_RenderableManager() {
    assert_eq!(
        ::std::mem::size_of::<filament_RenderableManager>(),
        1usize,
        concat!("Size of: ", stringify!(filament_RenderableManager))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_RenderableManager>(),
        1usize,
        concat!("Alignment of ", stringify!(filament_RenderableManager))
    );
}
extern "C" {
    #[doc = " Checks if the given entity already has a renderable component."]
    #[link_name = "\u{1}_ZNK8filament17RenderableManager12hasComponentEN5utils6EntityE"]
    pub fn filament_RenderableManager_hasComponent(
        this: *const filament_RenderableManager,
        e: utils_Entity,
    ) -> bool;
}
extern "C" {
    #[doc = " Gets a temporary handle that can be used to access the renderable state."]
    #[doc = ""]
    #[doc = " @return Non-zero handle if the entity has a renderable component, 0 otherwise."]
    #[link_name = "\u{1}_ZNK8filament17RenderableManager11getInstanceEN5utils6EntityE"]
    pub fn filament_RenderableManager_getInstance(
        this: *const filament_RenderableManager,
        e: utils_Entity,
    ) -> filament_RenderableManager_Instance;
}
extern "C" {
    #[doc = " Destroys the renderable component in the given entity."]
    #[link_name = "\u{1}_ZN8filament17RenderableManager7destroyEN5utils6EntityE"]
    pub fn filament_RenderableManager_destroy(
        this: *mut filament_RenderableManager,
        e: utils_Entity,
    );
}
extern "C" {
    #[doc = " Changes the bounding box used for frustum culling."]
    #[doc = ""]
    #[doc = " \\see Builder::boundingBox()"]
    #[doc = " \\see RenderableManager::getAxisAlignedBoundingBox()"]
    #[link_name = "\u{1}_ZN8filament17RenderableManager25setAxisAlignedBoundingBoxEN5utils14EntityInstanceIS0_Lb0EEERKNS_3BoxE"]
    pub fn filament_RenderableManager_setAxisAlignedBoundingBox(
        this: *mut filament_RenderableManager,
        instance: filament_RenderableManager_Instance,
        aabb: *const filament_Box,
    );
}
extern "C" {
    #[doc = " Changes the visibility bits."]
    #[doc = ""]
    #[doc = " \\see Builder::layerMask()"]
    #[doc = " \\see View::setVisibleLayers()."]
    #[doc = " \\see RenderableManager::getLayerMask()"]
    #[link_name = "\u{1}_ZN8filament17RenderableManager12setLayerMaskEN5utils14EntityInstanceIS0_Lb0EEEhh"]
    pub fn filament_RenderableManager_setLayerMask(
        this: *mut filament_RenderableManager,
        instance: filament_RenderableManager_Instance,
        select: u8,
        values: u8,
    );
}
extern "C" {
    #[doc = " Changes the coarse-level draw ordering."]
    #[doc = ""]
    #[doc = " \\see Builder::priority()."]
    #[link_name = "\u{1}_ZN8filament17RenderableManager11setPriorityEN5utils14EntityInstanceIS0_Lb0EEEh"]
    pub fn filament_RenderableManager_setPriority(
        this: *mut filament_RenderableManager,
        instance: filament_RenderableManager_Instance,
        priority: u8,
    );
}
extern "C" {
    #[doc = " Changes whether or not frustum culling is on."]
    #[doc = ""]
    #[doc = " \\see Builder::culling()"]
    #[link_name = "\u{1}_ZN8filament17RenderableManager10setCullingEN5utils14EntityInstanceIS0_Lb0EEEb"]
    pub fn filament_RenderableManager_setCulling(
        this: *mut filament_RenderableManager,
        instance: filament_RenderableManager_Instance,
        enable: bool,
    );
}
extern "C" {
    #[doc = " Enables or disables a light channel."]
    #[doc = " Light channel 0 is enabled by default."]
    #[doc = ""]
    #[doc = " \\see Builder::lightChannel()"]
    #[link_name = "\u{1}_ZN8filament17RenderableManager15setLightChannelEN5utils14EntityInstanceIS0_Lb0EEEjb"]
    pub fn filament_RenderableManager_setLightChannel(
        this: *mut filament_RenderableManager,
        instance: filament_RenderableManager_Instance,
        channel: ::std::os::raw::c_uint,
        enable: bool,
    );
}
extern "C" {
    #[doc = " Returns whether a light channel is enabled on a specified renderable."]
    #[doc = " @param instance Instance of the component obtained from getInstance()."]
    #[doc = " @param channel  Light channel to query"]
    #[doc = " @return         true if the light channel is enabled, false otherwise"]
    #[link_name = "\u{1}_ZNK8filament17RenderableManager15getLightChannelEN5utils14EntityInstanceIS0_Lb0EEEj"]
    pub fn filament_RenderableManager_getLightChannel(
        this: *const filament_RenderableManager,
        instance: filament_RenderableManager_Instance,
        channel: ::std::os::raw::c_uint,
    ) -> bool;
}
extern "C" {
    #[doc = " Changes whether or not the renderable casts shadows."]
    #[doc = ""]
    #[doc = " \\see Builder::castShadows()"]
    #[link_name = "\u{1}_ZN8filament17RenderableManager14setCastShadowsEN5utils14EntityInstanceIS0_Lb0EEEb"]
    pub fn filament_RenderableManager_setCastShadows(
        this: *mut filament_RenderableManager,
        instance: filament_RenderableManager_Instance,
        enable: bool,
    );
}
extern "C" {
    #[doc = " Changes whether or not the renderable can receive shadows."]
    #[doc = ""]
    #[doc = " \\see Builder::receiveShadows()"]
    #[link_name = "\u{1}_ZN8filament17RenderableManager17setReceiveShadowsEN5utils14EntityInstanceIS0_Lb0EEEb"]
    pub fn filament_RenderableManager_setReceiveShadows(
        this: *mut filament_RenderableManager,
        instance: filament_RenderableManager_Instance,
        enable: bool,
    );
}
extern "C" {
    #[doc = " Changes whether or not the renderable can use screen-space contact shadows."]
    #[doc = ""]
    #[doc = " \\see Builder::screenSpaceContactShadows()"]
    #[link_name = "\u{1}_ZN8filament17RenderableManager28setScreenSpaceContactShadowsEN5utils14EntityInstanceIS0_Lb0EEEb"]
    pub fn filament_RenderableManager_setScreenSpaceContactShadows(
        this: *mut filament_RenderableManager,
        instance: filament_RenderableManager_Instance,
        enable: bool,
    );
}
extern "C" {
    #[doc = " Checks if the renderable can cast shadows."]
    #[doc = ""]
    #[doc = " \\see Builder::castShadows()."]
    #[link_name = "\u{1}_ZNK8filament17RenderableManager14isShadowCasterEN5utils14EntityInstanceIS0_Lb0EEE"]
    pub fn filament_RenderableManager_isShadowCaster(
        this: *const filament_RenderableManager,
        instance: filament_RenderableManager_Instance,
    ) -> bool;
}
extern "C" {
    #[doc = " Checks if the renderable can receive shadows."]
    #[doc = ""]
    #[doc = " \\see Builder::receiveShadows()."]
    #[link_name = "\u{1}_ZNK8filament17RenderableManager16isShadowReceiverEN5utils14EntityInstanceIS0_Lb0EEE"]
    pub fn filament_RenderableManager_isShadowReceiver(
        this: *const filament_RenderableManager,
        instance: filament_RenderableManager_Instance,
    ) -> bool;
}
extern "C" {
    #[doc = " Updates the bone transforms in the range [offset, offset + boneCount)."]
    #[doc = " The bones must be pre-allocated using Builder::skinning()."]
    #[link_name = "\u{1}_ZN8filament17RenderableManager8setBonesEN5utils14EntityInstanceIS0_Lb0EEEPKNS0_4BoneEmm"]
    pub fn filament_RenderableManager_setBones(
        this: *mut filament_RenderableManager,
        instance: filament_RenderableManager_Instance,
        transforms: *const filament_RenderableManager_Bone,
        boneCount: size_t,
        offset: size_t,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament17RenderableManager8setBonesEN5utils14EntityInstanceIS0_Lb0EEEPKNS_4math7details6TMat44IfEEmm"]
    pub fn filament_RenderableManager_setBones1(
        this: *mut filament_RenderableManager,
        instance: filament_RenderableManager_Instance,
        transforms: *const filament_math_mat4f,
        boneCount: size_t,
        offset: size_t,
    );
}
extern "C" {
    #[doc = " Associates a SkinningBuffer to a renderable instance"]
    #[link_name = "\u{1}_ZN8filament17RenderableManager17setSkinningBufferEN5utils14EntityInstanceIS0_Lb0EEEPNS_14SkinningBufferEmm"]
    pub fn filament_RenderableManager_setSkinningBuffer(
        this: *mut filament_RenderableManager,
        instance: filament_RenderableManager_Instance,
        skinningBuffer: *mut filament_SkinningBuffer,
        count: size_t,
        offset: size_t,
    );
}
extern "C" {
    #[doc = " Updates the vertex morphing weights on a renderable, all zeroes by default."]
    #[doc = ""]
    #[doc = " The renderable must be built with morphing enabled, see Builder::morphing(). In legacy"]
    #[doc = " morphing mode, only the first 4 weights are considered."]
    #[doc = ""]
    #[doc = " @param instance Instance of the component obtained from getInstance()."]
    #[doc = " @param weights Pointer to morph target weights to be update."]
    #[doc = " @param count Number of morph target weights."]
    #[doc = " @param offset Index of the first first morph target weight to set at instance."]
    #[link_name = "\u{1}_ZN8filament17RenderableManager15setMorphWeightsEN5utils14EntityInstanceIS0_Lb0EEEPKfmm"]
    pub fn filament_RenderableManager_setMorphWeights(
        this: *mut filament_RenderableManager,
        instance: filament_RenderableManager_Instance,
        weights: *const f32,
        count: size_t,
        offset: size_t,
    );
}
extern "C" {
    #[doc = " Associates a MorphTargetBuffer to the given primitive."]
    #[link_name = "\u{1}_ZN8filament17RenderableManager22setMorphTargetBufferAtEN5utils14EntityInstanceIS0_Lb0EEEhmPNS_17MorphTargetBufferEmm"]
    pub fn filament_RenderableManager_setMorphTargetBufferAt(
        this: *mut filament_RenderableManager,
        instance: filament_RenderableManager_Instance,
        level: u8,
        primitiveIndex: size_t,
        morphTargetBuffer: *mut filament_MorphTargetBuffer,
        offset: size_t,
        count: size_t,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament17RenderableManager22setMorphTargetBufferAtEN5utils14EntityInstanceIS0_Lb0EEEhmPNS_17MorphTargetBufferEm"]
    pub fn filament_RenderableManager_setMorphTargetBufferAt1(
        this: *mut filament_RenderableManager,
        instance: filament_RenderableManager_Instance,
        level: u8,
        primitiveIndex: size_t,
        morphTargetBuffer: *mut filament_MorphTargetBuffer,
        count: size_t,
    );
}
extern "C" {
    #[doc = " Gets t"]
    #[doc = " number of morphing in the given entity."]
    #[link_name = "\u{1}_ZNK8filament17RenderableManager19getMorphTargetCountEN5utils14EntityInstanceIS0_Lb0EEE"]
    pub fn filament_RenderableManager_getMorphTargetCount(
        this: *const filament_RenderableManager,
        instance: filament_RenderableManager_Instance,
    ) -> size_t;
}
extern "C" {
    #[doc = " Gets the bounding box used for frustum culling."]
    #[doc = ""]
    #[doc = " \\see Builder::boundingBox()"]
    #[doc = " \\see RenderableManager::setAxisAlignedBoundingBox()"]
    #[link_name = "\u{1}_ZNK8filament17RenderableManager25getAxisAlignedBoundingBoxEN5utils14EntityInstanceIS0_Lb0EEE"]
    pub fn filament_RenderableManager_getAxisAlignedBoundingBox(
        this: *const filament_RenderableManager,
        instance: filament_RenderableManager_Instance,
    ) -> *const filament_Box;
}
extern "C" {
    #[doc = " Get the visibility bits."]
    #[doc = ""]
    #[doc = " \\see Builder::layerMask()"]
    #[doc = " \\see View::setVisibleLayers()."]
    #[doc = " \\see RenderableManager::getLayerMask()"]
    #[link_name = "\u{1}_ZNK8filament17RenderableManager12getLayerMaskEN5utils14EntityInstanceIS0_Lb0EEE"]
    pub fn filament_RenderableManager_getLayerMask(
        this: *const filament_RenderableManager,
        instance: filament_RenderableManager_Instance,
    ) -> u8;
}
extern "C" {
    #[doc = " Gets the immutable number of primitives in the given renderable."]
    #[link_name = "\u{1}_ZNK8filament17RenderableManager17getPrimitiveCountEN5utils14EntityInstanceIS0_Lb0EEE"]
    pub fn filament_RenderableManager_getPrimitiveCount(
        this: *const filament_RenderableManager,
        instance: filament_RenderableManager_Instance,
    ) -> size_t;
}
extern "C" {
    #[doc = " Changes the material instance binding for the given primitive."]
    #[doc = ""]
    #[doc = " \\see Builder::material()"]
    #[link_name = "\u{1}_ZN8filament17RenderableManager21setMaterialInstanceAtEN5utils14EntityInstanceIS0_Lb0EEEmPKNS_16MaterialInstanceE"]
    pub fn filament_RenderableManager_setMaterialInstanceAt(
        this: *mut filament_RenderableManager,
        instance: filament_RenderableManager_Instance,
        primitiveIndex: size_t,
        materialInstance: *const filament_MaterialInstance,
    );
}
extern "C" {
    #[doc = " Retrieves the material instance that is bound to the given primitive."]
    #[link_name = "\u{1}_ZNK8filament17RenderableManager21getMaterialInstanceAtEN5utils14EntityInstanceIS0_Lb0EEEm"]
    pub fn filament_RenderableManager_getMaterialInstanceAt(
        this: *const filament_RenderableManager,
        instance: filament_RenderableManager_Instance,
        primitiveIndex: size_t,
    ) -> *mut filament_MaterialInstance;
}
extern "C" {
    #[doc = " Changes the geometry for the given primitive."]
    #[doc = ""]
    #[doc = " \\see Builder::geometry()"]
    #[link_name = "\u{1}_ZN8filament17RenderableManager13setGeometryAtEN5utils14EntityInstanceIS0_Lb0EEEmNS_7backend13PrimitiveTypeEPNS_12VertexBufferEPNS_11IndexBufferEmm"]
    pub fn filament_RenderableManager_setGeometryAt(
        this: *mut filament_RenderableManager,
        instance: filament_RenderableManager_Instance,
        primitiveIndex: size_t,
        type_: filament_RenderableManager_PrimitiveType,
        vertices: *mut filament_VertexBuffer,
        indices: *mut filament_IndexBuffer,
        offset: size_t,
        count: size_t,
    );
}
extern "C" {
    #[doc = " Changes the active range of indices or topology for the given primitive."]
    #[doc = ""]
    #[doc = " \\see Builder::geometry()"]
    #[link_name = "\u{1}_ZN8filament17RenderableManager13setGeometryAtEN5utils14EntityInstanceIS0_Lb0EEEmNS_7backend13PrimitiveTypeEmm"]
    pub fn filament_RenderableManager_setGeometryAt1(
        this: *mut filament_RenderableManager,
        instance: filament_RenderableManager_Instance,
        primitiveIndex: size_t,
        type_: filament_RenderableManager_PrimitiveType,
        offset: size_t,
        count: size_t,
    );
}
extern "C" {
    #[doc = " Changes the ordering index for blended primitives that all live at the same Z value."]
    #[doc = ""]
    #[doc = " \\see Builder::blendOrder()"]
    #[doc = ""]
    #[doc = " @param instance the renderable of interest"]
    #[doc = " @param primitiveIndex the primitive of interest"]
    #[doc = " @param order draw order number (0 by default). Only the lowest 15 bits are used."]
    #[link_name = "\u{1}_ZN8filament17RenderableManager15setBlendOrderAtEN5utils14EntityInstanceIS0_Lb0EEEmt"]
    pub fn filament_RenderableManager_setBlendOrderAt(
        this: *mut filament_RenderableManager,
        instance: filament_RenderableManager_Instance,
        primitiveIndex: size_t,
        order: u16,
    );
}
extern "C" {
    #[doc = " Retrieves the set of enabled attribute slots in the given primitive's VertexBuffer."]
    #[link_name = "\u{1}_ZNK8filament17RenderableManager22getEnabledAttributesAtEN5utils14EntityInstanceIS0_Lb0EEEm"]
    pub fn filament_RenderableManager_getEnabledAttributesAt(
        this: *const filament_RenderableManager,
        instance: filament_RenderableManager_Instance,
        primitiveIndex: size_t,
    ) -> filament_AttributeBitset;
}
impl filament_RenderableManager {
    #[inline]
    pub unsafe fn hasComponent(&self, e: utils_Entity) -> bool {
        filament_RenderableManager_hasComponent(self, e)
    }
    #[inline]
    pub unsafe fn getInstance(&self, e: utils_Entity) -> filament_RenderableManager_Instance {
        filament_RenderableManager_getInstance(self, e)
    }
    #[inline]
    pub unsafe fn destroy(&mut self, e: utils_Entity) {
        filament_RenderableManager_destroy(self, e)
    }
    #[inline]
    pub unsafe fn setAxisAlignedBoundingBox(
        &mut self,
        instance: filament_RenderableManager_Instance,
        aabb: *const filament_Box,
    ) {
        filament_RenderableManager_setAxisAlignedBoundingBox(self, instance, aabb)
    }
    #[inline]
    pub unsafe fn setLayerMask(
        &mut self,
        instance: filament_RenderableManager_Instance,
        select: u8,
        values: u8,
    ) {
        filament_RenderableManager_setLayerMask(self, instance, select, values)
    }
    #[inline]
    pub unsafe fn setPriority(
        &mut self,
        instance: filament_RenderableManager_Instance,
        priority: u8,
    ) {
        filament_RenderableManager_setPriority(self, instance, priority)
    }
    #[inline]
    pub unsafe fn setCulling(
        &mut self,
        instance: filament_RenderableManager_Instance,
        enable: bool,
    ) {
        filament_RenderableManager_setCulling(self, instance, enable)
    }
    #[inline]
    pub unsafe fn setLightChannel(
        &mut self,
        instance: filament_RenderableManager_Instance,
        channel: ::std::os::raw::c_uint,
        enable: bool,
    ) {
        filament_RenderableManager_setLightChannel(self, instance, channel, enable)
    }
    #[inline]
    pub unsafe fn getLightChannel(
        &self,
        instance: filament_RenderableManager_Instance,
        channel: ::std::os::raw::c_uint,
    ) -> bool {
        filament_RenderableManager_getLightChannel(self, instance, channel)
    }
    #[inline]
    pub unsafe fn setCastShadows(
        &mut self,
        instance: filament_RenderableManager_Instance,
        enable: bool,
    ) {
        filament_RenderableManager_setCastShadows(self, instance, enable)
    }
    #[inline]
    pub unsafe fn setReceiveShadows(
        &mut self,
        instance: filament_RenderableManager_Instance,
        enable: bool,
    ) {
        filament_RenderableManager_setReceiveShadows(self, instance, enable)
    }
    #[inline]
    pub unsafe fn setScreenSpaceContactShadows(
        &mut self,
        instance: filament_RenderableManager_Instance,
        enable: bool,
    ) {
        filament_RenderableManager_setScreenSpaceContactShadows(self, instance, enable)
    }
    #[inline]
    pub unsafe fn isShadowCaster(&self, instance: filament_RenderableManager_Instance) -> bool {
        filament_RenderableManager_isShadowCaster(self, instance)
    }
    #[inline]
    pub unsafe fn isShadowReceiver(&self, instance: filament_RenderableManager_Instance) -> bool {
        filament_RenderableManager_isShadowReceiver(self, instance)
    }
    #[inline]
    pub unsafe fn setBones(
        &mut self,
        instance: filament_RenderableManager_Instance,
        transforms: *const filament_RenderableManager_Bone,
        boneCount: size_t,
        offset: size_t,
    ) {
        filament_RenderableManager_setBones(self, instance, transforms, boneCount, offset)
    }
    #[inline]
    pub unsafe fn setBones1(
        &mut self,
        instance: filament_RenderableManager_Instance,
        transforms: *const filament_math_mat4f,
        boneCount: size_t,
        offset: size_t,
    ) {
        filament_RenderableManager_setBones1(self, instance, transforms, boneCount, offset)
    }
    #[inline]
    pub unsafe fn setSkinningBuffer(
        &mut self,
        instance: filament_RenderableManager_Instance,
        skinningBuffer: *mut filament_SkinningBuffer,
        count: size_t,
        offset: size_t,
    ) {
        filament_RenderableManager_setSkinningBuffer(self, instance, skinningBuffer, count, offset)
    }
    #[inline]
    pub unsafe fn setMorphWeights(
        &mut self,
        instance: filament_RenderableManager_Instance,
        weights: *const f32,
        count: size_t,
        offset: size_t,
    ) {
        filament_RenderableManager_setMorphWeights(self, instance, weights, count, offset)
    }
    #[inline]
    pub unsafe fn setMorphTargetBufferAt(
        &mut self,
        instance: filament_RenderableManager_Instance,
        level: u8,
        primitiveIndex: size_t,
        morphTargetBuffer: *mut filament_MorphTargetBuffer,
        offset: size_t,
        count: size_t,
    ) {
        filament_RenderableManager_setMorphTargetBufferAt(
            self,
            instance,
            level,
            primitiveIndex,
            morphTargetBuffer,
            offset,
            count,
        )
    }
    #[inline]
    pub unsafe fn setMorphTargetBufferAt1(
        &mut self,
        instance: filament_RenderableManager_Instance,
        level: u8,
        primitiveIndex: size_t,
        morphTargetBuffer: *mut filament_MorphTargetBuffer,
        count: size_t,
    ) {
        filament_RenderableManager_setMorphTargetBufferAt1(
            self,
            instance,
            level,
            primitiveIndex,
            morphTargetBuffer,
            count,
        )
    }
    #[inline]
    pub unsafe fn getMorphTargetCount(
        &self,
        instance: filament_RenderableManager_Instance,
    ) -> size_t {
        filament_RenderableManager_getMorphTargetCount(self, instance)
    }
    #[inline]
    pub unsafe fn getAxisAlignedBoundingBox(
        &self,
        instance: filament_RenderableManager_Instance,
    ) -> *const filament_Box {
        filament_RenderableManager_getAxisAlignedBoundingBox(self, instance)
    }
    #[inline]
    pub unsafe fn getLayerMask(&self, instance: filament_RenderableManager_Instance) -> u8 {
        filament_RenderableManager_getLayerMask(self, instance)
    }
    #[inline]
    pub unsafe fn getPrimitiveCount(
        &self,
        instance: filament_RenderableManager_Instance,
    ) -> size_t {
        filament_RenderableManager_getPrimitiveCount(self, instance)
    }
    #[inline]
    pub unsafe fn setMaterialInstanceAt(
        &mut self,
        instance: filament_RenderableManager_Instance,
        primitiveIndex: size_t,
        materialInstance: *const filament_MaterialInstance,
    ) {
        filament_RenderableManager_setMaterialInstanceAt(
            self,
            instance,
            primitiveIndex,
            materialInstance,
        )
    }
    #[inline]
    pub unsafe fn getMaterialInstanceAt(
        &self,
        instance: filament_RenderableManager_Instance,
        primitiveIndex: size_t,
    ) -> *mut filament_MaterialInstance {
        filament_RenderableManager_getMaterialInstanceAt(self, instance, primitiveIndex)
    }
    #[inline]
    pub unsafe fn setGeometryAt(
        &mut self,
        instance: filament_RenderableManager_Instance,
        primitiveIndex: size_t,
        type_: filament_RenderableManager_PrimitiveType,
        vertices: *mut filament_VertexBuffer,
        indices: *mut filament_IndexBuffer,
        offset: size_t,
        count: size_t,
    ) {
        filament_RenderableManager_setGeometryAt(
            self,
            instance,
            primitiveIndex,
            type_,
            vertices,
            indices,
            offset,
            count,
        )
    }
    #[inline]
    pub unsafe fn setGeometryAt1(
        &mut self,
        instance: filament_RenderableManager_Instance,
        primitiveIndex: size_t,
        type_: filament_RenderableManager_PrimitiveType,
        offset: size_t,
        count: size_t,
    ) {
        filament_RenderableManager_setGeometryAt1(
            self,
            instance,
            primitiveIndex,
            type_,
            offset,
            count,
        )
    }
    #[inline]
    pub unsafe fn setBlendOrderAt(
        &mut self,
        instance: filament_RenderableManager_Instance,
        primitiveIndex: size_t,
        order: u16,
    ) {
        filament_RenderableManager_setBlendOrderAt(self, instance, primitiveIndex, order)
    }
    #[inline]
    pub unsafe fn getEnabledAttributesAt(
        &self,
        instance: filament_RenderableManager_Instance,
        primitiveIndex: size_t,
    ) -> filament_AttributeBitset {
        filament_RenderableManager_getEnabledAttributesAt(self, instance, primitiveIndex)
    }
}
#[doc = " A Renderer instance represents an operating system's window."]
#[doc = ""]
#[doc = " Typically, applications create a Renderer per window. The Renderer generates drawing commands"]
#[doc = " for the render thread and manages frame latency."]
#[doc = ""]
#[doc = " A Renderer generates drawing commands from a View, itself containing a Scene description."]
#[doc = ""]
#[doc = " Creation and Destruction"]
#[doc = " ========================"]
#[doc = ""]
#[doc = " A Renderer is created using Engine.createRenderer() and destroyed using"]
#[doc = " Engine.destroy(const Renderer*)."]
#[doc = ""]
#[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
#[doc = " #include <filament/Renderer.h>"]
#[doc = " #include <filament/Engine.h>"]
#[doc = " using namespace filament;"]
#[doc = ""]
#[doc = " Engine* engine = Engine::create();"]
#[doc = ""]
#[doc = " Renderer* renderer = engine->createRenderer();"]
#[doc = " engine->destroy(&renderer);"]
#[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
#[doc = ""]
#[doc = " @see Engine, View"]
#[repr(C)]
#[derive(Debug, Default)]
pub struct filament_Renderer {
    pub _address: u8,
}
#[doc = " Use DisplayInfo to set important Display properties. This is used to achieve correct"]
#[doc = " frame pacing and dynamic resolution scaling."]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_Renderer_DisplayInfo {
    pub refreshRate: f32,
    pub presentationDeadlineNanos: u64,
    pub vsyncOffsetNanos: u64,
}
#[test]
fn bindgen_test_layout_filament_Renderer_DisplayInfo() {
    assert_eq!(
        ::std::mem::size_of::<filament_Renderer_DisplayInfo>(),
        24usize,
        concat!("Size of: ", stringify!(filament_Renderer_DisplayInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_Renderer_DisplayInfo>(),
        8usize,
        concat!("Alignment of ", stringify!(filament_Renderer_DisplayInfo))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_Renderer_DisplayInfo>())).refreshRate as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_Renderer_DisplayInfo),
            "::",
            stringify!(refreshRate)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_Renderer_DisplayInfo>())).presentationDeadlineNanos
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_Renderer_DisplayInfo),
            "::",
            stringify!(presentationDeadlineNanos)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_Renderer_DisplayInfo>())).vsyncOffsetNanos as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_Renderer_DisplayInfo),
            "::",
            stringify!(vsyncOffsetNanos)
        )
    );
}
#[doc = " Use FrameRateOptions to set the desired frame rate and control how quickly the system"]
#[doc = " reacts to GPU load changes."]
#[doc = ""]
#[doc = " interval: desired frame interval in multiple of the refresh period, set in DisplayInfo"]
#[doc = "           (as 1 / DisplayInfo::refreshRate)"]
#[doc = ""]
#[doc = " The parameters below are relevant when some Views are using dynamic resolution scaling:"]
#[doc = ""]
#[doc = " headRoomRatio: additional headroom for the GPU as a ratio of the targetFrameTime."]
#[doc = "                Useful for taking into account constant costs like post-processing or"]
#[doc = "                GPU drivers on different platforms."]
#[doc = " history:   History size. higher values, tend to filter more (clamped to 31)"]
#[doc = " scaleRate: rate at which the gpu load is adjusted to reach the target frame rate"]
#[doc = "            This value can be computed as 1 / N, where N is the number of frames"]
#[doc = "            needed to reach 64% of the target scale factor."]
#[doc = "            Higher values make the dynamic resolution react faster."]
#[doc = ""]
#[doc = " @see View::DynamicResolutionOptions"]
#[doc = " @see Renderer::DisplayInfo"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_Renderer_FrameRateOptions {
    #[doc = "!< additional headroom for the GPU"]
    pub headRoomRatio: f32,
    #[doc = "!< rate at which the system reacts to load changes"]
    pub scaleRate: f32,
    #[doc = "!< history size"]
    pub history: u8,
    #[doc = "!< desired frame interval in unit of 1.0 / DisplayInfo::refreshRate"]
    pub interval: u8,
}
#[test]
fn bindgen_test_layout_filament_Renderer_FrameRateOptions() {
    assert_eq!(
        ::std::mem::size_of::<filament_Renderer_FrameRateOptions>(),
        12usize,
        concat!("Size of: ", stringify!(filament_Renderer_FrameRateOptions))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_Renderer_FrameRateOptions>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(filament_Renderer_FrameRateOptions)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_Renderer_FrameRateOptions>())).headRoomRatio as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_Renderer_FrameRateOptions),
            "::",
            stringify!(headRoomRatio)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_Renderer_FrameRateOptions>())).scaleRate as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_Renderer_FrameRateOptions),
            "::",
            stringify!(scaleRate)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_Renderer_FrameRateOptions>())).history as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_Renderer_FrameRateOptions),
            "::",
            stringify!(history)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_Renderer_FrameRateOptions>())).interval as *const _
                as usize
        },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_Renderer_FrameRateOptions),
            "::",
            stringify!(interval)
        )
    );
}
#[doc = " ClearOptions are used at the beginning of a frame to clear or retain the SwapChain content."]
#[repr(C)]
pub struct filament_Renderer_ClearOptions {
    #[doc = " Color to use to clear the SwapChain"]
    pub clearColor: filament_math_float4,
    #[doc = " Whether the SwapChain should be cleared using the clearColor. Use this if translucent"]
    #[doc = " View will be drawn, for instance."]
    pub clear: bool,
    #[doc = " Whether the SwapChain content should be discarded. clear implies discard. Set this"]
    #[doc = " to false (along with clear to false as well) if the SwapChain already has content that"]
    #[doc = " needs to be preserved"]
    pub discard: bool,
}
#[test]
fn bindgen_test_layout_filament_Renderer_ClearOptions() {
    assert_eq!(
        ::std::mem::size_of::<filament_Renderer_ClearOptions>(),
        20usize,
        concat!("Size of: ", stringify!(filament_Renderer_ClearOptions))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_Renderer_ClearOptions>(),
        4usize,
        concat!("Alignment of ", stringify!(filament_Renderer_ClearOptions))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_Renderer_ClearOptions>())).clearColor as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_Renderer_ClearOptions),
            "::",
            stringify!(clearColor)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_Renderer_ClearOptions>())).clear as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_Renderer_ClearOptions),
            "::",
            stringify!(clear)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_Renderer_ClearOptions>())).discard as *const _ as usize
        },
        17usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_Renderer_ClearOptions),
            "::",
            stringify!(discard)
        )
    );
}
impl Default for filament_Renderer_ClearOptions {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " Flags used to configure the behavior of copyFrame()."]
#[doc = ""]
#[doc = " @see"]
#[doc = " copyFrame()"]
pub type filament_Renderer_CopyFrameFlag = u32;
#[doc = " Indicates that the dstSwapChain passed into copyFrame() should be"]
#[doc = " committed after the frame has been copied."]
#[doc = ""]
#[doc = " @see"]
#[doc = " copyFrame()"]
pub const filament_Renderer_COMMIT: filament_Renderer_CopyFrameFlag = 1;
#[doc = " Indicates that the presentation time should be set on the dstSwapChain"]
#[doc = " passed into copyFrame to the monotonic clock time when the frame is"]
#[doc = " copied."]
#[doc = ""]
#[doc = " @see"]
#[doc = " copyFrame()"]
pub const filament_Renderer_SET_PRESENTATION_TIME: filament_Renderer_CopyFrameFlag = 2;
#[doc = " Indicates that the dstSwapChain passed into copyFrame() should be"]
#[doc = " cleared to black before the frame is copied into the specified viewport."]
#[doc = ""]
#[doc = " @see"]
#[doc = " copyFrame()"]
pub const filament_Renderer_CLEAR: filament_Renderer_CopyFrameFlag = 4;
#[test]
fn bindgen_test_layout_filament_Renderer() {
    assert_eq!(
        ::std::mem::size_of::<filament_Renderer>(),
        1usize,
        concat!("Size of: ", stringify!(filament_Renderer))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_Renderer>(),
        1usize,
        concat!("Alignment of ", stringify!(filament_Renderer))
    );
}
extern "C" {
    #[doc = " Information about the display this Renderer is associated to. This information is needed"]
    #[doc = " to accurately compute dynamic-resolution scaling and for frame-pacing."]
    #[link_name = "\u{1}_ZN8filament8Renderer14setDisplayInfoERKNS0_11DisplayInfoE"]
    pub fn filament_Renderer_setDisplayInfo(
        this: *mut filament_Renderer,
        info: *const filament_Renderer_DisplayInfo,
    );
}
extern "C" {
    #[doc = " Set options controlling the desired frame-rate."]
    #[link_name = "\u{1}_ZN8filament8Renderer19setFrameRateOptionsERKNS0_16FrameRateOptionsE"]
    pub fn filament_Renderer_setFrameRateOptions(
        this: *mut filament_Renderer,
        options: *const filament_Renderer_FrameRateOptions,
    );
}
extern "C" {
    #[doc = " Set ClearOptions which are used at the beginning of a frame to clear or retain the"]
    #[doc = " SwapChain content."]
    #[link_name = "\u{1}_ZN8filament8Renderer15setClearOptionsERKNS0_12ClearOptionsE"]
    pub fn filament_Renderer_setClearOptions(
        this: *mut filament_Renderer,
        options: *const filament_Renderer_ClearOptions,
    );
}
extern "C" {
    #[doc = " Get the Engine that created this Renderer."]
    #[doc = ""]
    #[doc = " @return A pointer to the Engine instance this Renderer is associated to."]
    #[link_name = "\u{1}_ZN8filament8Renderer9getEngineEv"]
    pub fn filament_Renderer_getEngine(this: *mut filament_Renderer) -> *mut filament_Engine;
}
extern "C" {
    #[doc = " Set-up a frame for this Renderer."]
    #[doc = ""]
    #[doc = " beginFrame() manages frame pacing, and returns whether or not a frame should be drawn. The"]
    #[doc = " goal of this is to skip frames when the GPU falls behind in order to keep the frame"]
    #[doc = " latency low."]
    #[doc = ""]
    #[doc = " If a given frame takes too much time in the GPU, the CPU will get ahead of the GPU. The"]
    #[doc = " display will draw the same frame twice producing a stutter. At this point, the CPU is"]
    #[doc = " ahead of the GPU and depending on how many frames are buffered, latency increases."]
    #[doc = ""]
    #[doc = " beginFrame() attempts to detect this situation and returns false in that case, indicating"]
    #[doc = " to the caller to skip the current frame."]
    #[doc = ""]
    #[doc = " When beginFrame() returns true, it is mandatory to render the frame and call endFrame()."]
    #[doc = " However, when beginFrame() returns false, the caller has the choice to either skip the"]
    #[doc = " frame and not call endFrame(), or proceed as though true was returned."]
    #[doc = ""]
    #[doc = " @param vsyncSteadyClockTimeNano The time in nanosecond of when the current frame started,"]
    #[doc = "                                 or 0 if unknown. This value should be the timestamp of"]
    #[doc = "                                 the last h/w vsync. It is expressed in the"]
    #[doc = "                                 std::chrono::steady_clock time base."]
    #[doc = " @param swapChain A pointer to the SwapChain instance to use."]
    #[doc = ""]
    #[doc = " @return"]
    #[doc = "      *false* the current frame should be skipped,"]
    #[doc = "      *true* the current frame must be drawn and endFrame() must be called."]
    #[doc = ""]
    #[doc = " @remark"]
    #[doc = " When skipping a frame, the whole frame is canceled, and endFrame() must not be called."]
    #[doc = ""]
    #[doc = " @note"]
    #[doc = " All calls to render() must happen *after* beginFrame()."]
    #[doc = ""]
    #[doc = " @see"]
    #[doc = " endFrame()"]
    #[link_name = "\u{1}_ZN8filament8Renderer10beginFrameEPNS_9SwapChainEm"]
    pub fn filament_Renderer_beginFrame(
        this: *mut filament_Renderer,
        swapChain: *mut filament_SwapChain,
        vsyncSteadyClockTimeNano: u64,
    ) -> bool;
}
extern "C" {
    #[doc = " Render a View into this renderer's window."]
    #[doc = ""]
    #[doc = " This is filament main rendering method, most of the CPU-side heavy lifting is performed"]
    #[doc = " here. render() main function is to generate render commands which are asynchronously"]
    #[doc = " executed by the Engine's render thread."]
    #[doc = ""]
    #[doc = " render() generates commands for each of the following stages:"]
    #[doc = ""]
    #[doc = " 1. Shadow map passes, if needed."]
    #[doc = " 2. Depth pre-pass."]
    #[doc = " 3. Color pass."]
    #[doc = " 4. Post-processing pass."]
    #[doc = ""]
    #[doc = " A typical render loop looks like this:"]
    #[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
    #[doc = " #include <filament/Renderer.h>"]
    #[doc = " #include <filament/View.h>"]
    #[doc = " using namespace filament;"]
    #[doc = ""]
    #[doc = " void renderLoop(Renderer* renderer, SwapChain* swapChain) {"]
    #[doc = "     do {"]
    #[doc = "         // typically we wait for VSYNC and user input events"]
    #[doc = "         if (renderer->beginFrame(swapChain)) {"]
    #[doc = "             renderer->render(mView);"]
    #[doc = "             renderer->endFrame();"]
    #[doc = "         }"]
    #[doc = "     } while (!quit());"]
    #[doc = " }"]
    #[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
    #[doc = ""]
    #[doc = ""]
    #[doc = " @param view A pointer to the view to render."]
    #[doc = ""]
    #[doc = " @attention"]
    #[doc = " render() must be called *after* beginFrame() and *before* endFrame()."]
    #[doc = ""]
    #[doc = " @note"]
    #[doc = " render() must be called from the Engine's main thread (or external synchronization"]
    #[doc = " must be provided). In particular, calls to render() on different Renderer instances"]
    #[doc = " **must** be synchronized."]
    #[doc = ""]
    #[doc = " @remark"]
    #[doc = " render() perform potentially heavy computations and cannot be multi-threaded. However,"]
    #[doc = " internally, render() is highly multi-threaded to both improve performance in mitigate"]
    #[doc = " the call's latency."]
    #[doc = ""]
    #[doc = " @remark"]
    #[doc = " render() is typically called once per frame (but not necessarily)."]
    #[doc = ""]
    #[doc = " @see"]
    #[doc = " beginFrame(), endFrame(), View"]
    #[doc = ""]
    #[link_name = "\u{1}_ZN8filament8Renderer6renderEPKNS_4ViewE"]
    pub fn filament_Renderer_render(this: *mut filament_Renderer, view: *const filament_View);
}
extern "C" {
    #[doc = " Copy the currently rendered view to the indicated swap chain, using the"]
    #[doc = " indicated source and destination rectangle."]
    #[doc = ""]
    #[doc = " @param dstSwapChain The swap chain into which the frame should be copied."]
    #[doc = " @param dstViewport The destination rectangle in which to draw the view."]
    #[doc = " @param srcViewport The source rectangle to be copied."]
    #[doc = " @param flags One or more CopyFrameFlag behavior configuration flags."]
    #[doc = ""]
    #[doc = " @remark"]
    #[doc = " copyFrame() should be called after a frame is rendered using render()"]
    #[doc = " but before endFrame() is called."]
    #[link_name = "\u{1}_ZN8filament8Renderer9copyFrameEPNS_9SwapChainERKNS_8ViewportES5_j"]
    pub fn filament_Renderer_copyFrame(
        this: *mut filament_Renderer,
        dstSwapChain: *mut filament_SwapChain,
        dstViewport: *const filament_Viewport,
        srcViewport: *const filament_Viewport,
        flags: u32,
    );
}
extern "C" {
    #[doc = " Reads back the content of the SwapChain associated with this Renderer."]
    #[doc = ""]
    #[doc = " @param xoffset   Left offset of the sub-region to read back."]
    #[doc = " @param yoffset   Bottom offset of the sub-region to read back."]
    #[doc = " @param width     Width of the sub-region to read back."]
    #[doc = " @param height    Height of the sub-region to read back."]
    #[doc = " @param buffer    Client-side buffer where the read-back will be written."]
    #[doc = ""]
    #[doc = " The following formats are always supported:"]
    #[doc = "                      - PixelBufferDescriptor::PixelDataFormat::RGBA"]
    #[doc = "                      - PixelBufferDescriptor::PixelDataFormat::RGBA_INTEGER"]
    #[doc = ""]
    #[doc = " The following types are always supported:"]
    #[doc = "                      - PixelBufferDescriptor::PixelDataType::UBYTE"]
    #[doc = "                      - PixelBufferDescriptor::PixelDataType::UINT"]
    #[doc = "                      - PixelBufferDescriptor::PixelDataType::INT"]
    #[doc = "                      - PixelBufferDescriptor::PixelDataType::FLOAT"]
    #[doc = ""]
    #[doc = " Other combinations of format/type may be supported. If a combination is"]
    #[doc = " not supported, this operation may fail silently. Use a DEBUG build"]
    #[doc = " to get some logs about the failure."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "  Framebuffer as seen on User buffer (PixelBufferDescriptor&)"]
    #[doc = "  screen"]
    #[doc = ""]
    #[doc = "      +--------------------+"]
    #[doc = "      |                    |                .stride         .alignment"]
    #[doc = "      |                    |         ----------------------->-->"]
    #[doc = "      |                    |         O----------------------+--+   low addresses"]
    #[doc = "      |                    |         |          |           |  |"]
    #[doc = "      |             w      |         |          | .top      |  |"]
    #[doc = "      |       <--------->  |         |          V           |  |"]
    #[doc = "      |       +---------+  |         |     +---------+      |  |"]
    #[doc = "      |       |     ^   |  | ======> |     |         |      |  |"]
    #[doc = "      |   x   |    h|   |  |         |.left|         |      |  |"]
    #[doc = "      +------>|     v   |  |         +---->|         |      |  |"]
    #[doc = "      |       +.........+  |         |     +.........+      |  |"]
    #[doc = "      |            ^       |         |                      |  |"]
    #[doc = "      |          y |       |         +----------------------+--+  high addresses"]
    #[doc = "      O------------+-------+"]
    #[doc = ""]
    #[doc = ""]
    #[doc = " readPixels() must be called within a frame, meaning after beginFrame() and before endFrame()."]
    #[doc = " Typically, readPixels() will be called after render()."]
    #[doc = ""]
    #[doc = " After issuing this method, the callback associated with `buffer` will be invoked on the"]
    #[doc = " main thread, indicating that the read-back has completed. Typically, this will happen"]
    #[doc = " after multiple calls to beginFrame(), render(), endFrame()."]
    #[doc = ""]
    #[doc = " It is also possible to use a Fence to wait for the read-back."]
    #[doc = ""]
    #[doc = " @remark"]
    #[doc = " readPixels() is intended for debugging and testing. It will impact performance significantly."]
    #[doc = ""]
    #[link_name = "\u{1}_ZN8filament8Renderer10readPixelsEjjjjONS_7backend21PixelBufferDescriptorE"]
    pub fn filament_Renderer_readPixels(
        this: *mut filament_Renderer,
        xoffset: u32,
        yoffset: u32,
        width: u32,
        height: u32,
        buffer: *mut filament_backend_PixelBufferDescriptor,
    );
}
extern "C" {
    #[doc = " Finishes the current frame and schedules it for display."]
    #[doc = ""]
    #[doc = " endFrame() schedules the current frame to be displayed on the Renderer's window."]
    #[doc = ""]
    #[doc = " @note"]
    #[doc = " All calls to render() must happen *before* endFrame(). endFrame() must be called if"]
    #[doc = " beginFrame() returned true, otherwise, endFrame() must not be called unless the caller"]
    #[doc = " ignored beginFrame()'s return value."]
    #[doc = ""]
    #[doc = " @see"]
    #[doc = " beginFrame()"]
    #[link_name = "\u{1}_ZN8filament8Renderer8endFrameEv"]
    pub fn filament_Renderer_endFrame(this: *mut filament_Renderer);
}
extern "C" {
    #[doc = " Reads back the content of the provided RenderTarget."]
    #[doc = ""]
    #[doc = " @param renderTarget  RenderTarget to read back from."]
    #[doc = " @param xoffset       Left offset of the sub-region to read back."]
    #[doc = " @param yoffset       Bottom offset of the sub-region to read back."]
    #[doc = " @param width         Width of the sub-region to read back."]
    #[doc = " @param height        Height of the sub-region to read back."]
    #[doc = " @param buffer        Client-side buffer where the read-back will be written."]
    #[doc = ""]
    #[doc = " The following formats are always supported:"]
    #[doc = "                      - PixelBufferDescriptor::PixelDataFormat::RGBA"]
    #[doc = "                      - PixelBufferDescriptor::PixelDataFormat::RGBA_INTEGER"]
    #[doc = ""]
    #[doc = " The following types are always supported:"]
    #[doc = "                      - PixelBufferDescriptor::PixelDataType::UBYTE"]
    #[doc = "                      - PixelBufferDescriptor::PixelDataType::UINT"]
    #[doc = "                      - PixelBufferDescriptor::PixelDataType::INT"]
    #[doc = "                      - PixelBufferDescriptor::PixelDataType::FLOAT"]
    #[doc = ""]
    #[doc = " Other combinations of format/type may be supported. If a combination is"]
    #[doc = " not supported, this operation may fail silently. Use a DEBUG build"]
    #[doc = " to get some logs about the failure."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "  Framebuffer as seen on User buffer (PixelBufferDescriptor&)"]
    #[doc = "  screen"]
    #[doc = ""]
    #[doc = "      +--------------------+"]
    #[doc = "      |                    |                .stride         .alignment"]
    #[doc = "      |                    |         ----------------------->-->"]
    #[doc = "      |                    |         O----------------------+--+   low addresses"]
    #[doc = "      |                    |         |          |           |  |"]
    #[doc = "      |             w      |         |          | .top      |  |"]
    #[doc = "      |       <--------->  |         |          V           |  |"]
    #[doc = "      |       +---------+  |         |     +---------+      |  |"]
    #[doc = "      |       |     ^   |  | ======> |     |         |      |  |"]
    #[doc = "      |   x   |    h|   |  |         |.left|         |      |  |"]
    #[doc = "      +------>|     v   |  |         +---->|         |      |  |"]
    #[doc = "      |       +.........+  |         |     +.........+      |  |"]
    #[doc = "      |            ^       |         |                      |  |"]
    #[doc = "      |          y |       |         +----------------------+--+  high addresses"]
    #[doc = "      O------------+-------+"]
    #[doc = ""]
    #[doc = ""]
    #[doc = " Typically readPixels() will be called after render() and before endFrame()."]
    #[doc = ""]
    #[doc = " After issuing this method, the callback associated with `buffer` will be invoked on the"]
    #[doc = " main thread, indicating that the read-back has completed. Typically, this will happen"]
    #[doc = " after multiple calls to beginFrame(), render(), endFrame()."]
    #[doc = ""]
    #[doc = " It is also possible to use a Fence to wait for the read-back."]
    #[doc = ""]
    #[doc = " @remark"]
    #[doc = " readPixels() is intended for debugging and testing. It will impact performance significantly."]
    #[doc = ""]
    #[link_name = "\u{1}_ZN8filament8Renderer10readPixelsEPNS_12RenderTargetEjjjjONS_7backend21PixelBufferDescriptorE"]
    pub fn filament_Renderer_readPixels1(
        this: *mut filament_Renderer,
        renderTarget: *mut filament_RenderTarget,
        xoffset: u32,
        yoffset: u32,
        width: u32,
        height: u32,
        buffer: *mut filament_backend_PixelBufferDescriptor,
    );
}
extern "C" {
    #[doc = " Render a standalone View into its associated RenderTarget"]
    #[doc = ""]
    #[doc = " This call is mostly equivalent to calling render(View*) inside a"]
    #[doc = " beginFrame / endFrame block, but incurs less overhead. It can be used"]
    #[doc = " as a poor man's compute API."]
    #[doc = ""]
    #[doc = " @param view A pointer to the view to render. This View must have a RenderTarget associated"]
    #[doc = "             to it."]
    #[doc = ""]
    #[doc = " @attention"]
    #[doc = " renderStandaloneView() must be called outside of beginFrame() / endFrame()."]
    #[doc = ""]
    #[doc = " @note"]
    #[doc = " renderStandaloneView() must be called from the Engine's main thread"]
    #[doc = " (or external synchronization must be provided). In particular, calls to"]
    #[doc = " renderStandaloneView() on different Renderer instances **must** be synchronized."]
    #[doc = ""]
    #[doc = " @remark"]
    #[doc = " renderStandaloneView() perform potentially heavy computations and cannot be multi-threaded."]
    #[doc = " However, internally, renderStandaloneView() is highly multi-threaded to both improve"]
    #[doc = " performance in mitigate the call's latency."]
    #[link_name = "\u{1}_ZN8filament8Renderer20renderStandaloneViewEPKNS_4ViewE"]
    pub fn filament_Renderer_renderStandaloneView(
        this: *mut filament_Renderer,
        view: *const filament_View,
    );
}
extern "C" {
    #[doc = " Returns the time in second of the last call to beginFrame(). This value is constant for all"]
    #[doc = " views rendered during a frame. The epoch is set with resetUserTime()."]
    #[doc = ""]
    #[doc = " In materials, this value can be queried using `vec4 getUserTime()`. The value returned"]
    #[doc = " is a highp vec4 encoded as follows:"]
    #[doc = ""]
    #[doc = "      time.x = (float)Renderer.getUserTime();"]
    #[doc = "      time.y = Renderer.getUserTime() - time.x;"]
    #[doc = ""]
    #[doc = " It follows that the following invariants are true:"]
    #[doc = ""]
    #[doc = "      (double)time.x + (double)time.y == Renderer.getUserTime()"]
    #[doc = "      time.x == (float)Renderer.getUserTime()"]
    #[doc = ""]
    #[doc = " This encoding allows the shader code to perform high precision (i.e. double) time"]
    #[doc = " calculations when needed despite the lack of double precision in the shader, for e.g.:"]
    #[doc = ""]
    #[doc = "      To compute (double)time * vertex in the material, use the following construct:"]
    #[doc = ""]
    #[doc = "              vec3 result = time.x * vertex + time.y * vertex;"]
    #[doc = ""]
    #[doc = ""]
    #[doc = " Most of the time, high precision computations are not required, but be aware that the"]
    #[doc = " precision of time.x rapidly diminishes as time passes:"]
    #[doc = ""]
    #[doc = "          time    | precision"]
    #[doc = "          --------+----------"]
    #[doc = "          16.7s   |    us"]
    #[doc = "          4h39    |    ms"]
    #[doc = "         77h      |   1/60s"]
    #[doc = ""]
    #[doc = ""]
    #[doc = " In other words, it only possible to get microsecond accuracy for about 16s or millisecond"]
    #[doc = " accuracy for just under 5h."]
    #[doc = ""]
    #[doc = " This problem can be mitigated by calling resetUserTime(), or using high precision time as"]
    #[doc = " described above."]
    #[doc = ""]
    #[doc = " @return The time is seconds since resetUserTime() was last called."]
    #[doc = ""]
    #[doc = " @see"]
    #[doc = " resetUserTime()"]
    #[link_name = "\u{1}_ZNK8filament8Renderer11getUserTimeEv"]
    pub fn filament_Renderer_getUserTime(this: *const filament_Renderer) -> f64;
}
extern "C" {
    #[doc = " Sets the user time epoch to now, i.e. resets the user time to zero."]
    #[doc = ""]
    #[doc = " Use this method used to keep the precision of time high in materials, in practice it should"]
    #[doc = " be called at least when the application is paused, e.g. Activity.onPause() in Android."]
    #[doc = ""]
    #[doc = " @see"]
    #[doc = " getUserTime()"]
    #[link_name = "\u{1}_ZN8filament8Renderer13resetUserTimeEv"]
    pub fn filament_Renderer_resetUserTime(this: *mut filament_Renderer);
}
impl filament_Renderer {
    #[inline]
    pub unsafe fn setDisplayInfo(&mut self, info: *const filament_Renderer_DisplayInfo) {
        filament_Renderer_setDisplayInfo(self, info)
    }
    #[inline]
    pub unsafe fn setFrameRateOptions(
        &mut self,
        options: *const filament_Renderer_FrameRateOptions,
    ) {
        filament_Renderer_setFrameRateOptions(self, options)
    }
    #[inline]
    pub unsafe fn setClearOptions(&mut self, options: *const filament_Renderer_ClearOptions) {
        filament_Renderer_setClearOptions(self, options)
    }
    #[inline]
    pub unsafe fn getEngine(&mut self) -> *mut filament_Engine {
        filament_Renderer_getEngine(self)
    }
    #[inline]
    pub unsafe fn beginFrame(
        &mut self,
        swapChain: *mut filament_SwapChain,
        vsyncSteadyClockTimeNano: u64,
    ) -> bool {
        filament_Renderer_beginFrame(self, swapChain, vsyncSteadyClockTimeNano)
    }
    #[inline]
    pub unsafe fn render(&mut self, view: *const filament_View) {
        filament_Renderer_render(self, view)
    }
    #[inline]
    pub unsafe fn copyFrame(
        &mut self,
        dstSwapChain: *mut filament_SwapChain,
        dstViewport: *const filament_Viewport,
        srcViewport: *const filament_Viewport,
        flags: u32,
    ) {
        filament_Renderer_copyFrame(self, dstSwapChain, dstViewport, srcViewport, flags)
    }
    #[inline]
    pub unsafe fn readPixels(
        &mut self,
        xoffset: u32,
        yoffset: u32,
        width: u32,
        height: u32,
        buffer: *mut filament_backend_PixelBufferDescriptor,
    ) {
        filament_Renderer_readPixels(self, xoffset, yoffset, width, height, buffer)
    }
    #[inline]
    pub unsafe fn endFrame(&mut self) {
        filament_Renderer_endFrame(self)
    }
    #[inline]
    pub unsafe fn readPixels1(
        &mut self,
        renderTarget: *mut filament_RenderTarget,
        xoffset: u32,
        yoffset: u32,
        width: u32,
        height: u32,
        buffer: *mut filament_backend_PixelBufferDescriptor,
    ) {
        filament_Renderer_readPixels1(self, renderTarget, xoffset, yoffset, width, height, buffer)
    }
    #[inline]
    pub unsafe fn renderStandaloneView(&mut self, view: *const filament_View) {
        filament_Renderer_renderStandaloneView(self, view)
    }
    #[inline]
    pub unsafe fn getUserTime(&self) -> f64 {
        filament_Renderer_getUserTime(self)
    }
    #[inline]
    pub unsafe fn resetUserTime(&mut self) {
        filament_Renderer_resetUserTime(self)
    }
}
#[doc = " An offscreen render target that can be associated with a View and contains"]
#[doc = " weak references to a set of attached Texture objects."]
#[doc = ""]
#[doc = " RenderTarget is intended to be used with the View's post-processing disabled for the most part."]
#[doc = " especially when a DEPTH attachment is also used (see Builder::texture())."]
#[doc = ""]
#[doc = " Custom RenderTarget are ultimately intended to render into textures that might be used during"]
#[doc = " the main render pass."]
#[doc = ""]
#[doc = " Clients are responsible for the lifetime of all associated Texture attachments."]
#[doc = ""]
#[doc = " @see View"]
#[repr(C)]
#[derive(Debug, Default)]
pub struct filament_RenderTarget {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct filament_RenderTarget_BuilderDetails {
    _unused: [u8; 0],
}
#[doc = "! Texture Cubemap Face"]
pub use self::filament_backend_TextureCubemapFace as filament_RenderTarget_CubemapFace;
#[doc = "!< identifies the 1st color attachment"]
pub const filament_RenderTarget_AttachmentPoint_COLOR0: filament_RenderTarget_AttachmentPoint = 0;
#[doc = "!< identifies the 2nd color attachment"]
pub const filament_RenderTarget_AttachmentPoint_COLOR1: filament_RenderTarget_AttachmentPoint = 1;
#[doc = "!< identifies the 3rd color attachment"]
pub const filament_RenderTarget_AttachmentPoint_COLOR2: filament_RenderTarget_AttachmentPoint = 2;
#[doc = "!< identifies the 4th color attachment"]
pub const filament_RenderTarget_AttachmentPoint_COLOR3: filament_RenderTarget_AttachmentPoint = 3;
#[doc = "!< identifies the 5th color attachment"]
pub const filament_RenderTarget_AttachmentPoint_COLOR4: filament_RenderTarget_AttachmentPoint = 4;
#[doc = "!< identifies the 6th color attachment"]
pub const filament_RenderTarget_AttachmentPoint_COLOR5: filament_RenderTarget_AttachmentPoint = 5;
#[doc = "!< identifies the 7th color attachment"]
pub const filament_RenderTarget_AttachmentPoint_COLOR6: filament_RenderTarget_AttachmentPoint = 6;
#[doc = "!< identifies the 8th color attachment"]
pub const filament_RenderTarget_AttachmentPoint_COLOR7: filament_RenderTarget_AttachmentPoint = 7;
#[doc = "!< identifies the depth attachment"]
pub const filament_RenderTarget_AttachmentPoint_DEPTH: filament_RenderTarget_AttachmentPoint = 8;
#[doc = "!< identifies the 1st color attachment"]
pub const filament_RenderTarget_AttachmentPoint_COLOR: filament_RenderTarget_AttachmentPoint = 0;
#[doc = " Attachment identifiers"]
pub type filament_RenderTarget_AttachmentPoint = u8;
#[doc = "! Use Builder to construct a RenderTarget object instance"]
#[repr(C)]
#[derive(Debug)]
pub struct filament_RenderTarget_Builder {
    pub _base: filament_BuilderBase<filament_RenderTarget_BuilderDetails>,
}
#[test]
fn bindgen_test_layout_filament_RenderTarget_Builder() {
    assert_eq!(
        ::std::mem::size_of::<filament_RenderTarget_Builder>(),
        8usize,
        concat!("Size of: ", stringify!(filament_RenderTarget_Builder))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_RenderTarget_Builder>(),
        8usize,
        concat!("Alignment of ", stringify!(filament_RenderTarget_Builder))
    );
}
extern "C" {
    #[doc = " Sets a texture to a given attachment point."]
    #[doc = ""]
    #[doc = " All RenderTargets must have a non-null COLOR attachment."]
    #[doc = ""]
    #[doc = " When using a DEPTH attachment, it is important to always disable post-processing"]
    #[doc = " in the View. Failing to do so will cause the DEPTH attachment to be ignored in most"]
    #[doc = " cases."]
    #[doc = ""]
    #[doc = " When the intention is to keep the content of the DEPTH attachment after rendering,"]
    #[doc = " Usage::SAMPLEABLE must be set on the DEPTH attachment, otherwise the content of the"]
    #[doc = " DEPTH buffer may be discarded."]
    #[doc = ""]
    #[doc = " @param attachment The attachment point of the texture."]
    #[doc = " @param texture The associated texture object."]
    #[doc = " @return A reference to this Builder for chaining calls."]
    #[link_name = "\u{1}_ZN8filament12RenderTarget7Builder7textureENS0_15AttachmentPointEPNS_7TextureE"]
    pub fn filament_RenderTarget_Builder_texture(
        this: *mut filament_RenderTarget_Builder,
        attachment: filament_RenderTarget_AttachmentPoint,
        texture: *mut filament_Texture,
    ) -> *mut filament_RenderTarget_Builder;
}
extern "C" {
    #[doc = " Sets the mipmap level for a given attachment point."]
    #[doc = ""]
    #[doc = " @param attachment The attachment point of the texture."]
    #[doc = " @param level The associated mipmap level, 0 by default."]
    #[doc = " @return A reference to this Builder for chaining calls."]
    #[link_name = "\u{1}_ZN8filament12RenderTarget7Builder8mipLevelENS0_15AttachmentPointEh"]
    pub fn filament_RenderTarget_Builder_mipLevel(
        this: *mut filament_RenderTarget_Builder,
        attachment: filament_RenderTarget_AttachmentPoint,
        level: u8,
    ) -> *mut filament_RenderTarget_Builder;
}
extern "C" {
    #[doc = " Sets the cubemap face for a given attachment point."]
    #[doc = ""]
    #[doc = " @param attachment The attachment point."]
    #[doc = " @param face The associated cubemap face."]
    #[doc = " @return A reference to this Builder for chaining calls."]
    #[link_name = "\u{1}_ZN8filament12RenderTarget7Builder4faceENS0_15AttachmentPointENS_7backend18TextureCubemapFaceE"]
    pub fn filament_RenderTarget_Builder_face(
        this: *mut filament_RenderTarget_Builder,
        attachment: filament_RenderTarget_AttachmentPoint,
        face: filament_RenderTarget_CubemapFace,
    ) -> *mut filament_RenderTarget_Builder;
}
extern "C" {
    #[doc = " Sets the layer for a given attachment point (for 3D textures)."]
    #[doc = ""]
    #[doc = " @param attachment The attachment point."]
    #[doc = " @param layer The associated cubemap layer."]
    #[doc = " @return A reference to this Builder for chaining calls."]
    #[link_name = "\u{1}_ZN8filament12RenderTarget7Builder5layerENS0_15AttachmentPointEj"]
    pub fn filament_RenderTarget_Builder_layer(
        this: *mut filament_RenderTarget_Builder,
        attachment: filament_RenderTarget_AttachmentPoint,
        layer: u32,
    ) -> *mut filament_RenderTarget_Builder;
}
extern "C" {
    #[doc = " Creates the RenderTarget object and returns a pointer to it."]
    #[doc = ""]
    #[doc = " @return pointer to the newly created object or nullptr if exceptions are disabled and"]
    #[doc = "         an error occurred."]
    #[link_name = "\u{1}_ZN8filament12RenderTarget7Builder5buildERNS_6EngineE"]
    pub fn filament_RenderTarget_Builder_build(
        this: *mut filament_RenderTarget_Builder,
        engine: *mut filament_Engine,
    ) -> *mut filament_RenderTarget;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament12RenderTarget7BuilderC1Ev"]
    pub fn filament_RenderTarget_Builder_Builder(this: *mut filament_RenderTarget_Builder);
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament12RenderTarget7BuilderC1ERKS1_"]
    pub fn filament_RenderTarget_Builder_Builder1(
        this: *mut filament_RenderTarget_Builder,
        rhs: *const filament_RenderTarget_Builder,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament12RenderTarget7BuilderC1EOS1_"]
    pub fn filament_RenderTarget_Builder_Builder2(
        this: *mut filament_RenderTarget_Builder,
        rhs: *mut filament_RenderTarget_Builder,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament12RenderTarget7BuilderD1Ev"]
    pub fn filament_RenderTarget_Builder_Builder_destructor(
        this: *mut filament_RenderTarget_Builder,
    );
}
impl Default for filament_RenderTarget_Builder {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl filament_RenderTarget_Builder {
    #[inline]
    pub unsafe fn texture(
        &mut self,
        attachment: filament_RenderTarget_AttachmentPoint,
        texture: *mut filament_Texture,
    ) -> *mut filament_RenderTarget_Builder {
        filament_RenderTarget_Builder_texture(self, attachment, texture)
    }
    #[inline]
    pub unsafe fn mipLevel(
        &mut self,
        attachment: filament_RenderTarget_AttachmentPoint,
        level: u8,
    ) -> *mut filament_RenderTarget_Builder {
        filament_RenderTarget_Builder_mipLevel(self, attachment, level)
    }
    #[inline]
    pub unsafe fn face(
        &mut self,
        attachment: filament_RenderTarget_AttachmentPoint,
        face: filament_RenderTarget_CubemapFace,
    ) -> *mut filament_RenderTarget_Builder {
        filament_RenderTarget_Builder_face(self, attachment, face)
    }
    #[inline]
    pub unsafe fn layer(
        &mut self,
        attachment: filament_RenderTarget_AttachmentPoint,
        layer: u32,
    ) -> *mut filament_RenderTarget_Builder {
        filament_RenderTarget_Builder_layer(self, attachment, layer)
    }
    #[inline]
    pub unsafe fn build(&mut self, engine: *mut filament_Engine) -> *mut filament_RenderTarget {
        filament_RenderTarget_Builder_build(self, engine)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_RenderTarget_Builder_Builder(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(rhs: *const filament_RenderTarget_Builder) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_RenderTarget_Builder_Builder1(__bindgen_tmp.as_mut_ptr(), rhs);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new2(rhs: *mut filament_RenderTarget_Builder) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_RenderTarget_Builder_Builder2(__bindgen_tmp.as_mut_ptr(), rhs);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        filament_RenderTarget_Builder_Builder_destructor(self)
    }
}
#[doc = " Minimum number of color attachment supported"]
pub const filament_RenderTarget_MIN_SUPPORTED_COLOR_ATTACHMENTS_COUNT: u8 = 4;
#[doc = " Maximum number of color attachment supported"]
pub const filament_RenderTarget_MAX_SUPPORTED_COLOR_ATTACHMENTS_COUNT: u8 = 8;
#[test]
fn bindgen_test_layout_filament_RenderTarget() {
    assert_eq!(
        ::std::mem::size_of::<filament_RenderTarget>(),
        1usize,
        concat!("Size of: ", stringify!(filament_RenderTarget))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_RenderTarget>(),
        1usize,
        concat!("Alignment of ", stringify!(filament_RenderTarget))
    );
}
extern "C" {
    #[doc = " Gets the texture set on the given attachment point"]
    #[doc = " @param attachment Attachment point"]
    #[doc = " @return A Texture object or nullptr if no texture is set for this attachment point"]
    #[link_name = "\u{1}_ZNK8filament12RenderTarget10getTextureENS0_15AttachmentPointE"]
    pub fn filament_RenderTarget_getTexture(
        this: *const filament_RenderTarget,
        attachment: filament_RenderTarget_AttachmentPoint,
    ) -> *mut filament_Texture;
}
extern "C" {
    #[doc = " Returns the mipmap level set on the given attachment point"]
    #[doc = " @param attachment Attachment point"]
    #[doc = " @return the mipmap level set on the given attachment point"]
    #[link_name = "\u{1}_ZNK8filament12RenderTarget11getMipLevelENS0_15AttachmentPointE"]
    pub fn filament_RenderTarget_getMipLevel(
        this: *const filament_RenderTarget,
        attachment: filament_RenderTarget_AttachmentPoint,
    ) -> u8;
}
extern "C" {
    #[doc = " Returns the face of a cubemap set on the given attachment point"]
    #[doc = " @param attachment Attachment point"]
    #[doc = " @return A cubemap face identifier. This is only relevant if the attachment's texture is"]
    #[doc = " a cubemap."]
    #[link_name = "\u{1}_ZNK8filament12RenderTarget7getFaceENS0_15AttachmentPointE"]
    pub fn filament_RenderTarget_getFace(
        this: *const filament_RenderTarget,
        attachment: filament_RenderTarget_AttachmentPoint,
    ) -> filament_RenderTarget_CubemapFace;
}
extern "C" {
    #[doc = " Returns the texture-layer set on the given attachment point"]
    #[doc = " @param attachment Attachment point"]
    #[doc = " @return A texture layer. This is only relevant if the attachment's texture is a 3D texture."]
    #[link_name = "\u{1}_ZNK8filament12RenderTarget8getLayerENS0_15AttachmentPointE"]
    pub fn filament_RenderTarget_getLayer(
        this: *const filament_RenderTarget,
        attachment: filament_RenderTarget_AttachmentPoint,
    ) -> u32;
}
extern "C" {
    #[doc = " Returns the number of color attachments usable by this instance of Engine. This method is"]
    #[doc = " guaranteed to return at least MIN_SUPPORTED_COLOR_ATTACHMENTS_COUNT and at most"]
    #[doc = " MAX_SUPPORTED_COLOR_ATTACHMENTS_COUNT."]
    #[doc = " @return Number of color attachments usable in a render target."]
    #[link_name = "\u{1}_ZNK8filament12RenderTarget33getSupportedColorAttachmentsCountEv"]
    pub fn filament_RenderTarget_getSupportedColorAttachmentsCount(
        this: *const filament_RenderTarget,
    ) -> u8;
}
impl filament_RenderTarget {
    #[inline]
    pub unsafe fn getTexture(
        &self,
        attachment: filament_RenderTarget_AttachmentPoint,
    ) -> *mut filament_Texture {
        filament_RenderTarget_getTexture(self, attachment)
    }
    #[inline]
    pub unsafe fn getMipLevel(&self, attachment: filament_RenderTarget_AttachmentPoint) -> u8 {
        filament_RenderTarget_getMipLevel(self, attachment)
    }
    #[inline]
    pub unsafe fn getFace(
        &self,
        attachment: filament_RenderTarget_AttachmentPoint,
    ) -> filament_RenderTarget_CubemapFace {
        filament_RenderTarget_getFace(self, attachment)
    }
    #[inline]
    pub unsafe fn getLayer(&self, attachment: filament_RenderTarget_AttachmentPoint) -> u32 {
        filament_RenderTarget_getLayer(self, attachment)
    }
    #[inline]
    pub unsafe fn getSupportedColorAttachmentsCount(&self) -> u8 {
        filament_RenderTarget_getSupportedColorAttachmentsCount(self)
    }
}
#[doc = " A Scene is a flat container of Renderable and Light instances."]
#[doc = ""]
#[doc = " A Scene doesn't provide a hierarchy of Renderable objects, i.e.: it's not a scene-graph."]
#[doc = " However, it manages the list of objects to render and the list of lights. Renderable"]
#[doc = " and Light objects can be added or removed from a Scene at any time."]
#[doc = ""]
#[doc = " A Renderable *must* be added to a Scene in order to be rendered, and the Scene must be"]
#[doc = " provided to a View."]
#[doc = ""]
#[doc = ""]
#[doc = " Creation and Destruction"]
#[doc = " ========================"]
#[doc = ""]
#[doc = " A Scene is created using Engine.createScene() and destroyed using"]
#[doc = " Engine.destroy(const Scene*)."]
#[doc = ""]
#[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
#[doc = " #include <filament/Scene.h>"]
#[doc = " #include <filament/Engine.h>"]
#[doc = " using namespace filament;"]
#[doc = ""]
#[doc = " Engine* engine = Engine::create();"]
#[doc = ""]
#[doc = " Scene* scene = engine->createScene();"]
#[doc = " engine->destroy(&scene);"]
#[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
#[doc = ""]
#[doc = " @see View, Renderable, Light"]
#[repr(C)]
#[derive(Debug, Default)]
pub struct filament_Scene {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_filament_Scene() {
    assert_eq!(
        ::std::mem::size_of::<filament_Scene>(),
        1usize,
        concat!("Size of: ", stringify!(filament_Scene))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_Scene>(),
        1usize,
        concat!("Alignment of ", stringify!(filament_Scene))
    );
}
extern "C" {
    #[doc = " Sets the Skybox."]
    #[doc = ""]
    #[doc = " The Skybox is drawn last and covers all pixels not touched by geometry."]
    #[doc = ""]
    #[doc = " @param skybox The Skybox to use to fill untouched pixels, or nullptr to unset the Skybox."]
    #[link_name = "\u{1}_ZN8filament5Scene9setSkyboxEPNS_6SkyboxE"]
    pub fn filament_Scene_setSkybox(this: *mut filament_Scene, skybox: *mut filament_Skybox);
}
extern "C" {
    #[doc = " Returns the Skybox associated with the Scene."]
    #[doc = ""]
    #[doc = " @return The associated Skybox, or nullptr if there is none."]
    #[link_name = "\u{1}_ZN8filament5Scene9getSkyboxEv"]
    pub fn filament_Scene_getSkybox(this: *mut filament_Scene) -> *mut filament_Skybox;
}
extern "C" {
    #[doc = " Returns an immutable Skybox associated with the Scene."]
    #[doc = ""]
    #[doc = " @return The associated Skybox, or nullptr if there is none."]
    #[link_name = "\u{1}_ZNK8filament5Scene9getSkyboxEv"]
    pub fn filament_Scene_getSkybox1(this: *const filament_Scene) -> *const filament_Skybox;
}
extern "C" {
    #[doc = " Set the IndirectLight to use when rendering the Scene."]
    #[doc = ""]
    #[doc = " Currently, a Scene may only have a single IndirectLight. This call replaces the current"]
    #[doc = " IndirectLight."]
    #[doc = ""]
    #[doc = " @param ibl The IndirectLight to use when rendering the Scene or nullptr to unset."]
    #[link_name = "\u{1}_ZN8filament5Scene16setIndirectLightEPKNS_13IndirectLightE"]
    pub fn filament_Scene_setIndirectLight(
        this: *mut filament_Scene,
        ibl: *const filament_IndirectLight,
    );
}
extern "C" {
    #[doc = " Adds an Entity to the Scene."]
    #[doc = ""]
    #[doc = " @param entity The entity is ignored if it doesn't have a Renderable or Light component."]
    #[doc = ""]
    #[doc = " \\attention"]
    #[doc = "  A given Entity object can only be added once to a Scene."]
    #[doc = ""]
    #[link_name = "\u{1}_ZN8filament5Scene9addEntityEN5utils6EntityE"]
    pub fn filament_Scene_addEntity(this: *mut filament_Scene, entity: utils_Entity);
}
extern "C" {
    #[doc = " Adds a list of entities to the Scene."]
    #[doc = ""]
    #[doc = " @param entities Array containing entities to add to the scene."]
    #[doc = " @param count Size of the entity array."]
    #[link_name = "\u{1}_ZN8filament5Scene11addEntitiesEPKN5utils6EntityEm"]
    pub fn filament_Scene_addEntities(
        this: *mut filament_Scene,
        entities: *const utils_Entity,
        count: size_t,
    );
}
extern "C" {
    #[doc = " Removes the Renderable from the Scene."]
    #[doc = ""]
    #[doc = " @param entity The Entity to remove from the Scene. If the specified"]
    #[doc = "                   \\p entity doesn't exist, this call is ignored."]
    #[link_name = "\u{1}_ZN8filament5Scene6removeEN5utils6EntityE"]
    pub fn filament_Scene_remove(this: *mut filament_Scene, entity: utils_Entity);
}
extern "C" {
    #[doc = " Removes a list of entities to the Scene."]
    #[doc = ""]
    #[doc = " This is equivalent to calling remove in a loop."]
    #[doc = " If any of the specified entities do not exist in the scene, they are skipped."]
    #[doc = ""]
    #[doc = " @param entities Array containing entities to remove from the scene."]
    #[doc = " @param count Size of the entity array."]
    #[link_name = "\u{1}_ZN8filament5Scene14removeEntitiesEPKN5utils6EntityEm"]
    pub fn filament_Scene_removeEntities(
        this: *mut filament_Scene,
        entities: *const utils_Entity,
        count: size_t,
    );
}
extern "C" {
    #[doc = " Returns the number of Renderable objects in the Scene."]
    #[doc = ""]
    #[doc = " @return number of Renderable objects in the Scene."]
    #[link_name = "\u{1}_ZNK8filament5Scene18getRenderableCountEv"]
    pub fn filament_Scene_getRenderableCount(this: *const filament_Scene) -> size_t;
}
extern "C" {
    #[doc = " Returns the total number of Light objects in the Scene."]
    #[doc = ""]
    #[doc = " @return The total number of Light objects in the Scene."]
    #[link_name = "\u{1}_ZNK8filament5Scene13getLightCountEv"]
    pub fn filament_Scene_getLightCount(this: *const filament_Scene) -> size_t;
}
extern "C" {
    #[doc = " Returns true if the given entity is in the Scene."]
    #[doc = ""]
    #[doc = " @return Whether the given entity is in the Scene."]
    #[link_name = "\u{1}_ZNK8filament5Scene9hasEntityEN5utils6EntityE"]
    pub fn filament_Scene_hasEntity(this: *const filament_Scene, entity: utils_Entity) -> bool;
}
impl filament_Scene {
    #[inline]
    pub unsafe fn setSkybox(&mut self, skybox: *mut filament_Skybox) {
        filament_Scene_setSkybox(self, skybox)
    }
    #[inline]
    pub unsafe fn getSkybox(&mut self) -> *mut filament_Skybox {
        filament_Scene_getSkybox(self)
    }
    #[inline]
    pub unsafe fn getSkybox1(&self) -> *const filament_Skybox {
        filament_Scene_getSkybox1(self)
    }
    #[inline]
    pub unsafe fn setIndirectLight(&mut self, ibl: *const filament_IndirectLight) {
        filament_Scene_setIndirectLight(self, ibl)
    }
    #[inline]
    pub unsafe fn addEntity(&mut self, entity: utils_Entity) {
        filament_Scene_addEntity(self, entity)
    }
    #[inline]
    pub unsafe fn addEntities(&mut self, entities: *const utils_Entity, count: size_t) {
        filament_Scene_addEntities(self, entities, count)
    }
    #[inline]
    pub unsafe fn remove(&mut self, entity: utils_Entity) {
        filament_Scene_remove(self, entity)
    }
    #[inline]
    pub unsafe fn removeEntities(&mut self, entities: *const utils_Entity, count: size_t) {
        filament_Scene_removeEntities(self, entities, count)
    }
    #[inline]
    pub unsafe fn getRenderableCount(&self) -> size_t {
        filament_Scene_getRenderableCount(self)
    }
    #[inline]
    pub unsafe fn getLightCount(&self) -> size_t {
        filament_Scene_getLightCount(self)
    }
    #[inline]
    pub unsafe fn hasEntity(&self, entity: utils_Entity) -> bool {
        filament_Scene_hasEntity(self, entity)
    }
}
#[doc = " SkinningBuffer is used to hold skinning data (bones). It is a simple wraper around"]
#[doc = " a structured UBO."]
#[repr(C)]
#[derive(Debug, Default)]
pub struct filament_SkinningBuffer {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct filament_SkinningBuffer_BuilderDetails {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug)]
pub struct filament_SkinningBuffer_Builder {
    pub _base: filament_BuilderBase<filament_SkinningBuffer_BuilderDetails>,
}
#[test]
fn bindgen_test_layout_filament_SkinningBuffer_Builder() {
    assert_eq!(
        ::std::mem::size_of::<filament_SkinningBuffer_Builder>(),
        8usize,
        concat!("Size of: ", stringify!(filament_SkinningBuffer_Builder))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_SkinningBuffer_Builder>(),
        8usize,
        concat!("Alignment of ", stringify!(filament_SkinningBuffer_Builder))
    );
}
extern "C" {
    #[doc = " Size of the skinning buffer in bones."]
    #[doc = ""]
    #[doc = " Due to limitation in the GLSL, the SkinningBuffer must always by a multiple of"]
    #[doc = " 256, this adjustment is done automatically, but can cause"]
    #[doc = " some memory overhead. This memory overhead can be mitigated by using the same"]
    #[doc = " SkinningBuffer to store the bone information for multiple RenderPrimitives."]
    #[doc = ""]
    #[doc = " @param boneCount Number of bones the skinning buffer can hold."]
    #[doc = " @return A reference to this Builder for chaining calls."]
    #[link_name = "\u{1}_ZN8filament14SkinningBuffer7Builder9boneCountEj"]
    pub fn filament_SkinningBuffer_Builder_boneCount(
        this: *mut filament_SkinningBuffer_Builder,
        boneCount: u32,
    ) -> *mut filament_SkinningBuffer_Builder;
}
extern "C" {
    #[doc = " The new buffer is created with identity bones"]
    #[doc = " @param initialize true to initializing the buffer, false to not."]
    #[doc = " @return A reference to this Builder for chaining calls."]
    #[link_name = "\u{1}_ZN8filament14SkinningBuffer7Builder10initializeEb"]
    pub fn filament_SkinningBuffer_Builder_initialize(
        this: *mut filament_SkinningBuffer_Builder,
        initialize: bool,
    ) -> *mut filament_SkinningBuffer_Builder;
}
extern "C" {
    #[doc = " Creates the SkinningBuffer object and returns a pointer to it."]
    #[doc = ""]
    #[doc = " @param engine Reference to the filament::Engine to associate this SkinningBuffer with."]
    #[doc = ""]
    #[doc = " @return pointer to the newly created object or nullptr if exceptions are disabled and"]
    #[doc = "         an error occurred."]
    #[doc = ""]
    #[doc = " @exception utils::PostConditionPanic if a runtime error occurred, such as running out of"]
    #[doc = "            memory or other resources."]
    #[doc = " @exception utils::PreConditionPanic if a parameter to a builder function was invalid."]
    #[doc = ""]
    #[doc = " @see SkinningBuffer::setBones"]
    #[link_name = "\u{1}_ZN8filament14SkinningBuffer7Builder5buildERNS_6EngineE"]
    pub fn filament_SkinningBuffer_Builder_build(
        this: *mut filament_SkinningBuffer_Builder,
        engine: *mut filament_Engine,
    ) -> *mut filament_SkinningBuffer;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament14SkinningBuffer7BuilderC1Ev"]
    pub fn filament_SkinningBuffer_Builder_Builder(this: *mut filament_SkinningBuffer_Builder);
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament14SkinningBuffer7BuilderC1ERKS1_"]
    pub fn filament_SkinningBuffer_Builder_Builder1(
        this: *mut filament_SkinningBuffer_Builder,
        rhs: *const filament_SkinningBuffer_Builder,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament14SkinningBuffer7BuilderC1EOS1_"]
    pub fn filament_SkinningBuffer_Builder_Builder2(
        this: *mut filament_SkinningBuffer_Builder,
        rhs: *mut filament_SkinningBuffer_Builder,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament14SkinningBuffer7BuilderD1Ev"]
    pub fn filament_SkinningBuffer_Builder_Builder_destructor(
        this: *mut filament_SkinningBuffer_Builder,
    );
}
impl Default for filament_SkinningBuffer_Builder {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl filament_SkinningBuffer_Builder {
    #[inline]
    pub unsafe fn boneCount(&mut self, boneCount: u32) -> *mut filament_SkinningBuffer_Builder {
        filament_SkinningBuffer_Builder_boneCount(self, boneCount)
    }
    #[inline]
    pub unsafe fn initialize(&mut self, initialize: bool) -> *mut filament_SkinningBuffer_Builder {
        filament_SkinningBuffer_Builder_initialize(self, initialize)
    }
    #[inline]
    pub unsafe fn build(&mut self, engine: *mut filament_Engine) -> *mut filament_SkinningBuffer {
        filament_SkinningBuffer_Builder_build(self, engine)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_SkinningBuffer_Builder_Builder(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(rhs: *const filament_SkinningBuffer_Builder) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_SkinningBuffer_Builder_Builder1(__bindgen_tmp.as_mut_ptr(), rhs);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new2(rhs: *mut filament_SkinningBuffer_Builder) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_SkinningBuffer_Builder_Builder2(__bindgen_tmp.as_mut_ptr(), rhs);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        filament_SkinningBuffer_Builder_Builder_destructor(self)
    }
}
#[test]
fn bindgen_test_layout_filament_SkinningBuffer() {
    assert_eq!(
        ::std::mem::size_of::<filament_SkinningBuffer>(),
        1usize,
        concat!("Size of: ", stringify!(filament_SkinningBuffer))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_SkinningBuffer>(),
        1usize,
        concat!("Alignment of ", stringify!(filament_SkinningBuffer))
    );
}
extern "C" {
    #[doc = " Updates the bone transforms in the range [offset, offset + count)."]
    #[doc = " @param engine Reference to the filament::Engine to associate this SkinningBuffer with."]
    #[doc = " @param transforms pointer to at least count Bone"]
    #[doc = " @param count number of Bone elements in transforms"]
    #[doc = " @param offset offset in elements (not bytes) in the SkinningBuffer (not in transforms)"]
    #[link_name = "\u{1}_ZN8filament14SkinningBuffer8setBonesERNS_6EngineEPKNS_17RenderableManager4BoneEmm"]
    pub fn filament_SkinningBuffer_setBones(
        this: *mut filament_SkinningBuffer,
        engine: *mut filament_Engine,
        transforms: *const filament_RenderableManager_Bone,
        count: size_t,
        offset: size_t,
    );
}
extern "C" {
    #[doc = " Updates the bone transforms in the range [offset, offset + count)."]
    #[doc = " @param engine Reference to the filament::Engine to associate this SkinningBuffer with."]
    #[doc = " @param transforms pointer to at least count mat4f"]
    #[doc = " @param count number of mat4f elements in transforms"]
    #[doc = " @param offset offset in elements (not bytes) in the SkinningBuffer (not in transforms)"]
    #[link_name = "\u{1}_ZN8filament14SkinningBuffer8setBonesERNS_6EngineEPKNS_4math7details6TMat44IfEEmm"]
    pub fn filament_SkinningBuffer_setBones1(
        this: *mut filament_SkinningBuffer,
        engine: *mut filament_Engine,
        transforms: *const filament_math_mat4f,
        count: size_t,
        offset: size_t,
    );
}
extern "C" {
    #[doc = " Returns the size of this SkinningBuffer in elements."]
    #[doc = " @return The number of bones the SkinningBuffer holds."]
    #[link_name = "\u{1}_ZNK8filament14SkinningBuffer12getBoneCountEv"]
    pub fn filament_SkinningBuffer_getBoneCount(this: *const filament_SkinningBuffer) -> size_t;
}
impl filament_SkinningBuffer {
    #[inline]
    pub unsafe fn setBones(
        &mut self,
        engine: *mut filament_Engine,
        transforms: *const filament_RenderableManager_Bone,
        count: size_t,
        offset: size_t,
    ) {
        filament_SkinningBuffer_setBones(self, engine, transforms, count, offset)
    }
    #[inline]
    pub unsafe fn setBones1(
        &mut self,
        engine: *mut filament_Engine,
        transforms: *const filament_math_mat4f,
        count: size_t,
        offset: size_t,
    ) {
        filament_SkinningBuffer_setBones1(self, engine, transforms, count, offset)
    }
    #[inline]
    pub unsafe fn getBoneCount(&self) -> size_t {
        filament_SkinningBuffer_getBoneCount(self)
    }
}
#[doc = " Skybox"]
#[doc = ""]
#[doc = " When added to a Scene, the Skybox fills all untouched pixels."]
#[doc = ""]
#[doc = " Creation and destruction"]
#[doc = " ========================"]
#[doc = ""]
#[doc = " A Skybox object is created using the Skybox::Builder and destroyed by calling"]
#[doc = " Engine::destroy(const Skybox*)."]
#[doc = ""]
#[doc = " ~~~~~~~~~~~{.cpp}"]
#[doc = "  filament::Engine* engine = filament::Engine::create();"]
#[doc = ""]
#[doc = "  filament::IndirectLight* skybox = filament::Skybox::Builder()"]
#[doc = "              .environment(cubemap)"]
#[doc = "              .build(*engine);"]
#[doc = ""]
#[doc = "  engine->destroy(skybox);"]
#[doc = " ~~~~~~~~~~~"]
#[doc = ""]
#[doc = ""]
#[doc = " @note"]
#[doc = " Currently only Texture based sky boxes are supported."]
#[doc = ""]
#[doc = " @see Scene, IndirectLight"]
#[repr(C)]
#[derive(Debug, Default)]
pub struct filament_Skybox {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct filament_Skybox_BuilderDetails {
    _unused: [u8; 0],
}
#[doc = "! Use Builder to construct an Skybox object instance"]
#[repr(C)]
#[derive(Debug)]
pub struct filament_Skybox_Builder {
    pub _base: filament_BuilderBase<filament_Skybox_BuilderDetails>,
}
#[test]
fn bindgen_test_layout_filament_Skybox_Builder() {
    assert_eq!(
        ::std::mem::size_of::<filament_Skybox_Builder>(),
        8usize,
        concat!("Size of: ", stringify!(filament_Skybox_Builder))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_Skybox_Builder>(),
        8usize,
        concat!("Alignment of ", stringify!(filament_Skybox_Builder))
    );
}
extern "C" {
    #[doc = " Set the environment map (i.e. the skybox content)."]
    #[doc = ""]
    #[doc = " The Skybox is rendered as though it were an infinitely large cube with the camera"]
    #[doc = " inside it. This means that the cubemap which is mapped onto the cube's exterior"]
    #[doc = " will appear mirrored. This follows the OpenGL conventions."]
    #[doc = ""]
    #[doc = " The cmgen tool generates reflection maps by default which are therefore ideal to use"]
    #[doc = " as skyboxes."]
    #[doc = ""]
    #[doc = " @param cubemap This Texture must be a cube map."]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls."]
    #[doc = ""]
    #[doc = " @see Texture"]
    #[link_name = "\u{1}_ZN8filament6Skybox7Builder11environmentEPNS_7TextureE"]
    pub fn filament_Skybox_Builder_environment(
        this: *mut filament_Skybox_Builder,
        cubemap: *mut filament_Texture,
    ) -> *mut filament_Skybox_Builder;
}
extern "C" {
    #[doc = " Indicates whether the sun should be rendered. The sun can only be"]
    #[doc = " rendered if there is at least one light of type SUN in the scene."]
    #[doc = " The default value is false."]
    #[doc = ""]
    #[doc = " @param show True if the sun should be rendered, false otherwise"]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls."]
    #[link_name = "\u{1}_ZN8filament6Skybox7Builder7showSunEb"]
    pub fn filament_Skybox_Builder_showSun(
        this: *mut filament_Skybox_Builder,
        show: bool,
    ) -> *mut filament_Skybox_Builder;
}
extern "C" {
    #[doc = " Skybox intensity when no IndirectLight is set on the Scene."]
    #[doc = ""]
    #[doc = " This call is ignored when an IndirectLight is set on the Scene, and the intensity"]
    #[doc = " of the IndirectLight is used instead."]
    #[doc = ""]
    #[doc = " @param envIntensity  Scale factor applied to the skybox texel values such that"]
    #[doc = "                      the result is in lux, or lumen/m^2 (default = 30000)"]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls."]
    #[doc = ""]
    #[doc = " @see IndirectLight::Builder::intensity"]
    #[link_name = "\u{1}_ZN8filament6Skybox7Builder9intensityEf"]
    pub fn filament_Skybox_Builder_intensity(
        this: *mut filament_Skybox_Builder,
        envIntensity: f32,
    ) -> *mut filament_Skybox_Builder;
}
extern "C" {
    #[doc = " Sets the skybox to a constant color. Default is opaque black."]
    #[doc = ""]
    #[doc = " Ignored if an environment is set."]
    #[doc = ""]
    #[doc = " @param color"]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls."]
    #[link_name = "\u{1}_ZN8filament6Skybox7Builder5colorENS_4math7details5TVec4IfEE"]
    pub fn filament_Skybox_Builder_color(
        this: *mut filament_Skybox_Builder,
        color: filament_math_float4,
    ) -> *mut filament_Skybox_Builder;
}
extern "C" {
    #[doc = " Creates the Skybox object and returns a pointer to it."]
    #[doc = ""]
    #[doc = " @param engine Reference to the filament::Engine to associate this Skybox with."]
    #[doc = ""]
    #[doc = " @return pointer to the newly created object, or nullptr if the light couldn't be created."]
    #[link_name = "\u{1}_ZN8filament6Skybox7Builder5buildERNS_6EngineE"]
    pub fn filament_Skybox_Builder_build(
        this: *mut filament_Skybox_Builder,
        engine: *mut filament_Engine,
    ) -> *mut filament_Skybox;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament6Skybox7BuilderC1Ev"]
    pub fn filament_Skybox_Builder_Builder(this: *mut filament_Skybox_Builder);
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament6Skybox7BuilderC1ERKS1_"]
    pub fn filament_Skybox_Builder_Builder1(
        this: *mut filament_Skybox_Builder,
        rhs: *const filament_Skybox_Builder,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament6Skybox7BuilderC1EOS1_"]
    pub fn filament_Skybox_Builder_Builder2(
        this: *mut filament_Skybox_Builder,
        rhs: *mut filament_Skybox_Builder,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament6Skybox7BuilderD1Ev"]
    pub fn filament_Skybox_Builder_Builder_destructor(this: *mut filament_Skybox_Builder);
}
impl Default for filament_Skybox_Builder {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl filament_Skybox_Builder {
    #[inline]
    pub unsafe fn environment(
        &mut self,
        cubemap: *mut filament_Texture,
    ) -> *mut filament_Skybox_Builder {
        filament_Skybox_Builder_environment(self, cubemap)
    }
    #[inline]
    pub unsafe fn showSun(&mut self, show: bool) -> *mut filament_Skybox_Builder {
        filament_Skybox_Builder_showSun(self, show)
    }
    #[inline]
    pub unsafe fn intensity(&mut self, envIntensity: f32) -> *mut filament_Skybox_Builder {
        filament_Skybox_Builder_intensity(self, envIntensity)
    }
    #[inline]
    pub unsafe fn color(&mut self, color: filament_math_float4) -> *mut filament_Skybox_Builder {
        filament_Skybox_Builder_color(self, color)
    }
    #[inline]
    pub unsafe fn build(&mut self, engine: *mut filament_Engine) -> *mut filament_Skybox {
        filament_Skybox_Builder_build(self, engine)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_Skybox_Builder_Builder(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(rhs: *const filament_Skybox_Builder) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_Skybox_Builder_Builder1(__bindgen_tmp.as_mut_ptr(), rhs);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new2(rhs: *mut filament_Skybox_Builder) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_Skybox_Builder_Builder2(__bindgen_tmp.as_mut_ptr(), rhs);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        filament_Skybox_Builder_Builder_destructor(self)
    }
}
#[test]
fn bindgen_test_layout_filament_Skybox() {
    assert_eq!(
        ::std::mem::size_of::<filament_Skybox>(),
        1usize,
        concat!("Size of: ", stringify!(filament_Skybox))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_Skybox>(),
        1usize,
        concat!("Alignment of ", stringify!(filament_Skybox))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament6Skybox8setColorENS_4math7details5TVec4IfEE"]
    pub fn filament_Skybox_setColor(this: *mut filament_Skybox, color: filament_math_float4);
}
extern "C" {
    #[doc = " Sets bits in a visibility mask. By default, this is 0x1."]
    #[doc = ""]
    #[doc = " This provides a simple mechanism for hiding or showing this Skybox in a Scene."]
    #[doc = ""]
    #[doc = " @see View::setVisibleLayers()."]
    #[doc = ""]
    #[doc = " For example, to set bit 1 and reset bits 0 and 2 while leaving all other bits unaffected,"]
    #[doc = " call: `setLayerMask(7, 2)`."]
    #[doc = ""]
    #[doc = " @param select the set of bits to affect"]
    #[doc = " @param values the replacement values for the affected bits"]
    #[link_name = "\u{1}_ZN8filament6Skybox12setLayerMaskEhh"]
    pub fn filament_Skybox_setLayerMask(this: *mut filament_Skybox, select: u8, values: u8);
}
extern "C" {
    #[doc = " @return the visibility mask bits"]
    #[link_name = "\u{1}_ZNK8filament6Skybox12getLayerMaskEv"]
    pub fn filament_Skybox_getLayerMask(this: *const filament_Skybox) -> u8;
}
extern "C" {
    #[doc = " Returns the skybox's intensity in lux, or lumen/m^2."]
    #[link_name = "\u{1}_ZNK8filament6Skybox12getIntensityEv"]
    pub fn filament_Skybox_getIntensity(this: *const filament_Skybox) -> f32;
}
extern "C" {
    #[doc = " @return the associated texture, or null if it does not exist"]
    #[link_name = "\u{1}_ZNK8filament6Skybox10getTextureEv"]
    pub fn filament_Skybox_getTexture(this: *const filament_Skybox) -> *const filament_Texture;
}
impl filament_Skybox {
    #[inline]
    pub unsafe fn setColor(&mut self, color: filament_math_float4) {
        filament_Skybox_setColor(self, color)
    }
    #[inline]
    pub unsafe fn setLayerMask(&mut self, select: u8, values: u8) {
        filament_Skybox_setLayerMask(self, select, values)
    }
    #[inline]
    pub unsafe fn getLayerMask(&self) -> u8 {
        filament_Skybox_getLayerMask(self)
    }
    #[inline]
    pub unsafe fn getIntensity(&self) -> f32 {
        filament_Skybox_getIntensity(self)
    }
    #[inline]
    pub unsafe fn getTexture(&self) -> *const filament_Texture {
        filament_Skybox_getTexture(self)
    }
}
#[doc = " Stream is used to attach a video stream to a Filament `Texture`."]
#[doc = ""]
#[doc = " Note that the `Stream` class is fairly Android centric. It supports three different"]
#[doc = " configurations:"]
#[doc = ""]
#[doc = "   - TEXTURE_ID...takes an OpenGL texture ID and incurs a copy"]
#[doc = "   - ACQUIRED.....connects to an Android AHardwareBuffer"]
#[doc = "   - NATIVE.......connects to an Android SurfaceTexture"]
#[doc = ""]
#[doc = " Before explaining these different configurations, let's review the high-level structure of an AR"]
#[doc = " or video application that uses Filament:"]
#[doc = ""]
#[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
#[doc = " while (true) {"]
#[doc = ""]
#[doc = "     // Misc application work occurs here, such as:"]
#[doc = "     // - Writing the image data for a video frame into a Stream"]
#[doc = "     // - Moving the Filament Camera"]
#[doc = ""]
#[doc = "     if (renderer->beginFrame(swapChain)) {"]
#[doc = "         renderer->render(view);"]
#[doc = "         renderer->endFrame();"]
#[doc = "     }"]
#[doc = " }"]
#[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
#[doc = ""]
#[doc = " Let's say that the video image data at the time of a particular invocation of `beginFrame`"]
#[doc = " becomes visible to users at time A. The 3D scene state (including the camera) at the time of"]
#[doc = " that same invocation becomes apparent to users at time B."]
#[doc = ""]
#[doc = " - If time A matches time B, we say that the stream is \\em{synchronized}."]
#[doc = " - Filament invokes low-level graphics commands on the \\em{driver thread}."]
#[doc = " - The thread that calls `beginFrame` is called the \\em{main thread}."]
#[doc = ""]
#[doc = " The TEXTURE_ID configuration achieves synchronization automatically. In this mode, Filament"]
#[doc = " performs a copy on the main thread during `beginFrame` by blitting the external image into"]
#[doc = " an internal round-robin queue of images. This copy has a run-time cost."]
#[doc = ""]
#[doc = " For ACQUIRED streams, there is no need to perform the copy because Filament explictly acquires"]
#[doc = " the stream, then releases it later via a callback function. This configuration is especially"]
#[doc = " useful when the Vulkan backend is enabled."]
#[doc = ""]
#[doc = " For NATIVE streams, Filament does not make any synchronization guarantee. However they are simple"]
#[doc = " to use and do not incur a copy. These are often appropriate in video applications."]
#[doc = ""]
#[doc = " Please see `sample-stream-test` and `sample-hello-camera` for usage examples."]
#[doc = ""]
#[doc = " @see backend::StreamType"]
#[doc = " @see Texture#setExternalStream"]
#[doc = " @see Engine#destroyStream"]
#[repr(C)]
#[derive(Debug, Default)]
pub struct filament_Stream {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct filament_Stream_BuilderDetails {
    _unused: [u8; 0],
}
pub type filament_Stream_Callback = filament_backend_StreamCallback;
#[doc = "! Stream for external textures"]
pub use self::filament_backend_StreamType as filament_Stream_StreamType;
#[doc = " Constructs a Stream object instance."]
#[doc = ""]
#[doc = " By default, Stream objects are ACQUIRED and must have external images pushed to them via"]
#[doc = " <pre>Stream::setAcquiredImage</pre>."]
#[doc = ""]
#[doc = " To create a NATIVE or TEXTURE_ID stream, call one of the <pre>stream</pre> methods"]
#[doc = " on the builder."]
#[repr(C)]
#[derive(Debug)]
pub struct filament_Stream_Builder {
    pub _base: filament_BuilderBase<filament_Stream_BuilderDetails>,
}
#[test]
fn bindgen_test_layout_filament_Stream_Builder() {
    assert_eq!(
        ::std::mem::size_of::<filament_Stream_Builder>(),
        8usize,
        concat!("Size of: ", stringify!(filament_Stream_Builder))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_Stream_Builder>(),
        8usize,
        concat!("Alignment of ", stringify!(filament_Stream_Builder))
    );
}
extern "C" {
    #[doc = " Creates a NATIVE stream. Native streams can sample data directly from an"]
    #[doc = " opaque platform object such as a SurfaceTexture on Android."]
    #[doc = ""]
    #[doc = " @param stream An opaque native stream handle. e.g.: on Android this is an"]
    #[doc = "                     `android/graphics/SurfaceTexture` JNI jobject. The wrap mode must"]
    #[doc = "                     be CLAMP_TO_EDGE."]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls."]
    #[link_name = "\u{1}_ZN8filament6Stream7Builder6streamEPv"]
    pub fn filament_Stream_Builder_stream(
        this: *mut filament_Stream_Builder,
        stream: *mut ::std::os::raw::c_void,
    ) -> *mut filament_Stream_Builder;
}
extern "C" {
    #[doc = " Creates a TEXTURE_ID stream. This will sample data from the supplied"]
    #[doc = " external texture and copy it into an internal private texture."]
    #[doc = ""]
    #[doc = " @param externalTextureId An opaque texture id (typically a GLuint created with glGenTextures)"]
    #[doc = "                          In a context shared with filament. In that case this texture's"]
    #[doc = "                          target must be GL_TEXTURE_EXTERNAL_OES and the wrap mode must"]
    #[doc = "                          be CLAMP_TO_EDGE."]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls."]
    #[doc = ""]
    #[doc = " @see Texture::setExternalStream()"]
    #[doc = " @deprecated this method existed only for ARCore which doesn't need this anymore, use Texture::import() instead."]
    #[link_name = "\u{1}_ZN8filament6Stream7Builder6streamEl"]
    pub fn filament_Stream_Builder_stream1(
        this: *mut filament_Stream_Builder,
        externalTextureId: isize,
    ) -> *mut filament_Stream_Builder;
}
extern "C" {
    #[doc = " @param width initial width of the incoming stream. Whether this value is used is"]
    #[doc = "              stream dependent. On Android, it must be set when using"]
    #[doc = "              Builder::stream(long externalTextureId)."]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls."]
    #[link_name = "\u{1}_ZN8filament6Stream7Builder5widthEj"]
    pub fn filament_Stream_Builder_width(
        this: *mut filament_Stream_Builder,
        width: u32,
    ) -> *mut filament_Stream_Builder;
}
extern "C" {
    #[doc = " @param height initial height of the incoming stream. Whether this value is used is"]
    #[doc = "              stream dependent. On Android, it must be set when using"]
    #[doc = "              Builder::stream(long externalTextureId)."]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls."]
    #[link_name = "\u{1}_ZN8filament6Stream7Builder6heightEj"]
    pub fn filament_Stream_Builder_height(
        this: *mut filament_Stream_Builder,
        height: u32,
    ) -> *mut filament_Stream_Builder;
}
extern "C" {
    #[doc = " Creates the Stream object and returns a pointer to it."]
    #[doc = ""]
    #[doc = " @param engine Reference to the filament::Engine to associate this Stream with."]
    #[doc = ""]
    #[doc = " @return pointer to the newly created object, or nullptr if the stream couldn't be created."]
    #[link_name = "\u{1}_ZN8filament6Stream7Builder5buildERNS_6EngineE"]
    pub fn filament_Stream_Builder_build(
        this: *mut filament_Stream_Builder,
        engine: *mut filament_Engine,
    ) -> *mut filament_Stream;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament6Stream7BuilderC1Ev"]
    pub fn filament_Stream_Builder_Builder(this: *mut filament_Stream_Builder);
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament6Stream7BuilderC1ERKS1_"]
    pub fn filament_Stream_Builder_Builder1(
        this: *mut filament_Stream_Builder,
        rhs: *const filament_Stream_Builder,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament6Stream7BuilderC1EOS1_"]
    pub fn filament_Stream_Builder_Builder2(
        this: *mut filament_Stream_Builder,
        rhs: *mut filament_Stream_Builder,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament6Stream7BuilderD1Ev"]
    pub fn filament_Stream_Builder_Builder_destructor(this: *mut filament_Stream_Builder);
}
impl Default for filament_Stream_Builder {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl filament_Stream_Builder {
    #[inline]
    pub unsafe fn stream(
        &mut self,
        stream: *mut ::std::os::raw::c_void,
    ) -> *mut filament_Stream_Builder {
        filament_Stream_Builder_stream(self, stream)
    }
    #[inline]
    pub unsafe fn stream1(&mut self, externalTextureId: isize) -> *mut filament_Stream_Builder {
        filament_Stream_Builder_stream1(self, externalTextureId)
    }
    #[inline]
    pub unsafe fn width(&mut self, width: u32) -> *mut filament_Stream_Builder {
        filament_Stream_Builder_width(self, width)
    }
    #[inline]
    pub unsafe fn height(&mut self, height: u32) -> *mut filament_Stream_Builder {
        filament_Stream_Builder_height(self, height)
    }
    #[inline]
    pub unsafe fn build(&mut self, engine: *mut filament_Engine) -> *mut filament_Stream {
        filament_Stream_Builder_build(self, engine)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_Stream_Builder_Builder(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(rhs: *const filament_Stream_Builder) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_Stream_Builder_Builder1(__bindgen_tmp.as_mut_ptr(), rhs);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new2(rhs: *mut filament_Stream_Builder) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_Stream_Builder_Builder2(__bindgen_tmp.as_mut_ptr(), rhs);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        filament_Stream_Builder_Builder_destructor(self)
    }
}
#[test]
fn bindgen_test_layout_filament_Stream() {
    assert_eq!(
        ::std::mem::size_of::<filament_Stream>(),
        1usize,
        concat!("Size of: ", stringify!(filament_Stream))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_Stream>(),
        1usize,
        concat!("Alignment of ", stringify!(filament_Stream))
    );
}
extern "C" {
    #[doc = " Indicates whether this stream is a NATIVE stream, TEXTURE_ID stream, or ACQUIRED stream."]
    #[link_name = "\u{1}_ZNK8filament6Stream13getStreamTypeEv"]
    pub fn filament_Stream_getStreamType(
        this: *const filament_Stream,
    ) -> filament_Stream_StreamType;
}
extern "C" {
    #[doc = " Updates an ACQUIRED stream with an image that is guaranteed to be used in the next frame."]
    #[doc = ""]
    #[doc = " This method tells Filament to immediately \"acquire\" the image and trigger a callback"]
    #[doc = " when it is done with it. This should be called by the user outside of beginFrame / endFrame,"]
    #[doc = " and should be called only once per frame. If the user pushes images to the same stream"]
    #[doc = " multiple times in a single frame, only the final image is honored, but all callbacks are"]
    #[doc = " invoked."]
    #[doc = ""]
    #[doc = " This method should be called on the same thread that calls Renderer::beginFrame, which is"]
    #[doc = " also where the callback is invoked. This method can only be used for streams that were"]
    #[doc = " constructed without calling the `stream` method on the builder."]
    #[doc = ""]
    #[doc = " @see Stream for more information about NATIVE, TEXTURE_ID, and ACQUIRED configurations."]
    #[doc = ""]
    #[doc = " @param image      Pointer to AHardwareBuffer, casted to void* since this is a public header."]
    #[doc = " @param callback   This is triggered by Filament when it wishes to release the image."]
    #[doc = "                   It callback tales two arguments: the AHardwareBuffer and the userdata."]
    #[doc = " @param userdata   Optional closure data. Filament will pass this into the callback when it"]
    #[doc = "                   releases the image."]
    #[link_name = "\u{1}_ZN8filament6Stream16setAcquiredImageEPvPFvS1_S1_ES1_"]
    pub fn filament_Stream_setAcquiredImage(
        this: *mut filament_Stream,
        image: *mut ::std::os::raw::c_void,
        callback: filament_Stream_Callback,
        userdata: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    #[doc = " @see setAcquiredImage(void*, Callback, void*)"]
    #[doc = ""]
    #[doc = " @param image      Pointer to AHardwareBuffer, casted to void* since this is a public header."]
    #[doc = " @param handler    Handler to dispatch the AcquiredImage or nullptr for the default handler."]
    #[doc = " @param callback   This is triggered by Filament when it wishes to release the image."]
    #[doc = "                   It callback tales two arguments: the AHardwareBuffer and the userdata."]
    #[doc = " @param userdata   Optional closure data. Filament will pass this into the callback when it"]
    #[doc = "                   releases the image."]
    #[link_name = "\u{1}_ZN8filament6Stream16setAcquiredImageEPvPNS_7backend15CallbackHandlerEPFvS1_S1_ES1_"]
    pub fn filament_Stream_setAcquiredImage1(
        this: *mut filament_Stream,
        image: *mut ::std::os::raw::c_void,
        handler: *mut filament_backend_CallbackHandler,
        callback: filament_Stream_Callback,
        userdata: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    #[doc = " Updates the size of the incoming stream. Whether this value is used is"]
    #[doc = "              stream dependent. On Android, it must be set when using"]
    #[doc = "              Builder::stream(long externalTextureId)."]
    #[doc = ""]
    #[doc = " @param width     new width of the incoming stream"]
    #[doc = " @param height    new height of the incoming stream"]
    #[link_name = "\u{1}_ZN8filament6Stream13setDimensionsEjj"]
    pub fn filament_Stream_setDimensions(this: *mut filament_Stream, width: u32, height: u32);
}
extern "C" {
    #[doc = " Read-back the content of the last frame of a Stream since the last call to"]
    #[doc = " Renderer.beginFrame()."]
    #[doc = ""]
    #[doc = " The Stream must be of type externalTextureId. This function is a no-op otherwise."]
    #[doc = ""]
    #[doc = " @param xoffset   Left offset of the sub-region to read back."]
    #[doc = " @param yoffset   Bottom offset of the sub-region to read back."]
    #[doc = " @param width     Width of the sub-region to read back."]
    #[doc = " @param height    Height of the sub-region to read back."]
    #[doc = " @param buffer    Client-side buffer where the read-back will be written."]
    #[doc = ""]
    #[doc = "                  The following format are always supported:"]
    #[doc = "                      - PixelBufferDescriptor::PixelDataFormat::RGBA"]
    #[doc = "                      - PixelBufferDescriptor::PixelDataFormat::RGBA_INTEGER"]
    #[doc = ""]
    #[doc = "                  The following types are always supported:"]
    #[doc = "                      - PixelBufferDescriptor::PixelDataType::UBYTE"]
    #[doc = "                      - PixelBufferDescriptor::PixelDataType::UINT"]
    #[doc = "                      - PixelBufferDescriptor::PixelDataType::INT"]
    #[doc = "                      - PixelBufferDescriptor::PixelDataType::FLOAT"]
    #[doc = ""]
    #[doc = "                  Other combination of format/type may be supported. If a combination is"]
    #[doc = "                  not supported, this operation may fail silently. Use a DEBUG build"]
    #[doc = "                  to get some logs about the failure."]
    #[doc = ""]
    #[doc = "  Stream buffer                  User buffer (PixelBufferDescriptor&)"]
    #[doc = "  +--------------------+"]
    #[doc = "  |                    |                .stride         .alignment"]
    #[doc = "  |                    |         ----------------------->-->"]
    #[doc = "  |                    |         O----------------------+--+   low addresses"]
    #[doc = "  |                    |         |          |           |  |"]
    #[doc = "  |             w      |         |          | .top      |  |"]
    #[doc = "  |       <--------->  |         |          V           |  |"]
    #[doc = "  |       +---------+  |         |     +---------+      |  |"]
    #[doc = "  |       |     ^   |  | ======> |     |         |      |  |"]
    #[doc = "  |   x   |    h|   |  |         |.left|         |      |  |"]
    #[doc = "  +------>|     v   |  |         +---->|         |      |  |"]
    #[doc = "  |       +.........+  |         |     +.........+      |  |"]
    #[doc = "  |            ^       |         |                      |  |"]
    #[doc = "  |          y |       |         +----------------------+--+  high addresses"]
    #[doc = "  O------------+-------+"]
    #[doc = ""]
    #[doc = " Typically readPixels() will be called after Renderer.beginFrame()."]
    #[doc = ""]
    #[doc = " After issuing this method, the callback associated with `buffer` will be invoked on the"]
    #[doc = " main thread, indicating that the read-back has completed. Typically, this will happen"]
    #[doc = " after multiple calls to beginFrame(), render(), endFrame()."]
    #[doc = ""]
    #[doc = " It is also possible to use a Fence to wait for the read-back."]
    #[doc = ""]
    #[doc = " @remark"]
    #[doc = " readPixels() is intended for debugging and testing. It will impact performance significantly."]
    #[link_name = "\u{1}_ZN8filament6Stream10readPixelsEjjjjONS_7backend21PixelBufferDescriptorE"]
    pub fn filament_Stream_readPixels(
        this: *mut filament_Stream,
        xoffset: u32,
        yoffset: u32,
        width: u32,
        height: u32,
        buffer: *mut filament_backend_PixelBufferDescriptor,
    );
}
extern "C" {
    #[doc = " Returns the presentation time of the currently displayed frame in nanosecond."]
    #[doc = ""]
    #[doc = " This value can change at any time."]
    #[doc = ""]
    #[doc = " @return timestamp in nanosecond."]
    #[link_name = "\u{1}_ZNK8filament6Stream12getTimestampEv"]
    pub fn filament_Stream_getTimestamp(this: *const filament_Stream) -> i64;
}
impl filament_Stream {
    #[inline]
    pub unsafe fn getStreamType(&self) -> filament_Stream_StreamType {
        filament_Stream_getStreamType(self)
    }
    #[inline]
    pub unsafe fn setAcquiredImage(
        &mut self,
        image: *mut ::std::os::raw::c_void,
        callback: filament_Stream_Callback,
        userdata: *mut ::std::os::raw::c_void,
    ) {
        filament_Stream_setAcquiredImage(self, image, callback, userdata)
    }
    #[inline]
    pub unsafe fn setAcquiredImage1(
        &mut self,
        image: *mut ::std::os::raw::c_void,
        handler: *mut filament_backend_CallbackHandler,
        callback: filament_Stream_Callback,
        userdata: *mut ::std::os::raw::c_void,
    ) {
        filament_Stream_setAcquiredImage1(self, image, handler, callback, userdata)
    }
    #[inline]
    pub unsafe fn setDimensions(&mut self, width: u32, height: u32) {
        filament_Stream_setDimensions(self, width, height)
    }
    #[inline]
    pub unsafe fn readPixels(
        &mut self,
        xoffset: u32,
        yoffset: u32,
        width: u32,
        height: u32,
        buffer: *mut filament_backend_PixelBufferDescriptor,
    ) {
        filament_Stream_readPixels(self, xoffset, yoffset, width, height, buffer)
    }
    #[inline]
    pub unsafe fn getTimestamp(&self) -> i64 {
        filament_Stream_getTimestamp(self)
    }
}
#[doc = " A swap chain represents an Operating System's *native* renderable surface."]
#[doc = ""]
#[doc = " Typically it's a native window or a view. Because a SwapChain is initialized from a"]
#[doc = " native object, it is given to filament as a `void *`, which must be of the proper type"]
#[doc = " for each platform filament is running on."]
#[doc = ""]
#[doc = " \\code"]
#[doc = " SwapChain* swapChain = engine->createSwapChain(nativeWindow);"]
#[doc = " \\endcode"]
#[doc = ""]
#[doc = " When Engine::create() is used without specifying a Platform, the `nativeWindow`"]
#[doc = " parameter above must be of type:"]
#[doc = ""]
#[doc = " Platform        | nativeWindow type"]
#[doc = " :---------------|:----------------------------:"]
#[doc = " Android         | ANativeWindow*"]
#[doc = " macOS - OpenGL  | NSView*"]
#[doc = " macOS - Metal   | CAMetalLayer*"]
#[doc = " iOS - OpenGL    | CAEAGLLayer*"]
#[doc = " iOS - Metal     | CAMetalLayer*"]
#[doc = " X11             | Window"]
#[doc = " Windows         | HWND"]
#[doc = ""]
#[doc = " Otherwise, the `nativeWindow` is defined by the concrete implementation of Platform."]
#[doc = ""]
#[doc = ""]
#[doc = " Examples:"]
#[doc = ""]
#[doc = " Android"]
#[doc = " -------"]
#[doc = ""]
#[doc = " On Android, an `ANativeWindow*` can be obtained from a Java `Surface` object using:"]
#[doc = ""]
#[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
#[doc = "  #include <android/native_window_jni.h>"]
#[doc = "  // parameters"]
#[doc = "  // env:         JNIEnv*"]
#[doc = "  // surface:     jobject"]
#[doc = "  ANativeWindow* win = ANativeWindow_fromSurface(env, surface);"]
#[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
#[doc = ""]
#[doc = " \\warning"]
#[doc = " Don't use reflection to access the `mNativeObject` field, it won't work."]
#[doc = ""]
#[doc = " A `Surface` can be retrieved from a `SurfaceView` or `SurfaceHolder` easily using"]
#[doc = " `SurfaceHolder.getSurface()` and/or `SurfaceView.getHolder()`."]
#[doc = ""]
#[doc = " \\note"]
#[doc = " To use a `TextureView` as a SwapChain, it is necessary to first get its `SurfaceTexture`,"]
#[doc = " for instance using `TextureView.SurfaceTextureListener` and then create a `Surface`:"]
#[doc = ""]
#[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~{.java}"]
#[doc = "  // using a TextureView.SurfaceTextureListener:"]
#[doc = "  public void onSurfaceTextureAvailable(SurfaceTexture surfaceTexture, int width, int height) {"]
#[doc = "      mSurface = new Surface(surfaceTexture);"]
#[doc = "      // mSurface can now be used in JNI to create an ANativeWindow."]
#[doc = "  }"]
#[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
#[doc = ""]
#[doc = " Linux"]
#[doc = " -----"]
#[doc = ""]
#[doc = " Example using SDL:"]
#[doc = ""]
#[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
#[doc = " SDL_SysWMinfo wmi;"]
#[doc = " SDL_VERSION(&wmi.version);"]
#[doc = " SDL_GetWindowWMInfo(sdlWindow, &wmi);"]
#[doc = " Window nativeWindow = (Window) wmi.info.x11.window;"]
#[doc = ""]
#[doc = " using namespace filament;"]
#[doc = " Engine* engine       = Engine::create();"]
#[doc = " SwapChain* swapChain = engine->createSwapChain((void*) nativeWindow);"]
#[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
#[doc = ""]
#[doc = " Windows"]
#[doc = " -------"]
#[doc = ""]
#[doc = " Example using SDL:"]
#[doc = ""]
#[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
#[doc = " SDL_SysWMinfo wmi;"]
#[doc = " SDL_VERSION(&wmi.version);"]
#[doc = " ASSERT_POSTCONDITION(SDL_GetWindowWMInfo(sdlWindow, &wmi), \"SDL version unsupported!\");"]
#[doc = " HDC nativeWindow = (HDC) wmi.info.win.hdc;"]
#[doc = ""]
#[doc = " using namespace filament;"]
#[doc = " Engine* engine       = Engine::create();"]
#[doc = " SwapChain* swapChain = engine->createSwapChain((void*) nativeWindow);"]
#[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
#[doc = ""]
#[doc = " OSX"]
#[doc = " ---"]
#[doc = ""]
#[doc = " On OSX, any `NSView` can be used *directly* as a `nativeWindow` with createSwapChain()."]
#[doc = ""]
#[doc = " Example using SDL/Objective-C:"]
#[doc = ""]
#[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~{.mm}"]
#[doc = "  #include <filament/Engine.h>"]
#[doc = ""]
#[doc = "  #include <Cocoa/Cocoa.h>"]
#[doc = "  #include <SDL_syswm.h>"]
#[doc = ""]
#[doc = "  SDL_SysWMinfo wmi;"]
#[doc = "  SDL_VERSION(&wmi.version);"]
#[doc = "  NSWindow* win = (NSWindow*) wmi.info.cocoa.window;"]
#[doc = "  NSView* view = [win contentView];"]
#[doc = "  void* nativeWindow = view;"]
#[doc = ""]
#[doc = "  using namespace filament;"]
#[doc = "  Engine* engine       = Engine::create();"]
#[doc = "  SwapChain* swapChain = engine->createSwapChain(nativeWindow);"]
#[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
#[doc = ""]
#[doc = " @see Engine"]
#[repr(C)]
#[derive(Debug, Default)]
pub struct filament_SwapChain {
    pub _address: u8,
}
pub type filament_SwapChain_FrameScheduledCallback = filament_backend_FrameScheduledCallback;
pub type filament_SwapChain_FrameCompletedCallback = filament_backend_FrameCompletedCallback;
pub const filament_SwapChain_CONFIG_TRANSPARENT: u64 = 1;
#[doc = " This flag indicates that the swap chain may be used as a source surface"]
#[doc = " for reading back render results.  This config must be set when creating"]
#[doc = " any swap chain that will be used as the source for a blit operation."]
#[doc = ""]
#[doc = " @see"]
#[doc = " Renderer.copyFrame()"]
pub const filament_SwapChain_CONFIG_READABLE: u64 = 2;
#[doc = " Indicates that the native X11 window is an XCB window rather than an XLIB window."]
#[doc = " This is ignored on non-Linux platforms and in builds that support only one X11 API."]
pub const filament_SwapChain_CONFIG_ENABLE_XCB: u64 = 4;
#[doc = " Indicates that the native window is a CVPixelBufferRef."]
#[doc = ""]
#[doc = " This is only supported by the Metal backend. The CVPixelBuffer must be in the"]
#[doc = " kCVPixelFormatType_32BGRA format."]
#[doc = ""]
#[doc = " It is not necessary to add an additional retain call before passing the pixel buffer to"]
#[doc = " Filament. Filament will call CVPixelBufferRetain during Engine::createSwapChain, and"]
#[doc = " CVPixelBufferRelease when the swap chain is destroyed."]
pub const filament_SwapChain_CONFIG_APPLE_CVPIXELBUFFER: u64 = 8;
#[test]
fn bindgen_test_layout_filament_SwapChain() {
    assert_eq!(
        ::std::mem::size_of::<filament_SwapChain>(),
        1usize,
        concat!("Size of: ", stringify!(filament_SwapChain))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_SwapChain>(),
        1usize,
        concat!("Alignment of ", stringify!(filament_SwapChain))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNK8filament9SwapChain15getNativeWindowEv"]
    pub fn filament_SwapChain_getNativeWindow(
        this: *const filament_SwapChain,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[doc = " FrameScheduledCallback is a callback function that notifies an application when Filament has"]
    #[doc = " completed processing a frame and that frame is ready to be scheduled for presentation."]
    #[doc = ""]
    #[doc = " Typically, Filament is responsible for scheduling the frame's presentation to the SwapChain."]
    #[doc = " If a SwapChain::FrameScheduledCallback is set, however, the application bares the"]
    #[doc = " responsibility of scheduling a frame for presentation by calling the backend::PresentCallable"]
    #[doc = " passed to the callback function. Currently this functionality is only supported by the Metal"]
    #[doc = " backend."]
    #[doc = ""]
    #[doc = " A FrameScheduledCallback can be set on an individual SwapChain through"]
    #[doc = " SwapChain::setFrameScheduledCallback. If the callback is set, then the SwapChain will *not*"]
    #[doc = " automatically schedule itself for presentation. Instead, the application must call the"]
    #[doc = " PresentCallable passed to the FrameScheduledCallback."]
    #[doc = ""]
    #[doc = " @param callback    A callback, or nullptr to unset."]
    #[doc = " @param user        An optional pointer to user data passed to the callback function."]
    #[doc = ""]
    #[doc = " @remark Only Filament's Metal backend supports PresentCallables and frame callbacks. Other"]
    #[doc = " backends ignore the callback (which will never be called) and proceed normally."]
    #[doc = ""]
    #[doc = " @remark The SwapChain::FrameScheduledCallback is called on an arbitrary thread."]
    #[doc = ""]
    #[doc = " @see PresentCallable"]
    #[link_name = "\u{1}_ZN8filament9SwapChain25setFrameScheduledCallbackEPFvNS_7backend15PresentCallableEPvES3_"]
    pub fn filament_SwapChain_setFrameScheduledCallback(
        this: *mut filament_SwapChain,
        callback: filament_SwapChain_FrameScheduledCallback,
        user: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    #[doc = " FrameCompletedCallback is a callback function that notifies an application when a frame's"]
    #[doc = " contents have completed rendering on the GPU."]
    #[doc = ""]
    #[doc = " Use SwapChain::setFrameCompletedCallback to set a callback on an individual SwapChain. Each"]
    #[doc = " time a frame completes GPU rendering, the callback will be called with optional user data."]
    #[doc = ""]
    #[doc = " The FrameCompletedCallback is guaranteed to be called on the main Filament thread."]
    #[doc = ""]
    #[doc = " @param callback    A callback, or nullptr to unset."]
    #[doc = " @param user        An optional pointer to user data passed to the callback function."]
    #[doc = ""]
    #[doc = " @remark Only Filament's Metal backend supports frame callbacks. Other backends ignore the"]
    #[doc = " callback (which will never be called) and proceed normally."]
    #[link_name = "\u{1}_ZN8filament9SwapChain25setFrameCompletedCallbackEPFvPvES1_"]
    pub fn filament_SwapChain_setFrameCompletedCallback(
        this: *mut filament_SwapChain,
        callback: filament_SwapChain_FrameCompletedCallback,
        user: *mut ::std::os::raw::c_void,
    );
}
impl filament_SwapChain {
    #[inline]
    pub unsafe fn getNativeWindow(&self) -> *mut ::std::os::raw::c_void {
        filament_SwapChain_getNativeWindow(self)
    }
    #[inline]
    pub unsafe fn setFrameScheduledCallback(
        &mut self,
        callback: filament_SwapChain_FrameScheduledCallback,
        user: *mut ::std::os::raw::c_void,
    ) {
        filament_SwapChain_setFrameScheduledCallback(self, callback, user)
    }
    #[inline]
    pub unsafe fn setFrameCompletedCallback(
        &mut self,
        callback: filament_SwapChain_FrameCompletedCallback,
        user: *mut ::std::os::raw::c_void,
    ) {
        filament_SwapChain_setFrameCompletedCallback(self, callback, user)
    }
}
#[doc = " Texture"]
#[doc = ""]
#[doc = " The Texture class supports:"]
#[doc = "  - 2D textures"]
#[doc = "  - 3D textures"]
#[doc = "  - Cube maps"]
#[doc = "  - mip mapping"]
#[doc = ""]
#[doc = ""]
#[doc = " Creation and destruction"]
#[doc = " ========================"]
#[doc = ""]
#[doc = " A Texture object is created using the Texture::Builder and destroyed by calling"]
#[doc = " Engine::destroy(const Texture*)."]
#[doc = ""]
#[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~{.cpp}"]
#[doc = "  filament::Engine* engine = filament::Engine::create();"]
#[doc = ""]
#[doc = "  filament::Texture* texture = filament::Texture::Builder()"]
#[doc = "              .width(64)"]
#[doc = "              .height(64)"]
#[doc = "              .build(*engine);"]
#[doc = ""]
#[doc = "  engine->destroy(texture);"]
#[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Default)]
pub struct filament_Texture {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct filament_Texture_BuilderDetails {
    _unused: [u8; 0],
}
#[doc = " A descriptor to an image in main memory, typically used to transfer image data from the CPU"]
#[doc = " to the GPU."]
#[doc = ""]
#[doc = " A PixelBufferDescriptor owns the memory buffer it references, therefore PixelBufferDescriptor"]
#[doc = " cannot be copied, but can be moved."]
#[doc = ""]
#[doc = " PixelBufferDescriptor releases ownership of the memory-buffer when it's destroyed."]
pub type filament_Texture_PixelBufferDescriptor = filament_backend_PixelBufferDescriptor;
#[doc = "! Compressed pixel data types"]
pub use self::filament_backend_CompressedPixelDataType as filament_Texture_CompressedType;
#[doc = "! Pixel Data Format"]
pub use self::filament_backend_PixelDataFormat as filament_Texture_Format;
#[doc = "! Pixel Data Type"]
pub use self::filament_backend_PixelDataType as filament_Texture_Type;
#[doc = "! Texture sampler type"]
pub use self::filament_backend_SamplerType as filament_Texture_Sampler;
#[doc = "! Texture Cubemap Face"]
pub use self::filament_backend_TextureCubemapFace as filament_Texture_CubemapFace;
#[doc = " Supported texel formats"]
#[doc = " These formats are typically used to specify a texture's internal storage format."]
#[doc = ""]
#[doc = " Enumerants syntax format"]
#[doc = " ========================"]
#[doc = ""]
#[doc = " `[components][size][type]`"]
#[doc = ""]
#[doc = " `components` : List of stored components by this format.\\n"]
#[doc = " `size`       : Size in bit of each component.\\n"]
#[doc = " `type`       : Type this format is stored as.\\n"]
#[doc = ""]
#[doc = ""]
#[doc = " Name     | Component"]
#[doc = " :--------|:-------------------------------"]
#[doc = " R        | Linear Red"]
#[doc = " RG       | Linear Red, Green"]
#[doc = " RGB      | Linear Red, Green, Blue"]
#[doc = " RGBA     | Linear Red, Green Blue, Alpha"]
#[doc = " SRGB     | sRGB encoded Red, Green, Blue"]
#[doc = " DEPTH    | Depth"]
#[doc = " STENCIL  | Stencil"]
#[doc = ""]
#[doc = " \\n"]
#[doc = " Name     | Type"]
#[doc = " :--------|:---------------------------------------------------"]
#[doc = " (none)   | Unsigned Normalized Integer [0, 1]"]
#[doc = " _SNORM   | Signed Normalized Integer [-1, 1]"]
#[doc = " UI       | Unsigned Integer @f$ [0, 2^{size}] @f$"]
#[doc = " I        | Signed Integer @f$ [-2^{size-1}, 2^{size-1}-1] @f$"]
#[doc = " F        | Floating-point"]
#[doc = ""]
#[doc = ""]
#[doc = " Special color formats"]
#[doc = " ---------------------"]
#[doc = ""]
#[doc = " There are a few special color formats that don't follow the convention above:"]
#[doc = ""]
#[doc = " Name             | Format"]
#[doc = " :----------------|:--------------------------------------------------------------------------"]
#[doc = " RGB565           |  5-bits for R and B, 6-bits for G."]
#[doc = " RGB5_A1          |  5-bits for R, G and B, 1-bit for A."]
#[doc = " RGB10_A2         | 10-bits for R, G and B, 2-bits for A."]
#[doc = " RGB9_E5          | **Unsigned** floating point. 9-bits mantissa for RGB, 5-bits shared exponent"]
#[doc = " R11F_G11F_B10F   | **Unsigned** floating point. 6-bits mantissa, for R and G, 5-bits for B. 5-bits exponent."]
#[doc = " SRGB8_A8         | sRGB 8-bits with linear 8-bits alpha."]
#[doc = " DEPTH24_STENCIL8 | 24-bits unsigned normalized integer depth, 8-bits stencil."]
#[doc = " DEPTH32F_STENCIL8| 32-bits floating-point depth, 8-bits stencil."]
#[doc = ""]
#[doc = ""]
#[doc = " Compressed texture formats"]
#[doc = " --------------------------"]
#[doc = ""]
#[doc = " Many compressed texture formats are supported as well, which include (but are not limited to)"]
#[doc = " the following list:"]
#[doc = ""]
#[doc = " Name             | Format"]
#[doc = " :----------------|:--------------------------------------------------------------------------"]
#[doc = " EAC_R11          | Compresses R11UI"]
#[doc = " EAC_R11_SIGNED   | Compresses R11I"]
#[doc = " EAC_RG11         | Compresses RG11UI"]
#[doc = " EAC_RG11_SIGNED  | Compresses RG11I"]
#[doc = " ETC2_RGB8        | Compresses RGB8"]
#[doc = " ETC2_SRGB8       | compresses SRGB8"]
#[doc = " ETC2_EAC_RGBA8   | Compresses RGBA8"]
#[doc = " ETC2_EAC_SRGBA8  | Compresses SRGB8_A8"]
#[doc = " ETC2_RGB8_A1     | Compresses RGB8 with 1-bit alpha"]
#[doc = " ETC2_SRGB8_A1    | Compresses sRGB8 with 1-bit alpha"]
#[doc = ""]
#[doc = ""]
#[doc = " @see Texture"]
pub use self::filament_backend_TextureFormat as filament_Texture_InternalFormat;
#[doc = "! Face offsets for all faces of a cubemap"]
pub type filament_Texture_FaceOffsets = filament_backend_FaceOffsets;
#[doc = "! Texture swizzle"]
pub use self::filament_backend_TextureSwizzle as filament_Texture_Swizzle;
#[doc = "! Bitmask describing the intended Texture Usage"]
pub use self::filament_backend_TextureUsage as filament_Texture_Usage;
#[doc = " Options for environment prefiltering into reflection map"]
#[doc = ""]
#[doc = " @see generatePrefilterMipmap()"]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_Texture_PrefilterOptions {
    #[doc = "!< sample count used for filtering"]
    pub sampleCount: u16,
    #[doc = "!< whether the environment must be mirrored"]
    pub mirror: bool,
    pub reserved: [usize; 3usize],
}
#[test]
fn bindgen_test_layout_filament_Texture_PrefilterOptions() {
    assert_eq!(
        ::std::mem::size_of::<filament_Texture_PrefilterOptions>(),
        32usize,
        concat!("Size of: ", stringify!(filament_Texture_PrefilterOptions))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_Texture_PrefilterOptions>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(filament_Texture_PrefilterOptions)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_Texture_PrefilterOptions>())).sampleCount as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_Texture_PrefilterOptions),
            "::",
            stringify!(sampleCount)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_Texture_PrefilterOptions>())).mirror as *const _
                as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_Texture_PrefilterOptions),
            "::",
            stringify!(mirror)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_Texture_PrefilterOptions>())).reserved as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_Texture_PrefilterOptions),
            "::",
            stringify!(reserved)
        )
    );
}
#[doc = "! Use Builder to construct a Texture object instance"]
#[repr(C)]
#[derive(Debug)]
pub struct filament_Texture_Builder {
    pub _base: filament_BuilderBase<filament_Texture_BuilderDetails>,
}
#[test]
fn bindgen_test_layout_filament_Texture_Builder() {
    assert_eq!(
        ::std::mem::size_of::<filament_Texture_Builder>(),
        8usize,
        concat!("Size of: ", stringify!(filament_Texture_Builder))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_Texture_Builder>(),
        8usize,
        concat!("Alignment of ", stringify!(filament_Texture_Builder))
    );
}
extern "C" {
    #[doc = " Specifies the width in texels of the texture. Doesn't need to be a power-of-two."]
    #[doc = " @param width Width of the texture in texels (default: 1)."]
    #[doc = " @return This Builder, for chaining calls."]
    #[link_name = "\u{1}_ZN8filament7Texture7Builder5widthEj"]
    pub fn filament_Texture_Builder_width(
        this: *mut filament_Texture_Builder,
        width: u32,
    ) -> *mut filament_Texture_Builder;
}
extern "C" {
    #[doc = " Specifies the height in texels of the texture. Doesn't need to be a power-of-two."]
    #[doc = " @param height Height of the texture in texels (default: 1)."]
    #[doc = " @return This Builder, for chaining calls."]
    #[link_name = "\u{1}_ZN8filament7Texture7Builder6heightEj"]
    pub fn filament_Texture_Builder_height(
        this: *mut filament_Texture_Builder,
        height: u32,
    ) -> *mut filament_Texture_Builder;
}
extern "C" {
    #[doc = " Specifies the depth in texels of the texture. Doesn't need to be a power-of-two."]
    #[doc = " The depth controls the number of layers in a 2D array texture. Values greater than 1"]
    #[doc = " effectively create a 3D texture."]
    #[doc = " @param depth Depth of the texture in texels (default: 1)."]
    #[doc = " @return This Builder, for chaining calls."]
    #[doc = " @attention This Texture instance must use Sampler::SAMPLER_3D or Sampler::SAMPLER_2D_ARRAY or it has no effect."]
    #[link_name = "\u{1}_ZN8filament7Texture7Builder5depthEj"]
    pub fn filament_Texture_Builder_depth(
        this: *mut filament_Texture_Builder,
        depth: u32,
    ) -> *mut filament_Texture_Builder;
}
extern "C" {
    #[doc = " Specifies the numbers of mip map levels."]
    #[doc = " This creates a mip-map pyramid. The maximum number of levels a texture can have is"]
    #[doc = " such that max(width, height, level) / 2^MAX_LEVELS = 1"]
    #[doc = " @param levels Number of mipmap levels for this texture."]
    #[doc = " @return This Builder, for chaining calls."]
    #[link_name = "\u{1}_ZN8filament7Texture7Builder6levelsEh"]
    pub fn filament_Texture_Builder_levels(
        this: *mut filament_Texture_Builder,
        levels: u8,
    ) -> *mut filament_Texture_Builder;
}
extern "C" {
    #[doc = " Specifies the type of sampler to use."]
    #[doc = " @param target Sampler type"]
    #[doc = " @return This Builder, for chaining calls."]
    #[doc = " @see Sampler"]
    #[link_name = "\u{1}_ZN8filament7Texture7Builder7samplerENS_7backend11SamplerTypeE"]
    pub fn filament_Texture_Builder_sampler(
        this: *mut filament_Texture_Builder,
        target: filament_Texture_Sampler,
    ) -> *mut filament_Texture_Builder;
}
extern "C" {
    #[doc = " Specifies the *internal* format of this texture."]
    #[doc = ""]
    #[doc = " The internal format specifies how texels are stored (which may be different from how"]
    #[doc = " they're specified in setImage()). InternalFormat specifies both the color components"]
    #[doc = " and the data type used."]
    #[doc = ""]
    #[doc = " @param format Format of the texture's texel."]
    #[doc = " @return This Builder, for chaining calls."]
    #[doc = " @see InternalFormat, setImage"]
    #[link_name = "\u{1}_ZN8filament7Texture7Builder6formatENS_7backend13TextureFormatE"]
    pub fn filament_Texture_Builder_format(
        this: *mut filament_Texture_Builder,
        format: filament_Texture_InternalFormat,
    ) -> *mut filament_Texture_Builder;
}
extern "C" {
    #[doc = " Specifies if the texture will be used as a render target attachment."]
    #[doc = ""]
    #[doc = " If the texture is potentially rendered into, it may require a different memory layout,"]
    #[doc = " which needs to be known during construction."]
    #[doc = ""]
    #[doc = " @param usage Defaults to Texture::Usage::DEFAULT; c.f. Texture::Usage::COLOR_ATTACHMENT."]
    #[doc = " @return This Builder, for chaining calls."]
    #[link_name = "\u{1}_ZN8filament7Texture7Builder5usageENS_7backend12TextureUsageE"]
    pub fn filament_Texture_Builder_usage(
        this: *mut filament_Texture_Builder,
        usage: filament_Texture_Usage,
    ) -> *mut filament_Texture_Builder;
}
extern "C" {
    #[doc = " Specifies how a texture's channels map to color components"]
    #[doc = ""]
    #[doc = " Texture Swizzle is only supported if isTextureSwizzleSupported() returns true."]
    #[doc = ""]
    #[doc = " @param r  texture channel for red component"]
    #[doc = " @param g  texture channel for green component"]
    #[doc = " @param b  texture channel for blue component"]
    #[doc = " @param a  texture channel for alpha component"]
    #[doc = " @return This Builder, for chaining calls."]
    #[doc = " @see Texture::isTextureSwizzleSupported()"]
    #[link_name = "\u{1}_ZN8filament7Texture7Builder7swizzleENS_7backend14TextureSwizzleES3_S3_S3_"]
    pub fn filament_Texture_Builder_swizzle(
        this: *mut filament_Texture_Builder,
        r: filament_Texture_Swizzle,
        g: filament_Texture_Swizzle,
        b: filament_Texture_Swizzle,
        a: filament_Texture_Swizzle,
    ) -> *mut filament_Texture_Builder;
}
extern "C" {
    #[doc = " Creates the Texture object and returns a pointer to it."]
    #[doc = ""]
    #[doc = " @param engine Reference to the filament::Engine to associate this Texture with."]
    #[doc = ""]
    #[doc = " @return pointer to the newly created object or nullptr if exceptions are disabled and"]
    #[doc = "         an error occurred."]
    #[doc = ""]
    #[doc = " @exception utils::PostConditionPanic if a runtime error occurred, such as running out of"]
    #[doc = "            memory or other resources."]
    #[doc = " @exception utils::PreConditionPanic if a parameter to a builder function was invalid."]
    #[link_name = "\u{1}_ZN8filament7Texture7Builder5buildERNS_6EngineE"]
    pub fn filament_Texture_Builder_build(
        this: *mut filament_Texture_Builder,
        engine: *mut filament_Engine,
    ) -> *mut filament_Texture;
}
extern "C" {
    #[doc = " Specify a native texture to import as a Filament texture."]
    #[doc = ""]
    #[doc = " The texture id is backend-specific:"]
    #[doc = "   - OpenGL: GLuint texture ID"]
    #[doc = "   - Metal: id<MTLTexture>"]
    #[doc = ""]
    #[doc = " With Metal, the id<MTLTexture> object should be cast to an intptr_t using"]
    #[doc = " CFBridgingRetain to transfer ownership to Filament. Filament will release ownership of"]
    #[doc = " the texture object when the Filament texture is destroyed."]
    #[doc = ""]
    #[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~{.cpp}"]
    #[doc = "  id <MTLTexture> metalTexture = ..."]
    #[doc = "  filamentTexture->import((intptr_t) CFBridgingRetain(metalTexture));"]
    #[doc = "  // free to release metalTexture"]
    #[doc = ""]
    #[doc = "  // after using texture:"]
    #[doc = "  engine->destroy(filamentTexture);   // metalTexture is released"]
    #[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
    #[doc = ""]
    #[doc = " @warning This method should be used as a last resort. This API is subject to change or"]
    #[doc = " removal."]
    #[doc = ""]
    #[doc = " @param id a backend specific texture identifier"]
    #[doc = ""]
    #[doc = " @return This Builder, for chaining calls."]
    #[link_name = "\u{1}_ZN8filament7Texture7Builder6importEl"]
    pub fn filament_Texture_Builder_import(
        this: *mut filament_Texture_Builder,
        id: isize,
    ) -> *mut filament_Texture_Builder;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament7Texture7BuilderC1Ev"]
    pub fn filament_Texture_Builder_Builder(this: *mut filament_Texture_Builder);
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament7Texture7BuilderC1ERKS1_"]
    pub fn filament_Texture_Builder_Builder1(
        this: *mut filament_Texture_Builder,
        rhs: *const filament_Texture_Builder,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament7Texture7BuilderC1EOS1_"]
    pub fn filament_Texture_Builder_Builder2(
        this: *mut filament_Texture_Builder,
        rhs: *mut filament_Texture_Builder,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament7Texture7BuilderD1Ev"]
    pub fn filament_Texture_Builder_Builder_destructor(this: *mut filament_Texture_Builder);
}
impl Default for filament_Texture_Builder {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl filament_Texture_Builder {
    #[inline]
    pub unsafe fn width(&mut self, width: u32) -> *mut filament_Texture_Builder {
        filament_Texture_Builder_width(self, width)
    }
    #[inline]
    pub unsafe fn height(&mut self, height: u32) -> *mut filament_Texture_Builder {
        filament_Texture_Builder_height(self, height)
    }
    #[inline]
    pub unsafe fn depth(&mut self, depth: u32) -> *mut filament_Texture_Builder {
        filament_Texture_Builder_depth(self, depth)
    }
    #[inline]
    pub unsafe fn levels(&mut self, levels: u8) -> *mut filament_Texture_Builder {
        filament_Texture_Builder_levels(self, levels)
    }
    #[inline]
    pub unsafe fn sampler(
        &mut self,
        target: filament_Texture_Sampler,
    ) -> *mut filament_Texture_Builder {
        filament_Texture_Builder_sampler(self, target)
    }
    #[inline]
    pub unsafe fn format(
        &mut self,
        format: filament_Texture_InternalFormat,
    ) -> *mut filament_Texture_Builder {
        filament_Texture_Builder_format(self, format)
    }
    #[inline]
    pub unsafe fn usage(&mut self, usage: filament_Texture_Usage) -> *mut filament_Texture_Builder {
        filament_Texture_Builder_usage(self, usage)
    }
    #[inline]
    pub unsafe fn swizzle(
        &mut self,
        r: filament_Texture_Swizzle,
        g: filament_Texture_Swizzle,
        b: filament_Texture_Swizzle,
        a: filament_Texture_Swizzle,
    ) -> *mut filament_Texture_Builder {
        filament_Texture_Builder_swizzle(self, r, g, b, a)
    }
    #[inline]
    pub unsafe fn build(&mut self, engine: *mut filament_Engine) -> *mut filament_Texture {
        filament_Texture_Builder_build(self, engine)
    }
    #[inline]
    pub unsafe fn import(&mut self, id: isize) -> *mut filament_Texture_Builder {
        filament_Texture_Builder_import(self, id)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_Texture_Builder_Builder(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(rhs: *const filament_Texture_Builder) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_Texture_Builder_Builder1(__bindgen_tmp.as_mut_ptr(), rhs);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new2(rhs: *mut filament_Texture_Builder) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_Texture_Builder_Builder2(__bindgen_tmp.as_mut_ptr(), rhs);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        filament_Texture_Builder_Builder_destructor(self)
    }
}
pub const filament_Texture_BASE_LEVEL: size_t = 0;
#[test]
fn bindgen_test_layout_filament_Texture() {
    assert_eq!(
        ::std::mem::size_of::<filament_Texture>(),
        1usize,
        concat!("Size of: ", stringify!(filament_Texture))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_Texture>(),
        1usize,
        concat!("Alignment of ", stringify!(filament_Texture))
    );
}
extern "C" {
    #[doc = " @return whether a backend supports a particular format."]
    #[link_name = "\u{1}_ZN8filament7Texture24isTextureFormatSupportedERNS_6EngineENS_7backend13TextureFormatE"]
    pub fn filament_Texture_isTextureFormatSupported(
        engine: *mut filament_Engine,
        format: filament_Texture_InternalFormat,
    ) -> bool;
}
extern "C" {
    #[doc = " @return whether a backend supports texture swizzling."]
    #[link_name = "\u{1}_ZN8filament7Texture25isTextureSwizzleSupportedERNS_6EngineE"]
    pub fn filament_Texture_isTextureSwizzleSupported(engine: *mut filament_Engine) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament7Texture22computeTextureDataSizeENS_7backend15PixelDataFormatENS1_13PixelDataTypeEmmm"]
    pub fn filament_Texture_computeTextureDataSize(
        format: filament_Texture_Format,
        type_: filament_Texture_Type,
        stride: size_t,
        height: size_t,
        alignment: size_t,
    ) -> size_t;
}
extern "C" {
    #[doc = " Returns the width of a 2D or 3D texture level"]
    #[doc = " @param level texture level."]
    #[doc = " @return Width in texel of the specified \\p level, clamped to 1."]
    #[doc = " @attention If this texture is using Sampler::SAMPLER_EXTERNAL, the dimension"]
    #[doc = " of the texture are unknown and this method always returns whatever was set on the Builder."]
    #[link_name = "\u{1}_ZNK8filament7Texture8getWidthEm"]
    pub fn filament_Texture_getWidth(this: *const filament_Texture, level: size_t) -> size_t;
}
extern "C" {
    #[doc = " Returns the height of a 2D or 3D texture level"]
    #[doc = " @param level texture level."]
    #[doc = " @return Height in texel of the specified \\p level, clamped to 1."]
    #[doc = " @attention If this texture is using Sampler::SAMPLER_EXTERNAL, the dimension"]
    #[doc = " of the texture are unknown and this method always returns whatever was set on the Builder."]
    #[link_name = "\u{1}_ZNK8filament7Texture9getHeightEm"]
    pub fn filament_Texture_getHeight(this: *const filament_Texture, level: size_t) -> size_t;
}
extern "C" {
    #[doc = " Returns the depth of a 3D texture level"]
    #[doc = " @param level texture level."]
    #[doc = " @return Depth in texel of the specified \\p level, clamped to 1."]
    #[doc = " @attention If this texture is using Sampler::SAMPLER_EXTERNAL, the dimension"]
    #[doc = " of the texture are unknown and this method always returns whatever was set on the Builder."]
    #[link_name = "\u{1}_ZNK8filament7Texture8getDepthEm"]
    pub fn filament_Texture_getDepth(this: *const filament_Texture, level: size_t) -> size_t;
}
extern "C" {
    #[doc = " Returns the maximum number of levels this texture can have."]
    #[doc = " @return maximum number of levels this texture can have."]
    #[doc = " @attention If this texture is using Sampler::SAMPLER_EXTERNAL, the dimension"]
    #[doc = " of the texture are unknown and this method always returns whatever was set on the Builder."]
    #[link_name = "\u{1}_ZNK8filament7Texture9getLevelsEv"]
    pub fn filament_Texture_getLevels(this: *const filament_Texture) -> size_t;
}
extern "C" {
    #[doc = " Return this texture Sampler as set by Builder::sampler()."]
    #[doc = " @return this texture Sampler as set by Builder::sampler()"]
    #[link_name = "\u{1}_ZNK8filament7Texture9getTargetEv"]
    pub fn filament_Texture_getTarget(this: *const filament_Texture) -> filament_Texture_Sampler;
}
extern "C" {
    #[doc = " Return this texture InternalFormat as set by Builder::format()."]
    #[doc = " @return this texture InternalFormat as set by Builder::format()."]
    #[link_name = "\u{1}_ZNK8filament7Texture9getFormatEv"]
    pub fn filament_Texture_getFormat(
        this: *const filament_Texture,
    ) -> filament_Texture_InternalFormat;
}
extern "C" {
    #[doc = " Specify the image of a 2D texture for a level."]
    #[doc = ""]
    #[doc = " @param engine    Engine this texture is associated to."]
    #[doc = " @param level     Level to set the image for."]
    #[doc = " @param buffer    Client-side buffer containing the image to set."]
    #[doc = ""]
    #[doc = " @attention \\p engine must be the instance passed to Builder::build()"]
    #[doc = " @attention \\p level must be less than getLevels()."]
    #[doc = " @attention \\p buffer's Texture::Format must match that of getFormat()."]
    #[doc = " @attention This Texture instance must use Sampler::SAMPLER_2D or"]
    #[doc = "            Sampler::SAMPLER_EXTERNAL. IF the later is specified"]
    #[doc = "            and external textures are supported by the driver implementation,"]
    #[doc = "            this method will have no effect, otherwise it will behave as if the"]
    #[doc = "            texture was specified with driver::SamplerType::SAMPLER_2D."]
    #[doc = ""]
    #[doc = " @note"]
    #[doc = " This is equivalent to calling:"]
    #[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
    #[doc = " setImage(engine, level, 0, 0, getWidth(level), getHeight(level), buffer);"]
    #[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
    #[doc = ""]
    #[doc = " @see Builder::sampler()"]
    #[link_name = "\u{1}_ZNK8filament7Texture8setImageERNS_6EngineEmONS_7backend21PixelBufferDescriptorE"]
    pub fn filament_Texture_setImage(
        this: *const filament_Texture,
        engine: *mut filament_Engine,
        level: size_t,
        buffer: *mut filament_Texture_PixelBufferDescriptor,
    );
}
extern "C" {
    #[doc = " Updates a sub-image of a 2D texture for a level."]
    #[doc = ""]
    #[doc = " @param engine    Engine this texture is associated to."]
    #[doc = " @param level     Level to set the image for."]
    #[doc = " @param xoffset   Left offset of the sub-region to update."]
    #[doc = " @param yoffset   Bottom offset of the sub-region to update."]
    #[doc = " @param width     Width of the sub-region to update."]
    #[doc = " @param height    Height of the sub-region to update."]
    #[doc = " @param buffer    Client-side buffer containing the image to set."]
    #[doc = ""]
    #[doc = " @attention \\p engine must be the instance passed to Builder::build()"]
    #[doc = " @attention \\p level must be less than getLevels()."]
    #[doc = " @attention \\p buffer's Texture::Format must match that of getFormat()."]
    #[doc = " @attention This Texture instance must use Sampler::SAMPLER_2D or"]
    #[doc = "            Sampler::SAMPLER_EXTERNAL. IF the later is specified"]
    #[doc = "            and external textures are supported by the driver implementation,"]
    #[doc = "            this method will have no effect, otherwise it will behave as if the"]
    #[doc = "            texture was specified with Sampler::SAMPLER_2D."]
    #[doc = ""]
    #[doc = " @see Builder::sampler()"]
    #[link_name = "\u{1}_ZNK8filament7Texture8setImageERNS_6EngineEmjjjjONS_7backend21PixelBufferDescriptorE"]
    pub fn filament_Texture_setImage1(
        this: *const filament_Texture,
        engine: *mut filament_Engine,
        level: size_t,
        xoffset: u32,
        yoffset: u32,
        width: u32,
        height: u32,
        buffer: *mut filament_Texture_PixelBufferDescriptor,
    );
}
extern "C" {
    #[doc = " Updates a sub-image of a 3D texture or 2D texture array for a level."]
    #[doc = ""]
    #[doc = " @param engine    Engine this texture is associated to."]
    #[doc = " @param level     Level to set the image for."]
    #[doc = " @param xoffset   Left offset of the sub-region to update."]
    #[doc = " @param yoffset   Bottom offset of the sub-region to update."]
    #[doc = " @param zoffset   Depth offset of the sub-region to update."]
    #[doc = " @param width     Width of the sub-region to update."]
    #[doc = " @param height    Height of the sub-region to update."]
    #[doc = " @param depth     Depth of the sub-region to update."]
    #[doc = " @param buffer    Client-side buffer containing the image to set."]
    #[doc = ""]
    #[doc = " @attention \\p engine must be the instance passed to Builder::build()"]
    #[doc = " @attention \\p level must be less than getLevels()."]
    #[doc = " @attention \\p buffer's Texture::Format must match that of getFormat()."]
    #[doc = " @attention This Texture instance must use Sampler::SAMPLER_3D or Sampler::SAMPLER_2D_array."]
    #[doc = ""]
    #[doc = " @see Builder::sampler()"]
    #[link_name = "\u{1}_ZNK8filament7Texture8setImageERNS_6EngineEmjjjjjjONS_7backend21PixelBufferDescriptorE"]
    pub fn filament_Texture_setImage2(
        this: *const filament_Texture,
        engine: *mut filament_Engine,
        level: size_t,
        xoffset: u32,
        yoffset: u32,
        zoffset: u32,
        width: u32,
        height: u32,
        depth: u32,
        buffer: *mut filament_Texture_PixelBufferDescriptor,
    );
}
extern "C" {
    #[doc = " Specify all six images of a cube map level."]
    #[doc = ""]
    #[doc = " This method follows exactly the OpenGL conventions."]
    #[doc = ""]
    #[doc = " @param engine        Engine this texture is associated to."]
    #[doc = " @param level         Level to set the image for."]
    #[doc = " @param buffer        Client-side buffer containing the images to set."]
    #[doc = " @param faceOffsets   Offsets in bytes into \\p buffer for all six images. The offsets"]
    #[doc = "                      are specified in the following order: +x, -x, +y, -y, +z, -z"]
    #[doc = ""]
    #[doc = " @attention \\p engine must be the instance passed to Builder::build()"]
    #[doc = " @attention \\p level must be less than getLevels()."]
    #[doc = " @attention \\p buffer's Texture::Format must match that of getFormat()."]
    #[doc = " @attention This Texture instance must use Sampler::SAMPLER_CUBEMAP or it has no effect"]
    #[doc = ""]
    #[doc = " @see Texture::CubemapFace, Builder::sampler()"]
    #[link_name = "\u{1}_ZNK8filament7Texture8setImageERNS_6EngineEmONS_7backend21PixelBufferDescriptorERKNS3_11FaceOffsetsE"]
    pub fn filament_Texture_setImage3(
        this: *const filament_Texture,
        engine: *mut filament_Engine,
        level: size_t,
        buffer: *mut filament_Texture_PixelBufferDescriptor,
        faceOffsets: *const filament_Texture_FaceOffsets,
    );
}
extern "C" {
    #[doc = " Specify the external image to associate with this Texture. Typically the external"]
    #[doc = " image is OS specific, and can be a video or camera frame."]
    #[doc = " There are many restrictions when using an external image as a texture, such as:"]
    #[doc = "   - only the level of detail (lod) 0 can be specified"]
    #[doc = "   - only nearest or linear filtering is supported"]
    #[doc = "   - the size and format of the texture is defined by the external image"]
    #[doc = "   - only the CLAMP_TO_EDGE wrap mode is supported"]
    #[doc = ""]
    #[doc = " @param engine        Engine this texture is associated to."]
    #[doc = " @param image         An opaque handle to a platform specific image. Supported types are"]
    #[doc = "                      eglImageOES on Android and CVPixelBufferRef on iOS."]
    #[doc = ""]
    #[doc = "                      On iOS the following pixel formats are supported:"]
    #[doc = "                        - kCVPixelFormatType_32BGRA"]
    #[doc = "                        - kCVPixelFormatType_420YpCbCr8BiPlanarFullRange"]
    #[doc = ""]
    #[doc = " @attention \\p engine must be the instance passed to Builder::build()"]
    #[doc = " @attention This Texture instance must use Sampler::SAMPLER_EXTERNAL or it has no effect"]
    #[doc = ""]
    #[doc = " @see Builder::sampler()"]
    #[doc = ""]
    #[link_name = "\u{1}_ZN8filament7Texture16setExternalImageERNS_6EngineEPv"]
    pub fn filament_Texture_setExternalImage(
        this: *mut filament_Texture,
        engine: *mut filament_Engine,
        image: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    #[doc = " Specify the external image and plane to associate with this Texture. Typically the external"]
    #[doc = " image is OS specific, and can be a video or camera frame. When using this method, the"]
    #[doc = " external image must be a planar type (such as a YUV camera frame). The plane parameter"]
    #[doc = " selects which image plane is bound to this texture."]
    #[doc = ""]
    #[doc = " A single external image can be bound to different Filament textures, with each texture"]
    #[doc = " associated with a separate plane:"]
    #[doc = ""]
    #[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
    #[doc = " textureA->setExternalImage(engine, image, 0);"]
    #[doc = " textureB->setExternalImage(engine, image, 1);"]
    #[doc = " ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"]
    #[doc = ""]
    #[doc = " There are many restrictions when using an external image as a texture, such as:"]
    #[doc = "   - only the level of detail (lod) 0 can be specified"]
    #[doc = "   - only nearest or linear filtering is supported"]
    #[doc = "   - the size and format of the texture is defined by the external image"]
    #[doc = "   - only the CLAMP_TO_EDGE wrap mode is supported"]
    #[doc = ""]
    #[doc = " @param engine        Engine this texture is associated to."]
    #[doc = " @param image         An opaque handle to a platform specific image. Supported types are"]
    #[doc = "                      eglImageOES on Android and CVPixelBufferRef on iOS."]
    #[doc = " @param plane         The plane index of the external image to associate with this texture."]
    #[doc = ""]
    #[doc = "                      This method is only meaningful on iOS with"]
    #[doc = "                      kCVPixelFormatType_420YpCbCr8BiPlanarFullRange images. On platforms"]
    #[doc = "                      other than iOS, this method is a no-op."]
    #[link_name = "\u{1}_ZN8filament7Texture16setExternalImageERNS_6EngineEPvm"]
    pub fn filament_Texture_setExternalImage1(
        this: *mut filament_Texture,
        engine: *mut filament_Engine,
        image: *mut ::std::os::raw::c_void,
        plane: size_t,
    );
}
extern "C" {
    #[doc = " Specify the external stream to associate with this Texture. Typically the external"]
    #[doc = " stream is OS specific, and can be a video or camera stream."]
    #[doc = " There are many restrictions when using an external stream as a texture, such as:"]
    #[doc = "   - only the level of detail (lod) 0 can be specified"]
    #[doc = "   - only nearest or linear filtering is supported"]
    #[doc = "   - the size and format of the texture is defined by the external stream"]
    #[doc = ""]
    #[doc = " @param engine        Engine this texture is associated to."]
    #[doc = " @param stream        A Stream object"]
    #[doc = ""]
    #[doc = " @attention \\p engine must be the instance passed to Builder::build()"]
    #[doc = " @attention This Texture instance must use Sampler::SAMPLER_EXTERNAL or it has no effect"]
    #[doc = ""]
    #[doc = " @see Builder::sampler(), Stream"]
    #[doc = ""]
    #[link_name = "\u{1}_ZN8filament7Texture17setExternalStreamERNS_6EngineEPNS_6StreamE"]
    pub fn filament_Texture_setExternalStream(
        this: *mut filament_Texture,
        engine: *mut filament_Engine,
        stream: *mut filament_Stream,
    );
}
extern "C" {
    #[doc = " Generates all the mipmap levels automatically. This requires the texture to have a"]
    #[doc = " color-renderable format."]
    #[doc = ""]
    #[doc = " @param engine        Engine this texture is associated to."]
    #[doc = ""]
    #[doc = " @attention \\p engine must be the instance passed to Builder::build()"]
    #[doc = " @attention This Texture instance must NOT use Sampler::SAMPLER_CUBEMAP or it has no effect"]
    #[link_name = "\u{1}_ZNK8filament7Texture15generateMipmapsERNS_6EngineE"]
    pub fn filament_Texture_generateMipmaps(
        this: *const filament_Texture,
        engine: *mut filament_Engine,
    );
}
extern "C" {
    #[doc = " Creates a reflection map from an environment map."]
    #[doc = ""]
    #[doc = " This is a utility function that replaces calls to Texture::setImage()."]
    #[doc = " The provided environment map is processed and all mipmap levels are populated. The"]
    #[doc = " processing is similar to the offline tool `cmgen` as a lower quality setting."]
    #[doc = ""]
    #[doc = " This function is intended to be used when the environment cannot be processed offline,"]
    #[doc = " for instance if it's generated at runtime."]
    #[doc = ""]
    #[doc = " The source data must obey to some constraints:"]
    #[doc = "   - the data type must be PixelDataFormat::RGB"]
    #[doc = "   - the data format must be one of"]
    #[doc = "          - PixelDataType::FLOAT"]
    #[doc = "          - PixelDataType::HALF"]
    #[doc = ""]
    #[doc = " The current texture must be a cubemap"]
    #[doc = ""]
    #[doc = " The reflections cubemap's internal format cannot be a compressed format."]
    #[doc = ""]
    #[doc = " The reflections cubemap's dimension must be a power-of-two."]
    #[doc = ""]
    #[doc = " @warning This operation is computationally intensive, especially with large environments and"]
    #[doc = "          is currently synchronous. Expect about 1ms for a 16x16 cubemap."]
    #[doc = ""]
    #[doc = " @param engine        Reference to the filament::Engine to associate this IndirectLight with."]
    #[doc = " @param buffer        Client-side buffer containing the images to set."]
    #[doc = " @param faceOffsets   Offsets in bytes into \\p buffer for all six images. The offsets"]
    #[doc = "                      are specified in the following order: +x, -x, +y, -y, +z, -z"]
    #[doc = " @param options       Optional parameter to controlling user-specified quality and options."]
    #[doc = ""]
    #[doc = " @exception utils::PreConditionPanic if the source data constraints are not respected."]
    #[doc = ""]
    #[link_name = "\u{1}_ZN8filament7Texture23generatePrefilterMipmapERNS_6EngineEONS_7backend21PixelBufferDescriptorERKNS3_11FaceOffsetsEPKNS0_16PrefilterOptionsE"]
    pub fn filament_Texture_generatePrefilterMipmap(
        this: *mut filament_Texture,
        engine: *mut filament_Engine,
        buffer: *mut filament_Texture_PixelBufferDescriptor,
        faceOffsets: *const filament_Texture_FaceOffsets,
        options: *const filament_Texture_PrefilterOptions,
    );
}
impl filament_Texture {
    #[inline]
    pub unsafe fn isTextureFormatSupported(
        engine: *mut filament_Engine,
        format: filament_Texture_InternalFormat,
    ) -> bool {
        filament_Texture_isTextureFormatSupported(engine, format)
    }
    #[inline]
    pub unsafe fn isTextureSwizzleSupported(engine: *mut filament_Engine) -> bool {
        filament_Texture_isTextureSwizzleSupported(engine)
    }
    #[inline]
    pub unsafe fn computeTextureDataSize(
        format: filament_Texture_Format,
        type_: filament_Texture_Type,
        stride: size_t,
        height: size_t,
        alignment: size_t,
    ) -> size_t {
        filament_Texture_computeTextureDataSize(format, type_, stride, height, alignment)
    }
    #[inline]
    pub unsafe fn getWidth(&self, level: size_t) -> size_t {
        filament_Texture_getWidth(self, level)
    }
    #[inline]
    pub unsafe fn getHeight(&self, level: size_t) -> size_t {
        filament_Texture_getHeight(self, level)
    }
    #[inline]
    pub unsafe fn getDepth(&self, level: size_t) -> size_t {
        filament_Texture_getDepth(self, level)
    }
    #[inline]
    pub unsafe fn getLevels(&self) -> size_t {
        filament_Texture_getLevels(self)
    }
    #[inline]
    pub unsafe fn getTarget(&self) -> filament_Texture_Sampler {
        filament_Texture_getTarget(self)
    }
    #[inline]
    pub unsafe fn getFormat(&self) -> filament_Texture_InternalFormat {
        filament_Texture_getFormat(self)
    }
    #[inline]
    pub unsafe fn setImage(
        &self,
        engine: *mut filament_Engine,
        level: size_t,
        buffer: *mut filament_Texture_PixelBufferDescriptor,
    ) {
        filament_Texture_setImage(self, engine, level, buffer)
    }
    #[inline]
    pub unsafe fn setImage1(
        &self,
        engine: *mut filament_Engine,
        level: size_t,
        xoffset: u32,
        yoffset: u32,
        width: u32,
        height: u32,
        buffer: *mut filament_Texture_PixelBufferDescriptor,
    ) {
        filament_Texture_setImage1(self, engine, level, xoffset, yoffset, width, height, buffer)
    }
    #[inline]
    pub unsafe fn setImage2(
        &self,
        engine: *mut filament_Engine,
        level: size_t,
        xoffset: u32,
        yoffset: u32,
        zoffset: u32,
        width: u32,
        height: u32,
        depth: u32,
        buffer: *mut filament_Texture_PixelBufferDescriptor,
    ) {
        filament_Texture_setImage2(
            self, engine, level, xoffset, yoffset, zoffset, width, height, depth, buffer,
        )
    }
    #[inline]
    pub unsafe fn setImage3(
        &self,
        engine: *mut filament_Engine,
        level: size_t,
        buffer: *mut filament_Texture_PixelBufferDescriptor,
        faceOffsets: *const filament_Texture_FaceOffsets,
    ) {
        filament_Texture_setImage3(self, engine, level, buffer, faceOffsets)
    }
    #[inline]
    pub unsafe fn setExternalImage(
        &mut self,
        engine: *mut filament_Engine,
        image: *mut ::std::os::raw::c_void,
    ) {
        filament_Texture_setExternalImage(self, engine, image)
    }
    #[inline]
    pub unsafe fn setExternalImage1(
        &mut self,
        engine: *mut filament_Engine,
        image: *mut ::std::os::raw::c_void,
        plane: size_t,
    ) {
        filament_Texture_setExternalImage1(self, engine, image, plane)
    }
    #[inline]
    pub unsafe fn setExternalStream(
        &mut self,
        engine: *mut filament_Engine,
        stream: *mut filament_Stream,
    ) {
        filament_Texture_setExternalStream(self, engine, stream)
    }
    #[inline]
    pub unsafe fn generateMipmaps(&self, engine: *mut filament_Engine) {
        filament_Texture_generateMipmaps(self, engine)
    }
    #[inline]
    pub unsafe fn generatePrefilterMipmap(
        &mut self,
        engine: *mut filament_Engine,
        buffer: *mut filament_Texture_PixelBufferDescriptor,
        faceOffsets: *const filament_Texture_FaceOffsets,
        options: *const filament_Texture_PrefilterOptions,
    ) {
        filament_Texture_generatePrefilterMipmap(self, engine, buffer, faceOffsets, options)
    }
}
#[doc = " TextureSampler defines how a texture is accessed."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct filament_TextureSampler {
    pub mSamplerParams: filament_backend_SamplerParams,
}
#[doc = "! comparison function for the depth sampler"]
pub use self::filament_backend_SamplerCompareFunc as filament_TextureSampler_CompareFunc;
#[doc = "! Sampler compare mode"]
pub use self::filament_backend_SamplerCompareMode as filament_TextureSampler_CompareMode;
#[doc = "! Sampler magnification filter"]
pub use self::filament_backend_SamplerMagFilter as filament_TextureSampler_MagFilter;
#[doc = "! Sampler minification filter"]
pub use self::filament_backend_SamplerMinFilter as filament_TextureSampler_MinFilter;
#[doc = "! Sampler Wrap mode"]
pub use self::filament_backend_SamplerWrapMode as filament_TextureSampler_WrapMode;
#[test]
fn bindgen_test_layout_filament_TextureSampler() {
    assert_eq!(
        ::std::mem::size_of::<filament_TextureSampler>(),
        4usize,
        concat!("Size of: ", stringify!(filament_TextureSampler))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_TextureSampler>(),
        4usize,
        concat!("Alignment of ", stringify!(filament_TextureSampler))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_TextureSampler>())).mSamplerParams as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_TextureSampler),
            "::",
            stringify!(mSamplerParams)
        )
    );
}
impl Default for filament_TextureSampler {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " TransformManager is used to add transform components to entities."]
#[doc = ""]
#[doc = " A Transform component gives an entity a position and orientation in space in the coordinate"]
#[doc = " space of its parent transform. The TransformManager takes care of computing the world-space"]
#[doc = " transform of each component (i.e. its transform relative to the root)."]
#[doc = ""]
#[doc = " Creation and destruction"]
#[doc = " ========================"]
#[doc = ""]
#[doc = " A transform component is created using TransformManager::create() and destroyed by calling"]
#[doc = " TransformManager::destroy()."]
#[doc = ""]
#[doc = " ~~~~~~~~~~~{.cpp}"]
#[doc = "  filament::Engine* engine = filament::Engine::create();"]
#[doc = "  utils::Entity object = utils::EntityManager.get().create();"]
#[doc = ""]
#[doc = "  auto& tcm = engine->getTransformManager();"]
#[doc = ""]
#[doc = "  // create the transform component"]
#[doc = "  tcm.create(object);"]
#[doc = ""]
#[doc = "  // set its transform"]
#[doc = "  auto i = tcm.getInstance(object);"]
#[doc = "  tcm.setTransform(i, mat4f::translation({ 0, 0, -1 }));"]
#[doc = ""]
#[doc = "  // destroy the transform component"]
#[doc = "  tcm.destroy(object);"]
#[doc = " ~~~~~~~~~~~"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Default)]
pub struct filament_TransformManager {
    pub _address: u8,
}
pub type filament_TransformManager_Instance = u32;
#[repr(C)]
#[derive(Debug)]
pub struct filament_TransformManager_children_iterator {
    pub mManager: *const filament_TransformManager,
    pub mInstance: filament_TransformManager_Instance,
}
#[test]
fn bindgen_test_layout_filament_TransformManager_children_iterator() {
    assert_eq!(
        ::std::mem::size_of::<filament_TransformManager_children_iterator>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(filament_TransformManager_children_iterator)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_TransformManager_children_iterator>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(filament_TransformManager_children_iterator)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_TransformManager_children_iterator>())).mManager
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_TransformManager_children_iterator),
            "::",
            stringify!(mManager)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_TransformManager_children_iterator>())).mInstance
                as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_TransformManager_children_iterator),
            "::",
            stringify!(mInstance)
        )
    );
}
impl Default for filament_TransformManager_children_iterator {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn bindgen_test_layout_filament_TransformManager() {
    assert_eq!(
        ::std::mem::size_of::<filament_TransformManager>(),
        1usize,
        concat!("Size of: ", stringify!(filament_TransformManager))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_TransformManager>(),
        1usize,
        concat!("Alignment of ", stringify!(filament_TransformManager))
    );
}
extern "C" {
    #[doc = " Returns whether a particular Entity is associated with a component of this TransformManager"]
    #[doc = " @param e An Entity."]
    #[doc = " @return true if this Entity has a component associated with this manager."]
    #[link_name = "\u{1}_ZNK8filament16TransformManager12hasComponentEN5utils6EntityE"]
    pub fn filament_TransformManager_hasComponent(
        this: *const filament_TransformManager,
        e: utils_Entity,
    ) -> bool;
}
extern "C" {
    #[doc = " Gets an Instance representing the transform component associated with the given Entity."]
    #[doc = " @param e An Entity."]
    #[doc = " @return An Instance object, which represents the transform component associated with the Entity e."]
    #[doc = " @note Use Instance::isValid() to make sure the component exists."]
    #[doc = " @see hasComponent()"]
    #[link_name = "\u{1}_ZNK8filament16TransformManager11getInstanceEN5utils6EntityE"]
    pub fn filament_TransformManager_getInstance(
        this: *const filament_TransformManager,
        e: utils_Entity,
    ) -> filament_TransformManager_Instance;
}
extern "C" {
    #[doc = " Enables or disable the accurate translation mode. Disabled by default."]
    #[doc = ""]
    #[doc = " When accurate translation mode is active, the translation component of all transforms is"]
    #[doc = " maintained at double precision. This is only useful if the mat4 version of setTransform()"]
    #[doc = " is used, as well as getTransformAccurate()."]
    #[doc = ""]
    #[doc = " @param enable true to enable the accurate translation mode, false to disable."]
    #[doc = ""]
    #[doc = " @see isAccurateTranslationsEnabled"]
    #[doc = " @see create(utils::Entity, Instance, const math::mat4&);"]
    #[doc = " @see setTransform(Instance, const math::mat4&)"]
    #[doc = " @see getTransformAccurate"]
    #[doc = " @see getWorldTransformAccurate"]
    #[link_name = "\u{1}_ZN8filament16TransformManager30setAccurateTranslationsEnabledEb"]
    pub fn filament_TransformManager_setAccurateTranslationsEnabled(
        this: *mut filament_TransformManager,
        enable: bool,
    );
}
extern "C" {
    #[doc = " Returns whether the high precision translation mode is active."]
    #[doc = " @return true if accurate translations mode is active, false otherwise"]
    #[doc = " @see setAccurateTranslationsEnabled"]
    #[link_name = "\u{1}_ZNK8filament16TransformManager29isAccurateTranslationsEnabledEv"]
    pub fn filament_TransformManager_isAccurateTranslationsEnabled(
        this: *const filament_TransformManager,
    ) -> bool;
}
extern "C" {
    #[doc = " Creates a transform component and associate it with the given entity."]
    #[doc = " @param entity            An Entity to associate a transform component to."]
    #[doc = " @param parent            The Instance of the parent transform, or Instance{} if no parent."]
    #[doc = " @param localTransform    The transform to initialize the transform component with."]
    #[doc = "                          This is always relative to the parent."]
    #[doc = ""]
    #[doc = " If this component already exists on the given entity, it is first destroyed as if"]
    #[doc = " destroy(utils::Entity e) was called."]
    #[doc = ""]
    #[doc = " @see destroy()"]
    #[link_name = "\u{1}_ZN8filament16TransformManager6createEN5utils6EntityENS1_14EntityInstanceIS0_Lb0EEERKNS_4math7details6TMat44IfEE"]
    pub fn filament_TransformManager_create(
        this: *mut filament_TransformManager,
        entity: utils_Entity,
        parent: filament_TransformManager_Instance,
        localTransform: *const filament_math_mat4f,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament16TransformManager6createEN5utils6EntityENS1_14EntityInstanceIS0_Lb0EEERKNS_4math7details6TMat44IdEE"]
    pub fn filament_TransformManager_create1(
        this: *mut filament_TransformManager,
        entity: utils_Entity,
        parent: filament_TransformManager_Instance,
        localTransform: *const filament_math_mat4,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament16TransformManager6createEN5utils6EntityENS1_14EntityInstanceIS0_Lb0EEE"]
    pub fn filament_TransformManager_create2(
        this: *mut filament_TransformManager,
        entity: utils_Entity,
        parent: filament_TransformManager_Instance,
    );
}
extern "C" {
    #[doc = " Destroys this component from the given entity, children are orphaned."]
    #[doc = " @param e An entity."]
    #[doc = ""]
    #[doc = " @note If this transform had children, these are orphaned, which means their local"]
    #[doc = " transform becomes a world transform. Usually it's nonsensical. It's recommended to make"]
    #[doc = " sure that a destroyed transform doesn't have children."]
    #[doc = ""]
    #[doc = " @see create()"]
    #[link_name = "\u{1}_ZN8filament16TransformManager7destroyEN5utils6EntityE"]
    pub fn filament_TransformManager_destroy(this: *mut filament_TransformManager, e: utils_Entity);
}
extern "C" {
    #[doc = " Re-parents an entity to a new one."]
    #[doc = " @param i             The instance of the transform component to re-parent"]
    #[doc = " @param newParent     The instance of the new parent transform"]
    #[doc = " @attention It is an error to re-parent an entity to a descendant and will cause undefined behaviour."]
    #[doc = " @see getInstance()"]
    #[link_name = "\u{1}_ZN8filament16TransformManager9setParentEN5utils14EntityInstanceIS0_Lb0EEES3_"]
    pub fn filament_TransformManager_setParent(
        this: *mut filament_TransformManager,
        i: filament_TransformManager_Instance,
        newParent: filament_TransformManager_Instance,
    );
}
extern "C" {
    #[doc = " Returns the parent of a transform component, or the null entity if it is a root."]
    #[doc = " @param i The instance of the transform component to query."]
    #[link_name = "\u{1}_ZNK8filament16TransformManager9getParentEN5utils14EntityInstanceIS0_Lb0EEE"]
    pub fn filament_TransformManager_getParent(
        this: *const filament_TransformManager,
        i: filament_TransformManager_Instance,
    ) -> utils_Entity;
}
extern "C" {
    #[doc = " Returns the number of children of a transform component."]
    #[doc = " @param i The instance of the transform component to query."]
    #[doc = " @return The number of children of the queried component."]
    #[link_name = "\u{1}_ZNK8filament16TransformManager13getChildCountEN5utils14EntityInstanceIS0_Lb0EEE"]
    pub fn filament_TransformManager_getChildCount(
        this: *const filament_TransformManager,
        i: filament_TransformManager_Instance,
    ) -> size_t;
}
extern "C" {
    #[doc = " Gets a list of children for a transform component."]
    #[doc = ""]
    #[doc = " @param i The instance of the transform component to query."]
    #[doc = " @param children Pointer to array-of-Entity. The array must have at least \"count\" elements."]
    #[doc = " @param count The maximum number of children to retrieve."]
    #[doc = " @return The number of children written to the pointer."]
    #[link_name = "\u{1}_ZNK8filament16TransformManager11getChildrenEN5utils14EntityInstanceIS0_Lb0EEEPNS1_6EntityEm"]
    pub fn filament_TransformManager_getChildren(
        this: *const filament_TransformManager,
        i: filament_TransformManager_Instance,
        children: *mut utils_Entity,
        count: size_t,
    ) -> size_t;
}
extern "C" {
    #[doc = " Returns an iterator to the Instance of the first child of the given parent."]
    #[doc = ""]
    #[doc = " @param parent Instance of the parent"]
    #[doc = " @return A forward iterator pointing to the first child of the given parent."]
    #[doc = ""]
    #[doc = " A child_iterator can only safely be dereferenced if it's different from getChildrenEnd(parent)"]
    #[link_name = "\u{1}_ZNK8filament16TransformManager16getChildrenBeginEN5utils14EntityInstanceIS0_Lb0EEE"]
    pub fn filament_TransformManager_getChildrenBegin(
        this: *const filament_TransformManager,
        parent: filament_TransformManager_Instance,
    ) -> filament_TransformManager_children_iterator;
}
extern "C" {
    #[doc = " Returns an undreferencable iterator representing the end of the children list"]
    #[doc = ""]
    #[doc = " @param parent Instance of the parent"]
    #[doc = " @return A forward iterator."]
    #[doc = ""]
    #[doc = " This iterator cannot be dereferenced"]
    #[link_name = "\u{1}_ZNK8filament16TransformManager14getChildrenEndEN5utils14EntityInstanceIS0_Lb0EEE"]
    pub fn filament_TransformManager_getChildrenEnd(
        this: *const filament_TransformManager,
        parent: filament_TransformManager_Instance,
    ) -> filament_TransformManager_children_iterator;
}
extern "C" {
    #[doc = " Sets a local transform of a transform component."]
    #[doc = " @param ci              The instance of the transform component to set the local transform to."]
    #[doc = " @param localTransform  The local transform (i.e. relative to the parent)."]
    #[doc = " @see getTransform()"]
    #[doc = " @attention This operation can be slow if the hierarchy of transform is too deep, and this"]
    #[doc = "            will be particularly bad when updating a lot of transforms. In that case,"]
    #[doc = "            consider using openLocalTransformTransaction() / commitLocalTransformTransaction()."]
    #[link_name = "\u{1}_ZN8filament16TransformManager12setTransformEN5utils14EntityInstanceIS0_Lb0EEERKNS_4math7details6TMat44IfEE"]
    pub fn filament_TransformManager_setTransform(
        this: *mut filament_TransformManager,
        ci: filament_TransformManager_Instance,
        localTransform: *const filament_math_mat4f,
    );
}
extern "C" {
    #[doc = " Sets a local transform of a transform component and keeps double precision translation."]
    #[doc = " All other values of the transform are stored at single precision."]
    #[doc = " @param ci              The instance of the transform component to set the local transform to."]
    #[doc = " @param localTransform  The local transform (i.e. relative to the parent)."]
    #[doc = " @see getTransform()"]
    #[doc = " @attention This operation can be slow if the hierarchy of transform is too deep, and this"]
    #[doc = "            will be particularly bad when updating a lot of transforms. In that case,"]
    #[doc = "            consider using openLocalTransformTransaction() / commitLocalTransformTransaction()."]
    #[link_name = "\u{1}_ZN8filament16TransformManager12setTransformEN5utils14EntityInstanceIS0_Lb0EEERKNS_4math7details6TMat44IdEE"]
    pub fn filament_TransformManager_setTransform1(
        this: *mut filament_TransformManager,
        ci: filament_TransformManager_Instance,
        localTransform: *const filament_math_mat4,
    );
}
extern "C" {
    #[doc = " Returns the local transform of a transform component."]
    #[doc = " @param ci The instance of the transform component to query the local transform from."]
    #[doc = " @return The local transform of the component (i.e. relative to the parent). This always"]
    #[doc = "         returns the value set by setTransform()."]
    #[doc = " @see setTransform()"]
    #[link_name = "\u{1}_ZNK8filament16TransformManager12getTransformEN5utils14EntityInstanceIS0_Lb0EEE"]
    pub fn filament_TransformManager_getTransform(
        this: *const filament_TransformManager,
        ci: filament_TransformManager_Instance,
    ) -> *const filament_math_mat4f;
}
extern "C" {
    #[doc = " Returns the local transform of a transform component."]
    #[doc = " @param ci The instance of the transform component to query the local transform from."]
    #[doc = " @return The local transform of the component (i.e. relative to the parent). This always"]
    #[doc = "         returns the value set by setTransform()."]
    #[doc = " @see setTransform()"]
    #[link_name = "\u{1}_ZNK8filament16TransformManager20getTransformAccurateEN5utils14EntityInstanceIS0_Lb0EEE"]
    pub fn filament_TransformManager_getTransformAccurate(
        this: *const filament_TransformManager,
        ci: filament_TransformManager_Instance,
    ) -> filament_math_mat4;
}
extern "C" {
    #[doc = " Return the world transform of a transform component."]
    #[doc = " @param ci The instance of the transform component to query the world transform from."]
    #[doc = " @return The world transform of the component (i.e. relative to the root). This is the"]
    #[doc = "         composition of this component's local transform with its parent's world transform."]
    #[doc = " @see setTransform()"]
    #[link_name = "\u{1}_ZNK8filament16TransformManager17getWorldTransformEN5utils14EntityInstanceIS0_Lb0EEE"]
    pub fn filament_TransformManager_getWorldTransform(
        this: *const filament_TransformManager,
        ci: filament_TransformManager_Instance,
    ) -> *const filament_math_mat4f;
}
extern "C" {
    #[doc = " Return the world transform of a transform component."]
    #[doc = " @param ci The instance of the transform component to query the world transform from."]
    #[doc = " @return The world transform of the component (i.e. relative to the root). This is the"]
    #[doc = "         composition of this component's local transform with its parent's world transform."]
    #[doc = " @see setTransform()"]
    #[link_name = "\u{1}_ZNK8filament16TransformManager25getWorldTransformAccurateEN5utils14EntityInstanceIS0_Lb0EEE"]
    pub fn filament_TransformManager_getWorldTransformAccurate(
        this: *const filament_TransformManager,
        ci: filament_TransformManager_Instance,
    ) -> filament_math_mat4;
}
extern "C" {
    #[doc = " Opens a local transform transaction. During a transaction, getWorldTransform() can"]
    #[doc = " return an invalid transform until commitLocalTransformTransaction() is called. However,"]
    #[doc = " setTransform() will perform significantly better and in constant time."]
    #[doc = ""]
    #[doc = " This is useful when updating many transforms and the transform hierarchy is deep (say more"]
    #[doc = " than 4 or 5 levels)."]
    #[doc = ""]
    #[doc = " @note If the local transform transaction is already open, this is a no-op."]
    #[doc = ""]
    #[doc = " @see commitLocalTransformTransaction(), setTransform()"]
    #[link_name = "\u{1}_ZN8filament16TransformManager29openLocalTransformTransactionEv"]
    pub fn filament_TransformManager_openLocalTransformTransaction(
        this: *mut filament_TransformManager,
    );
}
extern "C" {
    #[doc = " Commits the currently open local transform transaction. When this returns, calls"]
    #[doc = " to getWorldTransform() will return the proper value."]
    #[doc = ""]
    #[doc = " @attention failing to call this method when done updating the local transform will cause"]
    #[doc = "            a lot of rendering problems. The system never closes the transaction"]
    #[doc = "            automatically."]
    #[doc = ""]
    #[doc = " @note If the local transform transaction is not open, this is a no-op."]
    #[doc = ""]
    #[doc = " @see openLocalTransformTransaction(), setTransform()"]
    #[link_name = "\u{1}_ZN8filament16TransformManager31commitLocalTransformTransactionEv"]
    pub fn filament_TransformManager_commitLocalTransformTransaction(
        this: *mut filament_TransformManager,
    );
}
impl filament_TransformManager {
    #[inline]
    pub unsafe fn hasComponent(&self, e: utils_Entity) -> bool {
        filament_TransformManager_hasComponent(self, e)
    }
    #[inline]
    pub unsafe fn getInstance(&self, e: utils_Entity) -> filament_TransformManager_Instance {
        filament_TransformManager_getInstance(self, e)
    }
    #[inline]
    pub unsafe fn setAccurateTranslationsEnabled(&mut self, enable: bool) {
        filament_TransformManager_setAccurateTranslationsEnabled(self, enable)
    }
    #[inline]
    pub unsafe fn isAccurateTranslationsEnabled(&self) -> bool {
        filament_TransformManager_isAccurateTranslationsEnabled(self)
    }
    #[inline]
    pub unsafe fn create(
        &mut self,
        entity: utils_Entity,
        parent: filament_TransformManager_Instance,
        localTransform: *const filament_math_mat4f,
    ) {
        filament_TransformManager_create(self, entity, parent, localTransform)
    }
    #[inline]
    pub unsafe fn create1(
        &mut self,
        entity: utils_Entity,
        parent: filament_TransformManager_Instance,
        localTransform: *const filament_math_mat4,
    ) {
        filament_TransformManager_create1(self, entity, parent, localTransform)
    }
    #[inline]
    pub unsafe fn create2(
        &mut self,
        entity: utils_Entity,
        parent: filament_TransformManager_Instance,
    ) {
        filament_TransformManager_create2(self, entity, parent)
    }
    #[inline]
    pub unsafe fn destroy(&mut self, e: utils_Entity) {
        filament_TransformManager_destroy(self, e)
    }
    #[inline]
    pub unsafe fn setParent(
        &mut self,
        i: filament_TransformManager_Instance,
        newParent: filament_TransformManager_Instance,
    ) {
        filament_TransformManager_setParent(self, i, newParent)
    }
    #[inline]
    pub unsafe fn getParent(&self, i: filament_TransformManager_Instance) -> utils_Entity {
        filament_TransformManager_getParent(self, i)
    }
    #[inline]
    pub unsafe fn getChildCount(&self, i: filament_TransformManager_Instance) -> size_t {
        filament_TransformManager_getChildCount(self, i)
    }
    #[inline]
    pub unsafe fn getChildren(
        &self,
        i: filament_TransformManager_Instance,
        children: *mut utils_Entity,
        count: size_t,
    ) -> size_t {
        filament_TransformManager_getChildren(self, i, children, count)
    }
    #[inline]
    pub unsafe fn getChildrenBegin(
        &self,
        parent: filament_TransformManager_Instance,
    ) -> filament_TransformManager_children_iterator {
        filament_TransformManager_getChildrenBegin(self, parent)
    }
    #[inline]
    pub unsafe fn getChildrenEnd(
        &self,
        parent: filament_TransformManager_Instance,
    ) -> filament_TransformManager_children_iterator {
        filament_TransformManager_getChildrenEnd(self, parent)
    }
    #[inline]
    pub unsafe fn setTransform(
        &mut self,
        ci: filament_TransformManager_Instance,
        localTransform: *const filament_math_mat4f,
    ) {
        filament_TransformManager_setTransform(self, ci, localTransform)
    }
    #[inline]
    pub unsafe fn setTransform1(
        &mut self,
        ci: filament_TransformManager_Instance,
        localTransform: *const filament_math_mat4,
    ) {
        filament_TransformManager_setTransform1(self, ci, localTransform)
    }
    #[inline]
    pub unsafe fn getTransform(
        &self,
        ci: filament_TransformManager_Instance,
    ) -> *const filament_math_mat4f {
        filament_TransformManager_getTransform(self, ci)
    }
    #[inline]
    pub unsafe fn getTransformAccurate(
        &self,
        ci: filament_TransformManager_Instance,
    ) -> filament_math_mat4 {
        filament_TransformManager_getTransformAccurate(self, ci)
    }
    #[inline]
    pub unsafe fn getWorldTransform(
        &self,
        ci: filament_TransformManager_Instance,
    ) -> *const filament_math_mat4f {
        filament_TransformManager_getWorldTransform(self, ci)
    }
    #[inline]
    pub unsafe fn getWorldTransformAccurate(
        &self,
        ci: filament_TransformManager_Instance,
    ) -> filament_math_mat4 {
        filament_TransformManager_getWorldTransformAccurate(self, ci)
    }
    #[inline]
    pub unsafe fn openLocalTransformTransaction(&mut self) {
        filament_TransformManager_openLocalTransformTransaction(self)
    }
    #[inline]
    pub unsafe fn commitLocalTransformTransaction(&mut self) {
        filament_TransformManager_commitLocalTransformTransaction(self)
    }
}
#[doc = " Holds a set of buffers that define the geometry of a Renderable."]
#[doc = ""]
#[doc = " The geometry of the Renderable itself is defined by a set of vertex attributes such as"]
#[doc = " position, color, normals, tangents, etc..."]
#[doc = ""]
#[doc = " There is no need to have a 1-to-1 mapping between attributes and buffer. A buffer can hold the"]
#[doc = " data of several attributes -- attributes are then referred as being \"interleaved\"."]
#[doc = ""]
#[doc = " The buffers themselves are GPU resources, therefore mutating their data can be relatively slow."]
#[doc = " For this reason, it is best to separate the constant data from the dynamic data into multiple"]
#[doc = " buffers."]
#[doc = ""]
#[doc = " It is possible, and even encouraged, to use a single vertex buffer for several Renderables."]
#[doc = ""]
#[doc = " @see IndexBuffer, RenderableManager"]
#[repr(C)]
#[derive(Debug, Default)]
pub struct filament_VertexBuffer {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct filament_VertexBuffer_BuilderDetails {
    _unused: [u8; 0],
}
#[doc = " Supported element types"]
pub use self::filament_backend_ElementType as filament_VertexBuffer_AttributeType;
#[doc = " A CPU memory-buffer descriptor, typically used to transfer data from the CPU to the GPU."]
#[doc = ""]
#[doc = " A BufferDescriptor owns the memory buffer it references, therefore BufferDescriptor cannot"]
#[doc = " be copied, but can be moved."]
#[doc = ""]
#[doc = " BufferDescriptor releases ownership of the memory-buffer when it's destroyed."]
pub type filament_VertexBuffer_BufferDescriptor = filament_backend_BufferDescriptor;
#[repr(C)]
#[derive(Debug)]
pub struct filament_VertexBuffer_Builder {
    pub _base: filament_BuilderBase<filament_VertexBuffer_BuilderDetails>,
}
#[test]
fn bindgen_test_layout_filament_VertexBuffer_Builder() {
    assert_eq!(
        ::std::mem::size_of::<filament_VertexBuffer_Builder>(),
        8usize,
        concat!("Size of: ", stringify!(filament_VertexBuffer_Builder))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_VertexBuffer_Builder>(),
        8usize,
        concat!("Alignment of ", stringify!(filament_VertexBuffer_Builder))
    );
}
extern "C" {
    #[doc = " Defines how many buffers will be created in this vertex buffer set. These buffers are"]
    #[doc = " later referenced by index from 0 to \\p bufferCount - 1."]
    #[doc = ""]
    #[doc = " This call is mandatory. The default is 0."]
    #[doc = ""]
    #[doc = " @param bufferCount Number of buffers in this vertex buffer set. The maximum value is 8."]
    #[doc = " @return A reference to this Builder for chaining calls."]
    #[link_name = "\u{1}_ZN8filament12VertexBuffer7Builder11bufferCountEh"]
    pub fn filament_VertexBuffer_Builder_bufferCount(
        this: *mut filament_VertexBuffer_Builder,
        bufferCount: u8,
    ) -> *mut filament_VertexBuffer_Builder;
}
extern "C" {
    #[doc = " Size of each buffer in the set in vertex."]
    #[doc = ""]
    #[doc = " @param vertexCount Number of vertices in each buffer in this set."]
    #[doc = " @return A reference to this Builder for chaining calls."]
    #[link_name = "\u{1}_ZN8filament12VertexBuffer7Builder11vertexCountEj"]
    pub fn filament_VertexBuffer_Builder_vertexCount(
        this: *mut filament_VertexBuffer_Builder,
        vertexCount: u32,
    ) -> *mut filament_VertexBuffer_Builder;
}
extern "C" {
    #[doc = " Allows buffers to be swapped out and shared using BufferObject."]
    #[doc = ""]
    #[doc = " If buffer objects mode is enabled, clients must call setBufferObjectAt rather than"]
    #[doc = " setBufferAt. This allows sharing of data between VertexBuffer objects, but it may"]
    #[doc = " slightly increase the memory footprint of Filament's internal bookkeeping."]
    #[doc = ""]
    #[doc = " @param enabled If true, enables buffer object mode.  False by default."]
    #[link_name = "\u{1}_ZN8filament12VertexBuffer7Builder19enableBufferObjectsEb"]
    pub fn filament_VertexBuffer_Builder_enableBufferObjects(
        this: *mut filament_VertexBuffer_Builder,
        enabled: bool,
    ) -> *mut filament_VertexBuffer_Builder;
}
extern "C" {
    #[doc = " Sets up an attribute for this vertex buffer set."]
    #[doc = ""]
    #[doc = " Using \\p byteOffset and \\p byteStride, attributes can be interleaved in the same buffer."]
    #[doc = ""]
    #[doc = " @param attribute The attribute to set up."]
    #[doc = " @param bufferIndex  The index of the buffer containing the data for this attribute. Must"]
    #[doc = "                     be between 0 and bufferCount() - 1."]
    #[doc = " @param attributeType The type of the attribute data (e.g. byte, float3, etc...)"]
    #[doc = " @param byteOffset Offset in *bytes* into the buffer \\p bufferIndex"]
    #[doc = " @param byteStride Stride in *bytes* to the next element of this attribute. When set to"]
    #[doc = "                   zero the attribute size, as defined by \\p attributeType is used."]
    #[doc = ""]
    #[doc = " @return A reference to this Builder for chaining calls."]
    #[doc = ""]
    #[doc = " @warning VertexAttribute::TANGENTS must be specified as a quaternion and is how normals"]
    #[doc = "          are specified."]
    #[doc = ""]
    #[doc = " @warning Not all backends support 3-component attributes that are not floats. For help"]
    #[doc = "          with conversion, see geometry::Transcoder."]
    #[doc = ""]
    #[doc = " @see VertexAttribute"]
    #[doc = ""]
    #[doc = " This is a no-op if the \\p attribute is an invalid enum."]
    #[doc = " This is a no-op if the \\p bufferIndex is out of bounds."]
    #[doc = ""]
    #[link_name = "\u{1}_ZN8filament12VertexBuffer7Builder9attributeENS_15VertexAttributeEhNS_7backend11ElementTypeEjh"]
    pub fn filament_VertexBuffer_Builder_attribute(
        this: *mut filament_VertexBuffer_Builder,
        attribute: filament_VertexAttribute,
        bufferIndex: u8,
        attributeType: filament_VertexBuffer_AttributeType,
        byteOffset: u32,
        byteStride: u8,
    ) -> *mut filament_VertexBuffer_Builder;
}
extern "C" {
    #[doc = " Sets whether a given attribute should be normalized. By default attributes are not"]
    #[doc = " normalized. A normalized attribute is mapped between 0 and 1 in the shader. This applies"]
    #[doc = " only to integer types."]
    #[doc = ""]
    #[doc = " @param attribute Enum of the attribute to set the normalization flag to."]
    #[doc = " @param normalize true to automatically normalize the given attribute."]
    #[doc = " @return A reference to this Builder for chaining calls."]
    #[doc = ""]
    #[doc = " This is a no-op if the \\p attribute is an invalid enum."]
    #[link_name = "\u{1}_ZN8filament12VertexBuffer7Builder10normalizedENS_15VertexAttributeEb"]
    pub fn filament_VertexBuffer_Builder_normalized(
        this: *mut filament_VertexBuffer_Builder,
        attribute: filament_VertexAttribute,
        normalize: bool,
    ) -> *mut filament_VertexBuffer_Builder;
}
extern "C" {
    #[doc = " Creates the VertexBuffer object and returns a pointer to it."]
    #[doc = ""]
    #[doc = " @param engine Reference to the filament::Engine to associate this VertexBuffer with."]
    #[doc = ""]
    #[doc = " @return pointer to the newly created object or nullptr if exceptions are disabled and"]
    #[doc = "         an error occurred."]
    #[doc = ""]
    #[doc = " @exception utils::PostConditionPanic if a runtime error occurred, such as running out of"]
    #[doc = "            memory or other resources."]
    #[doc = " @exception utils::PreConditionPanic if a parameter to a builder function was invalid."]
    #[link_name = "\u{1}_ZN8filament12VertexBuffer7Builder5buildERNS_6EngineE"]
    pub fn filament_VertexBuffer_Builder_build(
        this: *mut filament_VertexBuffer_Builder,
        engine: *mut filament_Engine,
    ) -> *mut filament_VertexBuffer;
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament12VertexBuffer7BuilderC1Ev"]
    pub fn filament_VertexBuffer_Builder_Builder(this: *mut filament_VertexBuffer_Builder);
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament12VertexBuffer7BuilderC1ERKS1_"]
    pub fn filament_VertexBuffer_Builder_Builder1(
        this: *mut filament_VertexBuffer_Builder,
        rhs: *const filament_VertexBuffer_Builder,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament12VertexBuffer7BuilderC1EOS1_"]
    pub fn filament_VertexBuffer_Builder_Builder2(
        this: *mut filament_VertexBuffer_Builder,
        rhs: *mut filament_VertexBuffer_Builder,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament12VertexBuffer7BuilderD1Ev"]
    pub fn filament_VertexBuffer_Builder_Builder_destructor(
        this: *mut filament_VertexBuffer_Builder,
    );
}
impl Default for filament_VertexBuffer_Builder {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl filament_VertexBuffer_Builder {
    #[inline]
    pub unsafe fn bufferCount(&mut self, bufferCount: u8) -> *mut filament_VertexBuffer_Builder {
        filament_VertexBuffer_Builder_bufferCount(self, bufferCount)
    }
    #[inline]
    pub unsafe fn vertexCount(&mut self, vertexCount: u32) -> *mut filament_VertexBuffer_Builder {
        filament_VertexBuffer_Builder_vertexCount(self, vertexCount)
    }
    #[inline]
    pub unsafe fn enableBufferObjects(
        &mut self,
        enabled: bool,
    ) -> *mut filament_VertexBuffer_Builder {
        filament_VertexBuffer_Builder_enableBufferObjects(self, enabled)
    }
    #[inline]
    pub unsafe fn attribute(
        &mut self,
        attribute: filament_VertexAttribute,
        bufferIndex: u8,
        attributeType: filament_VertexBuffer_AttributeType,
        byteOffset: u32,
        byteStride: u8,
    ) -> *mut filament_VertexBuffer_Builder {
        filament_VertexBuffer_Builder_attribute(
            self,
            attribute,
            bufferIndex,
            attributeType,
            byteOffset,
            byteStride,
        )
    }
    #[inline]
    pub unsafe fn normalized(
        &mut self,
        attribute: filament_VertexAttribute,
        normalize: bool,
    ) -> *mut filament_VertexBuffer_Builder {
        filament_VertexBuffer_Builder_normalized(self, attribute, normalize)
    }
    #[inline]
    pub unsafe fn build(&mut self, engine: *mut filament_Engine) -> *mut filament_VertexBuffer {
        filament_VertexBuffer_Builder_build(self, engine)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_VertexBuffer_Builder_Builder(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(rhs: *const filament_VertexBuffer_Builder) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_VertexBuffer_Builder_Builder1(__bindgen_tmp.as_mut_ptr(), rhs);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new2(rhs: *mut filament_VertexBuffer_Builder) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        filament_VertexBuffer_Builder_Builder2(__bindgen_tmp.as_mut_ptr(), rhs);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        filament_VertexBuffer_Builder_Builder_destructor(self)
    }
}
#[test]
fn bindgen_test_layout_filament_VertexBuffer() {
    assert_eq!(
        ::std::mem::size_of::<filament_VertexBuffer>(),
        1usize,
        concat!("Size of: ", stringify!(filament_VertexBuffer))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_VertexBuffer>(),
        1usize,
        concat!("Alignment of ", stringify!(filament_VertexBuffer))
    );
}
extern "C" {
    #[doc = " Returns the vertex count."]
    #[doc = " @return Number of vertices in this vertex buffer set."]
    #[link_name = "\u{1}_ZNK8filament12VertexBuffer14getVertexCountEv"]
    pub fn filament_VertexBuffer_getVertexCount(this: *const filament_VertexBuffer) -> size_t;
}
extern "C" {
    #[doc = " Asynchronously copy-initializes the specified buffer from the given buffer data."]
    #[doc = ""]
    #[doc = " Do not use this if you called enableBufferObjects() on the Builder."]
    #[doc = ""]
    #[doc = " @param engine Reference to the filament::Engine to associate this VertexBuffer with."]
    #[doc = " @param bufferIndex Index of the buffer to initialize. Must be between 0"]
    #[doc = "                    and Builder::bufferCount() - 1."]
    #[doc = " @param buffer A BufferDescriptor representing the data used to initialize the buffer at"]
    #[doc = "               index \\p bufferIndex. BufferDescriptor points to raw, untyped data that will"]
    #[doc = "               be copied as-is into the buffer."]
    #[doc = " @param byteOffset Offset in *bytes* into the buffer at index \\p bufferIndex of this vertex"]
    #[doc = "                   buffer set."]
    #[link_name = "\u{1}_ZN8filament12VertexBuffer11setBufferAtERNS_6EngineEhONS_7backend16BufferDescriptorEj"]
    pub fn filament_VertexBuffer_setBufferAt(
        this: *mut filament_VertexBuffer,
        engine: *mut filament_Engine,
        bufferIndex: u8,
        buffer: *mut filament_VertexBuffer_BufferDescriptor,
        byteOffset: u32,
    );
}
extern "C" {
    #[doc = " Swaps in the given buffer object."]
    #[doc = ""]
    #[doc = " To use this, you must first call enableBufferObjects() on the Builder."]
    #[doc = ""]
    #[doc = " @param engine Reference to the filament::Engine to associate this VertexBuffer with."]
    #[doc = " @param bufferIndex Index of the buffer to initialize. Must be between 0"]
    #[doc = "                    and Builder::bufferCount() - 1."]
    #[doc = " @param bufferObject The handle to the GPU data that will be used in this buffer slot."]
    #[link_name = "\u{1}_ZN8filament12VertexBuffer17setBufferObjectAtERNS_6EngineEhPKNS_12BufferObjectE"]
    pub fn filament_VertexBuffer_setBufferObjectAt(
        this: *mut filament_VertexBuffer,
        engine: *mut filament_Engine,
        bufferIndex: u8,
        bufferObject: *const filament_BufferObject,
    );
}
impl filament_VertexBuffer {
    #[inline]
    pub unsafe fn getVertexCount(&self) -> size_t {
        filament_VertexBuffer_getVertexCount(self)
    }
    #[inline]
    pub unsafe fn setBufferAt(
        &mut self,
        engine: *mut filament_Engine,
        bufferIndex: u8,
        buffer: *mut filament_VertexBuffer_BufferDescriptor,
        byteOffset: u32,
    ) {
        filament_VertexBuffer_setBufferAt(self, engine, bufferIndex, buffer, byteOffset)
    }
    #[inline]
    pub unsafe fn setBufferObjectAt(
        &mut self,
        engine: *mut filament_Engine,
        bufferIndex: u8,
        bufferObject: *const filament_BufferObject,
    ) {
        filament_VertexBuffer_setBufferObjectAt(self, engine, bufferIndex, bufferObject)
    }
}
#[doc = " A View encompasses all the state needed for rendering a Scene."]
#[doc = ""]
#[doc = " Renderer::render() operates on View objects. These View objects specify important parameters"]
#[doc = " such as:"]
#[doc = "  - The Scene"]
#[doc = "  - The Camera"]
#[doc = "  - The Viewport"]
#[doc = "  - Some rendering parameters"]
#[doc = ""]
#[doc = " \\note"]
#[doc = " View instances are heavy objects that internally cache a lot of data needed for rendering."]
#[doc = " It is not advised for an application to use many View objects."]
#[doc = ""]
#[doc = " For example, in a game, a View could be used for the main scene and another one for the"]
#[doc = " game's user interface. More View instances could be used for creating special effects (e.g."]
#[doc = " a View is akin to a rendering pass)."]
#[doc = ""]
#[doc = ""]
#[doc = " @see Renderer, Scene, Camera, RenderTarget"]
#[repr(C)]
#[derive(Debug, Default)]
pub struct filament_View {
    pub _address: u8,
}
#[doc = " List of available post-processing anti-aliasing techniques."]
#[doc = " @see setAntiAliasing, getAntiAliasing, setSampleCount"]
pub use self::filament_AntiAliasing as filament_View_AntiAliasing;
pub use self::filament_BlendMode as filament_View_BlendMode;
#[doc = " List of available post-processing dithering techniques."]
pub use self::filament_Dithering as filament_View_Dithering;
pub use self::filament_QualityLevel as filament_View_QualityLevel;
#[doc = " List of available shadow mapping techniques."]
#[doc = " @see setShadowType"]
pub use self::filament_ShadowType as filament_View_ShadowType;
#[doc = " Dynamic resolution can be used to either reach a desired target frame rate"]
#[doc = " by lowering the resolution of a View, or to increase the quality when the"]
#[doc = " rendering is faster than the target frame rate."]
#[doc = ""]
#[doc = " This structure can be used to specify the minimum scale factor used when"]
#[doc = " lowering the resolution of a View, and the maximum scale factor used when"]
#[doc = " increasing the resolution for higher quality rendering. The scale factors"]
#[doc = " can be controlled on each X and Y axis independently. By default, all scale"]
#[doc = " factors are set to 1.0."]
#[doc = ""]
#[doc = " enabled:   enable or disables dynamic resolution on a View"]
#[doc = ""]
#[doc = " homogeneousScaling: by default the system scales the major axis first. Set this to true"]
#[doc = "                     to force homogeneous scaling."]
#[doc = ""]
#[doc = " minScale:  the minimum scale in X and Y this View should use"]
#[doc = ""]
#[doc = " maxScale:  the maximum scale in X and Y this View should use"]
#[doc = ""]
#[doc = " quality:   upscaling quality."]
#[doc = "            LOW: 1 bilinear tap, Medium: 4 bilinear taps, High: 9 bilinear taps (tent)"]
#[doc = ""]
#[doc = " \\note"]
#[doc = " Dynamic resolution is only supported on platforms where the time to render"]
#[doc = " a frame can be measured accurately. Dynamic resolution is currently only"]
#[doc = " supported on Android."]
#[doc = ""]
#[doc = " @see Renderer::FrameRateOptions"]
#[doc = ""]
pub type filament_View_DynamicResolutionOptions = filament_DynamicResolutionOptions;
#[doc = " Options to control the bloom effect"]
#[doc = ""]
#[doc = " enabled:     Enable or disable the bloom post-processing effect. Disabled by default."]
#[doc = ""]
#[doc = " levels:      Number of successive blurs to achieve the blur effect, the minimum is 3 and the"]
#[doc = "              maximum is 12. This value together with resolution influences the spread of the"]
#[doc = "              blur effect. This value can be silently reduced to accommodate the original"]
#[doc = "              image size."]
#[doc = ""]
#[doc = " resolution:  Resolution of bloom's minor axis. The minimum value is 2^levels and the"]
#[doc = "              the maximum is lower of the original resolution and 4096. This parameter is"]
#[doc = "              silently clamped to the minimum and maximum."]
#[doc = "              It is highly recommended that this value be smaller than the target resolution"]
#[doc = "              after dynamic resolution is applied (horizontally and vertically)."]
#[doc = ""]
#[doc = " strength:    how much of the bloom is added to the original image. Between 0 and 1."]
#[doc = ""]
#[doc = " blendMode:   Whether the bloom effect is purely additive (false) or mixed with the original"]
#[doc = "              image (true)."]
#[doc = ""]
#[doc = " anamorphism: Bloom's aspect ratio (x/y), for artistic purposes."]
#[doc = ""]
#[doc = " threshold:   When enabled, a threshold at 1.0 is applied on the source image, this is"]
#[doc = "              useful for artistic reasons and is usually needed when a dirt texture is used."]
#[doc = ""]
#[doc = " dirt:        A dirt/scratch/smudges texture (that can be RGB), which gets added to the"]
#[doc = "              bloom effect. Smudges are visible where bloom occurs. Threshold must be"]
#[doc = "              enabled for the dirt effect to work properly."]
#[doc = ""]
#[doc = " dirtStrength: Strength of the dirt texture."]
pub type filament_View_BloomOptions = filament_BloomOptions;
#[doc = " Options to control fog in the scene"]
pub type filament_View_FogOptions = filament_FogOptions;
#[doc = " Options to control Depth of Field (DoF) effect in the scene."]
#[doc = ""]
#[doc = " cocScale can be used to set the depth of field blur independently from the camera"]
#[doc = " aperture, e.g. for artistic reasons. This can be achieved by setting:"]
#[doc = "      cocScale = cameraAperture / desiredDoFAperture"]
#[doc = ""]
#[doc = " @see Camera"]
pub type filament_View_DepthOfFieldOptions = filament_DepthOfFieldOptions;
#[doc = " Options to control the vignetting effect."]
pub type filament_View_VignetteOptions = filament_VignetteOptions;
#[doc = " Structure used to set the precision of the color buffer and related quality settings."]
#[doc = ""]
#[doc = " @see setRenderQuality, getRenderQuality"]
pub type filament_View_RenderQuality = filament_RenderQuality;
#[doc = " Options for screen space Ambient Occlusion (SSAO) and Screen Space Cone Tracing (SSCT)"]
#[doc = " @see setAmbientOcclusionOptions()"]
pub type filament_View_AmbientOcclusionOptions = filament_AmbientOcclusionOptions;
#[doc = " Options for Temporal Anti-aliasing (TAA)"]
#[doc = " @see setTemporalAntiAliasingOptions()"]
pub type filament_View_TemporalAntiAliasingOptions = filament_TemporalAntiAliasingOptions;
#[doc = " Options for Temporal Multi-Sample Anti-aliasing (MSAA)"]
#[doc = " @see setMultiSampleAntiAliasingOptions()"]
pub type filament_View_MultiSampleAntiAliasingOptions = filament_MultiSampleAntiAliasingOptions;
#[doc = " View-level options for VSM Shadowing."]
#[doc = " @see setVsmShadowOptions()"]
#[doc = " @warning This API is still experimental and subject to change."]
pub type filament_View_VsmShadowOptions = filament_VsmShadowOptions;
#[doc = " View-level options for DPCF and PCSS Shadowing."]
#[doc = " @see setSoftShadowOptions()"]
#[doc = " @warning This API is still experimental and subject to change."]
pub type filament_View_SoftShadowOptions = filament_SoftShadowOptions;
#[doc = " Options for Screen-space Reflections."]
#[doc = " @see setScreenSpaceReflectionsOptions()"]
pub type filament_View_ScreenSpaceReflectionsOptions = filament_ScreenSpaceReflectionsOptions;
#[doc = " Result of a picking query"]
#[repr(C)]
pub struct filament_View_PickingQueryResult {
    pub renderable: utils_Entity,
    #[doc = "! RenderableManager Entity at the queried coordinates"]
    pub depth: f32,
    #[doc = "! Depth buffer value (1 (near plane) to 0 (infinity))"]
    pub reserved1: u32,
    pub reserved2: u32,
    #[doc = " screen space coordinates in GL convention, this can be used to compute the view or"]
    #[doc = " world space position of the picking hit. For e.g.:"]
    #[doc = "   clip_space_position  = (fragCoords.xy / viewport.wh, fragCoords.z) * 2.0 - 1.0"]
    #[doc = "   view_space_position  = inverse(projection) * clip_space_position"]
    #[doc = "   world_space_position = model * view_space_position"]
    #[doc = ""]
    #[doc = " The viewport, projection and model matrices can be obtained from Camera. Because"]
    #[doc = " pick() has some latency, it might be more accurate to obtain these values at the"]
    #[doc = " time the View::pick() call is made."]
    pub fragCoords: filament_math_float3,
}
#[test]
fn bindgen_test_layout_filament_View_PickingQueryResult() {
    assert_eq!(
        ::std::mem::size_of::<filament_View_PickingQueryResult>(),
        28usize,
        concat!("Size of: ", stringify!(filament_View_PickingQueryResult))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_View_PickingQueryResult>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(filament_View_PickingQueryResult)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_View_PickingQueryResult>())).renderable as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_View_PickingQueryResult),
            "::",
            stringify!(renderable)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_View_PickingQueryResult>())).depth as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_View_PickingQueryResult),
            "::",
            stringify!(depth)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_View_PickingQueryResult>())).reserved1 as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_View_PickingQueryResult),
            "::",
            stringify!(reserved1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_View_PickingQueryResult>())).reserved2 as *const _
                as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_View_PickingQueryResult),
            "::",
            stringify!(reserved2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_View_PickingQueryResult>())).fragCoords as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_View_PickingQueryResult),
            "::",
            stringify!(fragCoords)
        )
    );
}
impl Default for filament_View_PickingQueryResult {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " User data for PickingQueryResultCallback"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct filament_View_PickingQuery {
    pub storage: [*mut ::std::os::raw::c_void; 4usize],
}
#[test]
fn bindgen_test_layout_filament_View_PickingQuery() {
    assert_eq!(
        ::std::mem::size_of::<filament_View_PickingQuery>(),
        32usize,
        concat!("Size of: ", stringify!(filament_View_PickingQuery))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_View_PickingQuery>(),
        8usize,
        concat!("Alignment of ", stringify!(filament_View_PickingQuery))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<filament_View_PickingQuery>())).storage as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(filament_View_PickingQuery),
            "::",
            stringify!(storage)
        )
    );
}
impl Default for filament_View_PickingQuery {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " callback type used for picking queries."]
pub type filament_View_PickingQueryResultCallback = ::std::option::Option<
    unsafe extern "C" fn(
        result: *const filament_View_PickingQueryResult,
        pq: *mut filament_View_PickingQuery,
    ),
>;
#[doc = "!< No Ambient Occlusion"]
pub const filament_View_AmbientOcclusion_NONE: filament_View_AmbientOcclusion = 0;
#[doc = "!< Basic, sampling SSAO"]
pub const filament_View_AmbientOcclusion_SSAO: filament_View_AmbientOcclusion = 1;
#[doc = " List of available ambient occlusion techniques"]
#[doc = " @deprecated use AmbientOcclusionOptions::enabled instead"]
pub type filament_View_AmbientOcclusion = u8;
#[test]
fn bindgen_test_layout_filament_View() {
    assert_eq!(
        ::std::mem::size_of::<filament_View>(),
        1usize,
        concat!("Size of: ", stringify!(filament_View))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_View>(),
        1usize,
        concat!("Alignment of ", stringify!(filament_View))
    );
}
extern "C" {
    #[doc = " Sets the View's name. Only useful for debugging."]
    #[doc = " @param name Pointer to the View's name. The string is copied."]
    #[link_name = "\u{1}_ZN8filament4View7setNameEPKc"]
    pub fn filament_View_setName(this: *mut filament_View, name: *const ::std::os::raw::c_char);
}
extern "C" {
    #[doc = " Returns the View's name"]
    #[doc = ""]
    #[doc = " @return a pointer owned by the View instance to the View's name."]
    #[doc = ""]
    #[doc = " @attention Do *not* free the pointer or modify its content."]
    #[link_name = "\u{1}_ZNK8filament4View7getNameEv"]
    pub fn filament_View_getName(this: *const filament_View) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Set this View instance's Scene."]
    #[doc = ""]
    #[doc = " @param scene Associate the specified Scene to this View. A Scene can be associated to"]
    #[doc = "              several View instances.\\n"]
    #[doc = "              \\p scene can be nullptr to dissociate the currently set Scene"]
    #[doc = "              from this View.\\n"]
    #[doc = "              The View doesn't take ownership of the Scene pointer (which"]
    #[doc = "              acts as a reference)."]
    #[doc = ""]
    #[doc = " @note"]
    #[doc = "  There is no reference-counting."]
    #[doc = "  Make sure to dissociate a Scene from all Views before destroying it."]
    #[link_name = "\u{1}_ZN8filament4View8setSceneEPNS_5SceneE"]
    pub fn filament_View_setScene(this: *mut filament_View, scene: *mut filament_Scene);
}
extern "C" {
    #[doc = " Returns the Scene currently associated with this View."]
    #[doc = " @return A pointer to the Scene associated to this View. nullptr if no Scene is set."]
    #[link_name = "\u{1}_ZN8filament4View8getSceneEv"]
    pub fn filament_View_getScene(this: *mut filament_View) -> *mut filament_Scene;
}
extern "C" {
    #[doc = " Specifies an offscreen render target to render into."]
    #[doc = ""]
    #[doc = " By default, the view's associated render target is nullptr, which corresponds to the"]
    #[doc = " SwapChain associated with the engine."]
    #[doc = ""]
    #[doc = " A view with a custom render target cannot rely on Renderer::ClearOptions, which only apply"]
    #[doc = " to the SwapChain. Such view can use a Skybox instead."]
    #[doc = ""]
    #[doc = " @param renderTarget Render target associated with view, or nullptr for the swap chain."]
    #[link_name = "\u{1}_ZN8filament4View15setRenderTargetEPNS_12RenderTargetE"]
    pub fn filament_View_setRenderTarget(
        this: *mut filament_View,
        renderTarget: *mut filament_RenderTarget,
    );
}
extern "C" {
    #[doc = " Gets the offscreen render target associated with this view."]
    #[doc = ""]
    #[doc = " Returns nullptr if the render target is the swap chain (which is default)."]
    #[doc = ""]
    #[doc = " @see setRenderTarget"]
    #[link_name = "\u{1}_ZNK8filament4View15getRenderTargetEv"]
    pub fn filament_View_getRenderTarget(this: *const filament_View) -> *mut filament_RenderTarget;
}
extern "C" {
    #[doc = " Sets the rectangular region to render to."]
    #[doc = ""]
    #[doc = " The viewport specifies where the content of the View (i.e. the Scene) is rendered in"]
    #[doc = " the render target. The Render target is automatically clipped to the Viewport."]
    #[doc = ""]
    #[doc = " @param viewport  The Viewport to render the Scene into. The Viewport is a value-type, it is"]
    #[doc = "                  therefore copied. The parameter can be discarded after this call returns."]
    #[link_name = "\u{1}_ZN8filament4View11setViewportERKNS_8ViewportE"]
    pub fn filament_View_setViewport(this: *mut filament_View, viewport: *const filament_Viewport);
}
extern "C" {
    #[doc = " Returns the rectangular region that gets rendered to."]
    #[doc = " @return A constant reference to View's viewport."]
    #[link_name = "\u{1}_ZNK8filament4View11getViewportEv"]
    pub fn filament_View_getViewport(this: *const filament_View) -> *const filament_Viewport;
}
extern "C" {
    #[doc = " Sets this View's Camera."]
    #[doc = ""]
    #[doc = " @param camera    Associate the specified Camera to this View. A Camera can be associated to"]
    #[doc = "                  several View instances.\\n"]
    #[doc = "                  \\p camera can be nullptr to dissociate the currently set Camera from this"]
    #[doc = "                  View.\\n"]
    #[doc = "                  The View doesn't take ownership of the Camera pointer (which"]
    #[doc = "                  acts as a reference)."]
    #[doc = ""]
    #[doc = " @note"]
    #[doc = "  There is no reference-counting."]
    #[doc = "  Make sure to dissociate a Camera from all Views before destroying it."]
    #[link_name = "\u{1}_ZN8filament4View9setCameraEPNS_6CameraE"]
    pub fn filament_View_setCamera(this: *mut filament_View, camera: *mut filament_Camera);
}
extern "C" {
    #[doc = " Returns the Camera currently associated with this View."]
    #[doc = " @return A reference to the Camera associated to this View."]
    #[link_name = "\u{1}_ZN8filament4View9getCameraEv"]
    pub fn filament_View_getCamera(this: *mut filament_View) -> *mut filament_Camera;
}
extern "C" {
    #[doc = " Sets the blending mode used to draw the view into the SwapChain."]
    #[doc = ""]
    #[doc = " @param blendMode either BlendMode::OPAQUE or BlendMode::TRANSLUCENT"]
    #[doc = " @see getBlendMode"]
    #[link_name = "\u{1}_ZN8filament4View12setBlendModeENS_9BlendModeE"]
    pub fn filament_View_setBlendMode(this: *mut filament_View, blendMode: filament_View_BlendMode);
}
extern "C" {
    #[doc = " @return blending mode set by setBlendMode"]
    #[doc = " @see setBlendMode"]
    #[link_name = "\u{1}_ZNK8filament4View12getBlendModeEv"]
    pub fn filament_View_getBlendMode(this: *const filament_View) -> filament_View_BlendMode;
}
extern "C" {
    #[doc = " Sets which layers are visible."]
    #[doc = ""]
    #[doc = " Renderable objects can have one or several layers associated to them. Layers are"]
    #[doc = " represented with an 8-bits bitmask, where each bit corresponds to a layer."]
    #[doc = " @see RenderableManager::setLayerMask()."]
    #[doc = ""]
    #[doc = " This call sets which of those layers are visible. Renderables in invisible layers won't be"]
    #[doc = " rendered."]
    #[doc = ""]
    #[doc = " @param select    a bitmask specifying which layer to set or clear using \\p values."]
    #[doc = " @param values    a bitmask where each bit sets the visibility of the corresponding layer"]
    #[doc = "                  (1: visible, 0: invisible), only layers in \\p select are affected."]
    #[doc = ""]
    #[doc = " @note By default all layers are visible."]
    #[doc = " @note This is a convenient way to quickly show or hide sets of Renderable objects."]
    #[link_name = "\u{1}_ZN8filament4View16setVisibleLayersEhh"]
    pub fn filament_View_setVisibleLayers(this: *mut filament_View, select: u8, values: u8);
}
extern "C" {
    #[doc = " Get the visible layers."]
    #[doc = ""]
    #[doc = " @see View::setVisibleLayers()"]
    #[link_name = "\u{1}_ZNK8filament4View16getVisibleLayersEv"]
    pub fn filament_View_getVisibleLayers(this: *const filament_View) -> u8;
}
extern "C" {
    #[doc = " Enables or disables shadow mapping. Enabled by default."]
    #[doc = ""]
    #[doc = " @param enabled true enables shadow mapping, false disables it."]
    #[doc = ""]
    #[doc = " @see LightManager::Builder::castShadows(),"]
    #[doc = "      RenderableManager::Builder::receiveShadows(),"]
    #[doc = "      RenderableManager::Builder::castShadows(),"]
    #[link_name = "\u{1}_ZN8filament4View19setShadowingEnabledEb"]
    pub fn filament_View_setShadowingEnabled(this: *mut filament_View, enabled: bool);
}
extern "C" {
    #[doc = " @return whether shadowing is enabled"]
    #[link_name = "\u{1}_ZNK8filament4View18isShadowingEnabledEv"]
    pub fn filament_View_isShadowingEnabled(this: *const filament_View) -> bool;
}
extern "C" {
    #[doc = " Enables or disables screen space refraction. Enabled by default."]
    #[doc = ""]
    #[doc = " @param enabled true enables screen space refraction, false disables it."]
    #[link_name = "\u{1}_ZN8filament4View31setScreenSpaceRefractionEnabledEb"]
    pub fn filament_View_setScreenSpaceRefractionEnabled(this: *mut filament_View, enabled: bool);
}
extern "C" {
    #[doc = " @return whether screen space refraction is enabled"]
    #[link_name = "\u{1}_ZNK8filament4View30isScreenSpaceRefractionEnabledEv"]
    pub fn filament_View_isScreenSpaceRefractionEnabled(this: *const filament_View) -> bool;
}
extern "C" {
    #[doc = " Sets how many samples are to be used for MSAA in the post-process stage."]
    #[doc = " Default is 1 and disables MSAA."]
    #[doc = ""]
    #[doc = " @param count number of samples to use for multi-sampled anti-aliasing.\\n"]
    #[doc = "              0: treated as 1"]
    #[doc = "              1: no anti-aliasing"]
    #[doc = "              n: sample count. Effective sample could be different depending on the"]
    #[doc = "                 GPU capabilities."]
    #[doc = ""]
    #[doc = " @note Anti-aliasing can also be performed in the post-processing stage, generally at lower"]
    #[doc = "       cost. See setAntialiasing."]
    #[doc = ""]
    #[doc = " @see setAntialiasing"]
    #[doc = " @deprecated use setMultiSampleAntiAliasingOptions instead"]
    #[link_name = "\u{1}_ZN8filament4View14setSampleCountEh"]
    pub fn filament_View_setSampleCount(this: *mut filament_View, count: u8);
}
extern "C" {
    #[doc = " Returns the sample count set by setSampleCount(). Effective sample count could be different."]
    #[doc = " A value of 0 or 1 means MSAA is disabled."]
    #[doc = ""]
    #[doc = " @return value set by setSampleCount()."]
    #[doc = " @deprecated use getMultiSampleAntiAliasingOptions instead"]
    #[link_name = "\u{1}_ZNK8filament4View14getSampleCountEv"]
    pub fn filament_View_getSampleCount(this: *const filament_View) -> u8;
}
extern "C" {
    #[doc = " Enables or disables anti-aliasing in the post-processing stage. Enabled by default."]
    #[doc = " MSAA can be enabled in addition, see setSampleCount()."]
    #[doc = ""]
    #[doc = " @param type FXAA for enabling, NONE for disabling anti-aliasing."]
    #[doc = ""]
    #[doc = " @note For MSAA anti-aliasing, see setSamplerCount()."]
    #[doc = ""]
    #[doc = " @see setSampleCount"]
    #[link_name = "\u{1}_ZN8filament4View15setAntiAliasingENS_12AntiAliasingE"]
    pub fn filament_View_setAntiAliasing(
        this: *mut filament_View,
        type_: filament_View_AntiAliasing,
    );
}
extern "C" {
    #[doc = " Queries whether anti-aliasing is enabled during the post-processing stage. To query"]
    #[doc = " whether MSAA is enabled, see getSampleCount()."]
    #[doc = ""]
    #[doc = " @return The post-processing anti-aliasing method."]
    #[link_name = "\u{1}_ZNK8filament4View15getAntiAliasingEv"]
    pub fn filament_View_getAntiAliasing(this: *const filament_View) -> filament_View_AntiAliasing;
}
extern "C" {
    #[doc = " Enables or disable temporal anti-aliasing (TAA). Disabled by default."]
    #[doc = ""]
    #[doc = " @param options temporal anti-aliasing options"]
    #[link_name = "\u{1}_ZN8filament4View30setTemporalAntiAliasingOptionsENS_27TemporalAntiAliasingOptionsE"]
    pub fn filament_View_setTemporalAntiAliasingOptions(
        this: *mut filament_View,
        options: filament_View_TemporalAntiAliasingOptions,
    );
}
extern "C" {
    #[doc = " Returns temporal anti-aliasing options."]
    #[doc = ""]
    #[doc = " @return temporal anti-aliasing options"]
    #[link_name = "\u{1}_ZNK8filament4View30getTemporalAntiAliasingOptionsEv"]
    pub fn filament_View_getTemporalAntiAliasingOptions(
        this: *const filament_View,
    ) -> *const filament_View_TemporalAntiAliasingOptions;
}
extern "C" {
    #[doc = " Enables or disable screen-space reflections. Disabled by default."]
    #[doc = ""]
    #[doc = " @param options screen-space reflections options"]
    #[link_name = "\u{1}_ZN8filament4View32setScreenSpaceReflectionsOptionsENS_29ScreenSpaceReflectionsOptionsE"]
    pub fn filament_View_setScreenSpaceReflectionsOptions(
        this: *mut filament_View,
        options: filament_View_ScreenSpaceReflectionsOptions,
    );
}
extern "C" {
    #[doc = " Returns screen-space reflections options."]
    #[doc = ""]
    #[doc = " @return screen-space reflections options"]
    #[link_name = "\u{1}_ZNK8filament4View32getScreenSpaceReflectionsOptionsEv"]
    pub fn filament_View_getScreenSpaceReflectionsOptions(
        this: *const filament_View,
    ) -> *const filament_View_ScreenSpaceReflectionsOptions;
}
extern "C" {
    #[doc = " Enables or disable multi-sample anti-aliasing (MSAA). Disabled by default."]
    #[doc = ""]
    #[doc = " @param options multi-sample anti-aliasing options"]
    #[link_name = "\u{1}_ZN8filament4View33setMultiSampleAntiAliasingOptionsENS_30MultiSampleAntiAliasingOptionsE"]
    pub fn filament_View_setMultiSampleAntiAliasingOptions(
        this: *mut filament_View,
        options: filament_View_MultiSampleAntiAliasingOptions,
    );
}
extern "C" {
    #[doc = " Returns multi-sample anti-aliasing options."]
    #[doc = ""]
    #[doc = " @return multi-sample anti-aliasing options"]
    #[link_name = "\u{1}_ZNK8filament4View33getMultiSampleAntiAliasingOptionsEv"]
    pub fn filament_View_getMultiSampleAntiAliasingOptions(
        this: *const filament_View,
    ) -> *const filament_View_MultiSampleAntiAliasingOptions;
}
extern "C" {
    #[doc = " Sets this View's color grading transforms."]
    #[doc = ""]
    #[doc = " @param colorGrading Associate the specified ColorGrading to this View. A ColorGrading can be"]
    #[doc = "                     associated to several View instances.\\n"]
    #[doc = "                     \\p colorGrading can be nullptr to dissociate the currently set"]
    #[doc = "                     ColorGrading from this View. Doing so will revert to the use of the"]
    #[doc = "                     default color grading transforms.\\n"]
    #[doc = "                     The View doesn't take ownership of the ColorGrading pointer (which"]
    #[doc = "                     acts as a reference)."]
    #[doc = ""]
    #[doc = " @note"]
    #[doc = "  There is no reference-counting."]
    #[doc = "  Make sure to dissociate a ColorGrading from all Views before destroying it."]
    #[link_name = "\u{1}_ZN8filament4View15setColorGradingEPNS_12ColorGradingE"]
    pub fn filament_View_setColorGrading(
        this: *mut filament_View,
        colorGrading: *mut filament_ColorGrading,
    );
}
extern "C" {
    #[doc = " Returns the color grading transforms currently associated to this view."]
    #[doc = " @return A pointer to the ColorGrading associated to this View."]
    #[link_name = "\u{1}_ZNK8filament4View15getColorGradingEv"]
    pub fn filament_View_getColorGrading(
        this: *const filament_View,
    ) -> *const filament_ColorGrading;
}
extern "C" {
    #[doc = " Sets ambient occlusion options."]
    #[doc = ""]
    #[doc = " @param options Options for ambient occlusion."]
    #[link_name = "\u{1}_ZN8filament4View26setAmbientOcclusionOptionsERKNS_23AmbientOcclusionOptionsE"]
    pub fn filament_View_setAmbientOcclusionOptions(
        this: *mut filament_View,
        options: *const filament_View_AmbientOcclusionOptions,
    );
}
extern "C" {
    #[doc = " Gets the ambient occlusion options."]
    #[doc = ""]
    #[doc = " @return ambient occlusion options currently set."]
    #[link_name = "\u{1}_ZNK8filament4View26getAmbientOcclusionOptionsEv"]
    pub fn filament_View_getAmbientOcclusionOptions(
        this: *const filament_View,
    ) -> *const filament_View_AmbientOcclusionOptions;
}
extern "C" {
    #[doc = " Enables or disables bloom in the post-processing stage. Disabled by default."]
    #[doc = ""]
    #[doc = " @param options options"]
    #[link_name = "\u{1}_ZN8filament4View15setBloomOptionsENS_12BloomOptionsE"]
    pub fn filament_View_setBloomOptions(
        this: *mut filament_View,
        options: filament_View_BloomOptions,
    );
}
extern "C" {
    #[doc = " Queries the bloom options."]
    #[doc = ""]
    #[doc = " @return the current bloom options for this view."]
    #[link_name = "\u{1}_ZNK8filament4View15getBloomOptionsEv"]
    pub fn filament_View_getBloomOptions(this: *const filament_View) -> filament_View_BloomOptions;
}
extern "C" {
    #[doc = " Enables or disables fog. Disabled by default."]
    #[doc = ""]
    #[doc = " @param options options"]
    #[link_name = "\u{1}_ZN8filament4View13setFogOptionsENS_10FogOptionsE"]
    pub fn filament_View_setFogOptions(this: *mut filament_View, options: filament_View_FogOptions);
}
extern "C" {
    #[doc = " Queries the fog options."]
    #[doc = ""]
    #[doc = " @return the current fog options for this view."]
    #[link_name = "\u{1}_ZNK8filament4View13getFogOptionsEv"]
    pub fn filament_View_getFogOptions(this: *const filament_View) -> filament_View_FogOptions;
}
extern "C" {
    #[doc = " Enables or disables Depth of Field. Disabled by default."]
    #[doc = ""]
    #[doc = " @param options options"]
    #[link_name = "\u{1}_ZN8filament4View22setDepthOfFieldOptionsENS_19DepthOfFieldOptionsE"]
    pub fn filament_View_setDepthOfFieldOptions(
        this: *mut filament_View,
        options: filament_View_DepthOfFieldOptions,
    );
}
extern "C" {
    #[doc = " Queries the depth of field options."]
    #[doc = ""]
    #[doc = " @return the current depth of field options for this view."]
    #[link_name = "\u{1}_ZNK8filament4View22getDepthOfFieldOptionsEv"]
    pub fn filament_View_getDepthOfFieldOptions(
        this: *const filament_View,
    ) -> filament_View_DepthOfFieldOptions;
}
extern "C" {
    #[doc = " Enables or disables the vignetted effect in the post-processing stage. Disabled by default."]
    #[doc = ""]
    #[doc = " @param options options"]
    #[link_name = "\u{1}_ZN8filament4View18setVignetteOptionsENS_15VignetteOptionsE"]
    pub fn filament_View_setVignetteOptions(
        this: *mut filament_View,
        options: filament_View_VignetteOptions,
    );
}
extern "C" {
    #[doc = " Queries the vignette options."]
    #[doc = ""]
    #[doc = " @return the current vignette options for this view."]
    #[link_name = "\u{1}_ZNK8filament4View18getVignetteOptionsEv"]
    pub fn filament_View_getVignetteOptions(
        this: *const filament_View,
    ) -> filament_View_VignetteOptions;
}
extern "C" {
    #[doc = " Enables or disables dithering in the post-processing stage. Enabled by default."]
    #[doc = ""]
    #[doc = " @param dithering dithering type"]
    #[link_name = "\u{1}_ZN8filament4View12setDitheringENS_9DitheringE"]
    pub fn filament_View_setDithering(this: *mut filament_View, dithering: filament_View_Dithering);
}
extern "C" {
    #[doc = " Queries whether dithering is enabled during the post-processing stage."]
    #[doc = ""]
    #[doc = " @return the current dithering type for this view."]
    #[link_name = "\u{1}_ZNK8filament4View12getDitheringEv"]
    pub fn filament_View_getDithering(this: *const filament_View) -> filament_View_Dithering;
}
extern "C" {
    #[doc = " Sets the dynamic resolution options for this view. Dynamic resolution options"]
    #[doc = " controls whether dynamic resolution is enabled, and if it is, how it behaves."]
    #[doc = ""]
    #[doc = " @param options The dynamic resolution options to use on this view"]
    #[link_name = "\u{1}_ZN8filament4View27setDynamicResolutionOptionsERKNS_24DynamicResolutionOptionsE"]
    pub fn filament_View_setDynamicResolutionOptions(
        this: *mut filament_View,
        options: *const filament_View_DynamicResolutionOptions,
    );
}
extern "C" {
    #[doc = " Returns the dynamic resolution options associated with this view."]
    #[doc = " @return value set by setDynamicResolutionOptions()."]
    #[link_name = "\u{1}_ZNK8filament4View27getDynamicResolutionOptionsEv"]
    pub fn filament_View_getDynamicResolutionOptions(
        this: *const filament_View,
    ) -> filament_View_DynamicResolutionOptions;
}
extern "C" {
    #[doc = " Sets the rendering quality for this view. Refer to RenderQuality for more"]
    #[doc = " information about the different settings available."]
    #[doc = ""]
    #[doc = " @param renderQuality The render quality to use on this view"]
    #[link_name = "\u{1}_ZN8filament4View16setRenderQualityERKNS_13RenderQualityE"]
    pub fn filament_View_setRenderQuality(
        this: *mut filament_View,
        renderQuality: *const filament_View_RenderQuality,
    );
}
extern "C" {
    #[doc = " Returns the render quality used by this view."]
    #[doc = " @return value set by setRenderQuality()."]
    #[link_name = "\u{1}_ZNK8filament4View16getRenderQualityEv"]
    pub fn filament_View_getRenderQuality(
        this: *const filament_View,
    ) -> filament_View_RenderQuality;
}
extern "C" {
    #[doc = " Sets options relative to dynamic lighting for this view."]
    #[doc = ""]
    #[doc = " @param zLightNear Distance from the camera where the lights are expected to shine."]
    #[doc = "                   This parameter can affect performance and is useful because depending"]
    #[doc = "                   on the scene, lights that shine close to the camera may not be"]
    #[doc = "                   visible -- in this case, using a larger value can improve performance."]
    #[doc = "                   e.g. when standing and looking straight, several meters of the ground"]
    #[doc = "                   isn't visible and if lights are expected to shine there, there is no"]
    #[doc = "                   point using a short zLightNear. (Default 5m)."]
    #[doc = ""]
    #[doc = " @param zLightFar Distance from the camera after which lights are not expected to be visible."]
    #[doc = "                  Similarly to zLightNear, setting this value properly can improve"]
    #[doc = "                  performance. (Default 100m)."]
    #[doc = ""]
    #[doc = ""]
    #[doc = " Together zLightNear and zLightFar must be chosen so that the visible influence of lights"]
    #[doc = " is spread between these two values."]
    #[doc = ""]
    #[link_name = "\u{1}_ZN8filament4View25setDynamicLightingOptionsEff"]
    pub fn filament_View_setDynamicLightingOptions(
        this: *mut filament_View,
        zLightNear: f32,
        zLightFar: f32,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8filament4View13setShadowTypeENS_10ShadowTypeE"]
    pub fn filament_View_setShadowType(this: *mut filament_View, shadow: filament_View_ShadowType);
}
extern "C" {
    #[doc = " Sets VSM shadowing options that apply across the entire View."]
    #[doc = ""]
    #[doc = " Additional light-specific VSM options can be set with LightManager::setShadowOptions."]
    #[doc = ""]
    #[doc = " Only applicable when shadow type is set to ShadowType::VSM."]
    #[doc = ""]
    #[doc = " @param options Options for shadowing."]
    #[doc = ""]
    #[doc = " @see setShadowType"]
    #[doc = ""]
    #[doc = " @warning This API is still experimental and subject to change."]
    #[link_name = "\u{1}_ZN8filament4View19setVsmShadowOptionsERKNS_16VsmShadowOptionsE"]
    pub fn filament_View_setVsmShadowOptions(
        this: *mut filament_View,
        options: *const filament_View_VsmShadowOptions,
    );
}
extern "C" {
    #[doc = " Returns the VSM shadowing options associated with this View."]
    #[doc = ""]
    #[doc = " @return value set by setVsmShadowOptions()."]
    #[link_name = "\u{1}_ZNK8filament4View19getVsmShadowOptionsEv"]
    pub fn filament_View_getVsmShadowOptions(
        this: *const filament_View,
    ) -> filament_View_VsmShadowOptions;
}
extern "C" {
    #[doc = " Sets soft shadowing options that apply across the entire View."]
    #[doc = ""]
    #[doc = " Additional light-specific soft shadow parameters can be set with LightManager::setShadowOptions."]
    #[doc = ""]
    #[doc = " Only applicable when shadow type is set to ShadowType::DPCF or ShadowType::PCSS."]
    #[doc = ""]
    #[doc = " @param options Options for shadowing."]
    #[doc = ""]
    #[doc = " @see setShadowType"]
    #[doc = ""]
    #[doc = " @warning This API is still experimental and subject to change."]
    #[link_name = "\u{1}_ZN8filament4View20setSoftShadowOptionsERKNS_17SoftShadowOptionsE"]
    pub fn filament_View_setSoftShadowOptions(
        this: *mut filament_View,
        options: *const filament_View_SoftShadowOptions,
    );
}
extern "C" {
    #[doc = " Returns the soft shadowing options associated with this View."]
    #[doc = ""]
    #[doc = " @return value set by setSoftShadowOptions()."]
    #[link_name = "\u{1}_ZNK8filament4View20getSoftShadowOptionsEv"]
    pub fn filament_View_getSoftShadowOptions(
        this: *const filament_View,
    ) -> filament_View_SoftShadowOptions;
}
extern "C" {
    #[doc = " Enables or disables post processing. Enabled by default."]
    #[doc = ""]
    #[doc = " Post-processing includes:"]
    #[doc = "  - Depth-of-field"]
    #[doc = "  - Bloom"]
    #[doc = "  - Vignetting"]
    #[doc = "  - Temporal Anti-aliasing (TAA)"]
    #[doc = "  - Color grading & gamma encoding"]
    #[doc = "  - Dithering"]
    #[doc = "  - FXAA"]
    #[doc = "  - Dynamic scaling"]
    #[doc = ""]
    #[doc = " Disabling post-processing forgoes color correctness as well as some anti-aliasing techniques"]
    #[doc = " and should only be used for debugging, UI overlays or when using custom render targets"]
    #[doc = " (see RenderTarget)."]
    #[doc = ""]
    #[doc = " @param enabled true enables post processing, false disables it."]
    #[doc = ""]
    #[doc = " @see setBloomOptions, setColorGrading, setAntiAliasing, setDithering, setSampleCount"]
    #[link_name = "\u{1}_ZN8filament4View24setPostProcessingEnabledEb"]
    pub fn filament_View_setPostProcessingEnabled(this: *mut filament_View, enabled: bool);
}
extern "C" {
    #[doc = "! Returns true if post-processing is enabled. See setPostProcessingEnabled() for more info."]
    #[link_name = "\u{1}_ZNK8filament4View23isPostProcessingEnabledEv"]
    pub fn filament_View_isPostProcessingEnabled(this: *const filament_View) -> bool;
}
extern "C" {
    #[doc = " Inverts the winding order of front faces. By default front faces use a counter-clockwise"]
    #[doc = " winding order. When the winding order is inverted, front faces are faces with a clockwise"]
    #[doc = " winding order."]
    #[doc = ""]
    #[doc = " Changing the winding order will directly affect the culling mode in materials"]
    #[doc = " (see Material::getCullingMode())."]
    #[doc = ""]
    #[doc = " Inverting the winding order of front faces is useful when rendering mirrored reflections"]
    #[doc = " (water, mirror surfaces, front camera in AR, etc.)."]
    #[doc = ""]
    #[doc = " @param inverted True to invert front faces, false otherwise."]
    #[link_name = "\u{1}_ZN8filament4View27setFrontFaceWindingInvertedEb"]
    pub fn filament_View_setFrontFaceWindingInverted(this: *mut filament_View, inverted: bool);
}
extern "C" {
    #[doc = " Returns true if the winding order of front faces is inverted."]
    #[doc = " See setFrontFaceWindingInverted() for more information."]
    #[link_name = "\u{1}_ZNK8filament4View26isFrontFaceWindingInvertedEv"]
    pub fn filament_View_isFrontFaceWindingInverted(this: *const filament_View) -> bool;
}
extern "C" {
    #[doc = "! debugging: allows to entirely disable frustum culling. (culling enabled by default)."]
    #[link_name = "\u{1}_ZN8filament4View24setFrustumCullingEnabledEb"]
    pub fn filament_View_setFrustumCullingEnabled(this: *mut filament_View, culling: bool);
}
extern "C" {
    #[doc = "! debugging: returns whether frustum culling is enabled."]
    #[link_name = "\u{1}_ZNK8filament4View23isFrustumCullingEnabledEv"]
    pub fn filament_View_isFrustumCullingEnabled(this: *const filament_View) -> bool;
}
extern "C" {
    #[doc = "! debugging: sets the Camera used for rendering. It may be different from the culling camera."]
    #[link_name = "\u{1}_ZN8filament4View14setDebugCameraEPNS_6CameraE"]
    pub fn filament_View_setDebugCamera(this: *mut filament_View, camera: *mut filament_Camera);
}
extern "C" {
    #[doc = "! debugging: returns a Camera from the point of view of *the* dominant directional light used for shadowing."]
    #[link_name = "\u{1}_ZNK8filament4View25getDirectionalLightCameraEv"]
    pub fn filament_View_getDirectionalLightCamera(
        this: *const filament_View,
    ) -> *const filament_Camera;
}
extern "C" {
    #[doc = " Creates a picking query. Multiple queries can be created (e.g.: multi-touch)."]
    #[doc = " Picking queries are all executed when Renderer::render() is called on this View."]
    #[doc = " The provided callback is guaranteed to be called at some point in the future."]
    #[doc = ""]
    #[doc = " Typically it takes a couple frames to receive the result of a picking query."]
    #[doc = ""]
    #[doc = " @param x         Horizontal coordinate to query in the viewport with origin on the left."]
    #[doc = " @param y         Vertical coordinate to query on the viewport with origin at the bottom."]
    #[doc = " @param callback  User callback, called when the picking query result is available."]
    #[doc = " @param handler   Handler to dispatch the callback or nullptr for the default handler."]
    #[doc = " @return          A reference to a PickingQuery structure, which can be used to store up to"]
    #[doc = "                  8*sizeof(void*) bytes of user data. This user data is later accessible"]
    #[doc = "                  in the PickingQueryResultCallback callback 3rd parameter."]
    #[link_name = "\u{1}_ZN8filament4View4pickEjjPNS_7backend15CallbackHandlerEPFvRKNS0_18PickingQueryResultEPNS0_12PickingQueryEE"]
    pub fn filament_View_pick(
        this: *mut filament_View,
        x: u32,
        y: u32,
        handler: *mut filament_backend_CallbackHandler,
        callback: filament_View_PickingQueryResultCallback,
    ) -> *mut filament_View_PickingQuery;
}
extern "C" {
    #[doc = " Activates or deactivates ambient occlusion."]
    #[doc = " @deprecated use setAmbientOcclusionOptions() instead"]
    #[doc = " @see setAmbientOcclusionOptions"]
    #[doc = ""]
    #[doc = " @param ambientOcclusion Type of ambient occlusion to use."]
    #[link_name = "\u{1}_ZN8filament4View19setAmbientOcclusionENS0_16AmbientOcclusionE"]
    pub fn filament_View_setAmbientOcclusion(
        this: *mut filament_View,
        ambientOcclusion: filament_View_AmbientOcclusion,
    );
}
extern "C" {
    #[doc = " Queries the type of ambient occlusion active for this View."]
    #[doc = " @deprecated use getAmbientOcclusionOptions() instead"]
    #[doc = " @see getAmbientOcclusionOptions"]
    #[doc = ""]
    #[doc = " @return ambient occlusion type."]
    #[link_name = "\u{1}_ZNK8filament4View19getAmbientOcclusionEv"]
    pub fn filament_View_getAmbientOcclusion(
        this: *const filament_View,
    ) -> filament_View_AmbientOcclusion;
}
impl filament_View {
    #[inline]
    pub unsafe fn setName(&mut self, name: *const ::std::os::raw::c_char) {
        filament_View_setName(self, name)
    }
    #[inline]
    pub unsafe fn getName(&self) -> *const ::std::os::raw::c_char {
        filament_View_getName(self)
    }
    #[inline]
    pub unsafe fn setScene(&mut self, scene: *mut filament_Scene) {
        filament_View_setScene(self, scene)
    }
    #[inline]
    pub unsafe fn getScene(&mut self) -> *mut filament_Scene {
        filament_View_getScene(self)
    }
    #[inline]
    pub unsafe fn setRenderTarget(&mut self, renderTarget: *mut filament_RenderTarget) {
        filament_View_setRenderTarget(self, renderTarget)
    }
    #[inline]
    pub unsafe fn getRenderTarget(&self) -> *mut filament_RenderTarget {
        filament_View_getRenderTarget(self)
    }
    #[inline]
    pub unsafe fn setViewport(&mut self, viewport: *const filament_Viewport) {
        filament_View_setViewport(self, viewport)
    }
    #[inline]
    pub unsafe fn getViewport(&self) -> *const filament_Viewport {
        filament_View_getViewport(self)
    }
    #[inline]
    pub unsafe fn setCamera(&mut self, camera: *mut filament_Camera) {
        filament_View_setCamera(self, camera)
    }
    #[inline]
    pub unsafe fn getCamera(&mut self) -> *mut filament_Camera {
        filament_View_getCamera(self)
    }
    #[inline]
    pub unsafe fn setBlendMode(&mut self, blendMode: filament_View_BlendMode) {
        filament_View_setBlendMode(self, blendMode)
    }
    #[inline]
    pub unsafe fn getBlendMode(&self) -> filament_View_BlendMode {
        filament_View_getBlendMode(self)
    }
    #[inline]
    pub unsafe fn setVisibleLayers(&mut self, select: u8, values: u8) {
        filament_View_setVisibleLayers(self, select, values)
    }
    #[inline]
    pub unsafe fn getVisibleLayers(&self) -> u8 {
        filament_View_getVisibleLayers(self)
    }
    #[inline]
    pub unsafe fn setShadowingEnabled(&mut self, enabled: bool) {
        filament_View_setShadowingEnabled(self, enabled)
    }
    #[inline]
    pub unsafe fn isShadowingEnabled(&self) -> bool {
        filament_View_isShadowingEnabled(self)
    }
    #[inline]
    pub unsafe fn setScreenSpaceRefractionEnabled(&mut self, enabled: bool) {
        filament_View_setScreenSpaceRefractionEnabled(self, enabled)
    }
    #[inline]
    pub unsafe fn isScreenSpaceRefractionEnabled(&self) -> bool {
        filament_View_isScreenSpaceRefractionEnabled(self)
    }
    #[inline]
    pub unsafe fn setSampleCount(&mut self, count: u8) {
        filament_View_setSampleCount(self, count)
    }
    #[inline]
    pub unsafe fn getSampleCount(&self) -> u8 {
        filament_View_getSampleCount(self)
    }
    #[inline]
    pub unsafe fn setAntiAliasing(&mut self, type_: filament_View_AntiAliasing) {
        filament_View_setAntiAliasing(self, type_)
    }
    #[inline]
    pub unsafe fn getAntiAliasing(&self) -> filament_View_AntiAliasing {
        filament_View_getAntiAliasing(self)
    }
    #[inline]
    pub unsafe fn setTemporalAntiAliasingOptions(
        &mut self,
        options: filament_View_TemporalAntiAliasingOptions,
    ) {
        filament_View_setTemporalAntiAliasingOptions(self, options)
    }
    #[inline]
    pub unsafe fn getTemporalAntiAliasingOptions(
        &self,
    ) -> *const filament_View_TemporalAntiAliasingOptions {
        filament_View_getTemporalAntiAliasingOptions(self)
    }
    #[inline]
    pub unsafe fn setScreenSpaceReflectionsOptions(
        &mut self,
        options: filament_View_ScreenSpaceReflectionsOptions,
    ) {
        filament_View_setScreenSpaceReflectionsOptions(self, options)
    }
    #[inline]
    pub unsafe fn getScreenSpaceReflectionsOptions(
        &self,
    ) -> *const filament_View_ScreenSpaceReflectionsOptions {
        filament_View_getScreenSpaceReflectionsOptions(self)
    }
    #[inline]
    pub unsafe fn setMultiSampleAntiAliasingOptions(
        &mut self,
        options: filament_View_MultiSampleAntiAliasingOptions,
    ) {
        filament_View_setMultiSampleAntiAliasingOptions(self, options)
    }
    #[inline]
    pub unsafe fn getMultiSampleAntiAliasingOptions(
        &self,
    ) -> *const filament_View_MultiSampleAntiAliasingOptions {
        filament_View_getMultiSampleAntiAliasingOptions(self)
    }
    #[inline]
    pub unsafe fn setColorGrading(&mut self, colorGrading: *mut filament_ColorGrading) {
        filament_View_setColorGrading(self, colorGrading)
    }
    #[inline]
    pub unsafe fn getColorGrading(&self) -> *const filament_ColorGrading {
        filament_View_getColorGrading(self)
    }
    #[inline]
    pub unsafe fn setAmbientOcclusionOptions(
        &mut self,
        options: *const filament_View_AmbientOcclusionOptions,
    ) {
        filament_View_setAmbientOcclusionOptions(self, options)
    }
    #[inline]
    pub unsafe fn getAmbientOcclusionOptions(
        &self,
    ) -> *const filament_View_AmbientOcclusionOptions {
        filament_View_getAmbientOcclusionOptions(self)
    }
    #[inline]
    pub unsafe fn setBloomOptions(&mut self, options: filament_View_BloomOptions) {
        filament_View_setBloomOptions(self, options)
    }
    #[inline]
    pub unsafe fn getBloomOptions(&self) -> filament_View_BloomOptions {
        filament_View_getBloomOptions(self)
    }
    #[inline]
    pub unsafe fn setFogOptions(&mut self, options: filament_View_FogOptions) {
        filament_View_setFogOptions(self, options)
    }
    #[inline]
    pub unsafe fn getFogOptions(&self) -> filament_View_FogOptions {
        filament_View_getFogOptions(self)
    }
    #[inline]
    pub unsafe fn setDepthOfFieldOptions(&mut self, options: filament_View_DepthOfFieldOptions) {
        filament_View_setDepthOfFieldOptions(self, options)
    }
    #[inline]
    pub unsafe fn getDepthOfFieldOptions(&self) -> filament_View_DepthOfFieldOptions {
        filament_View_getDepthOfFieldOptions(self)
    }
    #[inline]
    pub unsafe fn setVignetteOptions(&mut self, options: filament_View_VignetteOptions) {
        filament_View_setVignetteOptions(self, options)
    }
    #[inline]
    pub unsafe fn getVignetteOptions(&self) -> filament_View_VignetteOptions {
        filament_View_getVignetteOptions(self)
    }
    #[inline]
    pub unsafe fn setDithering(&mut self, dithering: filament_View_Dithering) {
        filament_View_setDithering(self, dithering)
    }
    #[inline]
    pub unsafe fn getDithering(&self) -> filament_View_Dithering {
        filament_View_getDithering(self)
    }
    #[inline]
    pub unsafe fn setDynamicResolutionOptions(
        &mut self,
        options: *const filament_View_DynamicResolutionOptions,
    ) {
        filament_View_setDynamicResolutionOptions(self, options)
    }
    #[inline]
    pub unsafe fn getDynamicResolutionOptions(&self) -> filament_View_DynamicResolutionOptions {
        filament_View_getDynamicResolutionOptions(self)
    }
    #[inline]
    pub unsafe fn setRenderQuality(&mut self, renderQuality: *const filament_View_RenderQuality) {
        filament_View_setRenderQuality(self, renderQuality)
    }
    #[inline]
    pub unsafe fn getRenderQuality(&self) -> filament_View_RenderQuality {
        filament_View_getRenderQuality(self)
    }
    #[inline]
    pub unsafe fn setDynamicLightingOptions(&mut self, zLightNear: f32, zLightFar: f32) {
        filament_View_setDynamicLightingOptions(self, zLightNear, zLightFar)
    }
    #[inline]
    pub unsafe fn setShadowType(&mut self, shadow: filament_View_ShadowType) {
        filament_View_setShadowType(self, shadow)
    }
    #[inline]
    pub unsafe fn setVsmShadowOptions(&mut self, options: *const filament_View_VsmShadowOptions) {
        filament_View_setVsmShadowOptions(self, options)
    }
    #[inline]
    pub unsafe fn getVsmShadowOptions(&self) -> filament_View_VsmShadowOptions {
        filament_View_getVsmShadowOptions(self)
    }
    #[inline]
    pub unsafe fn setSoftShadowOptions(&mut self, options: *const filament_View_SoftShadowOptions) {
        filament_View_setSoftShadowOptions(self, options)
    }
    #[inline]
    pub unsafe fn getSoftShadowOptions(&self) -> filament_View_SoftShadowOptions {
        filament_View_getSoftShadowOptions(self)
    }
    #[inline]
    pub unsafe fn setPostProcessingEnabled(&mut self, enabled: bool) {
        filament_View_setPostProcessingEnabled(self, enabled)
    }
    #[inline]
    pub unsafe fn isPostProcessingEnabled(&self) -> bool {
        filament_View_isPostProcessingEnabled(self)
    }
    #[inline]
    pub unsafe fn setFrontFaceWindingInverted(&mut self, inverted: bool) {
        filament_View_setFrontFaceWindingInverted(self, inverted)
    }
    #[inline]
    pub unsafe fn isFrontFaceWindingInverted(&self) -> bool {
        filament_View_isFrontFaceWindingInverted(self)
    }
    #[inline]
    pub unsafe fn setFrustumCullingEnabled(&mut self, culling: bool) {
        filament_View_setFrustumCullingEnabled(self, culling)
    }
    #[inline]
    pub unsafe fn isFrustumCullingEnabled(&self) -> bool {
        filament_View_isFrustumCullingEnabled(self)
    }
    #[inline]
    pub unsafe fn setDebugCamera(&mut self, camera: *mut filament_Camera) {
        filament_View_setDebugCamera(self, camera)
    }
    #[inline]
    pub unsafe fn getDirectionalLightCamera(&self) -> *const filament_Camera {
        filament_View_getDirectionalLightCamera(self)
    }
    #[inline]
    pub unsafe fn pick(
        &mut self,
        x: u32,
        y: u32,
        handler: *mut filament_backend_CallbackHandler,
        callback: filament_View_PickingQueryResultCallback,
    ) -> *mut filament_View_PickingQuery {
        filament_View_pick(self, x, y, handler, callback)
    }
    #[inline]
    pub unsafe fn setAmbientOcclusion(&mut self, ambientOcclusion: filament_View_AmbientOcclusion) {
        filament_View_setAmbientOcclusion(self, ambientOcclusion)
    }
    #[inline]
    pub unsafe fn getAmbientOcclusion(&self) -> filament_View_AmbientOcclusion {
        filament_View_getAmbientOcclusion(self)
    }
}
#[doc = " Viewport describes a view port in pixel coordinates"]
#[doc = ""]
#[doc = " A view port is represented by its left-bottom coordinate, width and height in pixels."]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct filament_Viewport {
    pub _base: filament_backend_Viewport,
}
#[test]
fn bindgen_test_layout_filament_Viewport() {
    assert_eq!(
        ::std::mem::size_of::<filament_Viewport>(),
        16usize,
        concat!("Size of: ", stringify!(filament_Viewport))
    );
    assert_eq!(
        ::std::mem::align_of::<filament_Viewport>(),
        4usize,
        concat!("Alignment of ", stringify!(filament_Viewport))
    );
}
extern "C" {
    #[doc = " Computes a new scaled Viewport"]
    #[doc = " @param s scaling factor on the x and y axes."]
    #[doc = " @return A new scaled Viewport. The coordinates and dimensions of the new Viewport are"]
    #[doc = " rounded to the nearest integer value."]
    #[link_name = "\u{1}_ZNK8filament8Viewport5scaleENS_4math7details5TVec2IfEE"]
    pub fn filament_Viewport_scale(
        this: *const filament_Viewport,
        s: filament_math_float2,
    ) -> filament_Viewport;
}
impl filament_Viewport {
    #[inline]
    pub unsafe fn scale(&self, s: filament_math_float2) -> filament_Viewport {
        filament_Viewport_scale(self, s)
    }
}
pub type size_t = ::std::os::raw::c_ulong;
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type utils_bitset_container_type<T> = T;
pub type utils_bitset32 = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct utils_JobSystem {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct utils_Entity {
    pub mIdentity: utils_Entity_Type,
}
pub type utils_Entity_Type = u32;
#[test]
fn bindgen_test_layout_utils_Entity() {
    assert_eq!(
        ::std::mem::size_of::<utils_Entity>(),
        4usize,
        concat!("Size of: ", stringify!(utils_Entity))
    );
    assert_eq!(
        ::std::mem::align_of::<utils_Entity>(),
        4usize,
        concat!("Alignment of ", stringify!(utils_Entity))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<utils_Entity>())).mIdentity as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(utils_Entity),
            "::",
            stringify!(mIdentity)
        )
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct utils_EntityManager {
    pub mGens: *mut u8,
}
#[repr(C)]
pub struct utils_EntityManager_Listener__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug)]
pub struct utils_EntityManager_Listener {
    pub vtable_: *const utils_EntityManager_Listener__bindgen_vtable,
}
#[test]
fn bindgen_test_layout_utils_EntityManager_Listener() {
    assert_eq!(
        ::std::mem::size_of::<utils_EntityManager_Listener>(),
        8usize,
        concat!("Size of: ", stringify!(utils_EntityManager_Listener))
    );
    assert_eq!(
        ::std::mem::align_of::<utils_EntityManager_Listener>(),
        8usize,
        concat!("Alignment of ", stringify!(utils_EntityManager_Listener))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN5utils13EntityManager8ListenerD1Ev"]
    pub fn utils_EntityManager_Listener_Listener_destructor(
        this: *mut utils_EntityManager_Listener,
    );
}
impl Default for utils_EntityManager_Listener {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl utils_EntityManager_Listener {
    #[inline]
    pub unsafe fn destruct(&mut self) {
        utils_EntityManager_Listener_Listener_destructor(self)
    }
}
pub const utils_EntityManager_GENERATION_SHIFT: ::std::os::raw::c_int = 17;
pub const utils_EntityManager_RAW_INDEX_COUNT: size_t = 131072;
pub const utils_EntityManager_INDEX_MASK: utils_Entity_Type = 131071;
#[test]
fn bindgen_test_layout_utils_EntityManager() {
    assert_eq!(
        ::std::mem::size_of::<utils_EntityManager>(),
        8usize,
        concat!("Size of: ", stringify!(utils_EntityManager))
    );
    assert_eq!(
        ::std::mem::align_of::<utils_EntityManager>(),
        8usize,
        concat!("Alignment of ", stringify!(utils_EntityManager))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<utils_EntityManager>())).mGens as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(utils_EntityManager),
            "::",
            stringify!(mGens)
        )
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN5utils13EntityManager3getEv"]
    pub fn utils_EntityManager_get() -> *mut utils_EntityManager;
}
extern "C" {
    #[link_name = "\u{1}_ZN5utils13EntityManager6createEmPNS_6EntityE"]
    pub fn utils_EntityManager_create(
        this: *mut utils_EntityManager,
        n: size_t,
        entities: *mut utils_Entity,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN5utils13EntityManager7destroyEmPNS_6EntityE"]
    pub fn utils_EntityManager_destroy(
        this: *mut utils_EntityManager,
        n: size_t,
        entities: *mut utils_Entity,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN5utils13EntityManager16registerListenerEPNS0_8ListenerE"]
    pub fn utils_EntityManager_registerListener(
        this: *mut utils_EntityManager,
        l: *mut utils_EntityManager_Listener,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN5utils13EntityManager18unregisterListenerEPNS0_8ListenerE"]
    pub fn utils_EntityManager_unregisterListener(
        this: *mut utils_EntityManager,
        l: *mut utils_EntityManager_Listener,
    );
}
impl Default for utils_EntityManager {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl utils_EntityManager {
    #[inline]
    pub unsafe fn get() -> *mut utils_EntityManager {
        utils_EntityManager_get()
    }
    #[inline]
    pub unsafe fn create(&mut self, n: size_t, entities: *mut utils_Entity) {
        utils_EntityManager_create(self, n, entities)
    }
    #[inline]
    pub unsafe fn destroy(&mut self, n: size_t, entities: *mut utils_Entity) {
        utils_EntityManager_destroy(self, n, entities)
    }
    #[inline]
    pub unsafe fn registerListener(&mut self, l: *mut utils_EntityManager_Listener) {
        utils_EntityManager_registerListener(self, l)
    }
    #[inline]
    pub unsafe fn unregisterListener(&mut self, l: *mut utils_EntityManager_Listener) {
        utils_EntityManager_unregisterListener(self, l)
    }
}
#[test]
fn __bindgen_test_layout_filament_BuilderBase_open0_filament_BufferObject_BuilderDetails_close0_instantiation(
) {
    assert_eq!(
        ::std::mem::size_of::<filament_BuilderBase<filament_BufferObject_BuilderDetails>>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(filament_BuilderBase<filament_BufferObject_BuilderDetails>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_BuilderBase<filament_BufferObject_BuilderDetails>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(filament_BuilderBase<filament_BufferObject_BuilderDetails>)
        )
    );
}
#[test]
fn __bindgen_test_layout_filament_BuilderBase_open0_filament_ColorGrading_BuilderDetails_close0_instantiation(
) {
    assert_eq!(
        ::std::mem::size_of::<filament_BuilderBase<filament_ColorGrading_BuilderDetails>>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(filament_BuilderBase<filament_ColorGrading_BuilderDetails>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_BuilderBase<filament_ColorGrading_BuilderDetails>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(filament_BuilderBase<filament_ColorGrading_BuilderDetails>)
        )
    );
}
#[test]
fn __bindgen_test_layout_filament_BuilderBase_open0_filament_IndexBuffer_BuilderDetails_close0_instantiation(
) {
    assert_eq!(
        ::std::mem::size_of::<filament_BuilderBase<filament_IndexBuffer_BuilderDetails>>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(filament_BuilderBase<filament_IndexBuffer_BuilderDetails>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_BuilderBase<filament_IndexBuffer_BuilderDetails>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(filament_BuilderBase<filament_IndexBuffer_BuilderDetails>)
        )
    );
}
#[test]
fn __bindgen_test_layout_filament_BuilderBase_open0_filament_IndirectLight_BuilderDetails_close0_instantiation(
) {
    assert_eq!(
        ::std::mem::size_of::<filament_BuilderBase<filament_IndirectLight_BuilderDetails>>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(filament_BuilderBase<filament_IndirectLight_BuilderDetails>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_BuilderBase<filament_IndirectLight_BuilderDetails>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(filament_BuilderBase<filament_IndirectLight_BuilderDetails>)
        )
    );
}
#[test]
fn __bindgen_test_layout_filament_BuilderBase_open0_filament_LightManager_BuilderDetails_close0_instantiation(
) {
    assert_eq!(
        ::std::mem::size_of::<filament_BuilderBase<filament_LightManager_BuilderDetails>>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(filament_BuilderBase<filament_LightManager_BuilderDetails>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_BuilderBase<filament_LightManager_BuilderDetails>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(filament_BuilderBase<filament_LightManager_BuilderDetails>)
        )
    );
}
#[test]
fn __bindgen_test_layout_filament_BuilderBase_open0_filament_Material_BuilderDetails_close0_instantiation(
) {
    assert_eq!(
        ::std::mem::size_of::<filament_BuilderBase<filament_Material_BuilderDetails>>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(filament_BuilderBase<filament_Material_BuilderDetails>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_BuilderBase<filament_Material_BuilderDetails>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(filament_BuilderBase<filament_Material_BuilderDetails>)
        )
    );
}
#[test]
fn __bindgen_test_layout_filament_BuilderBase_open0_filament_MorphTargetBuffer_BuilderDetails_close0_instantiation(
) {
    assert_eq!(
        ::std::mem::size_of::<filament_BuilderBase<filament_MorphTargetBuffer_BuilderDetails>>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(filament_BuilderBase<filament_MorphTargetBuffer_BuilderDetails>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_BuilderBase<filament_MorphTargetBuffer_BuilderDetails>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(filament_BuilderBase<filament_MorphTargetBuffer_BuilderDetails>)
        )
    );
}
#[test]
fn __bindgen_test_layout_filament_BuilderBase_open0_filament_RenderableManager_BuilderDetails_close0_instantiation(
) {
    assert_eq!(
        ::std::mem::size_of::<filament_BuilderBase<filament_RenderableManager_BuilderDetails>>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(filament_BuilderBase<filament_RenderableManager_BuilderDetails>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_BuilderBase<filament_RenderableManager_BuilderDetails>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(filament_BuilderBase<filament_RenderableManager_BuilderDetails>)
        )
    );
}
#[test]
fn __bindgen_test_layout_filament_BuilderBase_open0_filament_RenderTarget_BuilderDetails_close0_instantiation(
) {
    assert_eq!(
        ::std::mem::size_of::<filament_BuilderBase<filament_RenderTarget_BuilderDetails>>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(filament_BuilderBase<filament_RenderTarget_BuilderDetails>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_BuilderBase<filament_RenderTarget_BuilderDetails>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(filament_BuilderBase<filament_RenderTarget_BuilderDetails>)
        )
    );
}
#[test]
fn __bindgen_test_layout_filament_BuilderBase_open0_filament_SkinningBuffer_BuilderDetails_close0_instantiation(
) {
    assert_eq!(
        ::std::mem::size_of::<filament_BuilderBase<filament_SkinningBuffer_BuilderDetails>>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(filament_BuilderBase<filament_SkinningBuffer_BuilderDetails>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_BuilderBase<filament_SkinningBuffer_BuilderDetails>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(filament_BuilderBase<filament_SkinningBuffer_BuilderDetails>)
        )
    );
}
#[test]
fn __bindgen_test_layout_filament_BuilderBase_open0_filament_Skybox_BuilderDetails_close0_instantiation(
) {
    assert_eq!(
        ::std::mem::size_of::<filament_BuilderBase<filament_Skybox_BuilderDetails>>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(filament_BuilderBase<filament_Skybox_BuilderDetails>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_BuilderBase<filament_Skybox_BuilderDetails>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(filament_BuilderBase<filament_Skybox_BuilderDetails>)
        )
    );
}
#[test]
fn __bindgen_test_layout_filament_BuilderBase_open0_filament_Stream_BuilderDetails_close0_instantiation(
) {
    assert_eq!(
        ::std::mem::size_of::<filament_BuilderBase<filament_Stream_BuilderDetails>>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(filament_BuilderBase<filament_Stream_BuilderDetails>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_BuilderBase<filament_Stream_BuilderDetails>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(filament_BuilderBase<filament_Stream_BuilderDetails>)
        )
    );
}
#[test]
fn __bindgen_test_layout_filament_BuilderBase_open0_filament_Texture_BuilderDetails_close0_instantiation(
) {
    assert_eq!(
        ::std::mem::size_of::<filament_BuilderBase<filament_Texture_BuilderDetails>>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(filament_BuilderBase<filament_Texture_BuilderDetails>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_BuilderBase<filament_Texture_BuilderDetails>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(filament_BuilderBase<filament_Texture_BuilderDetails>)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_iterator_open0_std_forward_iterator_tag_filament_TransformManager_Instance_long_ptr_EntityInstance_ref_EntityInstance_close0_instantiation(
) {
    assert_eq!(
        ::std::mem::size_of::<std_iterator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_iterator)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<std_iterator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_iterator)
        )
    );
}
#[test]
fn __bindgen_test_layout_filament_BuilderBase_open0_filament_VertexBuffer_BuilderDetails_close0_instantiation(
) {
    assert_eq!(
        ::std::mem::size_of::<filament_BuilderBase<filament_VertexBuffer_BuilderDetails>>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(filament_BuilderBase<filament_VertexBuffer_BuilderDetails>)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<filament_BuilderBase<filament_VertexBuffer_BuilderDetails>>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(filament_BuilderBase<filament_VertexBuffer_BuilderDetails>)
        )
    );
}
