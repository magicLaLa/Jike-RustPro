use std::vec;

use crate::css::{Color, Value};

use super::layout::{BoxType, LayoutBox, React};

pub use self::BoxType::{AnonymousBlock, BlockNode, InlineNode};

/**
 * https://limpet.net/mbrubeck/2014/11/05/toy-layout-engine-7-painting.html
 */

pub type DisplayList = Vec<DisplayCommand>;

pub enum DisplayCommand {
    SolidColor(Color, React),
}

pub fn build_display_list(layout_root: &LayoutBox) -> DisplayList {
    let mut list = vec![];
    render_layout_box(&mut list, layout_root);
    list
}

pub fn render_layout_box(list: &mut DisplayList, layout_box: &LayoutBox) {
    render_background(list, layout_box);
    redner_borders(list, layout_box);
    // TODO: render text

    for child in &layout_box.children {
        render_layout_box(list, child);
    }
}

pub fn render_background(list: &mut DisplayList, layout_box: &LayoutBox) {
    if let Some(color) = get_color(layout_box, "background") {
        list.push(DisplayCommand::SolidColor(
            color,
            layout_box.dimensions.border_box(),
        ))
    }
}

pub fn get_color(layout_box: &LayoutBox, name: &str) -> Option<Color> {
    if let BlockNode(style) | InlineNode(style) = layout_box.box_type {
        return match style.value(name) {
            Some(Value::ColorValue(color)) => Some(color),
            _ => None,
        };
    }
    None
}

pub fn redner_borders(list: &mut DisplayList, layout_box: &LayoutBox) {
    let color = match get_color(layout_box, "border-color") {
        Some(color) => color,
        _ => return,
    };

    let d = &layout_box.dimensions;
    let border_box = d.border_box();

    list.push(DisplayCommand::SolidColor(
        color,
        React {
            x: border_box.x,
            y: border_box.y,
            width: d.border.left,
            height: border_box.height,
        },
    ));

    // Right border
    list.push(DisplayCommand::SolidColor(
        color,
        React {
            x: border_box.x + border_box.width - d.border.right,
            y: border_box.y,
            width: d.border.right,
            height: border_box.height,
        },
    ));

    // Top border
    list.push(DisplayCommand::SolidColor(
        color,
        React {
            x: border_box.x,
            y: border_box.y,
            width: border_box.width,
            height: d.border.top,
        },
    ));

    // Bottom border
    list.push(DisplayCommand::SolidColor(
        color,
        React {
            x: border_box.x,
            y: border_box.y + border_box.height - d.border.bottom,
            width: border_box.width,
            height: d.border.bottom,
        },
    ));
}

#[derive(Debug)]
pub struct Canvas {
    pub pixels: Vec<Color>,
    pub width: usize,
    pub height: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let white = Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        };
        Canvas {
            pixels: vec![white; width * height],
            width,
            height,
        }
    }

    pub fn paint_item(&mut self, item: &DisplayCommand) {
        match *item {
            DisplayCommand::SolidColor(color, react) => {
                let x0 = react.x.clamp(0.0, self.width as f32) as usize;
                let y0 = react.y.clamp(0.0, self.height as f32) as usize;
                let x1 = (react.x + react.width).clamp(0.0, self.width as f32) as usize;
                let y1 = (react.y + react.height).clamp(0.0, self.height as f32) as usize;

                for y in y0..y1 {
                    for x in x0..x1 {
                        self.pixels[y * self.width + x] = color;
                    }
                }
            }
        }
    }
}

pub fn paint(layout_root: &LayoutBox, bounds: React) -> Canvas {
  let display_list = build_display_list(layout_root);
  let mut canvas = Canvas::new(bounds.width as usize, bounds.height as usize);
  for item in display_list {
      canvas.paint_item(&item);
  }
  canvas
}