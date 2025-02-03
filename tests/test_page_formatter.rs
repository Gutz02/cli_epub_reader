use std::{rc::Rc, result};
use colored::Colorize;
use cli_epub_reader::page_formatter::*;
use xml_reader::{file_parser::parse_contents, node::Node};


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
    let mut test_chapter = Chapter::new(test_file);
    let _ = test_chapter.start_chapter();
    assert_eq!(test_chapter.get_all_nodes().unwrap().len(), 7);

    assert_eq!(test_chapter.get_all_nodes().unwrap()[0].get_name(), &"body".to_string());
    assert_eq!(test_chapter.get_all_nodes().unwrap()[1].get_name(), &"article".to_string());
    assert_eq!(test_chapter.get_all_nodes().unwrap()[2].get_name(), &"h3".to_string());
    assert_eq!(test_chapter.get_all_nodes().unwrap()[3].get_name(), &"p".to_string());
    assert_eq!(test_chapter.get_all_nodes().unwrap()[4].get_name(), &"p".to_string());
    assert_eq!(test_chapter.get_all_nodes().unwrap()[5].get_name(), &"span".to_string());
    assert_eq!(test_chapter.get_all_nodes().unwrap()[6].get_name(), &"br".to_string());
}

#[test]
fn test_get_all_content_nodes(){
    let test_file = "tests\\test_start_chapter.xml".to_string();
    let mut test_chapter = Chapter::new(test_file);
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
    let mut test_chapter = Chapter::new(test_file);
    let _ = test_chapter.start_chapter();

    let expected_outcome = String::from(
        "This is some text \n".to_string()
            + &"Some bold text here ".bold().to_string()
            + &"Some italic text here\n".italic().to_string()
            + &"Some underlined text here ".underline().to_string()
            + &" And some strikethrough text here ".strikethrough().to_string()
            + " Perhaps I can even put a quote here? "
            + "“man suffers more in imagination than in reality”\n",
    );
    
}

#[test]
fn test_format_structural_nodes(){

}

#[test]
fn test_format_content(){

}

fn test(){
    let a = vec!["<p>“There <em>are</em> some angels without wings, little Grissel. Not many I admit; but I have known a few.”</p>".to_string()];
    let node = parse_contents(&a);
}

