use crate::{
    css::{Unit, Value},
    styles::{self, Display, StyleNode},
};

/**
 * https://limpet.net/mbrubeck/2014/09/08/toy-layout-engine-5-boxes.html
 */
pub use self::BoxType::{AnonymousBlock, BlockNode, InlineNode};
pub use self::Value::{KeyWord, Length};

#[derive(Debug, Default, Clone, Copy)]
pub struct React {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl React {
    pub fn expand_by(self, edge: EdgeSizes) -> Self {
        Self {
            x: self.x - edge.left,
            y: self.y - edge.top,
            width: self.width + edge.left + edge.right,
            height: self.height + edge.top + edge.bottom,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct EdgeSizes {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Dimensions {
    pub content: React,
    pub padding: EdgeSizes,
    pub border: EdgeSizes,
    pub margin: EdgeSizes,
}

impl Dimensions {
    pub fn padding_box(self) -> React {
        self.content.expand_by(self.padding)
    }

    pub fn border_box(self) -> React {
        self.padding_box().expand_by(self.border)
    }

    pub fn margin_box(self) -> React {
        self.border_box().expand_by(self.margin)
    }
}

#[derive(Debug)]
pub struct LayoutBox<'a> {
    pub dimensions: Dimensions,
    pub box_type: BoxType<'a>,
    pub children: Vec<LayoutBox<'a>>,
}

#[derive(Debug)]
pub enum BoxType<'a> {
    BlockNode(&'a StyleNode<'a>),
    InlineNode(&'a StyleNode<'a>),
    AnonymousBlock,
}

impl<'a> LayoutBox<'a> {
    pub fn new(box_type: BoxType) -> LayoutBox {
        LayoutBox {
            dimensions: Dimensions::default(),
            box_type,
            children: Vec::new(),
        }
    }

    pub fn get_style_node(&self) -> &'a StyleNode<'a> {
        match self.box_type {
            BlockNode(node) | InlineNode(node) => node,
            AnonymousBlock => todo!(),
        }
    }
}

impl<'a> LayoutBox<'a> {
    pub fn layout(&mut self, containing_block: Dimensions) {
        match self.box_type {
            BlockNode(_) => self.layout_block(containing_block),
            InlineNode(_) | AnonymousBlock => {
                // TODO:
            }
        }
    }

    pub fn layout_block(&mut self, containing_block: Dimensions) {
        // Child width can depend on parent width, so we need to calculate
        // this box's width before laying out its children.
        self.calculate_block_width(containing_block);

        // Determine where the box is located within its container.
        self.calculate_block_position(containing_block);

        // Recursively lay out the children of this box.
        self.layout_block_children();

        // Parent height can depend on child height, so `calculate_height`
        // must be called *after* the children are laid out.
        self.calculate_block_height();
    }

    pub fn calculate_block_width(&mut self, containing_block: Dimensions) {
        let style = self.get_style_node();
        let auto = KeyWord("auto".to_string());
        let mut width = style.value("width").unwrap_or(auto.clone());

        let zero = Length(0.0, Unit::Px);
        let mut margin_left = style.lookup("margin_left", "margin", &zero);
        let mut margin_right = style.lookup("margin_right", "margin", &zero);
        let mut border_left = style.lookup("border_left_width", "border_width", &zero);
        let mut border_right = style.lookup("border_right_width", "border_width", &zero);
        let mut padding_left = style.lookup("padding_left", "padding", &zero);
        let mut padding_right = style.lookup("padding_right", "padding", &zero);

        let total: f32 = [
            &margin_left,
            &margin_right,
            &border_left,
            &border_right,
            &padding_left,
            &padding_right,
            &width,
        ]
        .iter()
        .map(|v| v.to_px())
        .sum();

        if width != auto && total > containing_block.content.width {
            if margin_left == auto {
                margin_left = Length(0.0, Unit::Px);
            }
            if margin_right == auto {
                margin_right = Length(0.0, Unit::Px);
            }
        }

        let underflow = containing_block.content.width - total;

        match (width == auto, margin_left == auto, margin_right == auto) {
            (false, false, false) => {
                margin_right = Length(margin_right.to_px() + underflow, Unit::Px);
            }
            (false, false, true) => {
                margin_right = Length(underflow, Unit::Px);
            }
            (false, true, false) => {
                margin_left = Length(underflow, Unit::Px);
            }
            (true, _, _) => {
                if margin_left == auto {
                    margin_left = Length(0.0, Unit::Px);
                }
                if margin_right == auto {
                    margin_right = Length(0.0, Unit::Px);
                }

                if underflow >= 0.0 {
                    width = Length(underflow, Unit::Px);
                } else {
                    width = Length(0.0, Unit::Px);
                    margin_right = Length(margin_right.to_px(), Unit::Px);
                }
            }
            (false, true, true) => {
                margin_left = Length(underflow / 2.0, Unit::Px);
                margin_right = Length(underflow / 2.0, Unit::Px);
            }
        }

        let d = &mut self.dimensions;
        d.content.width = width.to_px();
        d.padding.left = padding_left.to_px();
        d.padding.right = padding_right.to_px();
        d.border.left = border_left.to_px();
        d.border.right = border_right.to_px();
        d.margin.left = margin_left.to_px();
        d.margin.right = margin_right.to_px();
    }

    fn calculate_block_position(&mut self, containing_block: Dimensions) {
        let style = self.get_style_node();
        let d = &mut self.dimensions;

        // margin, border, and padding have initial value 0.
        let zero = Length(0.0, Unit::Px);

        // If margin-top or margin-bottom is `auto`, the used value is zero.
        d.margin.top = style.lookup("margin-top", "margin", &zero).to_px();
        d.margin.bottom = style.lookup("margin-bottom", "margin", &zero).to_px();

        d.border.top = style
            .lookup("border-top-width", "border-width", &zero)
            .to_px();
        d.border.bottom = style
            .lookup("border-bottom-width", "border-width", &zero)
            .to_px();

        d.padding.top = style.lookup("padding-top", "padding", &zero).to_px();
        d.padding.bottom = style.lookup("padding-bottom", "padding", &zero).to_px();

        d.content.x = containing_block.content.x + d.margin.left + d.border.left + d.padding.left;

        // Position the box below all the previous boxes in the container.
        d.content.y = containing_block.content.height
            + containing_block.content.y
            + d.margin.top
            + d.border.top
            + d.padding.top;
    }

    pub fn layout_block_children(&mut self) {
        let d = &mut self.dimensions;
        for child in &mut self.children {
            child.layout(*d);
            d.content.height = d.content.height + child.dimensions.margin_box().height;
        }
    }

    pub fn calculate_block_height(&mut self) {
        if let Some(Length(h, Unit::Px)) = self.get_style_node().value("height") {
            self.dimensions.content.height = h;
        }
    }

    pub fn get_inline_container(&mut self) -> &mut LayoutBox<'a> {
        match self.box_type {
            InlineNode(_) | AnonymousBlock => self,
            BlockNode(_) => {
                match self.children.last() {
                    Some(&LayoutBox {
                        box_type: AnonymousBlock,
                        ..
                    }) => {}
                    _ => self.children.push(LayoutBox::new(AnonymousBlock)),
                }
                self.children.last_mut().unwrap()
            }
        }
    }
}

pub fn build_layout_tree<'a>(style_node: &'a StyleNode<'a>) -> LayoutBox<'a> {
    let mut root = LayoutBox::new(match style_node.display() {
        Display::Inline => InlineNode(style_node),
        Display::Block => BlockNode(style_node),
        Display::None => panic!("Root node has display: none"),
    });
    for child in &style_node.children {
        match child.display() {
            Display::Block => root.children.push(build_layout_tree(child)),
            Display::Inline => root
                .get_inline_container()
                .children
                .push(build_layout_tree(child)),
            Display::None => {
                // TODO:
            }
        }
    }
    root
}

pub fn layout_tree<'a>(node: &'a StyleNode<'a>, mut containing_block: Dimensions) -> LayoutBox<'a> {
    // The layout algorithm expects the container height to start at 0.
    // TODO: Save the initial containing block height, for calculating percent heights.
    containing_block.content.height = 0.0;

    let mut root_box = build_layout_tree(node);
    root_box.layout(containing_block);
    root_box
}
