/**
 * https://limpet.net/mbrubeck/2014/08/13/toy-layout-engine-3-css.html
 */
use std::string;

#[derive(Debug)]
struct Stylesheet {
    pub rules: Vec<Rule>,
}

#[derive(Debug)]
struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declarations>,
}

#[derive(Debug)]
pub enum Selector {
    Simple(SimpleSelector),
}

#[derive(Debug)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Option<String>,
}

#[derive(Debug)]
pub struct Declarations {
    name: String,
    value: Value,
}

#[derive(Debug)]
pub enum Value {
    KeyWord(String),
    Length(f32, Unit),
    ColorValue(Color),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Unit {
    Px,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Copy for Color {}

// TODO: 剩余 Parsing
