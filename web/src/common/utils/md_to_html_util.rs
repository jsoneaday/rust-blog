use std::{borrow::Cow, ops::Deref};

use regex::Regex;
use leptos::{logging::log, HtmlElement, html::{AnyElement, div, h1, h2, p, strong, span, ol, ul, li}, ev::ended};

pub struct MarkdownToHtmlConverter {
    pub heading_level_1_finder: Regex,
    pub heading_level_2_finder: Regex,
    pub ordered_list_finder: Regex,
    pub unordered_list_finder: Regex,
    /// make certain ol and ul are searched before paragraph
    pub paragraph_finder: Regex,    
    pub bold_finder: Regex,
    pub started_bold_finder: Regex,
    pub ended_bold_finder: Regex,
}

impl MarkdownToHtmlConverter {
    pub fn new() -> Self {
        MarkdownToHtmlConverter {
            heading_level_1_finder: Regex::new(r"^\#{1}\s+").unwrap(),
            heading_level_2_finder: Regex::new(r"^\#{2}\s+").unwrap(),
            ordered_list_finder: Regex::new(r"^\d+\.").unwrap(),
            unordered_list_finder: Regex::new(r"^\-\s").unwrap(),
            paragraph_finder: Regex::new(r"^\w+").unwrap(),            
            bold_finder: Regex::new(r"\*{2}[\w\s]+\*{2}").unwrap(),            
            started_bold_finder: Regex::new(r"\*{2}[\w\s]+").unwrap(),
            ended_bold_finder: Regex::new(r"[\w\s]+\*{2}").unwrap()
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

                current_found_ol.push(self.convert_md_to_html_element(line).unwrap());
            } else if self.unordered_list_finder.is_match(line) {
                if !ul_started {
                    ul_started = true;
                }

                current_found_ul.push(self.convert_md_to_html_element(line).unwrap());
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

                let html_view = self.convert_md_to_html_element(line);
                if let Some(html_view) = html_view {
                    html_lines.push(html_view);
                } else {
                    let copyable_line = Cow::from(line);
                    html_lines.push(div().child(copyable_line).into());
                }
            }
            // div().child(child)
            // div().child(("hello", strong().child("Greg"), "how are you"));
            // div().children()
        }
        html_lines
    }

    fn convert_md_to_html_element(&self, md_line: &str) -> Option<HtmlElement<AnyElement>> {
        if self.heading_level_1_finder.is_match(md_line) {            
            self.get_html_element_from_md(&self.heading_level_1_finder, md_line, TAG_NAME_H1)
        } else if self.heading_level_2_finder.is_match(md_line) {
            self.get_html_element_from_md(&self.heading_level_2_finder, md_line, TAG_NAME_H2)
        } else if self.ordered_list_finder.is_match(md_line) {
            self.get_html_element_from_md(&self.ordered_list_finder, md_line, TAG_NAME_OL)
        } else if self.unordered_list_finder.is_match(md_line) {
            self.get_html_element_from_md(&self.unordered_list_finder, md_line, TAG_NAME_UL)
        } else if self.paragraph_finder.is_match(md_line) {
            self.get_html_element_from_md(&self.paragraph_finder, md_line, TAG_NAME_P)
        } else {
            self.get_html_element_from_md(&self.paragraph_finder, md_line, TAG_NAME_NONE)
        }
    }

    fn get_html_element_from_md(&self, regex: &Regex, line_to_check: &str, replacement_html: &str) -> Option<HtmlElement<AnyElement>> {
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
            Some(self.get_html_element_inside_md(line_to_check, TAG_NAME_P))
        } else {
            Some(self.get_html_element_inside_md(line_to_check, TAG_NAME_NONE))
        }
    }

    fn get_html_element_inside_md(&self, line_to_check: &str, parent_html: &str) -> HtmlElement<AnyElement> {        
        let mut html_items: Vec<HtmlElement<AnyElement>> = vec![];
        let mut bold_items: Vec<HtmlElement<AnyElement>> = vec![];
        
        // create bold html elements
        for caps in self.bold_finder.captures_iter(line_to_check) {            
            for captured_item in caps.iter() {
                let match_item = captured_item.unwrap();
                let bold_item = match_item.as_str();

                let bold_item_str = format!("{}{}", bold_item.replace("**", ""), " ");
                bold_items.push(strong().child(bold_item_str.to_string()).into()); 
                log!("bold_item_str {:?}", bold_item_str);
            }                           
        }        
        
        // replace tagged items
        let mut line_items = line_to_check.split(' ').collect::<Vec<&str>>();
        log!("lines split by word: {:?}", line_items);
        let mut bold_count = 0;        
        let mut started_bold = false;
        let mut ended_bold = false;
        for item in line_items {
            if self.bold_finder.is_match(item) {
                html_items.push(bold_items[bold_count].clone());                
                bold_count += 1;
            } else if self.started_bold_finder.is_match(item) {
                log!("started bold found: {}", item);
                if !started_bold {
                    ended_bold = false;
                    started_bold = true;
                }
            } else if self.ended_bold_finder.is_match(item) {
                log!("ended bold found: {}", item);
                if !ended_bold {
                    ended_bold = true;
                    started_bold = false;
                }

                let bold_element = bold_items[bold_count].clone();                                
                let ends_with_asterisk = !item.ends_with("*");
                if ends_with_asterisk {
                    bold_element.set_inner_text(bold_element.inner_text().trim());
                }
                html_items.push(bold_element.clone());
                // last character may be a separator like comma           
                if ends_with_asterisk {
                    let str_index = item.len() - 1;
                    let final_str = format!("{} ", item.to_string().as_bytes()[str_index] as char);
                    html_items.push(span().child(final_str).into());                    
                }                     

                bold_count += 1;
            } else {
                if ended_bold {
                    log!("ended bold: {}", item);
                    let txt_item = Cow::from(format!("{}{}", item.deref(), " "));
                    html_items.push(span().child(txt_item).into());

                    started_bold = false;
                    ended_bold = false;
                } else if started_bold {
                    log!("started bold: {}", item);
                    // if bold has started nothing should be written until end
                } else {
                    log!("add text: {}", item);
                    let txt_item = Cow::from(format!("{}{}", item.deref(), " "));
                    html_items.push(span().child(txt_item).into());

                    started_bold = false;
                    ended_bold = false;
                }
            }
        }

        if parent_html == TAG_NAME_P {
            p().child(html_items).into()
        } else {
            div().child(html_items).into()
        }        
    }
}

const TAG_NAME_H1: &str = "h1";
const TAG_NAME_H2: &str = "h2";
const TAG_NAME_P: &str = "p";
const TAG_NAME_OL: &str = "ol";
const TAG_NAME_UL: &str = "ul";
// const TAG_NAME_BOLD: &str = "strong";
const TAG_NAME_NONE: &str = "";