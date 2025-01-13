use leptos::{
    html,
    prelude::{Get, NodeRef},
};

use wasm_bindgen::JsCast;
use web_sys::{
    HtmlCanvasElement, HtmlImageElement, WebGlBuffer, WebGlProgram, WebGlRenderingContext as GL,
    WebGlTexture, WebGlUniformLocation,
};

use crate::utils::{
    load_image::ImageFuture,
    resize_canvas_to_display_size::resize_canvas_to_display_size,
    webgl::{
        create_program, create_shader, get_canvas_to_clipspace_projection_matrix,
        get_texture_to_clipspace_projection_matrix, set_quad,
    },
};

use super::shaders::{FRAGMENT_SHADER, VERTEX_SHADER};

fn create_texture(gl: &GL) -> WebGlTexture {
    let texture = gl.create_texture().unwrap();
    gl.bind_texture(GL::TEXTURE_2D, Some(&texture));

    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as i32);

    texture
}

#[derive(Clone)]
pub struct Jigsaw {
    gl: GL,
    program: WebGlProgram,
    vertex_position_attribute_loc: i32,
    texture_position_attribute_loc: i32,
    vertex_position_buffer: WebGlBuffer,
    texture_position_buffer: WebGlBuffer,
    canvas_projection_matrix_uniform_loc: WebGlUniformLocation,
    canvas: HtmlCanvasElement,
    main_image: HtmlImageElement,
    duration_uniform_loc: WebGlUniformLocation,
}

impl Jigsaw {
    pub async fn new(canvas_ref: NodeRef<html::Canvas>, reveal_duration: f64) -> Self {
        let canvas = canvas_ref.get().unwrap();

        let gl = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into::<GL>()
            .unwrap();

        let vertex_shader = create_shader(&gl, GL::VERTEX_SHADER, VERTEX_SHADER).unwrap();
        let fragment_shader = create_shader(&gl, GL::FRAGMENT_SHADER, FRAGMENT_SHADER).unwrap();

        let program = create_program(&gl, &vertex_shader, &fragment_shader).unwrap();

        let canvas_projection_matrix_uniform_loc = gl
            .get_uniform_location(&program, "u_canvasProjectionMatrix")
            .unwrap();

        let texture_projection_matrix_uniform_loc = gl
            .get_uniform_location(&program, "u_textureProjectionMatrix")
            .unwrap();

        gl.pixel_storei(GL::UNPACK_FLIP_Y_WEBGL, 1);

        let main_image_uniform_loc = gl.get_uniform_location(&program, "u_mainImage").unwrap();

        let mask_image_uniform_loc = gl.get_uniform_location(&program, "u_maskImage").unwrap();

        let reveal_duration_uniform_loc = gl
            .get_uniform_location(&program, "u_revealDuration")
            .unwrap();

        let duration_uniform_loc = gl.get_uniform_location(&program, "u_duration").unwrap();

        let vertex_position_attribute_loc = gl.get_attrib_location(&program, "a_position");

        let texture_position_attribute_loc = gl.get_attrib_location(&program, "a_texCoord");

        let vertex_position_buffer = gl.create_buffer().unwrap();

        let texture_position_buffer = gl.create_buffer().unwrap();

        let main_image = ImageFuture::new("/assets/images/puzzle.jpg").await.unwrap();

        let mask_image = ImageFuture::new("/assets/images/puzzle-mask.png")
            .await
            .unwrap();

        gl.use_program(Some(&program));

        gl.uniform1f(Some(&reveal_duration_uniform_loc), reveal_duration as f32);

        gl.uniform_matrix3fv_with_f32_array(
            Some(&texture_projection_matrix_uniform_loc),
            false,
            &get_texture_to_clipspace_projection_matrix(
                main_image.width() as f32,
                main_image.height() as f32,
            ),
        );

        let main_image_texture = create_texture(&gl);
        gl.tex_image_2d_with_u32_and_u32_and_image(
            GL::TEXTURE_2D,
            0,
            GL::RGBA as i32,
            GL::RGBA,
            GL::UNSIGNED_BYTE,
            &main_image,
        )
        .unwrap();
        gl.uniform1i(Some(&main_image_uniform_loc), 0);

        let mask_image_texture = create_texture(&gl);
        gl.tex_image_2d_with_u32_and_u32_and_image(
            GL::TEXTURE_2D,
            0,
            GL::RGBA as i32,
            GL::RGBA,
            GL::UNSIGNED_BYTE,
            &mask_image,
        )
        .unwrap();
        gl.uniform1i(Some(&mask_image_uniform_loc), 1);

        gl.active_texture(GL::TEXTURE0);
        gl.bind_texture(GL::TEXTURE_2D, Some(&main_image_texture));

        gl.active_texture(GL::TEXTURE1);
        gl.bind_texture(GL::TEXTURE_2D, Some(&mask_image_texture));

        Jigsaw {
            gl,
            program,
            vertex_position_attribute_loc,
            texture_position_attribute_loc,
            vertex_position_buffer,
            texture_position_buffer,
            canvas_projection_matrix_uniform_loc,
            canvas,
            main_image,
            duration_uniform_loc,
        }
    }

    pub fn render(&self, duration: f64) {
        let Jigsaw {
            main_image,
            canvas,
            gl,
            program,
            vertex_position_attribute_loc,
            vertex_position_buffer,
            texture_position_buffer,
            texture_position_attribute_loc,
            canvas_projection_matrix_uniform_loc,
            duration_uniform_loc,
        } = self;

        gl.use_program(Some(&program));

        resize_canvas_to_display_size(&canvas);

        let canvas_width = canvas.width() as f32;
        let canvas_height = canvas.height() as f32;

        gl.uniform_matrix3fv_with_f32_array(
            Some(&canvas_projection_matrix_uniform_loc),
            false,
            &get_canvas_to_clipspace_projection_matrix(canvas_width, canvas_height),
        );

        gl.uniform1f(Some(&duration_uniform_loc), duration as f32);

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_position_buffer));
        set_quad(&gl, canvas_width, canvas_height);
        gl.enable_vertex_attrib_array(vertex_position_attribute_loc.clone() as u32);
        gl.vertex_attrib_pointer_with_i32(
            vertex_position_attribute_loc.clone() as u32,
            2,
            GL::FLOAT,
            false,
            0,
            0,
        );

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&texture_position_buffer));
        set_quad(&gl, main_image.width() as f32, main_image.height() as f32);
        gl.enable_vertex_attrib_array(texture_position_attribute_loc.clone() as u32);
        gl.vertex_attrib_pointer_with_i32(
            texture_position_attribute_loc.clone() as u32,
            2,
            GL::FLOAT,
            false,
            0,
            0,
        );

        gl.viewport(0, 0, canvas_width as i32, canvas_height as i32);

        gl.draw_arrays(GL::TRIANGLES, 0, 6);
    }
}
