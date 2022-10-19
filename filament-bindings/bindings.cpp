#include "string.h"

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
#include "gltfio/TextureProvider.h"
#include "gltfio/materials/uberarchive.h"

#include "image/ColorTransform.h"
#include "image/ImageOps.h"
#include "image/ImageSampler.h"
#include "image/Ktx1Bundle.h"
#include "image/LinearImage.h"

#include "ktxreader/Ktx1Reader.h"
#include "ktxreader/Ktx2Reader.h"

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

extern "C" image::Ktx1Bundle* helper_image_ktx1_bundle_create(uint32_t numMipLevels, uint32_t arrayLength, bool isCubemap) {
    return new image::Ktx1Bundle(numMipLevels, arrayLength, isCubemap);
}

extern "C" image::Ktx1Bundle* helper_image_ktx1_bundle_from(uint8_t const* bytes, uint32_t nbytes) {
    return new image::Ktx1Bundle(bytes, nbytes);
}

extern "C" void helper_image_ktx1_bundle_delete(image::Ktx1Bundle* self) {
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

extern "C" void helper_color_toLinear_fast_sRGB(filament::math::float3 const& sRGBColor, filament::math::float3* result) {
    *result = filament::Color::toLinear<filament::ColorConversion::FAST>(sRGBColor);
}

extern "C" filament::Texture* helper_ktxreader_ktx1_reader_createTexture(filament::Engine* engine, const image::Ktx1Bundle& bundle, bool srgb, ktxreader::Ktx1Reader::Callback callback, void* userdata) {
    return ktxreader::Ktx1Reader::createTexture(engine, bundle, srgb, callback, userdata);
}

extern "C" void helper_filament_transform_manager_get_instance(const filament::TransformManager& manager, const utils::Entity* e, filament::TransformManager::Instance* result) {
    *result = manager.getInstance(*e);
}

extern "C" void helper_filament_transform_manager_get_parent(const filament::TransformManager& manager, const filament::TransformManager::Instance* i, utils::Entity* result) {
    *result = manager.getParent(*i);
}

extern "C" void helper_filament_renderable_manager_get_instance(const filament::RenderableManager& manager, const utils::Entity* e, filament::RenderableManager::Instance* result) {
    *result = manager.getInstance(*e);
}

extern "C" void helper_filament_light_manager_get_instance(const filament::LightManager& manager, const utils::Entity* e, filament::LightManager::Instance* result) {
    *result = manager.getInstance(*e);
}

extern "C" void helper_filament_math_mat3f_pack_tangent_frame(const filament::math::mat3f& m, size_t storage_size, filament::math::quatf* result) {
    *result = filament::math::details::TMat33<float>::packTangentFrame(m, storage_size);
}

extern "C" void helper_filament_math_mat3_pack_tangent_frame(const filament::math::mat3& m, size_t storage_size, filament::math::quat* result) {
    *result = filament::math::details::TMat33<double>::packTangentFrame(m, storage_size);
}

extern "C" filament::gltfio::MaterialProvider* helper_filament_gltfio_material_provider_create_jit_shader_generator(filament::Engine* engine, bool optimize_shaders) {
    return filament::gltfio::createJitShaderProvider(engine, optimize_shaders);
}

extern "C" filament::gltfio::MaterialProvider* helper_filament_gltfio_material_provider_create_ubershader_loader(filament::Engine* engine) {
    return filament::gltfio::createUbershaderProvider(engine, UBERARCHIVE_DEFAULT_DATA, UBERARCHIVE_DEFAULT_SIZE);
}

extern "C" const filament::Material* const * helper_filament_gltfio_material_provider_get_materials(const filament::gltfio::MaterialProvider* provider) {
    return provider->getMaterials();
}

extern "C" size_t helper_filament_gltfio_material_provider_get_materials_count(const filament::gltfio::MaterialProvider* provider) {
    return provider->getMaterialsCount();
}

extern "C" void helper_filament_gltfio_material_provider_destroy_materials(filament::gltfio::MaterialProvider* provider) {
    provider->destroyMaterials();
}

extern "C" bool helper_filament_gltfio_material_provider_needs_dummy_data(const filament::gltfio::MaterialProvider* provider, filament::VertexAttribute attrib) {
    return provider->needsDummyData(attrib);
}

extern "C" void helper_filament_gltfio_material_provider_delete(filament::gltfio::MaterialProvider* provider) {
    delete provider;
}

extern "C" filament::gltfio::ResourceLoader* helper_filament_gltfio_resource_loader_create(const filament::gltfio::ResourceConfiguration& config) {
    return new filament::gltfio::ResourceLoader(config);
}

extern "C" void helper_filament_gltfio_resource_loader_delete(filament::gltfio::ResourceLoader* loader) {
    delete loader;
}

extern "C" filament::gltfio::TextureProvider* helper_filament_gltfio_create_stb_provider(filament::Engine* engine) {
    return filament::gltfio::createStbProvider(engine);
}

extern "C" filament::gltfio::TextureProvider* helper_filament_gltfio_create_ktx2_provider(filament::Engine* engine) {
    return filament::gltfio::createKtx2Provider(engine);
}

extern "C" void helper_filament_gltfio_texture_provider_delete(filament::gltfio::TextureProvider* provider) {
    delete provider;
}

extern "C" void helper_filament_gltfio_filament_asset_get_root(const filament::gltfio::FilamentAsset& asset, utils::Entity* result) {
    *result = asset.getRoot();
}

extern "C" void helper_filament_gltfio_filament_asset_get_bounding_box(const filament::gltfio::FilamentAsset& asset, filament::Aabb* result) {
    *result = asset.getBoundingBox();
}

extern "C" void helper_filament_gltfio_resource_loader_add_texture_provider(filament::gltfio::ResourceLoader* resourceLoader, const char* mimeType, filament::gltfio::TextureProvider* provider) {
    resourceLoader->addTextureProvider(mimeType, provider);
}
