use css::Unit::Px;
use css::Value::{Keyword, Length};
use std::default::Default;
use style::{Display, StyledNode};

pub use self::BoxType::{AnonymousBlock, BlockNode, InlineNode};

#[derive(Clone, Copy, Default, Debug)]
pub struct Dimensions {
    content: Rect,
    padding: EdgeSizes,
    border: EdgeSizes,
    margin: EdgeSizes,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}


#[derive(Clone, Copy, Default, Debug)]
pub struct EdgeSizes {
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
}

pub struct LayoutBox<'a> {
  dimensions: Dimensions,
  bot_type: BoxType<'a>,
  children: Vec<LayoutBox<'a>>,
}

pub enum BoxType<'a> {
  BlockNode(&'a StyledNode<'a>),
  InlineNode(&'a StyledNode<'a>),
  AnonymousBlock,
}

impl<'a> LayoutBox<'a> {
  fn new(box_type: BoxType) -> LayoutBox {
      LayoutBox {
          box_type: box_type,
          dimensions: Default::default(),
          children: Vec::new(),
      }
  }

  fn get_inline_container(&mut self) -> &mut LayoutBox {
    match self.box_type {
        InlineNode(_) | AnonymousBlock => self,
        BlockNode(_) => {
            match self.children.last() {
                Some(&LayoutBox { box_type: AnonymousBlock,..}) => {}
                _ => self.children.push(LayoutBox::new(AnonymousBlock))
            }
            self.children.last_mut().unwrap()
        }
    }
}

pub enum Display {
  Inline,
  Block,
  None,
}

impl StyledNode {
  fn value(&self, name: &str) -> Option<Value> {
      self.specified_values.get(name).map(|v| v.clone())
  }

  fn display(&self) -> Display {
      match self.value("display") {
          Some(Keyword(s)) => match &*s {
              "block" => Display::Block,
              "none" => Display::None,
              _ => Display::Inline
          },
          _ => Display::Inline
      }
  }
}

// Build the tree of LayoutBoxes
fn build_layout_tree<'a>(style_node: &'a StyledNode<'a>) -> LayoutBox<'a> {
  let mut root = LayoutBox::new(match style_node.display() {
      Block => BlockNode(style_node),
      Inline => InlineNode(style_node),
      DisplayNone => panic!("Root node has display: none.")
  });

  for child in &style_node.children {
      match child.display() {
          Block => root.children.push(build_layout_tree(child)),
          Inline => root.get_inline_container().children.push(build_layout_tree(child)),
          DisplayNone => {}
      }
  }
  return root;
}

