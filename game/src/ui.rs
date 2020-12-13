use std::rc::Rc;

use crate::components;
use megaui;

#[derive(Clone)]
pub struct Atlas;
impl megaui::CharacterAtlas for Atlas {
    fn get_advance(&self, _: char) -> f32 {
        0.
    }
    fn draw_character(
        &self,
        _: char,
        _: megaui::Vector2,
    ) -> Option<(megaui::Rect, megaui::Rect, f32)> {
        None
    }
}

pub struct WindowParams {
    pub label: String,
    pub movable: bool,
    pub close_button: bool,
    pub titlebar: bool,
}

impl Default for WindowParams {
    fn default() -> WindowParams {
        WindowParams {
            label: "".to_string(),
            movable: true,
            close_button: false,
            titlebar: true,
        }
    }
}

pub struct UiContext {
    ui: megaui::Ui,
    draw_list: Vec<megaui::DrawList>,
    input_processed_this_frame: bool,
}

impl UiContext {
    pub fn new() -> Self {
        let atlas = Rc::new(Atlas);
        let ui = megaui::Ui::new(atlas);
        let draw_list = Vec::with_capacity(64);

        Self {
            ui,
            draw_list,
            input_processed_this_frame: false,
        }
    }

    pub fn window<F: FnOnce(&mut megaui::Ui)>(
        &mut self,
        id: megaui::Id,
        position: glam::Vec2,
        size: glam::Vec2,
        _params: WindowParams,
        f: F,
    ) -> bool {
        let ui = &mut self.ui;

        megaui::widgets::Window::new(
            id,
            megaui::Vector2::new(position.x, position.y),
            megaui::Vector2::new(size.x, size.y),
        )
        .ui(ui, f)
    }

    pub fn process_input(&mut self, _input: &components::Input) {
        if self.input_processed_this_frame {
            return;
        }
        self.input_processed_this_frame = true;
    }

    pub fn draw(&mut self, delta: f32) {
        self.input_processed_this_frame = false;
        self.draw_list.clear();
        self.ui.render(&mut self.draw_list);
        let mut ui_draw_list = vec![];

        std::mem::swap(&mut ui_draw_list, &mut self.draw_list);

        self.ui.new_frame(delta);
    }
}
