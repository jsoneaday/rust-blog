use std::borrow::Cow;

use regex::Regex;
use leptos::{logging::log, HtmlElement, html::{AnyElement, Ul, Ol, div, h1, h2, p, strong, ol, ul, li}};

pub struct MarkdownToHtmlConverter {
    pub heading_level_1_finder: Regex,
    pub heading_level_2_finder: Regex,
    pub paragraph_finder: Regex,
    pub ordered_list_finder: Regex,
    pub unordered_list_finder: Regex,
    pub bold_finder: Regex
}

impl MarkdownToHtmlConverter {
    pub fn new() -> Self {
        MarkdownToHtmlConverter {
            heading_level_1_finder: Regex::new(r"^\#{1}\s+").unwrap(),
            heading_level_2_finder: Regex::new(r"^\#{2}\s+").unwrap(),
            paragraph_finder: Regex::new(r"^\w+").unwrap(),
            ordered_list_finder: Regex::new(r"^\d+\.").unwrap(),
            unordered_list_finder: Regex::new(r"^\-\s").unwrap(),
            bold_finder: Regex::new(r"\b[\w\s]+\b").unwrap()
        }
    }

    pub fn convert_md_to_html(&self, md_string: String) -> Vec<HtmlElement<AnyElement>> {
        let md_lines = md_string.split('\n').collect::<Vec<&str>>();
        let mut html_lines: Vec<HtmlElement<AnyElement>> = vec![];

        let mut ol_started = false;
        let mut current_found_ol: Vec<HtmlElement<AnyElement>> = vec![];

        let mut ul_started = false;
        let mut current_found_ul: Vec<HtmlElement<AnyElement>> = vec![];

        for md_line in md_lines {            
            let line = md_line.trim();
            
            if self.ordered_list_finder.is_match(line) {
                if !ol_started {
                    ol_started = true;
                }

                current_found_ol.push(self.convert_md_line_to_html(line).unwrap());
            } else if self.unordered_list_finder.is_match(line) {
                if !ul_started {
                    ul_started = true;
                }

                current_found_ul.push(self.convert_md_line_to_html(line).unwrap());
            } else {
                if ol_started {
                    html_lines.push(ol().child(current_found_ol.clone()).into());
                    current_found_ol.clear();
                    ol_started = false;
                }
                if ul_started {
                    html_lines.push(ul().child(current_found_ul.clone()).into());
                    current_found_ul.clear();
                    ul_started = false;
                }

                let html_view = self.convert_md_line_to_html(line);
                if let Some(html_view) = html_view {
                    html_lines.push(html_view);
                } else {
                    let copyable_line = Cow::from(line);
                    html_lines.push(div().child(copyable_line).into());
                }
            }
        }
        html_lines
    }

    fn convert_md_line_to_html(&self, md_line: &str) -> Option<HtmlElement<AnyElement>> {
        if self.heading_level_1_finder.is_match(md_line) {            
            MarkdownToHtmlConverter::get_html_line_from_md(&self.heading_level_1_finder, md_line, TAG_NAME_H1)
        } else if self.heading_level_2_finder.is_match(md_line) {
            MarkdownToHtmlConverter::get_html_line_from_md(&self.heading_level_2_finder, md_line, TAG_NAME_H2)
        } else if self.ordered_list_finder.is_match(md_line) {
            MarkdownToHtmlConverter::get_html_line_from_md(&self.ordered_list_finder, md_line, TAG_NAME_OL)
        } else if self.unordered_list_finder.is_match(md_line) {
            MarkdownToHtmlConverter::get_html_line_from_md(&self.unordered_list_finder, md_line, TAG_NAME_UL)
        } else if self.paragraph_finder.is_match(md_line) {
            MarkdownToHtmlConverter::get_html_line_from_md(&self.paragraph_finder, md_line, TAG_NAME_P)
        } else {
            None
        }
    }

    fn get_html_line_from_md(regex: &Regex, line_to_check: &str, replacement_html: &str) -> Option<HtmlElement<AnyElement>> {
        log!("{} has matched for line: {}", replacement_html, line_to_check);
        if replacement_html == TAG_NAME_H1 {
            let new_line = regex.replace(line_to_check, "");
            Some(h1().child(new_line).into())
        } else if replacement_html == TAG_NAME_H2 {
            let new_line = regex.replace(line_to_check, "");
            Some(h2().child(new_line).into())
        } else if replacement_html == TAG_NAME_OL {
            let new_line = regex.replace(line_to_check, "");
            Some(li().child(new_line).into())
        } else if replacement_html == TAG_NAME_UL {
            let new_line = regex.replace(line_to_check, "");
            Some(li().child(new_line).into())
        } else if replacement_html == TAG_NAME_P {
            let new_line = Cow::from(line_to_check);
            Some(p().child(new_line).into())
        } else {            
            None
        }
    }
}

const TAG_NAME_H1: &str = "h1";
const TAG_NAME_H2: &str = "h2";
const TAG_NAME_P: &str = "p";
const TAG_NAME_OL: &str = "ol";
const TAG_NAME_UL: &str = "ul";
//const TAG_NAME_BOLD: &str = "strong";