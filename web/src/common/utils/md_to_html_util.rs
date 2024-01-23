use std::borrow::Cow;
use regex::Regex;
use leptos::{logging::log, HtmlElement, html::{AnyElement, div, h1, h2, p, strong, span, code, em, ol, ul, li}};

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
            starting_bold_finder: Regex::new(r"\*{2}[\w\s]+").unwrap(),
            ending_bold_finder: Regex::new(r"[\w\s]+\*{2}").unwrap(),
            italic_finder: Regex::new(r"\*{1}[\w\s]+\*{1}").unwrap(),
            starting_italic_finder: Regex::new(r"\*{1}[\w\s]+").unwrap(),
            ending_italic_finder: Regex::new(r"[\w\s]+\*{1}").unwrap(),
            code_finder: Regex::new(r#"\`[\w\s\{\}\(\)<.*>\?\/\[\]\.\,\:\;\-\"]+\`|\`{2}[\w\s\{\}\(\)<.*>\?\/\[\]\.\,\:\;\-\"]+\`{2}"#).unwrap(),
            starting_code_finder: Regex::new(r#"\`[\w\s\{\}\(\)<.*>\?\/\[\]\.\,\:\;\-\"]+|\`{2}[\w\s\{\}\(\)<.*>\?\/\[\]\.\,\:\;\-\"]+"#).unwrap(),
            ending_code_finder: Regex::new(r#"[\w\s\{\}\(\)<.*>\?\/\[\]\.\,\:\;\-\"]+\`|[\w\s\{\}\(\)<.*>\?\/\[\]\.\,\:\;\-\"]+\`{2}"#).unwrap()
        }
    }

    pub fn convert_md_to_html(&self, md_string: String) -> Vec<HtmlElement<AnyElement>> {
        let md_lines = md_string.split('\n').collect::<Vec<&str>>();
        let mut html_lines: Vec<HtmlElement<AnyElement>> = vec![];

        let mut ol_started = false;
        let mut current_found_ol: Vec<HtmlElement<AnyElement>> = vec![];

        let mut ul_started = false;
        let mut current_found_ul: Vec<HtmlElement<AnyElement>> = vec![];

        let mut code_started = false;
        let mut code_ended = false;
        let mut current_found_code: Vec<HtmlElement<AnyElement>> = vec![];
        let mut code_sections: Vec<HtmlElement<AnyElement>> = vec![];
        let mut code_section_index = 0;

        for caps in self.code_finder.captures_iter(md_string.as_str()) {  
            for captured_item in caps.iter() {
                let match_item = captured_item.unwrap();
                let code_item = match_item.as_str();

                let code_item_str = format!("{}{}", code_item.replace("`", ""), " ");
                code_sections.push(code().child(code_item_str.to_string()).into()); 
                log!("code sections found: {:?}", code_item_str);
            }                           
        }   

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
            } else if self.starting_code_finder.is_match(line) {
                if !code_started {
                    code_started = true;
                    code_ended = false;
                }
            } else if self.ending_code_finder.is_match(line) {
                if !code_ended {
                    code_started = false;
                    code_ended = true;
                }
                if code_sections.len() > 0 {
                    html_lines.push(div().child(code_sections[code_section_index].clone()).into());
                    code_section_index += 1;
                }
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
                if code_ended {
                    code_started = false;
                    code_ended = false;
                }
                
                if !code_started {
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
            log!("code items");
            for captured_item in caps.iter() {
                let match_item = captured_item.unwrap();
                let code_item = match_item.as_str();

                let code_item_str = format!("{}{}", code_item.replace("`", ""), " ");
                code_items.push(code().child(code_item_str.to_string()).into()); 
                log!("code item: {:?}", code_item_str);
            }                           
        }       
        
        let line_items = line_to_check.split(' ').collect::<Vec<&str>>();
        log!("line_items: {:?}", line_items);

        let mut bold_count = 0;        
        let mut started_bold = false;
        let mut ended_bold = false;

        let mut italic_count = 0;        
        let mut started_italic = false;
        let mut ended_italic = false;
      
        let mut started_code = false;
        let mut ended_code = false;

        for item in line_items {
            log!("item: {:?}", item);
            if self.bold_finder.is_match(item) {
                log!("whole words bold item: {:?}", item);
                if item.starts_with("**") && item.ends_with("**") {
                    log!("one word item: {:?}", item);
                    let bold_item_str = format!("{}{}", item.replace("**", ""), " ");
                    html_items.push(strong().child(bold_item_str.to_string()).into());
                } else { // handles words like super**duper**fun
                    log!("multiple words item: {:?}", item);
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

                bold_count += 1;
            } else if self.starting_bold_finder.is_match(item) {
                log!("bold starts item: {:?}", item);
                if !started_bold {
                    ended_bold = false;
                    started_bold = true;
                }
            } else if self.ending_bold_finder.is_match(item) {
                log!("bold ends item: {:?}", item);
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
                log!("whole words italic item: {:?}", item);
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
                log!("italic starts item: {:?}", item);
                if !started_italic {
                    ended_italic = false;
                    started_italic = true;
                }
            } else if self.ending_italic_finder.is_match(item) {         
                log!("italic ends item: {:?}", item);       
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
                log!("whole code item: {}", item);
            } else if self.starting_code_finder.is_match(item) {
                log!("starting code: {}", item);
                if !started_code {
                    ended_code = false;
                    started_code = true;
                }
            } else if self.ending_code_finder.is_match(item) {
                log!("ending code: {}", item);
                if !ended_code {
                    ended_code = true;
                    started_code = false;
                }
            } else {
                if ended_bold {
                    log!("bold ending item: {:?}", item);
                    let txt_item = Cow::from(format!("{}{}", item, " "));
                    html_items.push(span().child(txt_item).into());

                    started_bold = false;
                    ended_bold = false;
                } else if ended_italic {
                    log!("italic ending item: {:?}", item);
                    let txt_item = Cow::from(format!("{}{}", item, " "));
                    html_items.push(span().child(txt_item).into());

                    started_italic = false;
                    ended_italic = false;
                } else if ended_code {
                    log!("ended code: {}", item);
                    // let txt_item = Cow::from(format!("{}{}", item, " "));
                    // html_items.push(span().child(txt_item).into());

                    started_code = false;
                    ended_code = false;
                } else if !started_bold && !started_italic && !started_code {
                    log!("not started any item: {:?}", item);
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
}

const TAG_NAME_H1: &str = "h1";
const TAG_NAME_H2: &str = "h2";
const TAG_NAME_P: &str = "p";
const TAG_NAME_OL: &str = "ol";
const TAG_NAME_UL: &str = "ul";
const TAG_NAME_NONE: &str = "";