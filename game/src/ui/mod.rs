use std::fmt::Debug;

use crate::{graphics, types::{Color, Rect}};

pub struct Ui {
    pub draw_cmds: Vec<DrawCommand>,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            draw_cmds: Vec::with_capacity(512),
        }
    }

    pub fn push_cmd(&mut self, cmd: DrawCommand) {
        self.draw_cmds.push(cmd);
    }

    fn clear(&mut self) {
        self.draw_cmds.clear();
    }

    pub fn draw(&mut self, renderer: &mut graphics::MainRenderer) {
        std::mem::swap(&mut self.draw_cmds, &mut renderer.ui_draw_list);
        self.clear();
    }
}


#[derive(Debug, Clone)]
pub enum DrawCommand {
    // DrawText { text: String, font: AssetIdentity },
    DrawRect { rect: Rect, fill: Color },
    // DrawLine { start: glam::Vec2, end: glam::Vec2, color: Color },
    // Clip(Rect),
}

impl DrawCommand {
    // pub fn draw_text(text: String, font: AssetIdentity) -> Self {
    //     DrawCommand::DrawText { text, font }
    // }

    pub fn draw_rect(rect: Rect, fill: Color) -> Self {
        DrawCommand::DrawRect { rect, fill }
    }

    // pub fn draw_line( start: glam::Vec2, end: glam::Vec2, color: Color) -> Self {
    //     DrawCommand::DrawLine { start, end, color }
    // }

    // pub fn clip(rect: Rect) -> Self {
    //     DrawCommand::Clip(rect)
    // }
}