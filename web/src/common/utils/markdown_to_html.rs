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
    pub link_url_finder: Regex
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
        }
    }

    pub fn convert_md_to_html(&self, md_string: String) -> Vec<HtmlElement<AnyElement>> {
        let cloned_md_string = md_string.clone();
        let md_lines = cloned_md_string.split('\n').collect::<Vec<&str>>();
        let mut html_lines: Vec<HtmlElement<AnyElement>> = vec![];

        for md_line in md_lines {                   
            let line = md_line.trim().to_string();   
            log!("current line: {}", line);
            let mut matched_sections: Vec<HtmlElement<AnyElement>> = vec![div().child(line).into()];    
                        
            matched_sections = self.get_html_element_from_md(&self.heading_level_1_finder, &matched_sections, TAG_NAME_H1);  
            matched_sections = self.get_html_element_from_md(&self.heading_level_2_finder, &matched_sections, TAG_NAME_H2);     
            matched_sections = self.get_html_element_from_md(&self.bold_finder, &matched_sections, TAG_NAME_STRONG);
            matched_sections = self.get_html_element_from_md(&self.italic_finder, &matched_sections, TAG_NAME_ITALIC);
            matched_sections = self.get_html_element_from_md(&self.link_finder, &matched_sections, TAG_NAME_A);
            // matched_sections = self.get_html_element_from_md(&self.ordered_list_finder, &matched_sections, TAG_NAME_OL);        
            // matched_sections = self.get_html_element_from_md(&self.unordered_list_finder, &matched_sections, TAG_NAME_UL);        
            
            //matched_sections = self.get_html_element_from_md(&self.paragraph_finder, &matched_sections, TAG_NAME_P);        
            // matched_sections = self.get_html_element_from_md(&self.paragraph_finder, &matched_sections, TAG_NAME_NONE);

            // todo: add converted html
            html_lines.append(&mut matched_sections);
        }
        html_lines
    }

    fn get_html_element_from_md(&self, regex: &Regex, elements_to_check: &Vec<HtmlElement<AnyElement>>, replacement_html: &str) -> Vec<HtmlElement<AnyElement>> {
        let mut updated_elements: Vec<HtmlElement<AnyElement>> = vec![];
        for element_to_check in elements_to_check {            
            let element = element_to_check.clone();
            let element_inner_text = element.inner_text().clone();
            let element_inner_text = element_inner_text.as_str();
            
            if regex.is_match(element_inner_text) && replacement_html == TAG_NAME_H1 {
                let new_line = regex.replace(element_inner_text, "");
                updated_elements.push(h1().child(new_line).into());
            } 
            else if regex.is_match(element_inner_text) && replacement_html == TAG_NAME_H2 {
                let new_line = regex.replace(element_inner_text, "");              
                updated_elements.push(h2().child(new_line).into());
            } 
            // else if regex.is_match(element_inner_text) && replacement_html == TAG_NAME_OL {
            //     log!("get_html_element_from_md: OL");
            //     let new_line = regex.replace(element_inner_text, "");
            //     updated_elements.push(li().child(new_line).into());
            // } 
            // else if regex.is_match(element_inner_text) && replacement_html == TAG_NAME_UL {
            //     log!("get_html_element_from_md: UL");
            //     let new_line = regex.replace(element_inner_text, "");
            //     updated_elements.push(li().child(new_line).into());
            // } 
            else if regex.is_match(element_inner_text) && replacement_html == TAG_NAME_STRONG {                        
                let elements = self.get_html_element_from_md_line(regex, element_inner_text, &SectionType::Strong, vec!["**"]);
                if let Some(elements) = elements {
                    let element = element.inner_html("");
                    updated_elements.push(element.child(elements));                    
                }
            } 
            else if regex.is_match(element_inner_text) && replacement_html == TAG_NAME_ITALIC {                        
                let elements = self.get_html_element_from_md_line(regex, element_inner_text, &SectionType::Italic, vec!["*"]);
                if let Some(elements) = elements {
                    let element = element.inner_html("");
                    updated_elements.push(element.child(elements));                    
                }
            } 
            else if regex.is_match(element_inner_text) && replacement_html == TAG_NAME_A {                        
                let elements = self.get_anchor_element_from_md_link(element_inner_text);
                if let Some(elements) = elements {       
                    let element = element.inner_html("");             
                    updated_elements.push(element.child(elements));
                }
            } 
            // else if regex.is_match(element_inner_text) && replacement_html == TAG_NAME_P {
            //     updated_elements.push(element.child(self.get_html_element_inside_md(element_inner_text, TAG_NAME_P)));
            // } 
            else {                
                updated_elements.push(element);
            }
        }        
        updated_elements
    }

    fn get_html_element_inside_md(&self, line_to_check: &str, parent_html: &str) -> HtmlElement<AnyElement> {
        let mut html_items: Vec<HtmlElement<AnyElement>> = vec![];
        let mut bold_items: Vec<HtmlElement<AnyElement>> = vec![];
        let mut italic_items: Vec<HtmlElement<AnyElement>> = vec![];
        let mut code_items: Vec<HtmlElement<AnyElement>> = vec![];
        
        for caps in self.bold_finder.captures_iter(line_to_check) {            
            for captured_item in caps.iter() {
                let match_item = captured_item.unwrap();
                let bold_item = match_item.as_str();

                let bold_item_str = format!("{}{}", bold_item.replace("**", ""), " ");
                bold_items.push(strong().child(bold_item_str.to_string()).into()); 
            }                           
        }    

        for caps in self.italic_finder.captures_iter(line_to_check) {            
            for captured_item in caps.iter() {
                let match_item = captured_item.unwrap();
                let italic_item = match_item.as_str();

                let italic_item_str = format!("{}{}", italic_item.replace("*", ""), " ");
                italic_items.push(em().child(italic_item_str.to_string()).into()); 
            }                           
        }        

        for caps in self.code_finder.captures_iter(line_to_check) {       
            for captured_item in caps.iter() {
                let match_item = captured_item.unwrap();
                let code_item = match_item.as_str();

                let code_item_str = format!("{}{}", code_item.replace("`", ""), " ");
                code_items.push(code().child(code_item_str.to_string()).into()); 
            }                           
        }       
        
        let line_items = line_to_check.split(' ').collect::<Vec<&str>>();

        let mut bold_count = 0;        
        let mut started_bold = false;
        let mut ended_bold = false;

        let mut italic_count = 0;        
        let mut started_italic = false;
        let mut ended_italic = false;
      
        let mut started_code = false;
        let mut ended_code = false;

        for item in line_items {
            if self.bold_finder.is_match(item) {
                if item.starts_with("**") && item.ends_with("**") {
                    let bold_item_str = format!("{}{}", item.replace("**", ""), " ");
                    html_items.push(strong().child(bold_item_str.to_string()).into());
                } else { // handles words like super**duper**fun
                    let mut bold_items_list: Vec<String> = vec![];
                    let mut bold_items_elements: Vec<HtmlElement<AnyElement>> = vec![];

                    for caps in self.bold_finder.captures_iter(item) {            
                        for captured_item in caps.iter() {
                            let match_item = captured_item.unwrap();
                            let bold_item = match_item.as_str();
            
                            bold_items_list.push(format!("{}", bold_item.replace("**", ""))); 
                        }                           
                    }

                    let words_split_by_asterisk = item.split("**").collect::<Vec<&str>>();
                    for word_or_words in words_split_by_asterisk {
                        if bold_items_list.contains(&(word_or_words.to_string())) {
                            bold_items_elements.push(strong().child(format!("{}", word_or_words)).into());
                        } else {
                            bold_items_elements.push(span().child(format!("{}", word_or_words)).into());
                        }
                    }

                    bold_items_elements.push(span().child(" ").into());
                    html_items.push(span().child(bold_items_elements).into());
                }

                bold_count += 1; // todo: can I remove this?
            } else if self.starting_bold_finder.is_match(item) {
                if !started_bold {
                    ended_bold = false;
                    started_bold = true;
                }
            } else if self.ending_bold_finder.is_match(item) {
                if !ended_bold {
                    ended_bold = true;
                    started_bold = false;
                }

                let bold_element = bold_items[bold_count].clone();                                
                let ends_with_asterisk = item.ends_with("*");
                if !ends_with_asterisk {
                    bold_element.set_inner_text(bold_element.inner_text().trim());
                }
                html_items.push(bold_element.clone());          
                if !ends_with_asterisk {
                    let str_index = item.len() - 1;
                    let final_str = format!("{} ", item.to_string().as_bytes()[str_index] as char);
                    html_items.push(span().child(final_str).into());                    
                }                     

                bold_count += 1;
            } else if self.italic_finder.is_match(item) {
                if item.starts_with("*") && item.ends_with("*") {
                    let italic_item_str = format!("{}{}", item.replace("*", ""), " ");
                    html_items.push(em().child(italic_item_str.to_string()).into());
                } else { // handles words like super*duper*fun
                    let mut italic_items_list: Vec<String> = vec![];
                    let mut italic_items_elements: Vec<HtmlElement<AnyElement>> = vec![];

                    for caps in self.italic_finder.captures_iter(item) {            
                        for captured_item in caps.iter() {
                            let match_item = captured_item.unwrap();
                            let italic_item = match_item.as_str();
            
                            italic_items_list.push(format!("{}", italic_item.replace("*", ""))); 
                        }                           
                    }

                    let words_split_by_asterisk = item.split('*').collect::<Vec<&str>>();
                    for word_or_words in words_split_by_asterisk {
                        if italic_items_list.contains(&(word_or_words.to_string())) {
                            italic_items_elements.push(em().child(format!("{}", word_or_words)).into());
                        } else {
                            italic_items_elements.push(span().child(format!("{}", word_or_words)).into());
                        }
                    }

                    italic_items_elements.push(span().child(" ").into());
                    html_items.push(span().child(italic_items_elements).into());
                }

                italic_count += 1;
            } else if self.starting_italic_finder.is_match(item) {
                if !started_italic {
                    ended_italic = false;
                    started_italic = true;
                }
            } else if self.ending_italic_finder.is_match(item) {   
                if !ended_italic {
                    ended_italic = true;
                    started_italic = false;
                }

                let italic_element = italic_items[italic_count].clone();
                let ends_with_asterisk = item.ends_with("*");
                if !ends_with_asterisk {
                    italic_element.set_inner_text(italic_element.inner_text().trim());
                }
                html_items.push(italic_element.clone());         
                if !ends_with_asterisk {
                    let last_index = item.len() - 1;
                    let final_char = format!("{} ", item.to_string().as_bytes()[last_index] as char);
                    html_items.push(span().child(final_char).into());                    
                }                     

                italic_count += 1;
            } else if self.code_finder.is_match(item) {          
                
            } else if self.starting_code_finder.is_match(item) {
                if !started_code {
                    ended_code = false;
                    started_code = true;
                }
            } else if self.ending_code_finder.is_match(item) {
                if !ended_code {
                    ended_code = true;
                    started_code = false;
                }
            } else {
                if ended_bold {
                    let txt_item = Cow::from(format!("{}{}", item, " "));
                    html_items.push(span().child(txt_item).into());

                    started_bold = false;
                    ended_bold = false;
                } else if ended_italic {
                    let txt_item = Cow::from(format!("{}{}", item, " "));
                    html_items.push(span().child(txt_item).into());

                    started_italic = false;
                    ended_italic = false;
                } else if ended_code {
                    started_code = false;
                    ended_code = false;
                } else if !started_bold && !started_italic && !started_code {
                    let txt_item = Cow::from(format!("{}{}", item, " "));
                    html_items.push(span().child(txt_item).into());

                    started_bold = false;
                    ended_bold = false;
                    started_italic = false;
                    ended_italic = false;
                    started_code = false;
                    ended_code = false;
                }
            }
        }

        if parent_html == TAG_NAME_P {
            p().child(html_items).into()
        } else {
            div().child(html_items).into()
        }        
    }

    fn get_html_element_from_md_line(&self, regex: &Regex, line: &str, section_type: &SectionType, markdown: Vec<&str>) -> Option<Vec<HtmlElement<AnyElement>>> {
        let match_list: Vec<String> = get_list_of_regex_matching_content(regex, line, markdown);    
        log!("match_list: {:?}", match_list);
        let non_match_sections: Vec<String> = get_list_of_non_matching_content(regex, line);
        log!("non_match_sections: {:?}", non_match_sections);
        
        let mut elements: Vec<HtmlElement<AnyElement>> = vec![];
        if non_match_sections.len() == 0 { // if no non-match sections then entire line is
            elements.push(convert_matched_sections_to_html(match_list[0].clone(), None, section_type));  
        } else {
            let mut match_list_index = 0;
            let mut line_starts_with_non_match_section = false;
            if line.starts_with(non_match_sections.get(0).unwrap()) {
                line_starts_with_non_match_section = true;
            }
            for non_match_section in non_match_sections {
                if line_starts_with_non_match_section {
                    elements.push(convert_matched_sections_to_html(non_match_section.clone(), None, &SectionType::String));
                }                    

                let next_match = format!("{}", match_list.get(match_list_index).unwrap());
                elements.push(convert_matched_sections_to_html(next_match, None, section_type));  

                if !line_starts_with_non_match_section {
                    elements.push(convert_matched_sections_to_html(non_match_section, None, &SectionType::String));
                }  
                                                                                
                match_list_index += 1;
            }
            // set last html element if there is one
            if match_list_index < match_list.len() {
                let next_match = format!("{} ", match_list.get(match_list_index).unwrap());
                elements.push(convert_matched_sections_to_html(next_match, None, section_type)); 
            }
        }
        if elements.len() == 0 {
            return None;
        }
        Some(elements)
    }

    /// Will get embedded anchor and other content as well
    fn get_anchor_element_from_md_link(&self, line: &str) -> Option<Vec<HtmlElement<AnyElement>>> {     
        log!("get_anchor_line_from_md_link line: {}", line);
        let link_names_list: Vec<String> = get_list_of_regex_matching_content(&self.link_name_finder, line, vec!["[", "]"]);
        let link_url_list: Vec<String> = get_list_of_regex_matching_content(&self.link_url_finder, line, vec!["(", ")"]);
        if link_names_list.len() == 0 || link_url_list.len() == 0 {
            warn!("link names list and link url list seem not to match!");
            return None;
        }        
        let non_match_sections: Vec<String> = get_list_of_non_matching_content(&self.link_finder, line);
        
        let mut elements: Vec<HtmlElement<AnyElement>> = vec![];
        if non_match_sections.len() == 0 { // if no non-match sections then entire line is a link
            elements.push(setup_anchor(link_url_list[0].clone().as_str(), link_names_list[0].clone().as_str()).into());  
        } else {
            let mut index = 0;
            let mut line_starts_with_non_match_section = false;
            if line.starts_with(non_match_sections.get(0).unwrap()) {
                line_starts_with_non_match_section = true;
            }
            for non_match_section in non_match_sections {
                if line_starts_with_non_match_section {
                    elements.push(span().child(non_match_section.clone()).into());
                }                    

                let anchor = set_anchor_element(&link_names_list, &link_url_list, index);
                if let Some(anchor) = anchor {
                    elements.push(anchor.into());
                }

                if !line_starts_with_non_match_section {
                    elements.push(span().child(non_match_section).into());
                }  
                                                                                
                index += 1;
            }
            // set last anchor element if there is one
            if index < link_names_list.len() {
                let anchor = set_anchor_element(&link_names_list, &link_url_list, index);
                if let Some(anchor) = anchor {
                    elements.push(anchor.into());
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
        SectionType::Italic => i().child(content).into()
    }
}

const TAG_NAME_H1: &str = "h1";
const TAG_NAME_H2: &str = "h2";
const TAG_NAME_P: &str = "p";
const TAG_NAME_OL: &str = "ol";
const TAG_NAME_UL: &str = "ul";
const TAG_NAME_A: &str = "a";
const TAG_NAME_STRONG: &str = "strong";
const TAG_NAME_ITALIC: &str = "I";
const TAG_NAME_NONE: &str = "";

const H1_REGEX: &str = r"^\#{1}\s+";
const H2_REGEX: &str = r"^\#{2}\s+";
const LINK_NAME_REGEX: &str = r#"\[(.+)\]"#;
const LINK_URL_REGEX: &str = r#"\(([^ ]+?)\)"#;

/// Used as a precursor object, before converting to html, while finding matches
#[derive(Clone, Debug)]
struct MatchedSection {
    pub section_type: SectionType,
    /// consider it same as inner html
    pub content: Option<String>,
    pub children: Option<Vec<MatchedSection>>,
    pub url: Option<String>
}

#[derive(Clone, Debug, PartialEq)]
enum SectionType {
    Anchor,
    String,
    Paragraph,
    Strong,
    Italic
}