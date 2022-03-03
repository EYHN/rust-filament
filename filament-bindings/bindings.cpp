#include "filament/Box.h"
#include "filament/BufferObject.h"
#include "filament/Camera.h"
#include "filament/ColorGrading.h"
#include "filament/Color.h"
#include "filament/DebugRegistry.h"
#include "filament/Engine.h"
#include "filament/Exposure.h"
#include "filament/Fence.h"
#include "filament/FilamentAPI.h"
#include "filament/Frustum.h"
#include "filament/IndexBuffer.h"
#include "filament/IndirectLight.h"
#include "filament/LightManager.h"
#include "filament/MaterialChunkType.h"
#include "filament/MaterialEnums.h"
#include "filament/Material.h"
#include "filament/MaterialInstance.h"
#include "filament/MorphTargetBuffer.h"
#include "filament/Options.h"
#include "filament/RenderableManager.h"
#include "filament/Renderer.h"
#include "filament/RenderTarget.h"
#include "filament/Scene.h"
#include "filament/SkinningBuffer.h"
#include "filament/Skybox.h"
#include "filament/Stream.h"
#include "filament/SwapChain.h"
#include "filament/Texture.h"
#include "filament/TextureSampler.h"
#include "filament/ToneMapper.h"
#include "filament/TransformManager.h"
#include "filament/VertexBuffer.h"
#include "filament/View.h"
#include "filament/Viewport.h"

#include "backend/BufferDescriptor.h"
#include "backend/CallbackHandler.h"
#include "backend/DriverEnums.h"
#include "backend/Handle.h"
#include "backend/PipelineState.h"
#include "backend/PixelBufferDescriptor.h"
#include "backend/Platform.h"
#include "backend/PresentCallable.h"
#include "backend/ShaderStageFlags.h"
#include "backend/TargetBufferInfo.h"

#include "utils/EntityManager.h"
#include "utils/Entity.h"
#include "utils/EntityInstance.h"

#include "math/compiler.h"
#include "math/fast.h"
#include "math/half.h"
#include "math/mat2.h"
#include "math/mat3.h"
#include "math/mat4.h"
#include "math/mathfwd.h"
#include "math/norm.h"
#include "math/quat.h"
#include "math/scalar.h"
#include "math/TMatHelpers.h"
#include "math/TQuatHelpers.h"
#include "math/TVecHelpers.h"
#include "math/vec2.h"
#include "math/vec3.h"
#include "math/vec4.h"

#include "filameshio/MeshReader.h"

#include "gltfio/Animator.h"
#include "gltfio/AssetLoader.h"
#include "gltfio/FilamentAsset.h"
#include "gltfio/FilamentInstance.h"
#include "gltfio/MaterialProvider.h"
#include "gltfio/ResourceLoader.h"

// #include "ibl/Cubemap.h"
// #include "ibl/CubemapIBL.h"
// #include "ibl/CubemapSH.h"
// #include "ibl/CubemapUtils.h"
// #include "ibl/Image.h"
// #include "ibl/utilities.h"

#include "image/ColorTransform.h"
#include "image/ImageOps.h"
#include "image/ImageSampler.h"
#include "image/KtxBundle.h"
#include "image/KtxUtility.h"
#include "image/LinearImage.h"

extern "C" filament::Material::Builder* helper_filament_material_builder_create() {
    return new filament::Material::Builder();
}

extern "C" void helper_filament_material_builder_delete(filament::Material::Builder* self) {
    delete self;
}

extern "C" filament::LightManager::Builder* helper_filament_light_manager_builder_create(filament::LightManager::Type type) {
    return new filament::LightManager::Builder(type);
}

extern "C" void helper_filament_light_manager_builder_delete(filament::LightManager::Builder* self) {
    delete self;
}

extern "C" image::KtxBundle* helper_image_ktx_bundle_create(uint32_t numMipLevels, uint32_t arrayLength, bool isCubemap) {
    return new image::KtxBundle(numMipLevels, arrayLength, isCubemap);
}

extern "C" image::KtxBundle* helper_image_ktx_bundle_from(uint8_t const* bytes, uint32_t nbytes) {
    return new image::KtxBundle(bytes, nbytes);
}

extern "C" void helper_image_ktx_bundle_delete(image::KtxBundle* self) {
    delete self;
}

extern "C" filament::IndexBuffer::Builder* helper_filament_index_buffer_create() {
    return new filament::IndexBuffer::Builder();
}

extern "C" void helper_filament_index_buffer_delete(filament::IndexBuffer::Builder* self) {
    delete self;
}

extern "C" filament::VertexBuffer::Builder* helper_filament_vertex_buffer_create() {
    return new filament::VertexBuffer::Builder();
}

extern "C" void helper_filament_vertex_buffer_delete(filament::VertexBuffer::Builder* self) {
    delete self;
}

extern "C" filament::Texture::Builder* helper_filament_texture_builder_create() {
    return new filament::Texture::Builder();
}

extern "C" void helper_filament_texture_builder_delete(filament::Texture::Builder* self) {
    delete self;
}

extern "C" filament::RenderableManager::Builder* helper_filament_renderable_manager_builder_create(size_t count) {
    return new filament::RenderableManager::Builder(count);
}

extern "C" void helper_filament_renderable_manager_builder_delete(filament::RenderableManager::Builder* self) {
    delete self;
}

extern "C" filament::Skybox::Builder* helper_filament_skybox_builder_create() {
    return new filament::Skybox::Builder();
}

extern "C" void helper_filament_skybox_builder_delete(filament::Skybox::Builder* self) {
    delete self;
}

extern "C" filament::IndirectLight::Builder* helper_filament_indirect_light_builder_create() {
    return new filament::IndirectLight::Builder();
}

extern "C" void helper_filament_indirect_light_builder_delete(filament::IndirectLight::Builder* self) {
    delete self;
}

extern "C" void helper_material_instance_setParameter_float(filament::MaterialInstance * instance, const char *name, float const& value) {
    instance->setParameter(name, value);
}

extern "C" void helper_material_instance_setParameter_int32(filament::MaterialInstance * instance, const char *name, int32_t const& value) {
    instance->setParameter(name, value);
}

extern "C" void helper_material_instance_setParameter_uint32(filament::MaterialInstance * instance, const char *name, uint32_t const& value) {
    instance->setParameter(name, value);
}

extern "C" void helper_material_instance_setParameter_int2(filament::MaterialInstance * instance, const char *name, filament::math::int2 const& value) {
    instance->setParameter(name, value);
}

extern "C" void helper_material_instance_setParameter_int3(filament::MaterialInstance * instance, const char *name, filament::math::int3 const& value) {
    instance->setParameter(name, value);
}

extern "C" void helper_material_instance_setParameter_int4(filament::MaterialInstance * instance, const char *name, filament::math::int4 const& value) {
    instance->setParameter(name, value);
}

extern "C" void helper_material_instance_setParameter_uint2(filament::MaterialInstance * instance, const char *name, filament::math::uint2 const& value) {
    instance->setParameter(name, value);
}

extern "C" void helper_material_instance_setParameter_uint3(filament::MaterialInstance * instance, const char *name, filament::math::uint3 const& value) {
    instance->setParameter(name, value);
}

extern "C" void helper_material_instance_setParameter_uint4(filament::MaterialInstance * instance, const char *name, filament::math::uint4 const& value) {
    instance->setParameter(name, value);
}

extern "C" void helper_material_instance_setParameter_float2(filament::MaterialInstance * instance, const char *name, filament::math::float2 const& value) {
    instance->setParameter(name, value);
}

extern "C" void helper_material_instance_setParameter_float3(filament::MaterialInstance * instance, const char *name, filament::math::float3 const& value) {
    instance->setParameter(name, value);
}

extern "C" void helper_material_instance_setParameter_float4(filament::MaterialInstance * instance, const char *name, filament::math::float4 const& value) {
    instance->setParameter(name, value);
}

extern "C" void helper_material_instance_setParameter_mat4f(filament::MaterialInstance * instance, const char *name, filament::math::mat4f const& value) {
    instance->setParameter(name, value);
}

extern "C" void helper_material_instance_setParameter_bool(filament::MaterialInstance * instance, const char *name, bool const& value) {
    instance->setParameter(name, value);
}

extern "C" void helper_material_instance_setParameter_bool2(filament::MaterialInstance * instance, const char *name, filament::math::bool2 const& value) {
    instance->setParameter(name, value);
}

extern "C" void helper_material_instance_setParameter_bool3(filament::MaterialInstance * instance, const char *name, filament::math::bool3 const& value) {
    instance->setParameter(name, value);
}

extern "C" void helper_material_instance_setParameter_bool4(filament::MaterialInstance * instance, const char *name, filament::math::bool4 const& value) {
    instance->setParameter(name, value);
}

extern "C" void helper_material_instance_setParameter_mat3f(filament::MaterialInstance * instance, const char *name, filament::math::mat3f const& value) {
    instance->setParameter(name, value);
}

extern "C" filament::math::float3 helper_color_toLinear_fast_sRGB(filament::math::float3 const& sRGBColor) {
    return filament::Color::toLinear<filament::ColorConversion::FAST>(sRGBColor);
}

extern "C" filament::Texture* helper_image_ktx_createTexture(filament::Engine* engine, const image::KtxBundle& bundle, bool srgb, image::ktx::Callback callback, void* userdata) {
    return image::ktx::createTexture(engine, bundle, srgb, callback, userdata);
}
