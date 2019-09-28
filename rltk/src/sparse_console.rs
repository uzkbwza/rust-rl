use super::{gui_helpers, rex::XpColor, rex::XpLayer, Console, Font, Shader, RGB};
//use glow::types::*;
use glow::HasContext;
use std::mem;

/// Internal storage structure for sparse tiles.
pub struct SparseTile {
    pub idx: usize,
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
}

/// A sparse console. Rather than storing every cell on the screen, it stores just cells that have
/// data.
pub struct SparseConsole {
    pub width: u32,
    pub height: u32,

    // Private
    tiles: Vec<SparseTile>,
    is_dirty: bool,

    // To handle offset tiles for people who want thin walls between tiles
    offset_x: f32,
    offset_y: f32,

    // GL Stuff
    vertex_buffer: Vec<f32>,
    index_buffer: Vec<i32>,

    #[cfg(not(target_arch = "wasm32"))]
    vbo: u32,

    #[cfg(not(target_arch = "wasm32"))]
    vao: u32,

    #[cfg(not(target_arch = "wasm32"))]
    ebo: u32,

    #[cfg(target_arch = "wasm32")]
    vbo: glow::WebBufferKey,

    #[cfg(target_arch = "wasm32")]
    vao: glow::WebVertexArrayKey,

    #[cfg(target_arch = "wasm32")]
    ebo: glow::WebBufferKey,
}

impl SparseConsole {
    /// Initializes the console.
    pub fn init(width: u32, height: u32, gl: &glow::Context) -> Box<SparseConsole> {
        // Console backing init

        let (vbo, vao, ebo) = SparseConsole::init_gl_for_console(gl);

        let new_console = SparseConsole {
            width,
            height,
            vbo,
            vao,
            ebo,
            tiles: Vec::new(),
            is_dirty: true,
            vertex_buffer: Vec::new(),
            index_buffer: Vec::new(),
            offset_x: 0.0,
            offset_y: 0.0,
        };

        Box::new(new_console)
    }

    /// Initializes OpenGL for the sparse console.
    #[cfg(not(target_arch = "wasm32"))]
    fn init_gl_for_console(gl: &glow::Context) -> (u32, u32, u32) {
        let (vbo, vao, ebo);

        unsafe {
            // Generate buffers and arrays, as well as attributes.
            vao = gl.create_vertex_array().unwrap();
            vbo = gl.create_buffer().unwrap();
            ebo = gl.create_buffer().unwrap();

            gl.bind_vertex_array(Some(vao));

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

            let stride = 11 * mem::size_of::<f32>() as i32;
            // position attribute
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, stride, 0);
            gl.enable_vertex_attrib_array(0);
            // color attribute
            gl.vertex_attrib_pointer_f32(
                1,
                3,
                glow::FLOAT,
                false,
                stride,
                (3 * mem::size_of::<f32>()) as i32,
            );
            gl.enable_vertex_attrib_array(1);
            // bgcolor attribute
            gl.vertex_attrib_pointer_f32(
                2,
                3,
                glow::FLOAT,
                false,
                stride,
                (6 * mem::size_of::<f32>()) as i32,
            );
            gl.enable_vertex_attrib_array(2);
            // texture coord attribute
            gl.vertex_attrib_pointer_f32(
                3,
                2,
                glow::FLOAT,
                false,
                stride,
                (9 * mem::size_of::<f32>()) as i32,
            );
            gl.enable_vertex_attrib_array(3);
        };

        (vbo, vao, ebo)
    }

    #[cfg(target_arch = "wasm32")]
    fn init_gl_for_console(
        gl: &glow::Context,
    ) -> (
        glow::WebBufferKey,
        glow::WebVertexArrayKey,
        glow::WebBufferKey,
    ) {
        let (vbo, vao, ebo);

        unsafe {
            // Generate buffers and arrays, as well as attributes.
            vao = gl.create_vertex_array().unwrap();
            vbo = gl.create_buffer().unwrap();
            ebo = gl.create_buffer().unwrap();

            gl.bind_vertex_array(Some(vao));

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

            let stride = 11 * mem::size_of::<f32>() as i32;
            // position attribute
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, stride, 0);
            gl.enable_vertex_attrib_array(0);
            // color attribute
            gl.vertex_attrib_pointer_f32(
                1,
                3,
                glow::FLOAT,
                false,
                stride,
                (3 * mem::size_of::<f32>()) as i32,
            );
            gl.enable_vertex_attrib_array(1);
            // bgcolor attribute
            gl.vertex_attrib_pointer_f32(
                2,
                3,
                glow::FLOAT,
                false,
                stride,
                (6 * mem::size_of::<f32>()) as i32,
            );
            gl.enable_vertex_attrib_array(2);
            // texture coord attribute
            gl.vertex_attrib_pointer_f32(
                3,
                2,
                glow::FLOAT,
                false,
                stride,
                (9 * mem::size_of::<f32>()) as i32,
            );
            gl.enable_vertex_attrib_array(3);
        };

        (vbo, vao, ebo)
    }

    /// Helper to push a point to the shader.
    fn push_point(
        vertex_buffer: &mut Vec<f32>,
        x: f32,
        y: f32,
        fg: RGB,
        bg: RGB,
        ux: f32,
        uy: f32,
    ) {
        vertex_buffer.extend_from_slice(&[x, y, 0.0, fg.r, fg.g, fg.b, bg.r, bg.g, bg.b, ux, uy]);
    }

    /// Helper to build vertices for the sparse grid.
    fn rebuild_vertices(&mut self, gl: &glow::Context) {
        if self.tiles.is_empty() {
            return;
        }

        self.vertex_buffer.clear();
        self.index_buffer.clear();

        let glyph_size_x: f32 = 1.0 / 16.0;
        let glyph_size_y: f32 = 1.0 / 16.0;

        let step_x: f32 = 2.0 / self.width as f32;
        let step_y: f32 = 2.0 / self.height as f32;

        let mut index_count: i32 = 0;
        for t in &self.tiles {
            let x = t.idx % self.width as usize;
            let y = t.idx / self.width as usize;

            let screen_x = ((step_x * x as f32) - 1.0) + self.offset_x;
            let screen_y = ((step_y * y as f32) - 1.0) + self.offset_y;
            let fg = t.fg;
            let bg = t.bg;
            let glyph = t.glyph;
            let glyph_x = glyph % 16;
            let glyph_y = 16 - (glyph / 16);

            let glyph_left = f32::from(glyph_x) * glyph_size_x;
            let glyph_right = f32::from(glyph_x + 1) * glyph_size_x;
            let glyph_top = f32::from(glyph_y) * glyph_size_y;
            let glyph_bottom = f32::from(glyph_y - 1) * glyph_size_y;

            SparseConsole::push_point(
                &mut self.vertex_buffer,
                screen_x + step_x,
                screen_y + step_y,
                fg,
                bg,
                glyph_right,
                glyph_top,
            );
            SparseConsole::push_point(
                &mut self.vertex_buffer,
                screen_x + step_x,
                screen_y,
                fg,
                bg,
                glyph_right,
                glyph_bottom,
            );
            SparseConsole::push_point(
                &mut self.vertex_buffer,
                screen_x,
                screen_y,
                fg,
                bg,
                glyph_left,
                glyph_bottom,
            );
            SparseConsole::push_point(
                &mut self.vertex_buffer,
                screen_x,
                screen_y + step_y,
                fg,
                bg,
                glyph_left,
                glyph_top,
            );

            self.index_buffer.push(index_count);
            self.index_buffer.push(1 + index_count);
            self.index_buffer.push(3 + index_count);
            self.index_buffer.push(1 + index_count);
            self.index_buffer.push(2 + index_count);
            self.index_buffer.push(3 + index_count);

            index_count += 4;
        }

        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                &self.vertex_buffer.align_to::<u8>().1,
                glow::STATIC_DRAW,
            );

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                &self.index_buffer.align_to::<u8>().1,
                glow::STATIC_DRAW,
            );
        }
    }
}

impl Console for SparseConsole {
    /// If the console has changed, rebuild the vertex buffer.
    fn rebuild_if_dirty(&mut self, gl: &glow::Context) {
        if self.is_dirty {
            self.rebuild_vertices(gl);
            self.is_dirty = false;
        }
    }

    /// Draws the console to OpenGL.
    fn gl_draw(&mut self, font: &Font, shader: &Shader, gl: &glow::Context) {
        unsafe {
            // bind Texture
            font.bind_texture(gl);

            // render container
            shader.useProgram(gl);
            gl.bind_vertex_array(Some(self.vao));
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            gl.draw_elements(
                glow::TRIANGLES,
                (self.tiles.len() * 6) as i32,
                glow::UNSIGNED_INT,
                0,
            );
        }
        self.is_dirty = false;
    }

    /// Translates x/y to an index entry. Not really useful.
    fn at(&self, x: i32, y: i32) -> usize {
        (((self.height - 1 - y as u32) * self.width) + x as u32) as usize
    }

    /// Clear the screen.
    fn cls(&mut self) {
        self.is_dirty = true;
        self.tiles.clear();
    }

    /// Clear the screen. Since we don't HAVE a background, it doesn't use it.
    fn cls_bg(&mut self, _background: RGB) {
        self.is_dirty = true;
        self.tiles.clear();
    }

    /// Prints a string to an x/y position.
    fn print(&mut self, x: i32, y: i32, output: &str) {
        self.is_dirty = true;
        let mut idx = self.at(x, y);

        let bytes = super::string_to_cp437(output);

        self.tiles.extend(bytes.into_iter().map(|glyph| {
            let tile = SparseTile {
                idx,
                glyph,
                fg: RGB::from_f32(1.0, 1.0, 1.0),
                bg: RGB::from_f32(0.0, 0.0, 0.0),
            };
            idx += 1;
            tile
        }));
    }

    /// Prints a string to an x/y position, with foreground and background colors.
    fn print_color(&mut self, x: i32, y: i32, fg: RGB, bg: RGB, output: &str) {
        self.is_dirty = true;
        let mut idx = self.at(x, y);

        let bytes = super::string_to_cp437(output);
        self.tiles.extend(bytes.into_iter().map(|glyph| {
            let tile = SparseTile { idx, glyph, fg, bg };
            idx += 1;
            tile
        }));
    }

    /// Sets a single cell in the console
    fn set(&mut self, x: i32, y: i32, fg: RGB, bg: RGB, glyph: u8) {
        let idx = self.at(x, y);
        self.tiles.push(SparseTile { idx, glyph, fg, bg });
    }

    /// Sets a single cell in the console's background
    fn set_bg(&mut self, x: i32, y: i32, bg: RGB) {
        let idx = self.at(x, y);
        self.tiles[idx].bg = bg;
    }

    /// Draws a box, starting at x/y with the extents width/height using CP437 line characters
    fn draw_box(&mut self, sx: i32, sy: i32, width: i32, height: i32, fg: RGB, bg: RGB) {
        gui_helpers::draw_box(self, sx, sy, width, height, fg, bg);
    }

    /// Draws a box, starting at x/y with the extents width/height using CP437 double line characters
    fn draw_box_double(&mut self, sx: i32, sy: i32, width: i32, height: i32, fg: RGB, bg: RGB) {
        gui_helpers::draw_box_double(self, sx, sy, width, height, fg, bg);
    }

    /// Draws a horizontal progress bar
    fn draw_bar_horizontal(
        &mut self,
        sx: i32,
        sy: i32,
        width: i32,
        n: i32,
        max: i32,
        fg: RGB,
        bg: RGB,
    ) {
        gui_helpers::draw_bar_horizontal(self, sx, sy, width, n, max, fg, bg);
    }

    /// Draws a vertical progress bar
    fn draw_bar_vertical(
        &mut self,
        sx: i32,
        sy: i32,
        height: i32,
        n: i32,
        max: i32,
        fg: RGB,
        bg: RGB,
    ) {
        gui_helpers::draw_bar_vertical(self, sx, sy, height, n, max, fg, bg);
    }

    /// Prints text, centered to the whole console width, at vertical location y.
    fn print_centered(&mut self, y: i32, text: &str) {
        self.is_dirty = true;
        self.print(
            (self.width as i32 / 2) - (text.to_string().len() as i32 / 2),
            y,
            text,
        );
    }

    /// Prints text in color, centered to the whole console width, at vertical location y.
    fn print_color_centered(&mut self, y: i32, fg: RGB, bg: RGB, text: &str) {
        self.is_dirty = true;
        self.print_color(
            (self.width as i32 / 2) - (text.to_string().len() as i32 / 2),
            y,
            fg,
            bg,
            text,
        );
    }

    /// Saves the layer to an XpFile structure
    fn to_xp_layer(&self) -> XpLayer {
        let mut layer = XpLayer::new(self.width as usize, self.height as usize);

        // Clear all to transparent
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = layer.get_mut(x as usize, y as usize).unwrap();
                cell.bg = XpColor::TRANSPARENT;
            }
        }

        for c in &self.tiles {
            let x = c.idx % self.width as usize;
            let y = c.idx / self.width as usize;
            let cell = layer.get_mut(x as usize, y as usize).unwrap();
            cell.ch = u32::from(c.glyph);
            cell.fg = c.fg.to_xp();
            cell.bg = c.bg.to_xp();
        }

        layer
    }

    /// Sets an offset to total console rendering, useful for layers that
    /// draw between tiles. Offsets are specified as a percentage of total
    /// character size; so -0.5 will offset half a character to the left/top.
    fn set_offset(&mut self, x: f32, y: f32) {
        self.offset_x = x * (2.0 / self.width as f32);
        self.offset_y = y * (2.0 / self.height as f32);
    }
}
