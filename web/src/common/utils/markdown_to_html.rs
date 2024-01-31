use regex::Regex;
use leptos::{logging::{log, warn}, HtmlElement, html::{a, code, pre, div, h1, h2, i, img, li, ol, p, span, strong, ul, AnyElement, Img, A}};

pub struct MarkdownToHtmlConverter {
    pub heading_level_1_finder: Regex,
    pub heading_level_2_finder: Regex,
    pub ordered_list_finder: Regex,
    pub unordered_list_finder: Regex,
    pub bold_finder: Regex,
    pub italic_bold_finder: Regex,
    pub italic_finder: Regex,
    pub code_finder: Regex,
    pub starting_code_finder: Regex,
    pub ending_code_finder: Regex,
    /// Regex to find a markdown link
    pub link_finder: Regex,
    /// Will match only the name portion of a link markdown
    pub link_name_finder: Regex,
    /// Will match only the url portion of a link markdown
    pub link_url_finder: Regex,
    pub image_link_finder: Regex,
    pub image_link_alt_finder: Regex,
    pub only_new_line_finder: Regex
}

impl MarkdownToHtmlConverter {
    pub fn new() -> Self {
        MarkdownToHtmlConverter {
            heading_level_1_finder: Regex::new(H1_REGEX).unwrap(),
            heading_level_2_finder: Regex::new(H2_REGEX).unwrap(),
            ordered_list_finder: Regex::new(r"^\d+\.").unwrap(),
            unordered_list_finder: Regex::new(r"^\-\s").unwrap(),     
            bold_finder: Regex::new(r"\*{2}[\w\s]+\*{2}").unwrap(),      
            italic_bold_finder: Regex::new(r"\*{3}[\w\s]+\*{3}").unwrap(),     
            italic_finder: Regex::new(r"\*{1}[\w\s]+\*{1}").unwrap(),
            code_finder: Regex::new(r#"\`[\w\s\{\}\(\)<.*>\?\/\[\]\.\,\:\;\-\"]+\`|\`{2}[\w\s\{\}\(\)<.*>\?\/\[\]\.\,\:\;\-\"]+\`{2}"#).unwrap(),
            starting_code_finder: Regex::new(STARTING_CODE_REGEX).unwrap(),
            ending_code_finder: Regex::new(ENDING_CODE_REGEX).unwrap(),
            link_finder: Regex::new(r#"\[([^\]]+)\]\(([^ )]+?)( "([^"]+)")?\)"#).unwrap(),
            link_name_finder: Regex::new(LINK_NAME_REGEX).unwrap(),
            link_url_finder: Regex::new(LINK_URL_REGEX).unwrap(),
            image_link_finder: Regex::new(r#"!\[([^\]]+)\]\(([^ )]+?)( "([^"]+)")?\)"#).unwrap(),
            image_link_alt_finder: Regex::new(r#"!\[([^\]]+)\]"#).unwrap(),
            only_new_line_finder: Regex::new(r"^\s+$").unwrap()
        }
    }

    pub fn convert_md_to_html(&self, md_string: String) -> Vec<HtmlElement<AnyElement>> {
        let cloned_md_string = md_string.clone();
        let md_lines = cloned_md_string.split('\n').map(|md_line| md_line.to_string()).collect::<Vec<String>>();
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
                        
            // lists need to be aggregated and then wrapped by single parent
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
                log!("ul li line: {}, {}", new_line, ul_started);
                gathered_ul.push(li().child(new_line).into());
            } // code lines need to be aggregated and then wrapped with single parent
            else if self.starting_code_finder.is_match(line_str) {
                if !code_started {
                    code_started = true;
                    code_ended = false;
                }
                
                let new_line = md_line.clone().replace(r"`", "");
                gathered_code.push(pre().child(new_line).into());
            } else if self.ending_code_finder.is_match(line_str) {
                if !code_ended {
                    code_started = false;
                    code_ended = true;
                }
                
                let new_line = md_line.clone().replace(r"`", "");
                gathered_code.push(pre().child(new_line).into());
            } else {
                if ol_started {                   
                    matched_sections.push(TypeElement { section_type: SectionType::Ol, element: ol().child(gathered_ol.clone()).into() });
                    gathered_ol.clear();
                    ol_started = false;
                } else if ul_started {       
                    log!("add ul parent");        
                    matched_sections.push(TypeElement { section_type: SectionType::Ul, element: ul().child(gathered_ul.clone()).into() });
                    gathered_ul.clear();
                    ul_started = false;
                }

                if code_started { // gets middle of code section
                    gathered_code.push(pre().child(md_line).into());
                } else if code_ended {
                    code_started = false;
                    code_ended = false;
                    
                    matched_sections.push(TypeElement { section_type: SectionType::Code, element: code().child(gathered_code.clone()).into() });                    
                    gathered_code.clear();
                } else {
                    matched_sections.push(TypeElement { section_type: SectionType::String, element: div().child(line.clone()).into() });
                    matched_sections = self.get_html_element_from_md(&self.heading_level_1_finder, &matched_sections, TAG_NAME_H1);
                    matched_sections = self.get_html_element_from_md(&self.heading_level_2_finder, &matched_sections, TAG_NAME_H2);
                    matched_sections = self.get_html_element_from_md(&self.italic_bold_finder, &matched_sections, TAG_NAME_ITALIC_BOLD);
                    matched_sections = self.get_html_element_from_md(&self.bold_finder, &matched_sections, TAG_NAME_STRONG);
                    matched_sections = self.get_html_element_from_md(&self.italic_finder, &matched_sections, TAG_NAME_ITALIC);

                    if self.image_link_finder.is_match(line_str) {
                        matched_sections = self.get_html_element_from_md(&self.image_link_finder, &matched_sections, TAG_NAME_IMG);
                    } else {
                        matched_sections = self.get_html_element_from_md(&self.link_finder, &matched_sections, TAG_NAME_A);
                    }
                }               
            }            
            // set final list of html elements                 
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

        // if ol or ul was last may need to finish the wrap here
        log!("ul_started {}, gathered_ul len {}", ul_started, gathered_ul.len());
        if ul_started && gathered_ul.len() > 0 {
            html_lines.push(div().child(ul().child(gathered_ul.clone())).into());
            gathered_ul.clear();
            ul_started = false;
        }

        html_lines
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
                        for element in elements {
                            updated_elements.push(TypeElement { section_type: element.section_type, element: element.element});
                        }                        
                    }
                } 
                else if regex.is_match(element_inner_text) && replacement_html == TAG_NAME_STRONG {                
                    let elements = self.get_html_element_from_md_line(regex, element_inner_text, &SectionType::Strong, vec!["**"]);
                    if let Some(elements) = elements {
                        for element in elements {
                            updated_elements.push(TypeElement { section_type: element.section_type, element: element.element});
                        }                     
                    }
                } 
                else if regex.is_match(element_inner_text) && replacement_html == TAG_NAME_ITALIC {   
                    let elements = self.get_html_element_from_md_line(regex, element_inner_text, &SectionType::Italic, vec!["*"]);
                    if let Some(elements) = elements {
                        for element in elements {
                            updated_elements.push(TypeElement { section_type: element.section_type, element: element.element});
                        }          
                    }
                }             
                else if regex.is_match(element_inner_text) && replacement_html == TAG_NAME_A {   
                    let elements = self.get_anchor_or_img_from_md_link(element_inner_text, false);
                    if let Some(elements) = elements {             
                        for element in elements {
                            updated_elements.push(TypeElement { section_type: element.section_type, element: element.element});
                        }   
                    }
                } 
                else if regex.is_match(element_inner_text) && replacement_html == TAG_NAME_IMG {   
                    let elements = self.get_anchor_or_img_from_md_link(element_inner_text, true);
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

    /// Returns a collection of elements within the given line
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

    /// Will get embedded anchor or image and other content as well
    fn get_anchor_or_img_from_md_link(&self, line: &str, md_is_image: bool) -> Option<Vec<TypeElement>> {  
        // grab name and url together for now
        let link_list: Vec<String> = if !md_is_image { 
            get_list_of_regex_matching_content(&self.link_finder, line, vec![])
        } else {
            get_list_of_regex_matching_content(&self.image_link_finder, line, vec![])
        };
        log!("link_list: {:?}", link_list);
        let link_names_list: Vec<String> = if !md_is_image {
            link_list.clone().iter().map(|link| {
                let found_name = self.link_name_finder.find(link).unwrap().as_str();
                let clean_name = &found_name.replace("[", "").replace("]", "");
                clean_name.clone()
            }).collect::<Vec<String>>()
        } else {
            link_list.clone().iter().map(|link| {
                let found_name = self.image_link_alt_finder.find(link).unwrap().as_str();
                let clean_name = &found_name.replace("![", "").replace("]", "");
                clean_name.clone()
            }).collect::<Vec<String>>()
        };
        log!("link_names_list: {:?}", link_names_list);
        let link_url_list: Vec<String> = link_list.iter().map(|link| {
            let mut found_url = self.link_url_finder.find(link).unwrap().as_str();
            let mut clean_url = &found_url.replace("(", "").replace(")", "").to_string();
            let url = clean_url.split(' ').nth(0).unwrap(); // get link without title
            url.to_string()
        }).collect::<Vec<String>>();
        log!("link_url_list: {:?}", link_url_list);

        let non_match_sections: Vec<String> = get_list_of_non_matching_content(if !md_is_image {
            &self.link_finder
        } else {
            &self.image_link_finder
        }, line);
        log!("non_match_sections: {:?}", non_match_sections);
        
        let mut elements: Vec<TypeElement> = vec![];
        if non_match_sections.len() == 0 { // if no non-match sections then entire line is a link            
            elements.push(TypeElement { 
                section_type: get_anchor_or_image_type(md_is_image), 
                element: if !md_is_image {
                        setup_anchor(link_url_list[0].clone().as_str(), link_names_list[0].clone().as_str()).into()
                    } else {
                        setup_image(link_url_list[0].clone().as_str(), link_names_list[0].clone().as_str()).into()
                    }
            });  
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

                let element = get_anchor_or_image_element(&link_names_list, &link_url_list, index, md_is_image);
                if let Some(element) = element {
                    elements.push(TypeElement { section_type: get_anchor_or_image_type(md_is_image), element });
                }

                if !line_starts_with_non_match_section {
                    elements.push(TypeElement { section_type: SectionType::String, element: span().child(non_match_section).into() });
                }  
                                                                                
                index += 1;
            }
            // set last anchor or image element if there is one
            if index < link_names_list.len() {
                let element = get_anchor_or_image_element(&link_names_list, &link_url_list, index, md_is_image);
                if let Some(element) = element {
                    elements.push(TypeElement { section_type: get_anchor_or_image_type(md_is_image), element });
                }
            }
        }
        if elements.len() == 0 {
            return None;
        }
        Some(elements)
    }
}

/// A link may have a title in quotes. This function will remove it
fn get_anchor_or_image_element(link_names_list: &Vec<String>, link_url_list: &Vec<String>, index: usize, is_image: bool) -> Option<HtmlElement<AnyElement>> {
    let mut element: Option<HtmlElement<AnyElement>> = None;
    if let Some(link_name_item) = link_names_list.get(index) {
        let next_link_name = format!("{}", link_name_item);

        if let Some(link_url_item) = link_url_list.get(index) {
            let mut url = link_url_item.split(' '); // e.g. (http://some.com "link title")
            let next_link_url = format!("{}", url.nth(0).unwrap());

            if !is_image {
                element = Some(setup_anchor(&next_link_url, &next_link_name).into());    
            } else {
                element = Some(setup_image(&next_link_url, &next_link_name).into()); 
            }
        } else {
            warn!("Cannot have a link name without a url");
            return None;
        }            
    } else if let Some(_link_url_item) = link_url_list.get(index) {
        warn!("Cannot have a link url without a name");
        return None;
    }
    element
}

/// Returns only the content without the markdown
fn get_list_of_regex_matching_content(finder: &Regex, line: &str, markdown: Vec<&str>) -> Vec<String> {
    let mut list: Vec<String> = vec![];

    for found in finder.find_iter(line) {
        let mut found_content = found.as_str().to_string();
        log!("found_content: {}", found_content);
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

fn setup_image(link_url: &str, link_alt: &str) -> HtmlElement<Img> {
    let img = img();
    img.set_src(link_url.trim());
    img.set_alt(link_alt);
    img
}

fn convert_matched_sections_to_html(content: String, url: Option<String>, section_type: &SectionType) -> HtmlElement<AnyElement> {
    if section_type == &SectionType::Anchor && url == None {
        panic!("An anchor requires the url parameter");
    }
    
    match section_type {
        SectionType::Anchor => setup_anchor(url.unwrap().as_str(), content.as_str()).into(),
        SectionType::Image => setup_image(url.unwrap().as_str(), content.as_str()).into(),
        SectionType::String => span().child(content).into(),
        SectionType::Strong => strong().child(content).into(),
        SectionType::Italic => i().child(content).into(),
        SectionType::ItalicBold => i().child(strong().child(content)).into(),
        SectionType::Ol => ol().child(content).into(),
        SectionType::Ul => ul().child(content).into(),
        SectionType::H1 => h1().child(content).into(),
        SectionType::H2 => h2().child(content).into(),
        SectionType::Code => code().child(content).into()
    }
}

fn get_anchor_or_image_type(is_image: bool) -> SectionType {
    if is_image {
        return SectionType::Image;
    }
    SectionType::Anchor
}

#[allow(unused)]
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
        inner_txt = format!("{}{}", "&amp;amp;nbsp;", inner_txt);
    }
    inner_txt
}

const TAG_NAME_H1: &str = "h1";
const TAG_NAME_H2: &str = "h2";
const TAG_NAME_A: &str = "a";
const TAG_NAME_IMG: &str = "img";
const TAG_NAME_STRONG: &str = "strong";
const TAG_NAME_ITALIC: &str = "i";
const TAG_NAME_ITALIC_BOLD: &str = "istrong";

const H1_REGEX: &str = r"^\#{1}\s+";
const H2_REGEX: &str = r"^\#{2}\s+";
const LINK_NAME_REGEX: &str = r#"\[([^\]]+)\]"#;
const LINK_URL_REGEX: &str = r#"\(([^ ]+?)( "(.+)")?\)"#;
const STARTING_CODE_REGEX: &str = r#"\`[\w\s\{\}\(\)<.*>\?\/\[\]\.\,\:\;\-\"]+|\`{2}[\w\s\{\}\(\)<.*>\?\/\[\]\.\,\:\;\-\"]+"#;
const ENDING_CODE_REGEX: &str = r#"[\w\s\{\}\(\)<.*>\?\/\[\]\.\,\:\;\-\"]+\`|[\w\s\{\}\(\)<.*>\?\/\[\]\.\,\:\;\-\"]+\`{2}"#;

/// Used as a precursor object, before converting to html, while finding matches
#[derive(Clone)]
pub struct TypeElement {
    pub section_type: SectionType,    
    pub element: HtmlElement<AnyElement>
}

#[derive(Clone, Debug, PartialEq)]
pub enum SectionType {
    Anchor,
    Image,
    /// String can be Div or Span
    String,
    Strong,
    Italic,
    ItalicBold,
    Ol,
    Ul,
    H1,
    H2,
    Code
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::logging::log;
    use wasm_bindgen_test::*;
    use super::MarkdownToHtmlConverter;

    wasm_bindgen_test_configure!(run_in_browser);

    #[allow(unused)]
    const STANDALONE_LINK: &str = "[This is the link name](https://helloworld.com)";
    #[allow(unused)]
    const ONE_LINK_INSIDE_SENTENCE: &str = r#"You can find more info here! [Go Here](https://gohere.com "funny link") click that link"#;
    #[allow(unused)]
    const TWO_LINKS_INSIDE_SENTENCE: &str = r#"[First Link](https://first.com "first")You can find more info here! [Second Link](https://second.com "second") click that link"#;

    #[wasm_bindgen_test]
    fn test_get_anchor_or_img_from_md_link_returns_anchor_when_passed_standalone_link() {
        let md = MarkdownToHtmlConverter::new();

        let elements = md.get_anchor_or_img_from_md_link(STANDALONE_LINK, false);

        assert!(elements.clone().unwrap().len() == 1);        
        assert!(elements.clone().unwrap().iter().find(|el| el.section_type == SectionType::Anchor && el.element.outer_html().contains("</a>")).is_some());
        assert!(elements.clone().unwrap().iter().find(|el| el.section_type == SectionType::Anchor && el.element.outer_html().contains("href=\"https://helloworld.com\"")).is_some());
        assert!(elements.unwrap().iter().find(|el| el.section_type == SectionType::Anchor && el.element.outer_html().contains("This is the link name")).is_some());
    }
    
    #[wasm_bindgen_test]
    fn test_get_anchor_or_img_from_md_link_returns_anchor_when_passed_one_link_inside_sentence() {
        let md = MarkdownToHtmlConverter::new();

        let elements = md.get_anchor_or_img_from_md_link(ONE_LINK_INSIDE_SENTENCE, false);

        assert!(elements.unwrap().len() == 3);
    }

    #[wasm_bindgen_test]
    fn test_get_anchor_or_img_from_md_link_returns_anchor_when_passed_two_links_inside_sentence() {
        let md = MarkdownToHtmlConverter::new();

        let elements = md.get_anchor_or_img_from_md_link(TWO_LINKS_INSIDE_SENTENCE, false);

        assert!(elements.clone().unwrap().len() == 4);
        assert!(elements.clone().unwrap().iter().find(|el| el.section_type == SectionType::Anchor && el.element.outer_html().contains("</a>")).is_some());
        assert!(
            elements.clone().unwrap().iter().find(|el| el.section_type == SectionType::Anchor && el.element.outer_html().contains("href=\"https://first.com\"")).is_some()
            &&
            elements.clone().unwrap().iter().find(|el| el.section_type == SectionType::Anchor && el.element.outer_html().contains("href=\"https://second.com\"")).is_some()
        );
        assert!(
            elements.clone().unwrap().iter().find(|el| el.section_type == SectionType::Anchor && el.element.outer_html().contains("First Link")).is_some()
            &&
            elements.clone().unwrap().iter().find(|el| el.section_type == SectionType::Anchor && el.element.outer_html().contains("Second Link")).is_some()
        );
        assert!(
            elements.clone().unwrap().iter().find(|el| el.section_type == SectionType::String && el.element.outer_html().contains("You can find more info here!")).is_some()
            &&
            elements.clone().unwrap().iter().find(|el| el.section_type == SectionType::String && el.element.outer_html().contains("click that link")).is_some()
        );
    }
}