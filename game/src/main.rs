use graphics::ui;
use miniquad::*;
use stages::GameState;
use std::time::Instant;

mod components;
mod events;
mod graphics;
mod shaders;
mod stages;
mod systems;
mod types;
mod utils;

pub use types::{AssetIdentity, Color};

struct SnakeGame {
    egui_ctx: std::sync::Arc<egui::Context>,
    input: components::Input,
    painter: ui::Painter,
    raw_input: egui::RawInput,
    renderer: graphics::MainRenderer,
    stages: stages::StageStack,
    start_time: Instant,
}

impl SnakeGame {
    pub fn new(mut ctx: Context) -> Self {
        let egui_ctx = egui::Context::new();

        let pixels_per_point = ctx.dpi_scale();
        let (width, height) = ctx.screen_size();
        let painter = ui::Painter::new(&mut ctx);
        let mut renderer = graphics::MainRenderer::new(ctx);
        let screen_size = egui::vec2(width as f32, height as f32) / pixels_per_point;

        let raw_input = egui::RawInput {
            screen_size,
            pixels_per_point: Some(pixels_per_point),
            ..Default::default()
        };

        let mut input = components::Input::default();
        input.width = width;
        input.height = height;
        let mut stages = stages::new_stage_stack(16);
        let init_state = GameState::new(&input, &mut renderer);
        let game_stage = Box::new(init_state);

        stages.push(game_stage as Box<dyn stages::Stage>);

        SnakeGame {
            egui_ctx,
            painter,
            raw_input,
            start_time: Instant::now(),
            stages,
            renderer,
            input,
        }
    }
}

impl EventHandlerFree for SnakeGame {
    fn resize_event(&mut self, width: f32, height: f32) {
        self.input.width = width;
        self.input.height = height;
        self.input.resized = true;
        self.raw_input.screen_size = egui::vec2(width, height);
    }

    fn mouse_button_up_event(&mut self, _button: MouseButton, _x: f32, _y: f32) {
        self.raw_input.mouse_down = false;
    }

    fn mouse_button_down_event(&mut self, _button: MouseButton, _x: f32, _y: f32) {
        self.raw_input.mouse_down = true;
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.raw_input.mouse_pos = Some(egui::pos2(x, y));
    }

    fn update(&mut self) {
        let stage = match self.stages.last_mut() {
            Some(s) => s,
            _ => return,
        };
        let next_stage = stage.update(&self.input, &mut self.renderer);
        match next_stage {
            stages::NextStage::Push(mut new_stage) => {
                stage.exit(&mut self.renderer);
                new_stage.enter(&mut self.renderer);
                self.stages.push(new_stage);
            }
            stages::NextStage::Pop => {
                stage.exit(&mut self.renderer);
                self.stages.pop().expect("Popped an Empty StageStack");
                match self.stages.last_mut() {
                    Some(s) => s.enter(&mut self.renderer),
                    _ => {}
                };
            }
            _ => {}
        };

        self.renderer.load_assets();
        self.raw_input.time = self.start_time.elapsed().as_nanos() as f64 * 1e-9;
        self.input.reset();
    }

    fn draw(&mut self) {
        for stage in self.stages.iter_mut() {
            stage.draw(&mut self.renderer);
        }
        let ctx = &mut self.renderer.ctx;
        ctx.clear(Some((1., 1., 1., 1.)), None, None);
        ctx.begin_default_pass(miniquad::PassAction::clear_color(0., 0., 0., 1.));
        ctx.end_render_pass();

        self.egui_ctx.begin_frame(self.raw_input.take());
        egui::Window::new("Debug").default_size(egui::vec2(200., 100.)).show(&self.egui_ctx, |ui| {
            ui.add(egui::Label::new("Egui on Snek").text_style(egui::TextStyle::Heading));
            ui.separator();
            ui.label("Woooooh");
        });
        let (_, cmds) = self.egui_ctx.end_frame();
        let jobs = self.egui_ctx.tesselate(cmds);
        self.painter.paint(ctx, jobs, self.egui_ctx.texture());

        // self.renderer.draw();
    }

    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, repeat: bool) {
        if repeat {
            return;
        }

        match keycode {
            KeyCode::Left | KeyCode::A => {
                self.input.go_left = true;
            }
            KeyCode::Right | KeyCode::D => {
                self.input.go_right = true;
            }
            KeyCode::Up | KeyCode::W => {
                self.input.go_up = true;
            }
            KeyCode::Down | KeyCode::S => {
                self.input.go_down = true;
            }
            KeyCode::Escape => {
                self.input.pause = true;
                self.input.go_back = true;
            }
            _ => {}
        }
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), |ctx| {
        UserData::free(SnakeGame::new(ctx))
    });
}
