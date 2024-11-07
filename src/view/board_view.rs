use sdl2::{pixels::Color, rect::Point, rect::Rect, render::Canvas, video::Window};

pub struct Renderer {
    pub screen_area: Rect,
    pub clear_color: Color,
}

impl Renderer {
    pub fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.clear_color);
        canvas.fill_rect(self.screen_area).ok().unwrap_or_default();

        canvas.set_draw_color(Color::RGB(0, 0, 0));

        let cell_width: i32 = self.screen_area.w / 5;
        let cell_height: i32 = self.screen_area.h / 5;

        for i in 0..5 {
            canvas
                .draw_line(
                    Point::new(cell_width / 2, cell_height / 2 + i * cell_height),
                    Point::new(
                        self.screen_area.w - cell_width / 2,
                        cell_height / 2 + i * cell_height,
                    ),
                )
                .ok()
                .unwrap();
            canvas
                .draw_line(
                    Point::new(cell_width / 2 + i * cell_width, cell_height / 2),
                    Point::new(
                        cell_width / 2 + i * cell_width,
                        self.screen_area.h - cell_height / 2,
                    ),
                )
                .ok()
                .unwrap()
        }
    }
}
