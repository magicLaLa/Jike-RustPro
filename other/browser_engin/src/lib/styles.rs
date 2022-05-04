/**
 * https://limpet.net/mbrubeck/2014/08/23/toy-layout-engine-4-style.html
 */
use std::collections::HashMap;

use crate::{
    css::{Rule, Selector, SimpleSelector, Specificity, Stylesheet, Value},
    dom::{ElementData, Node, NodeType},
};

pub type PropertyMap = HashMap<String, Value>;

#[derive(Debug)]
pub struct StyleNode<'a> {
    pub node: &'a Node,
    pub specified_values: PropertyMap,
    pub children: Vec<StyleNode<'a>>,
}

#[derive(Debug)]
pub enum Display {
    Inline,
    Block,
    None,
}

impl<'a> StyleNode<'a> {
    pub fn value(&self, name: &str) -> Option<Value> {
        self.specified_values.get(name).cloned()
    }

    pub fn lookup(&self, name: &str, fallback_name: &str, default: &Value) -> Value {
        self.value(name)
            .unwrap_or_else(|| self.value(fallback_name).unwrap_or_else(|| default.clone()))
    }

    pub fn display(&self) -> Display {
        if let Some(Value::KeyWord(s)) = self.value("display") {
            return match &*s {
                "block" => Display::Block,
                "none" => Display::None,
                _ => Display::Inline,
            };
        }
        Display::Inline
    }
}

pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a Stylesheet) -> StyleNode<'a> {
    StyleNode {
        node: root,
        specified_values: match root.node_type {
            NodeType::Element(ref elem) => specified_values(elem, stylesheet),
            NodeType::Text(_) => HashMap::default(),
        },
        children: root
            .children
            .iter()
            .map(|child| style_tree(child, stylesheet))
            .collect(),
    }
}

pub fn specified_values(elem: &ElementData, stylesheet: &Stylesheet) -> PropertyMap {
    let mut values = HashMap::new();
    let mut rules = matching_rules(elem, stylesheet);

    rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
    for (_, rule) in rules {
        for declaration in &rule.declarations {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }
    values
}

pub type MatchedRule<'a> = (Specificity, &'a Rule);

pub fn matching_rules<'a>(elem: &ElementData, stylesheet: &'a Stylesheet) -> Vec<MatchedRule<'a>> {
    stylesheet
        .rules
        .iter()
        .filter_map(|rule| match_rule(elem, rule))
        .collect()
}

pub fn match_rule<'a>(elem: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
    rule.selectors
        .iter()
        .find(|selector| matches(elem, *selector))
        .map(|selector| (selector.specificity(), rule))
}

pub fn matches(elem: &ElementData, selector: &Selector) -> bool {
    match *selector {
        Selector::Simple(ref simple_selector) => matches_simple_selector(elem, simple_selector),
    }
}

pub fn matches_simple_selector(elem: &ElementData, selector: &SimpleSelector) -> bool {
    if selector.tag_name.iter().any(|name| elem.tag_name != *name) {
        return false;
    }
    if selector.id.iter().any(|id| elem.id() != Some(id)) {
        return false;
    }

    let elem_classes = elem.classes();
    if selector
        .class
        .iter()
        .any(|class| !elem_classes.contains(&**class))
    {
        return false;
    }
    true
}
