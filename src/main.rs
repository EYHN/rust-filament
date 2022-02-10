use std::{
    alloc::{dealloc, Layout},
    fs::File,
    io::Write,
    os,
    ptr::{drop_in_place, null_mut, write},
    slice, process,
};

use filament_sys::{
    filament_Camera_Projection_ORTHO, filament_Camera_setProjection, filament_Engine,
    filament_Engine_createCamera, filament_Engine_createRenderer, filament_Engine_createScene,
    filament_Engine_createSwapChain1, filament_Engine_createView, filament_Engine_getEntityManager,
    filament_Renderer_ClearOptions, filament_Renderer_beginFrame, filament_Renderer_endFrame,
    filament_Renderer_readPixels, filament_Renderer_render, filament_Renderer_setClearOptions,
    filament_View_setCamera, filament_View_setScene, filament_View_setViewport, filament_Viewport,
    filament_backend_Backend_DEFAULT, filament_backend_BufferDescriptor,
    filament_backend_PixelBufferDescriptor, filament_backend_PixelBufferDescriptor__bindgen_ty_1,
    filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_1,
    filament_backend_PixelDataFormat_RGBA, filament_backend_PixelDataType_UBYTE,
    filament_backend_Viewport, filament_math_float4, utils_Entity, utils_EntityManager_create,
};

fn main() {
    unsafe {
        let engine =
            filament_Engine::create(filament_backend_Backend_DEFAULT, null_mut(), null_mut());
        let swap_chain = filament_Engine_createSwapChain1(engine, 100, 100, 0);
        let renderer = filament_Engine_createRenderer(engine);
        let view = filament_Engine_createView(engine);
        let scene = filament_Engine_createScene(engine);

        let entity_manager = filament_Engine_getEntityManager(engine);
        let mut buffer: [utils_Entity; 1] = [utils_Entity { mIdentity: 0 }; 1];
        utils_EntityManager_create(entity_manager, 1, buffer.as_mut_ptr());
        let camera_entity = buffer[0];

        let camera = filament_Engine_createCamera(engine, camera_entity);
        let aspect = 100.0 / 100.0;
        let zoom = 1.0;

        filament_Camera_setProjection(
            camera,
            filament_Camera_Projection_ORTHO,
            -aspect * zoom,
            aspect * zoom,
            -zoom,
            zoom,
            0.0,
            1.0,
        );

        let viewport = filament_Viewport {
            _base: filament_backend_Viewport {
                left: 0,
                bottom: 0,
                width: 100,
                height: 100,
            },
        };

        filament_View_setViewport(view, &viewport as *const _);
        filament_View_setScene(view, scene);
        filament_View_setCamera(view, camera);

        let clear_options = filament_Renderer_ClearOptions {
            clearColor: filament_math_float4 {
                inner: [0.0, 0.0, 1.0, 1.0],
            },
            clear: true,
            discard: true,
        };
        filament_Renderer_setClearOptions(renderer, &clear_options as *const _);

        filament_Renderer_beginFrame(renderer, swap_chain, 0);
        filament_Renderer_render(renderer, view);

        println!("123");

        const byte_count: usize = 100 * 100 * 4;
        let buffer = Box::new([0 as u8; byte_count]);
        let raw_buffer = Box::into_raw(buffer);
        let mut pixel = filament_backend_PixelBufferDescriptor {
            _base: filament_backend_BufferDescriptor {
                buffer: raw_buffer as *mut os::raw::c_void,
                size: byte_count as u64,
                mCallback: Some(pixelbuffer_read_callback),
                mUser: null_mut(),
                mHandler: null_mut(),
            },
            left: 0,
            top: 0,
            __bindgen_anon_1: filament_backend_PixelBufferDescriptor__bindgen_ty_1 {
                __bindgen_anon_1:
                    filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_1 {
                        stride: 0,
                        format: filament_backend_PixelDataFormat_RGBA,
                    },
            },
            _bitfield_1: filament_backend_PixelBufferDescriptor::new_bitfield_1(
                filament_backend_PixelDataType_UBYTE,
                1,
            ),
            ..Default::default()
        };
        filament_Renderer_readPixels(renderer, 0, 0, 100, 100, &mut pixel as *mut _);
        filament_Renderer_endFrame(renderer);

        loop {
            filament_Renderer_beginFrame(renderer, swap_chain, 0);
            filament_Renderer_render(renderer, view);
            filament_Renderer_endFrame(renderer);
        }
    }
}

unsafe extern "C" fn pixelbuffer_read_callback(
    buffer: *mut os::raw::c_void,
    _size: u64,
    _uesr: *mut os::raw::c_void,
) {
    let buffer = buffer as *mut u8;
    convert_rgba_to_rgb(buffer, 100, 100);

    let slice_u8 = slice::from_raw_parts(buffer, 100 * 100 * 3);

    let mut file = File::create("foo.ppm").unwrap();
    file.write(format!("P6 {} {} {}\n", 100, 100, 255).as_bytes()).unwrap();
    file.write_all(slice_u8).unwrap();
    file.flush().unwrap();

    println!("123");

    drop_in_place(buffer);
    dealloc(buffer, Layout::new::<[u8; 100 * 100 * 4]>());
}

unsafe fn convert_rgba_to_rgb(buffer: *mut u8, width: u32, height: u32) {
    let mut write_ptr = buffer;
    let mut read_ptr: *const u8 = buffer;

    let mut i: u32 = 0;
    let n = width * height;
    while i < n {
        write_ptr.write(*read_ptr);
        write_ptr.offset(1).write(*read_ptr.offset(1));
        write_ptr.offset(2).write(*read_ptr.offset(2));
        write_ptr = write_ptr.offset(3);
        read_ptr = read_ptr.offset(4);
        i += 1;
    }
}
