use std::borrow::Cow;
use regex::Regex;
use leptos::{logging::log, HtmlElement, html::{AnyElement, A, div, h1, h2, p, strong, a, span, code, em, ol, ul, li}};

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
    /// Regex to find an entire line, start to finish, that is a link
    pub entire_line_is_link_finder: Regex,
    /// Will match only the name portion of a link markdown
    pub link_name_finder: Regex,
    /// Will match only the url portion of a link markdown
    pub link_url_finder: Regex
}

const LINK_NAME_REGEX: &str = r#"\[(.+)\]"#;
const LINK_URL_REGEX: &str = r#"\(([^ ]+?)\)"#;

impl MarkdownToHtmlConverter {
    pub fn new() -> Self {
        MarkdownToHtmlConverter {
            heading_level_1_finder: Regex::new(r"^\#{1}\s+").unwrap(),
            heading_level_2_finder: Regex::new(r"^\#{2}\s+").unwrap(),
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
            entire_line_is_link_finder: Regex::new(r#"^\[(.+)\]\(([^ ]+?)( "(.+)")?\)$"#).unwrap(),
            link_name_finder: Regex::new(LINK_NAME_REGEX).unwrap(),
            link_url_finder: Regex::new(LINK_URL_REGEX).unwrap(),
        }
    }

    pub fn convert_md_to_html(&self, md_string: String) -> Vec<HtmlElement<AnyElement>> {
        let cloned_md_string = md_string.clone();
        let md_lines = cloned_md_string.split('\n').collect::<Vec<&str>>();
        let mut html_lines: Vec<HtmlElement<AnyElement>> = vec![];

        let mut ol_started = false;
        let mut current_found_ol: Vec<HtmlElement<AnyElement>> = vec![];

        let mut ul_started = false;
        let mut current_found_ul: Vec<HtmlElement<AnyElement>> = vec![];

        let mut code_started = false;
        let mut code_ended = false;
        let mut code_sections: Vec<HtmlElement<AnyElement>> = vec![];

        for md_line in md_lines.clone() {       
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
            } else if self.starting_code_finder.is_match(line) {
                if !code_started {
                    code_started = true;
                    code_ended = false;
                }          
                code_sections.push(div().inner_html(prefix_nbsp_for_whitespace_count(md_line.to_owned())).into());
            } else if self.ending_code_finder.is_match(line) {
                if !code_ended {
                    code_started = false;
                    code_ended = true;
                }       
                code_sections.push(div().inner_html(prefix_nbsp_for_whitespace_count(md_line.to_owned())).into());
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

                if code_started && !code_ended {           
                    code_sections.push(div().inner_html(prefix_nbsp_for_whitespace_count(md_line.to_owned())).into());
                } else if code_ended {
                    code_started = false;
                    code_ended = false;
                    
                    html_lines.push(code().child(code_sections.clone()).into());
                    code_sections = vec![];
                } else if !code_started {
                    let html_view = self.convert_md_to_html_element(line);
                    if let Some(html_view) = html_view {
                        html_lines.push(html_view);
                    } else {
                        let copyable_line = Cow::from(line);
                        html_lines.push(div().child(copyable_line).into());
                    }
                }
            }
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
        } else if self.link_finder.is_match(md_line) {            
            self.get_html_element_from_md(&self.link_finder, md_line, TAG_NAME_A)
        } else if self.paragraph_finder.is_match(md_line) {
            self.get_html_element_from_md(&self.paragraph_finder, md_line, TAG_NAME_P)
        } else {
            self.get_html_element_from_md(&self.paragraph_finder, md_line, TAG_NAME_NONE)
        }
    }

    fn get_html_element_from_md(&self, regex: &Regex, line_to_check: &str, replacement_html: &str) -> Option<HtmlElement<AnyElement>> {
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
        } else if replacement_html == TAG_NAME_A {                        
            self.get_html_from_md_link(line_to_check, replacement_html)
        } else if replacement_html == TAG_NAME_P {
            Some(self.get_html_element_inside_md(line_to_check, TAG_NAME_P))
        } else {
            Some(self.get_html_element_inside_md(line_to_check, TAG_NAME_NONE))
        }
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

    fn get_html_from_md_link(&self, line_to_check: &str, replacement_html: &str) -> Option<HtmlElement<AnyElement>> {
        if self.entire_line_is_link_finder.is_match(line_to_check) {
            let link_name = self.link_name_finder.captures(line_to_check).unwrap().get(0).unwrap().as_str();
            let link_url = self.link_url_finder.captures(line_to_check).unwrap().get(0).unwrap().as_str();
            let link_name_content = get_only_matching_content("[", "]", link_name);
            let link_url_content = get_only_matching_content("(", ")", link_url);

            let anchor = setup_anchor(&link_url_content, &link_name_content);
            Some(div().child(anchor).into())
        } else {
            log!("line_to_check {:?}", line_to_check);
            let mut link_names_list: Vec<String> = get_list_of_regex_matching_content(&self.link_name_finder, "[", "]", line_to_check);
            log!("link_names_list {:?}", link_names_list);
            let mut link_url_list: Vec<String> = get_list_of_regex_matching_content(&self.link_url_finder, "(", ")", line_to_check);
            let mut non_match_sections: Vec<String> = get_list_of_non_matching_content(&self.link_finder, line_to_check);
            let mut link_items_elements: Vec<HtmlElement<AnyElement>> = vec![];

            let mut index = 0;
            let mut elements: Vec<HtmlElement<AnyElement>> = vec![];
            for non_match_section in non_match_sections {
                log!("non_match_section {}", non_match_section);
                let non_matcher = span().child(format!("{} ", non_match_section));
                elements.push(non_matcher.into());

                if let Some(link_name_item) = link_names_list.get(index) {
                    log!("link_names_list[index] {}", link_name_item);
                    let next_link_name = format!("{} ", link_name_item);

                    if let Some(link_url_item) = link_url_list.get(index) {
                        log!("link_url_list[index] {}", link_url_item);
                        let next_link_url = format!("{} ", link_url_item);

                        let anchor = setup_anchor(&next_link_url, &next_link_name);
                        elements.push(anchor.into());    
                    } else {
                        panic!("Cannot have a link name without a url");
                    }            
                } else if let Some(link_url_item) = link_url_list.get(index) {
                    panic!("Cannot have a link url without a name");
                }
                                                                             
                index += 1;
            }
            Some(div().child(elements).into())
        }
    }
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
fn get_only_matching_content(md_start_str: &str, md_end_str: &str, matching_str: &str) -> String {
    let mut content = matching_str.replace(md_start_str, "");
    content = content.replace(md_end_str, "");
    content
}

fn get_list_of_regex_matching_content(finder: &Regex, start_md_remove_str: &str, end_md_remove_str: &str, line: &str) -> Vec<String> {
    let mut list: Vec<String> = vec![];

    for caps in finder.captures_iter(line) {            
        for captured_item in caps.iter() {
            let cap = captured_item.unwrap();
            let match_item = cap.as_str();

            if !match_item.starts_with(start_md_remove_str) && !match_item.ends_with(end_md_remove_str) {
                list.push(match_item.to_string()); 
            }
        }                           
    }
    list
}

fn get_list_of_non_matching_content(finder: &Regex, line: &str) -> Vec<String> {
    let mut list: Vec<String> = vec![];

    for non_match in finder.split(line) {
        list.push(non_match.to_string());
    }
    list
}

fn setup_anchor(link_url: &str, link_name: &str) -> HtmlElement<A> {
    let anchor = a();
    anchor.set_href(link_url);
    anchor.set_inner_text(link_name);
    anchor
}

const TAG_NAME_H1: &str = "h1";
const TAG_NAME_H2: &str = "h2";
const TAG_NAME_P: &str = "p";
const TAG_NAME_OL: &str = "ol";
const TAG_NAME_UL: &str = "ul";
const TAG_NAME_A: &str = "a";
const TAG_NAME_NONE: &str = "";