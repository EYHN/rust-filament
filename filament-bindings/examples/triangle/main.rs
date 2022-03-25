use std::{fs::File, io::Write};

use filament_bindings::{
    backend::{self, Backend, PixelBufferDescriptor},
    filament::{
        Bounds, ClearOptions, Engine, IndexBufferBuilder, IndexType, MaterialBuilder, Projection,
        RenderableBuilder, VertexAttribute, VertexBufferBuilder, Viewport,
    },
};

const MATERIAL_BYTES: &'static [u8] = include_bytes!("bakedColor.filamat");

fn main() {
    unsafe {
        let mut engine = Engine::create(Backend::OPENGL).unwrap();
        let mut swap_chain = engine
            .create_headless_swap_chain(800, 600, Default::default())
            .unwrap();
        let mut renderer = engine.create_renderer().unwrap();
        let mut view = engine.create_view().unwrap();
        let mut scene = engine.create_scene().unwrap();

        let mut entity_manager = engine.get_entity_manager().unwrap();
        let camera_entity = entity_manager.create();

        let mut camera = engine.create_camera(&camera_entity).unwrap();
        let aspect = 800.0 / 600.0;
        let zoom = 1.0;

        camera.set_projection(
            Projection::ORTHO,
            -aspect * zoom,
            aspect * zoom,
            -zoom,
            zoom,
            0.0,
            10.0,
        );

        let viewport = Viewport {
            left: 0,
            bottom: 0,
            width: 800,
            height: 600,
        };

        view.set_post_processing_enabled(false);
        view.set_viewport(&viewport);
        view.set_scene(&mut scene);
        view.set_camera(&mut camera);

        let clear_options = ClearOptions {
            clear_color: [0.0, 0.0, 1.0, 1.0].into(),
            clear: true,
            discard: true,
        };
        renderer.set_clear_options(&clear_options);

        let triangle = {
            let triangle_position: Vec<f32> = vec![
                1.0,
                0.0,
                f32::cos(std::f32::consts::PI * 2.0 / 3.0),
                f32::sin(std::f32::consts::PI * 2.0 / 3.0),
                f32::cos(std::f32::consts::PI * 4.0 / 3.0),
                f32::sin(std::f32::consts::PI * 4.0 / 3.0),
            ];
            let triangle_color: Vec<u32> = vec![0xffff0000, 0xff00ff00, 0xff0000ff];

            let mut vertex_buffer = {
                let mut vertex_buffer_builder = VertexBufferBuilder::new().unwrap();
                vertex_buffer_builder.vertex_count(3);
                vertex_buffer_builder.buffer_count(2);
                vertex_buffer_builder.attribute(
                    VertexAttribute::POSITION,
                    0,
                    backend::ElementType::FLOAT2,
                    0,
                    8,
                );
                vertex_buffer_builder.attribute(
                    VertexAttribute::COLOR,
                    1,
                    backend::ElementType::UBYTE4,
                    0,
                    4,
                );
                vertex_buffer_builder.normalized(VertexAttribute::COLOR, true);
                vertex_buffer_builder.build(&mut engine).unwrap()
            };
            vertex_buffer.set_buffer_at(
                &mut engine,
                0,
                backend::BufferDescriptor::new(triangle_position),
                0,
            );
            vertex_buffer.set_buffer_at(
                &mut engine,
                1,
                backend::BufferDescriptor::new(triangle_color),
                0,
            );

            let mut index_buffer = {
                let mut index_buffer_builder = IndexBufferBuilder::new().unwrap();
                index_buffer_builder.index_count(3);
                index_buffer_builder.buffer_type(IndexType::USHORT);
                index_buffer_builder.build(&mut engine).unwrap()
            };
            index_buffer.set_buffer(
                &mut engine,
                backend::BufferDescriptor::new(vec![0u16, 1u16, 2u16]),
                0,
            );

            let mut material_builder = MaterialBuilder::new().unwrap();
            material_builder.package(MATERIAL_BYTES);
            let mut material = material_builder.build(&mut engine).unwrap();
            let mut material_instance = material.get_default_instance().unwrap();

            let mut renderable_manager_builder = RenderableBuilder::new(1).unwrap();
            renderable_manager_builder.bounding_box(&mut Bounds {
                center: [-1.0, -1.0, -1.0].into(),
                half_extent: [1.0, 1.0, 1.0].into(),
            });
            renderable_manager_builder.material(0, &mut material_instance);
            renderable_manager_builder.geometry(
                0,
                backend::PrimitiveType::TRIANGLES,
                &mut vertex_buffer,
                &mut index_buffer,
            );
            let triangle = entity_manager.create();
            renderable_manager_builder.build(&mut engine, &triangle);
            triangle
        };

        scene.add_entity(&triangle);

        renderer.begin_frame(&mut swap_chain);

        println!("start rendering");

        renderer.render(&mut view);
        const BYTE_COUNT: usize = 800 * 600 * 4;
        let buffer = vec![0u8; BYTE_COUNT];
        let pixel = PixelBufferDescriptor::new_callback(
            buffer,
            backend::PixelDataFormat::RGBA,
            backend::PixelDataType::UBYTE,
            pixelbuffer_read_callback,
        );
        renderer.read_pixels(0, 0, 800, 600, pixel);

        renderer.end_frame();

        engine.flush_and_wait();

        Engine::destroy(&mut engine);
    }
}

fn pixelbuffer_read_callback(mut buffer: Vec<u8>) {
    unsafe {
        convert_rgba_to_rgb(buffer.as_mut_ptr(), 800, 600);
    }

    let slice_u8 = &buffer[..800 * 600 * 3];

    let mut file = File::create("triangle.ppm").unwrap();
    println!("The rendering result is written to triangle.ppm");
    file.write(format!("P6 {} {} {}\n", 800, 600, 255).as_bytes())
        .unwrap();
    file.write_all(slice_u8).unwrap();
    file.flush().unwrap();
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
