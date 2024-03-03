use std::fmt::Display;

use super::{comment::Comments, tag::CloseType, Props, Style, Tag};

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNodes<'a> {
    /// ### template tag
    /// - `<template>`
    /// - `<script>`
    /// - `<style>`
    /// - `<any_component>`
    /// - ...
    Tag(Box<Tag<'a>>),
    /// ### Comment
    /// display everywhere
    /// - `///`
    /// - `//`
    /// - `//!`
    Comment(Comments<'a>),
    /// ### Style (Properties)
    /// - `.`
    /// - `#`
    /// - `&::`
    Style(Box<Style<'a>>),
}

#[allow(dead_code)]
impl<'a> ASTNodes<'a> {
    pub fn is_tag(&self) -> bool {
        matches!(self, Self::Tag(_))
    }
    pub fn is_comment(&self) -> bool {
        matches!(self, Self::Comment(_))
    }
    pub fn is_style(&self) -> bool {
        matches!(self, Self::Style(_))
    }
    pub fn set_tag_type(&mut self, ty: CloseType) {
        match self {
            ASTNodes::Tag(t) => t.set_ty(ty),
            _ => panic!("only ASTNodes::Tag can use `set_tag_type()`"),
        }
    }
    pub fn set_tag_properties(&mut self, props: Props<'a>) {
        match self {
            ASTNodes::Tag(t) => t.set_props(props),
            _ => panic!("only ASTNodes::Tag can use `set_tag_properties()`"),
        }
    }
    pub fn set_style_properties(&mut self, props: Props<'a>) {
        match self {
            ASTNodes::Style(s) => s.set_props(props),
            _ => panic!("only ASTNodes::Style can use `set_style_properties()`"),
        }
    }
    pub fn set_properties(&mut self, props: Props<'a>) {
        match self {
            ASTNodes::Tag(t) => t.set_props(props),
            ASTNodes::Comment(_) => {}
            ASTNodes::Style(s) => s.set_props(props),
        }
    }
    pub fn get_tag_name(&self) -> &str {
        match self {
            ASTNodes::Tag(t) => t.get_name(),
            _ => panic!("only ASTNodes::Tag can use `get_tag_name()`"),
        }
    }
    pub fn set_tag_children(&mut self, children: Vec<ASTNodes<'a>>) {
        match self {
            ASTNodes::Tag(t) => t.set_children(children),
            _ => panic!("only ASTNodes::Tag can use `set_tag_children()`"),
        }
    }
    pub fn set_style_children(&mut self, children: Vec<ASTNodes<'a>>) {
        match self {
            ASTNodes::Style(s) => s.set_children(children),
            _ => panic!("only ASTNodes::Style can use `set_style_children()`"),
        }
    }
    pub fn set_children(&mut self, children: Vec<ASTNodes<'a>>) {
        match self {
            ASTNodes::Tag(t) => t.set_children(children),
            ASTNodes::Comment(_) => {}
            ASTNodes::Style(s) => s.set_children(children),
        }
    }
    pub fn set_tag_parent(&mut self, parent: ASTNodes<'a>) {
        match self {
            ASTNodes::Tag(t) => t.set_parent(parent),
            _ => panic!("only ASTNodes::Tag can use `set_tag_parent()`"),
        }
    }
    pub fn set_parent(&mut self, parent: ASTNodes<'a>) {
        match self {
            ASTNodes::Tag(t) => t.set_parent(parent),
            ASTNodes::Comment(_) => {}
            ASTNodes::Style(s) => s.set_parent(parent),
        }
    }
    // pub fn parse_template(input:&str) -> Vec<ASTNodes>{
    //     parse_template(input)
    // }

    // pub fn parse(input:&str)->IResult<Vec<>>{

    // }
}

impl<'a> From<Tag<'a>> for ASTNodes<'a> {
    fn from(value: Tag<'a>) -> Self {
        ASTNodes::Tag(Box::new(value))
    }
}

impl<'a> From<Comments<'a>> for ASTNodes<'a> {
    fn from(value: Comments<'a>) -> Self {
        ASTNodes::Comment(value)
    }
}

impl<'a> From<Style<'a>> for ASTNodes<'a> {
    fn from(value: Style<'a>) -> Self {
        ASTNodes::Style(Box::new(value))
    }
}

impl<'a> Display for ASTNodes<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            ASTNodes::Tag(t) => t.to_string(),
            ASTNodes::Comment(c) => c.to_string(),
            ASTNodes::Style(s) => s.to_string(),
        };
        f.write_str(&res)
    }
}
