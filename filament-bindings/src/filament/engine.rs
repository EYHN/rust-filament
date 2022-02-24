use std::ptr;

use crate::{
    backend::Backend,
    bindgen,
    utils::{Entity, EntityManager},
};

use super::{
    Camera, Fence, LightManager, RenderableManager, Renderer, Scene, SwapChain, SwapChainConfig,
    TransformManager, VertexBuffer, View, IndexBuffer, Material, MaterialInstance, Texture,
};

#[repr(transparent)]
pub struct Engine {
    native: ptr::NonNull<bindgen::filament_Engine>,
}

impl Engine {
    #[inline]
    pub fn try_from_native(native: *mut bindgen::filament_Engine) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(Engine { native: ptr })
    }

    #[inline]
    pub unsafe fn native(&self) -> *const bindgen::filament_Engine {
        self.native.as_ptr()
    }

    #[inline]
    pub unsafe fn native_mut(&mut self) -> *mut bindgen::filament_Engine {
        self.native.as_ptr()
    }

    #[inline]
    pub unsafe fn create(backend: Backend) -> Option<Engine> {
        Self::try_from_native(bindgen::filament_Engine_create(
            backend.into(),
            ptr::null_mut(),
            ptr::null_mut(),
        ))
    }

    #[inline]
    pub unsafe fn create_shared_gl_context(
        backend: Backend,
        shared_gl_context: *mut core::ffi::c_void,
    ) -> Option<Engine> {
        Self::try_from_native(bindgen::filament_Engine_create(
            backend.into(),
            ptr::null_mut(),
            shared_gl_context,
        ))
    }

    // TODO: create_platform

    #[inline]
    pub unsafe fn getEngine(token: *mut core::ffi::c_void) -> Option<Engine> {
        Self::try_from_native(bindgen::filament_Engine_getEngine(token))
    }

    #[inline]
    pub unsafe fn destroy(mut engine: Engine) {
        bindgen::filament_Engine_destroy1(engine.native_mut())
    }

    #[inline]
    pub unsafe fn getEntityManager(&mut self) -> Option<EntityManager> {
        EntityManager::try_from_native(bindgen::filament_Engine_getEntityManager(self.native_mut()))
    }

    #[inline]
    pub unsafe fn getRenderableManager(&mut self) -> Option<RenderableManager> {
        RenderableManager::try_from_native(bindgen::filament_Engine_getRenderableManager(
            self.native_mut(),
        ))
    }

    #[inline]
    pub unsafe fn getLightManager(&mut self) -> Option<LightManager> {
        LightManager::try_from_native(bindgen::filament_Engine_getLightManager(self.native_mut()))
    }

    #[inline]
    pub unsafe fn getTransformManager(&mut self) -> Option<TransformManager> {
        TransformManager::try_from_native(bindgen::filament_Engine_getTransformManager(
            self.native_mut(),
        ))
    }

    #[inline]
    pub unsafe fn enableAccurateTranslations(&mut self) {
        bindgen::filament_Engine_enableAccurateTranslations(self.native_mut())
    }

    #[inline]
    pub unsafe fn create_swap_chain(
        &mut self,
        native_window: *mut core::ffi::c_void,
        flags: SwapChainConfig,
    ) -> Option<SwapChain> {
        SwapChain::try_from_native(bindgen::filament_Engine_createSwapChain(
            self.native_mut(),
            native_window,
            flags.bits(),
        ))
    }

    #[inline]
    pub unsafe fn create_headless_swap_chain(
        &mut self,
        width: u32,
        height: u32,
        flags: SwapChainConfig,
    ) -> Option<SwapChain> {
        SwapChain::try_from_native(bindgen::filament_Engine_createSwapChain1(
            self.native_mut(),
            width,
            height,
            flags.bits(),
        ))
    }

    #[inline]
    pub unsafe fn createRenderer(&mut self) -> Option<Renderer> {
        Renderer::try_from_native(bindgen::filament_Engine_createRenderer(self.native_mut()))
    }

    #[inline]
    pub unsafe fn createView(&mut self) -> Option<View> {
        View::try_from_native(bindgen::filament_Engine_createView(self.native_mut()))
    }

    #[inline]
    pub unsafe fn createScene(&mut self) -> Option<Scene> {
        Scene::try_from_native(bindgen::filament_Engine_createScene(self.native_mut()))
    }

    #[inline]
    pub unsafe fn createCamera(&mut self, entity: &Entity) -> Option<Camera> {
        Camera::try_from_native(bindgen::filament_Engine_createCamera(
            self.native_mut(),
            entity.native_owned(),
        ))
    }

    #[inline]
    pub unsafe fn getCameraComponent(&mut self, entity: &Entity) -> Option<Camera> {
        Camera::try_from_native(bindgen::filament_Engine_getCameraComponent(
            self.native_mut(),
            entity.native_owned(),
        ))
    }

    #[inline]
    pub unsafe fn destroyCameraComponent(&mut self, entity: &Entity) {
        bindgen::filament_Engine_destroyCameraComponent(self.native_mut(), entity.native_owned())
    }

    #[inline]
    pub unsafe fn createFence(&mut self) -> Option<Fence> {
        Fence::try_from_native(bindgen::filament_Engine_createFence(self.native_mut()))
    }

    // TODO:
    // #[inline]
    // pub unsafe fn destroy2(&mut self, p: *const bindgen::filament_BufferObject) -> bool {
    //     bindgen::filament_Engine_destroy2(self, p)
    // }

    #[inline]
    pub unsafe fn destroy_vertex_buffer(&mut self, mut p: VertexBuffer) -> bool {
        bindgen::filament_Engine_destroy3(self.native_mut(), p.native_mut())
    }

    #[inline]
    pub unsafe fn destroy_fence(&mut self, mut p: Fence) -> bool {
        bindgen::filament_Engine_destroy4(self.native_mut(), p.native_mut())
    }
    #[inline]
    pub unsafe fn destroy_index_buffer(&mut self, mut p: IndexBuffer) -> bool {
        bindgen::filament_Engine_destroy5(self.native_mut(), p.native_mut())
    }
    // #[inline]
    // pub unsafe fn destroy_skinning_buffer(&mut self, p: SkinningBuffer) -> bool {
    //     bindgen::filament_Engine_destroy6(self.native_mut(), p.native_mut())
    // }
    // #[inline]
    // pub unsafe fn destroy_morph_target_buffer(&mut self, p: MorphTargetBuffer) -> bool {
    //     bindgen::filament_Engine_destroy7(self.native_mut(), p.native_mut())
    // }
    // #[inline]
    // pub unsafe fn destroy_indirect_light(&mut self, p: IndirectLight) -> bool {
    //     bindgen::filament_Engine_destroy8(self.native_mut(), p.native_mut())
    // }
    #[inline]
    pub unsafe fn destroy_material(&mut self, mut p: Material) -> bool {
        bindgen::filament_Engine_destroy9(self.native_mut(), p.native_mut())
    }
    #[inline]
    pub unsafe fn destroy_material_instance(&mut self, mut p: MaterialInstance) -> bool {
        bindgen::filament_Engine_destroy10(self.native_mut(), p.native_mut())
    }
    #[inline]
    pub unsafe fn destroy_renderer(&mut self, mut p: Renderer) -> bool {
        bindgen::filament_Engine_destroy11(self.native_mut(), p.native_mut())
    }
    #[inline]
    pub unsafe fn destroy_scene(&mut self, mut p: Scene) -> bool {
        bindgen::filament_Engine_destroy12(self.native_mut(), p.native_mut())
    }
    // #[inline]
    // pub unsafe fn destroy_skybox(&mut self, p: Skybox) -> bool {
    //     bindgen::filament_Engine_destroy13(self.native_mut(), p.native_mut())
    // }
    // #[inline]
    // pub unsafe fn destroy_color_grading(&mut self, p: ColorGrading) -> bool {
    //     bindgen::filament_Engine_destroy14(self.native_mut(), p.native_mut())
    // }
    #[inline]
    pub unsafe fn destroy_swap_chain(&mut self, mut p: SwapChain) -> bool {
        bindgen::filament_Engine_destroy15(self.native_mut(), p.native_mut())
    }
    // #[inline]
    // pub unsafe fn destroy_stream(&mut self, p: Stream) -> bool {
    //     bindgen::filament_Engine_destroy16(self.native_mut(), p.native_mut())
    // }
    #[inline]
    pub unsafe fn destroy_texture(&mut self, mut p: Texture) -> bool {
        bindgen::filament_Engine_destroy17(self.native_mut(), p.native_mut())
    }
    // #[inline]
    // pub unsafe fn destroy_render_target(&mut self, p: RenderTarget) -> bool {
    //     bindgen::filament_Engine_destroy18(self.native_mut(), p.native_mut())
    // }
    #[inline]
    pub unsafe fn destroy_view(&mut self, mut p: View) -> bool {
        bindgen::filament_Engine_destroy19(self.native_mut(), p.native_mut())
    }
    #[inline]
    pub unsafe fn destroy_entity_components(&mut self, e: &Entity) {
        bindgen::filament_Engine_destroy20(self.native_mut(), e.native_owned())
    }
    #[inline]
    pub unsafe fn flushAndWait(&mut self) {
        bindgen::filament_Engine_flushAndWait(self.native_mut())
    }
    #[inline]
    pub unsafe fn flush(&mut self) {
        bindgen::filament_Engine_flush(self.native_mut())
    }
    #[inline]
    pub unsafe fn pumpMessageQueues(&mut self) {
        bindgen::filament_Engine_pumpMessageQueues(self.native_mut())
    }
    #[inline]
    pub unsafe fn getDefaultMaterial(&self) -> *const bindgen::filament_Material {
        bindgen::filament_Engine_getDefaultMaterial(self.native())
    }
    #[inline]
    pub unsafe fn getBackend(&self) -> bindgen::filament_Engine_Backend {
        bindgen::filament_Engine_getBackend(self.native())
    }
    #[inline]
    pub unsafe fn getPlatform(&self) -> *mut bindgen::filament_Engine_Platform {
        bindgen::filament_Engine_getPlatform(self.native())
    }
    #[inline]
    pub unsafe fn streamAlloc(
        &mut self,
        size: usize,
        alignment: usize,
    ) -> *mut ::core::ffi::c_void {
        bindgen::filament_Engine_streamAlloc(self.native_mut(), size, alignment)
    }
    #[inline]
    pub unsafe fn execute(&mut self) {
        bindgen::filament_Engine_execute(self.native_mut())
    }
    // #[inline]
    // pub unsafe fn getJobSystem(&mut self) -> *mut utils_JobSystem {
    //     bindgen::filament_Engine_getJobSystem(self)
    // }
    // #[inline]
    // pub unsafe fn getDebugRegistry(&mut self) -> *mut bindgen::filament_DebugRegistry {
    //     bindgen::filament_Engine_getDebugRegistry(self)
    // }
}
