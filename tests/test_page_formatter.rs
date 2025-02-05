use std::{rc::Rc, result};
use colored::Colorize;
use cli_epub_reader::page_formatter::*;
use xml_reader::{file_parser::{self, parse_contents}, node::Node};


fn get_formatting_nodes(list_nodes : &Vec<Rc<Node>>) -> Vec<usize>{
    let mut result : Vec<usize> = Vec::new();
    for node in list_nodes{
        if FORMATTING_NODES.contains(&node.get_name().as_str()){
            result.push(node.get_id());
        }
    }
    result
}

#[test]
fn test_start_chapter(){
    let test_file = "tests\\test_start_chapter.xml".to_string();
    let mut test_chapter = Chapter::new(test_file, (500,500));
    let _ = test_chapter.start_chapter();
    assert_eq!(test_chapter.get_all_nodes().unwrap().len(), 8);

    assert_eq!(test_chapter.get_all_nodes().unwrap()[0].get_name(), &"body".to_string());
    assert_eq!(test_chapter.get_all_nodes().unwrap()[1].get_name(), &"article".to_string());
    assert_eq!(test_chapter.get_all_nodes().unwrap()[3].get_name(), &"h3".to_string());
    assert_eq!(test_chapter.get_all_nodes().unwrap()[4].get_name(), &"p".to_string());
    assert_eq!(test_chapter.get_all_nodes().unwrap()[5].get_name(), &"p".to_string());
    assert_eq!(test_chapter.get_all_nodes().unwrap()[6].get_name(), &"span".to_string());
    assert_eq!(test_chapter.get_all_nodes().unwrap()[7].get_name(), &"br".to_string());
}

#[test]
fn test_get_all_content_nodes(){
    let test_file = "tests\\test_start_chapter.xml".to_string();
    let (w,h) = term_size::dimensions().unwrap();
    let mut test_chapter = Chapter::new(test_file, (w,h));
    let _ = test_chapter.start_chapter();

    let expected_formatting_nodes = get_formatting_nodes(test_chapter.get_all_nodes().unwrap());
    let current_formatting_nodes = Chapter::get_content_nodes(&test_chapter.get_all_nodes().unwrap());
    assert_eq!(current_formatting_nodes.len(), expected_formatting_nodes.len());

    for (i, (actual, expected)) in current_formatting_nodes.iter().zip(expected_formatting_nodes.iter()).enumerate() {
        assert_eq!(actual, expected, "Mismatch at index {}: expected '{}', got '{}'", i, expected, actual);
    }
}

#[test]
fn test_format_text_nodes(){
    let test_file = "src\\processed_files\\kate-chopin_short-fiction\\epub\\text\\the-night-came-slowly.xhtml".to_string();
    let (w,h) = term_size::dimensions().unwrap();
    let mut test_chapter = Chapter::new(test_file, (w,h));
    let _ = test_chapter.start_chapter();
   
}

#[test]
fn test_format_structural_nodes(){

}

#[test]
fn test_format_content(){

}

#[test]
fn test(){
    let test_file = "tests\\test_error.xml".to_string();
    let (w,h) = term_size::dimensions().unwrap();
    let mut test_chapter = Chapter::new(test_file, (w,h));
    let _ = test_chapter.start_chapter();
}

