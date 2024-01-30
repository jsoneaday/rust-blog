use std::borrow::Cow;
use regex::Regex;
use leptos::{logging::{log, warn}, HtmlElement, html::{AnyElement, A, div, h1, h2, p, strong, a, i, span, code, em, ol, ul, li}};

pub struct MarkdownToHtmlConverter {
    pub heading_level_1_finder: Regex,
    pub heading_level_2_finder: Regex,
    pub ordered_list_finder: Regex,
    pub unordered_list_finder: Regex,
    /// make certain ol and ul are searched before paragraph
    pub paragraph_finder: Regex,    
    pub bold_finder: Regex,
    pub starting_bold_finder: Regex,
    pub ending_bold_finder: Regex,
    pub italic_bold_finder: Regex,
    pub italic_finder: Regex,
    pub starting_italic_finder: Regex,
    pub ending_italic_finder: Regex,
    pub code_finder: Regex,
    pub starting_code_finder: Regex,
    pub ending_code_finder: Regex,
    pub white_space_counter_finder: Regex,
    /// Regex to find a markdown link
    pub link_finder: Regex,
    /// Will match only the name portion of a link markdown
    pub link_name_finder: Regex,
    /// Will match only the url portion of a link markdown
    pub link_url_finder: Regex,
    /// Finds only new line for entire line
    pub only_new_line_finder: Regex
}

impl MarkdownToHtmlConverter {
    pub fn new() -> Self {
        MarkdownToHtmlConverter {
            heading_level_1_finder: Regex::new(H1_REGEX).unwrap(),
            heading_level_2_finder: Regex::new(H2_REGEX).unwrap(),
            ordered_list_finder: Regex::new(r"^\d+\.").unwrap(),
            unordered_list_finder: Regex::new(r"^\-\s").unwrap(),
            paragraph_finder: Regex::new(r"^\w+").unwrap(),            
            bold_finder: Regex::new(r"\*{2}[\w\s]+\*{2}").unwrap(),            
            starting_bold_finder: Regex::new(r"\*{2}[\w\s]+").unwrap(),
            ending_bold_finder: Regex::new(r"[\w\s]+\*{2}").unwrap(),
            italic_bold_finder: Regex::new(r"\*{3}[\w\s]+\*{3}").unwrap(),     
            italic_finder: Regex::new(r"\*{1}[\w\s]+\*{1}").unwrap(),
            starting_italic_finder: Regex::new(r"\*{1}[\w\s]+").unwrap(),
            ending_italic_finder: Regex::new(r"[\w\s]+\*{1}").unwrap(),
            code_finder: Regex::new(r#"\`[\w\s\{\}\(\)<.*>\?\/\[\]\.\,\:\;\-\"]+\`|\`{2}[\w\s\{\}\(\)<.*>\?\/\[\]\.\,\:\;\-\"]+\`{2}"#).unwrap(),
            starting_code_finder: Regex::new(r#"\`[\w\s\{\}\(\)<.*>\?\/\[\]\.\,\:\;\-\"]+|\`{2}[\w\s\{\}\(\)<.*>\?\/\[\]\.\,\:\;\-\"]+"#).unwrap(),
            ending_code_finder: Regex::new(r#"[\w\s\{\}\(\)<.*>\?\/\[\]\.\,\:\;\-\"]+\`|[\w\s\{\}\(\)<.*>\?\/\[\]\.\,\:\;\-\"]+\`{2}"#).unwrap(),
            white_space_counter_finder: Regex::new(r"^\s").unwrap(),
            link_finder: Regex::new(r#"\[(.+)\]\(([^ ]+?)( "(.+)")?\)"#).unwrap(),
            link_name_finder: Regex::new(LINK_NAME_REGEX).unwrap(),
            link_url_finder: Regex::new(LINK_URL_REGEX).unwrap(),
            only_new_line_finder: Regex::new(r"^\s+$").unwrap()
        }
    }

    pub fn convert_md_to_html(&self, md_string: String) -> Vec<HtmlElement<AnyElement>> {
        let cloned_md_string = md_string.clone();
        let md_lines = cloned_md_string.split('\n').collect::<Vec<&str>>();
        let mut html_lines: Vec<HtmlElement<AnyElement>> = vec![];

        let mut ol_started = false;
        let mut gathered_ol: Vec<HtmlElement<AnyElement>> = vec![];
        let mut ul_started = false;
        let mut gathered_ul: Vec<HtmlElement<AnyElement>> = vec![];
        let mut code_started = false;
        let mut code_ended = false;
        let mut gathered_code: Vec<HtmlElement<AnyElement>> = vec![];

        let mut prior_line_carriage_return = false;
            
        for md_line in md_lines {                               
            let line = md_line.trim().to_string();            
            let line_str = line.as_str();
            let mut matched_sections: Vec<TypeElement> = vec![];   
                        
            // lists behave differently and must be aggregated
            if self.ordered_list_finder.is_match(line_str) {
                if !ol_started {
                    ol_started = true;
                }
                let new_line = self.ordered_list_finder.replace(line_str, "");
                gathered_ol.push(li().child(new_line).into());
            } else if self.unordered_list_finder.is_match(line_str) {
                if !ul_started {
                    ul_started = true;
                }
                let new_line = self.unordered_list_finder.replace(line_str, "");
                gathered_ul.push(li().child(new_line).into());
            } else if self.starting_code_finder.is_match(line_str) {
                if !code_started {
                    code_started = true;
                    code_ended = false;
                }
                log!("code started: {}", line_str);
                let new_line = self.starting_code_finder.replace(line_str, "");
                gathered_code.push(div().child(new_line).into());
            } else if self.ending_code_finder.is_match(line_str) {
                if !code_ended {
                    code_started = false;
                    code_ended = true;
                }
                log!("code ended: {}", line_str);
                let new_line = self.ending_code_finder.replace(line_str, "");
                gathered_code.push(div().child(new_line).into());
            } else {
                if ol_started {                   
                    matched_sections.push(TypeElement { section_type: SectionType::Ol, element: ol().child(gathered_ol.clone()).into() });
                    gathered_ol.clear();
                    ol_started = false;
                } else if ul_started {               
                    matched_sections.push(TypeElement { section_type: SectionType::Ul, element: ul().child(gathered_ul.clone()).into() });
                    gathered_ul.clear();
                    ul_started = false;
                }

                if code_started { // gets middle of code section
                    log!("code middle: {}", line_str);
                    gathered_code.push(div().child(line.clone()).into());
                } else if code_ended {
                    code_started = false;
                    code_ended = false;
                    log!("code complete: {}", line_str);
                    // code content parsing
                    // ßlet mut code_matched_sections = gathered_code.iter().map(|code_element| TypeElement { section_type: SectionType::String, element: code_element.clone() }).collect::<Vec<TypeElement>>();
                    // self.parse_convert_md_to_html(&mut code_matched_sections);

                    // let parsed_code_elements = code_matched_sections.iter().map(|parsed_element| parsed_element.element.clone()).collect::<Vec<HtmlElement<AnyElement>>>();
                    matched_sections.push(TypeElement { section_type: SectionType::Code, element: code().child(gathered_code.clone()).into() });                    
                    gathered_code.clear();
                }

                matched_sections.push(TypeElement { section_type: SectionType::String, element: div().child(line.clone()).into() });
                self.parse_convert_md_to_html(&mut matched_sections);
            }
                                  
            if prior_line_carriage_return || self.starting_code_finder.is_match(line_str){
                html_lines.push(p().child(matched_sections.iter().map(|type_element| type_element.element.clone()).collect::<Vec<HtmlElement<AnyElement>>>()).into());
                prior_line_carriage_return = false;
            } else {
                html_lines.push(div().child(matched_sections.iter().map(|type_element| type_element.element.clone()).collect::<Vec<HtmlElement<AnyElement>>>()).into());
            }            

            if line.is_empty() || self.only_new_line_finder.is_match(line_str) {
                prior_line_carriage_return = true;
            }
        }
        html_lines
    }

    fn parse_convert_md_to_html(&self, matched_sections: &mut Vec<TypeElement>) {        
        *matched_sections = self.get_html_element_from_md(&self.heading_level_1_finder, &*matched_sections, TAG_NAME_H1);
        *matched_sections = self.get_html_element_from_md(&self.heading_level_2_finder, &*matched_sections, TAG_NAME_H2);
        *matched_sections = self.get_html_element_from_md(&self.italic_bold_finder, &*matched_sections, TAG_NAME_ITALIC_BOLD);
        *matched_sections = self.get_html_element_from_md(&self.bold_finder, &*matched_sections, TAG_NAME_STRONG);
        *matched_sections = self.get_html_element_from_md(&self.italic_finder, &*matched_sections, TAG_NAME_ITALIC);
        *matched_sections = self.get_html_element_from_md(&self.link_finder, &*matched_sections, TAG_NAME_A);
    }

    fn get_html_element_from_md(&self, regex: &Regex, elements_to_check: &Vec<TypeElement>, replacement_html: &str) -> Vec<TypeElement> {
        let mut updated_elements: Vec<TypeElement> = vec![];        
        
        for element_to_check in elements_to_check {            
            let section_type = element_to_check.section_type.clone();
            let element = element_to_check.element.clone();            
            let element_inner_text = element.inner_text().clone();
            let element_inner_text = element_inner_text.as_str();

            if section_type != SectionType::Anchor &&
                section_type != SectionType::Italic &&
                section_type != SectionType::Strong &&
                section_type != SectionType::ItalicBold {            
                if regex.is_match(element_inner_text) && replacement_html == TAG_NAME_H1 {
                    let new_line = regex.replace(element_inner_text, "");
                    updated_elements.push(TypeElement { section_type: SectionType::H1, element: h1().child(new_line).into() });
                } 
                else if regex.is_match(element_inner_text) && replacement_html == TAG_NAME_H2 {
                    let new_line = regex.replace(element_inner_text, "");              
                    updated_elements.push(TypeElement { section_type: SectionType::H2, element: h2().child(new_line).into() });
                } 
                else if regex.is_match(element_inner_text) && replacement_html == TAG_NAME_ITALIC_BOLD {
                    let elements = self.get_html_element_from_md_line(regex, element_inner_text, &SectionType::ItalicBold, vec!["***"]);
                    if let Some(elements) = elements {
                        let element = element.inner_html("");
                        for element in elements {
                            updated_elements.push(TypeElement { section_type: element.section_type, element: element.element});
                        }                        
                    }
                } 
                else if regex.is_match(element_inner_text) && replacement_html == TAG_NAME_STRONG {                
                    let elements = self.get_html_element_from_md_line(regex, element_inner_text, &SectionType::Strong, vec!["**"]);
                    if let Some(elements) = elements {
                        let element = element.inner_html("");
                        for element in elements {
                            updated_elements.push(TypeElement { section_type: element.section_type, element: element.element});
                        }                     
                    }
                } 
                else if regex.is_match(element_inner_text) && replacement_html == TAG_NAME_ITALIC {   
                    let elements = self.get_html_element_from_md_line(regex, element_inner_text, &SectionType::Italic, vec!["*"]);
                    if let Some(elements) = elements {
                        let element = element.inner_html("");
                        for element in elements {
                            updated_elements.push(TypeElement { section_type: element.section_type, element: element.element});
                        }          
                    }
                }             
                else if regex.is_match(element_inner_text) && replacement_html == TAG_NAME_A {   
                    let elements = self.get_anchor_element_from_md_link(element_inner_text);
                    if let Some(elements) = elements {             
                        for element in elements {
                            updated_elements.push(TypeElement { section_type: element.section_type, element: element.element});
                        }   
                    }
                } 
                else {                
                    updated_elements.push(TypeElement { section_type, element });
                }
            } else {                
                updated_elements.push(TypeElement { section_type, element });
            }
        }        
        updated_elements
    }

    /// Returns an inline set of elements
    fn get_html_element_from_md_line(&self, regex: &Regex, line: &str, section_type: &SectionType, markdown: Vec<&str>) -> Option<Vec<TypeElement>> {
        let match_list: Vec<String> = get_list_of_regex_matching_content(regex, line, markdown);   
        let non_match_sections: Vec<String> = get_list_of_non_matching_content(regex, line);
        
        let mut elements: Vec<TypeElement> = vec![];
        if non_match_sections.len() == 0 { // if no non-match sections then entire line is
            elements.push(TypeElement { section_type: section_type.clone(), element: convert_matched_sections_to_html(match_list[0].clone(), None, section_type) });  
        } else {
            let mut match_list_index = 0;
            let mut line_starts_with_non_match_section = false;
            if line.starts_with(non_match_sections.get(0).unwrap()) {
                line_starts_with_non_match_section = true;
            }
            for non_match_section in non_match_sections {
                if line_starts_with_non_match_section {
                    elements.push(TypeElement { section_type: SectionType::String, element: convert_matched_sections_to_html(non_match_section.clone(), None, &SectionType::String) });
                }                    

                if let Some(next_match) = match_list.get(match_list_index) {
                    elements.push(TypeElement { section_type: section_type.clone(), element: convert_matched_sections_to_html(next_match.to_string(), None, section_type) });  
                }

                if !line_starts_with_non_match_section {
                    elements.push(TypeElement { section_type: SectionType::String, element: convert_matched_sections_to_html(non_match_section, None, &SectionType::String) });
                }  
                
                match_list_index += 1;
            }
            // set last html element if there is one
            if match_list_index < match_list.len() {
                let next_match = format!("{} ", match_list.get(match_list_index).unwrap());
                elements.push(TypeElement { section_type: section_type.clone(), element: convert_matched_sections_to_html(next_match, None, section_type) }); 
            }
        }
        if elements.len() == 0 {
            return None;
        }
        Some(elements)
    }

    /// Will get embedded anchor and other content as well
    fn get_anchor_element_from_md_link(&self, line: &str) -> Option<Vec<TypeElement>> {  
        let link_names_list: Vec<String> = get_list_of_regex_matching_content(&self.link_name_finder, line, vec!["[", "]"]);
        let link_url_list: Vec<String> = get_list_of_regex_matching_content(&self.link_url_finder, line, vec!["(", ")"]);
        if link_names_list.len() == 0 || link_url_list.len() == 0 {
            warn!("link names list and link url list seem not to match!");
            return None;
        }        
        let non_match_sections: Vec<String> = get_list_of_non_matching_content(&self.link_finder, line);
        
        let mut elements: Vec<TypeElement> = vec![];
        if non_match_sections.len() == 0 { // if no non-match sections then entire line is a link
            elements.push(TypeElement { section_type: SectionType::Anchor, element: setup_anchor(link_url_list[0].clone().as_str(), link_names_list[0].clone().as_str()).into() });  
        } else {
            let mut index = 0;
            let mut line_starts_with_non_match_section = false;
            if line.starts_with(non_match_sections.get(0).unwrap()) {
                line_starts_with_non_match_section = true;
            }
            for non_match_section in non_match_sections {
                if line_starts_with_non_match_section {
                    elements.push(TypeElement { section_type: SectionType::String, element: span().child(non_match_section.clone()).into() });
                }                    

                let anchor = set_anchor_element(&link_names_list, &link_url_list, index);
                if let Some(anchor) = anchor {
                    elements.push(TypeElement { section_type: SectionType::Anchor, element: anchor.into() });
                }

                if !line_starts_with_non_match_section {
                    elements.push(TypeElement { section_type: SectionType::String, element: span().child(non_match_section).into() });
                }  
                                                                                
                index += 1;
            }
            // set last anchor element if there is one
            if index < link_names_list.len() {
                let anchor = set_anchor_element(&link_names_list, &link_url_list, index);
                if let Some(anchor) = anchor {
                    elements.push(TypeElement { section_type: SectionType::Anchor, element: anchor.into() });
                }
            }
        }
        if elements.len() == 0 {
            return None;
        }
        Some(elements)
    }
}

fn set_anchor_element(link_names_list: &Vec<String>, link_url_list: &Vec<String>, index: usize) -> Option<HtmlElement<A>> {
    let mut anchor: Option<HtmlElement<A>> = None;
    if let Some(link_name_item) = link_names_list.get(index) {
        let next_link_name = format!("{}", link_name_item);

        if let Some(link_url_item) = link_url_list.get(index) {
            let next_link_url = format!("{}", link_url_item);

            anchor = setup_anchor(&next_link_url, &next_link_name).into();    
        } else {
            warn!("Cannot have a link name without a url");
            return None;
        }            
    } else if let Some(_link_url_item) = link_url_list.get(index) {
        warn!("Cannot have a link url without a name");
        return None;
    }
    anchor
}

fn prefix_nbsp_for_whitespace_count(affected_txt: String) -> String {
    let whitespace_count: usize = affected_txt
        .chars()
        .take_while(|ch| ch.is_whitespace() && *ch != '\n')
        .map(|ch| ch.len_utf8())
        .sum();
    let mut whitespace_count_half = whitespace_count as f32 / 2.0;
    whitespace_count_half = whitespace_count_half.round();

    let mut inner_txt = affected_txt.clone();
    for _ in 0..(whitespace_count_half as i32) {
        inner_txt = format!("{}{}", "&nbsp;", inner_txt);
    }
    inner_txt
}

/// This function strips out markdown tags and returns only the affected strings
/// * `md_start_str` - Beginning characters of a regex matching string
/// * `md_end_str` - Ending characters of a regex matching string
/// * `matching_str` - Matched string to extract content from
#[allow(unused)]
fn get_only_matching_content_wo_md(md_start_str: &str, md_end_str: &str, matching_str: &str) -> String {
    let mut content = matching_str.replace(md_start_str, "");
    content = content.replace(md_end_str, "");
    content
}

/// Returns only the content without the markdown
fn get_list_of_regex_matching_content(finder: &Regex, line: &str, markdown: Vec<&str>) -> Vec<String> {
    let mut list: Vec<String> = vec![];

    for found in finder.find_iter(line) {
        let mut found_content = found.as_str().to_string();
        for md in &markdown {
            found_content = found_content.replace(md, "");
        }
        list.push(found_content.to_string());
    }
    list
}

fn get_list_of_non_matching_content(finder: &Regex, line: &str) -> Vec<String> {
    let mut list: Vec<String> = vec![];

    for non_match in finder.split(line) {
        if non_match.len() > 0 {
            list.push(non_match.to_string());
        }
    }
    list
}

fn setup_anchor(link_url: &str, link_name: &str) -> HtmlElement<A> {
    let anchor = a();
    anchor.set_href(link_url.trim());
    anchor.set_inner_text(link_name.trim());
    anchor
}

fn convert_matched_sections_to_html(content: String, url: Option<String>, section_type: &SectionType) -> HtmlElement<AnyElement> {
    if section_type == &SectionType::Anchor && url == None {
        panic!("An anchor requires the url parameter");
    }
    
    match section_type {
        SectionType::Anchor => setup_anchor(url.unwrap().as_str(), content.as_str()).into(),
        SectionType::String => span().child(content).into(),
        SectionType::Paragraph => p().child(content).into(),
        SectionType::Strong => strong().child(content).into(),
        SectionType::Italic => i().child(content).into(),
        SectionType::ItalicBold => {
            i().child(strong().child(content)).into()
        },
        SectionType::Ol => ol().child(content).into(),
        SectionType::Ul => ul().child(content).into(),
        SectionType::H1 => h1().child(content).into(),
        SectionType::H2 => h2().child(content).into(),
        SectionType::Code => code().child(content).into()
    }
}

const TAG_NAME_H1: &str = "h1";
const TAG_NAME_H2: &str = "h2";
const TAG_NAME_P: &str = "p";
const TAG_NAME_OL: &str = "ol";
const TAG_NAME_UL: &str = "ul";
const TAG_NAME_A: &str = "a";
const TAG_NAME_STRONG: &str = "strong";
const TAG_NAME_ITALIC: &str = "i";
const TAG_NAME_ITALIC_BOLD: &str = "istrong";
const TAG_NAME_NONE: &str = "";

const H1_REGEX: &str = r"^\#{1}\s+";
const H2_REGEX: &str = r"^\#{2}\s+";
const LINK_NAME_REGEX: &str = r#"\[(.+)\]"#;
const LINK_URL_REGEX: &str = r#"\(([^ ]+?)\)"#;

/// Used as a precursor object, before converting to html, while finding matches
#[derive(Clone)]
struct TypeElement {
    pub section_type: SectionType,    
    pub element: HtmlElement<AnyElement>
}

#[derive(Clone, Debug, PartialEq)]
enum SectionType {
    Anchor,
    String,
    Paragraph,
    Strong,
    Italic,
    ItalicBold,
    Ol,
    Ul,
    H1,
    H2,
    Code
}