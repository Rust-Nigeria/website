use leptos::wasm_bindgen::JsCast;

use web_sys::{
    HtmlCanvasElement, WebGlBuffer, WebGlProgram, WebGlRenderingContext as Gl, WebGlUniformLocation,
};

use super::shaders::{FRAGMENT_SHADER, VERTEX_SHADER};
use crate::{
    components::button::ButtonColorVariants,
    utils::{
        resize_canvas_to_display_size::resize_canvas_to_display_size,
        webgl::{
            create_program, create_shader, get_canvas_to_clipspace_projection_matrix, set_quad,
        },
    },
};

#[derive(Clone)]
pub struct ButtonBackdropInstance {
    gl: Gl,
    canvas: HtmlCanvasElement,
    program: WebGlProgram,
    vertex_position_attribute_loc: i32,
    vertex_position_buffer: WebGlBuffer,
    canvas_projection_matrix_uniform_loc: WebGlUniformLocation,
    canvas_resolution_uniform_location: WebGlUniformLocation,
    extension_dimension_uniform_location: WebGlUniformLocation,
    progression_uniform_location: WebGlUniformLocation,
}

#[derive(Clone)]
pub struct ButtonBackdropBuilder {}

pub struct ButtonBackdropCfg {
    pub color: ButtonColorVariants,
}

impl ButtonBackdropBuilder {
    pub async fn load() -> Self {
        // We might need this for loading textures etc. I forsee needing this to load the arrow/icon texture maybe
        Self {}
    }

    pub fn create_backdrop(
        self,
        canvas: HtmlCanvasElement,
        cfg: ButtonBackdropCfg,
    ) -> ButtonBackdropInstance {
        ButtonBackdropInstance::new(canvas, cfg)
    }
}

impl ButtonBackdropInstance {
    pub fn new(canvas: HtmlCanvasElement, cfg: ButtonBackdropCfg) -> Self {
        let gl = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into::<Gl>()
            .unwrap();

        let vertex_shader = create_shader(&gl, Gl::VERTEX_SHADER, VERTEX_SHADER).unwrap();
        let fragment_shader = create_shader(&gl, Gl::FRAGMENT_SHADER, FRAGMENT_SHADER).unwrap();

        let program = create_program(&gl, &vertex_shader, &fragment_shader).unwrap();

        gl.pixel_storei(Gl::UNPACK_FLIP_Y_WEBGL, 1);

        let canvas_projection_matrix_uniform_loc = gl
            .get_uniform_location(&program, "u_canvasProjectionMatrix")
            .unwrap();

        let vertex_position_attribute_loc = gl.get_attrib_location(&program, "a_position");

        let vertex_position_buffer = gl.create_buffer().unwrap();

        let canvas_resolution_uniform_location = gl
            .get_uniform_location(&program, "u_canvas_resolution")
            .unwrap();

        let extension_dimension_uniform_location = gl
            .get_uniform_location(&program, "u_extension_dimension")
            .unwrap();

        let progression_uniform_location =
            gl.get_uniform_location(&program, "u_progression").unwrap();

        let base_color_uniform_location =
            gl.get_uniform_location(&program, "u_base_color").unwrap();

        let hover_color_uniform_location =
            gl.get_uniform_location(&program, "u_hover_color").unwrap();

        gl.use_program(Some(&program));

        let colors = cfg.color.get_state_rgbs();

        gl.uniform4fv_with_f32_array(Some(&base_color_uniform_location), &colors.base);
        gl.uniform4fv_with_f32_array(Some(&hover_color_uniform_location), &colors.hover);

        ButtonBackdropInstance {
            gl,
            canvas,
            program,
            vertex_position_attribute_loc,
            vertex_position_buffer,
            canvas_projection_matrix_uniform_loc,
            canvas_resolution_uniform_location,
            extension_dimension_uniform_location,
            progression_uniform_location,
        }
    }

    pub fn render(&self, progression: f32) {
        let ButtonBackdropInstance {
            gl,
            canvas,
            program,
            vertex_position_attribute_loc,
            vertex_position_buffer,
            canvas_projection_matrix_uniform_loc,
            canvas_resolution_uniform_location,
            extension_dimension_uniform_location,
            progression_uniform_location,
        } = self;

        gl.use_program(Some(program));

        resize_canvas_to_display_size(canvas);

        let canvas_width = canvas.width() as f32;
        let canvas_height = canvas.height() as f32;

        gl.uniform_matrix3fv_with_f32_array(
            Some(canvas_projection_matrix_uniform_loc),
            false,
            &get_canvas_to_clipspace_projection_matrix(canvas_width, canvas_height),
        );

        gl.uniform2fv_with_f32_array(
            Some(&canvas_resolution_uniform_location),
            &[canvas_width, canvas_height],
        );

        gl.uniform1f(Some(&extension_dimension_uniform_location), canvas_height);

        gl.uniform1f(Some(&progression_uniform_location), progression);

        gl.bind_buffer(Gl::ARRAY_BUFFER, Some(vertex_position_buffer));
        set_quad(gl, canvas_width, canvas_height);
        gl.enable_vertex_attrib_array(*vertex_position_attribute_loc as u32);
        gl.vertex_attrib_pointer_with_i32(
            *vertex_position_attribute_loc as u32,
            2,
            Gl::FLOAT,
            false,
            0,
            0,
        );

        gl.viewport(0, 0, canvas_width as i32, canvas_height as i32);

        gl.draw_arrays(Gl::TRIANGLES, 0, 6);
    }
}
