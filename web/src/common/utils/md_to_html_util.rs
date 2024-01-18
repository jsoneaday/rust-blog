use std::borrow::Cow;

use regex::Regex;
use leptos::{logging::log, HtmlElement, html::{AnyElement, div, h1, h2, p}};

pub struct MarkdownToHtmlConverter {
    pub heading_level_1_finder: Regex,
    pub heading_level_2_finder: Regex,
    pub paragraph_finder: Regex
}

impl MarkdownToHtmlConverter {
    pub fn new() -> Self {
        MarkdownToHtmlConverter {
            heading_level_1_finder: Regex::new(r"^\#{1}\s+").unwrap(),
            heading_level_2_finder: Regex::new(r"^\#{2}\s+").unwrap(),
            paragraph_finder: Regex::new(r"^\w+").unwrap()
        }
    }

    pub fn convert_md_to_html(&self, md_string: String) -> Vec<HtmlElement<AnyElement>> {
        let md_lines = md_string.split('\n').collect::<Vec<&str>>();
        let mut html_lines: Vec<HtmlElement<AnyElement>> = vec![];

        for md_line in md_lines {            
            let line = md_line.trim();
            html_lines.push(self.convert_md_line_to_html(line));
        }
        html_lines
    }

    fn convert_md_line_to_html(&self, md_line: &str) -> HtmlElement<AnyElement> {
        if self.heading_level_1_finder.is_match(md_line) {            
            MarkdownToHtmlConverter::get_html_from_md(&self.heading_level_1_finder, md_line, TAG_NAME_H1)
        } else if self.heading_level_2_finder.is_match(md_line) {
            MarkdownToHtmlConverter::get_html_from_md(&self.heading_level_2_finder, md_line, TAG_NAME_H2)
        } else if self.paragraph_finder.is_match(md_line) {
            MarkdownToHtmlConverter::get_html_from_md(&self.paragraph_finder, md_line, TAG_NAME_P)
        } else {
            log!("no matches on line: {}", md_line);
            let new_line = Cow::from(md_line);
            div().child(new_line).into()
        }
    }

    fn get_html_from_md(regex: &Regex, line_to_check: &str, replacement_html: &str) -> HtmlElement<AnyElement> {
        log!("{} has matched for line: {}", replacement_html, line_to_check);
        if replacement_html == TAG_NAME_H1 {
            let new_line = regex.replace(line_to_check, "");
            h1().child(new_line).into()
        } else if replacement_html == TAG_NAME_H2 {
            let new_line = regex.replace(line_to_check, "");
            h2().child(new_line).into()
        } else {
            let new_line = Cow::from(line_to_check);
            p().child(new_line).into()
        }
    }
}

const TAG_NAME_H1: &str = "h1";
const TAG_NAME_H2: &str = "h2";
const TAG_NAME_P: &str = "p";