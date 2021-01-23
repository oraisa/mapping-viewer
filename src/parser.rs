use logos::Logos;
use crate::bindings::{Window, Tab, BindingGroup, Binding};

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[regex(r"###.*\n", |lex| title_from_string(lex.slice()))]
    WindowTitle(String),
    #[regex(r"##.*\n", |lex| title_from_string(lex.slice()))]
    TabTitle(String),
    #[regex(r"#.*\n", |lex| title_from_string(lex.slice()))]
    GroupTitle(String),
    #[regex(r".*|.*", |lex| binding_from_string(lex.slice()))]
    Binding(Binding),

    #[error]
    #[regex(r"[\t\n\f]+", logos::skip)]
    Error
}

fn title_from_string(val: &str) -> String {
    let trimmed = val.trim().trim_start_matches('#').trim();
    String::from(trimmed)
}

fn binding_from_string(val: &str) -> Binding {
    let mut parts = val.split("|");
    let part1 = parts.next();
    let part2 = parts.next();
    let keys = match part1 {
        Some(val) => val.trim(),
        None => ""
    };
    let action = match part2 {
        Some(val) => val.trim(),
        None => ""
    };
    Binding { keys : keys.to_string(), action : action.to_string() }
}

pub fn parse_bindings(input: &str) -> Window {
    let mut lex = Token::lexer(input);
    let mut window = Window {
        title: String::from("Mapping Viewer"), tabs: Vec::new()
    };
    loop {
        match lex.next() {
            Some(Token::WindowTitle(title)) => window.title = title,
            Some(Token::TabTitle(title)) => window.tabs.push(Tab {
                title: title, groups: Vec::new()
            }),
            Some(Token::GroupTitle(title)) => window
                .tabs.last_mut()
                     .expect("No previous tab")
                     .groups
                     .push(BindingGroup { title: title, bindings: Vec::new()}),
            Some(Token::Binding(binding)) => window
                .tabs.last_mut()
                     .expect("No previous tab")
                     .groups
                     .last_mut()
                     .expect("No previous group")
                     .bindings.push(binding),
            Some(Token::Error) => (),
            None => break()
        }
    }
    window
}

#[test]
fn test_binding_from_string() {
    let input = "Super-e | default";
    let result = binding_from_string(input);
    assert_eq!(result.keys, "Super-e");
    assert_eq!(result.action, "default");
}

#[test]
fn test_binding_from_string_with_two_pipes() {
    let input = "Super-e | default | hello";
    let result = binding_from_string(input);
    assert_eq!(result.keys, "Super-e");
    assert_eq!(result.action, "default");
}

#[test]
fn test_binding_from_string_with_incomplete_binding() {
    let input = "Super-e";
    let result = binding_from_string(input);
    assert_eq!(result.keys, "Super-e");
    assert_eq!(result.action, "");
}

#[test]
fn test_title_from_string() {
    let input = "## Hello world";
    assert_eq!(title_from_string(input), "Hello world");
}

#[test]
fn test_parsing_simple() {
    let input = "# Hello World\nSuper-q | quit";
    let mut lex = Token::lexer(input);
    assert_eq!(lex.next(), Some(Token::GroupTitle(String::from("Hello World"))));
    assert_eq!(lex.next(), Some(Token::Binding(Binding {
        keys: String::from("Super-q"), action: String::from("quit")
    })));
    assert_eq!(lex.next(), None);
}

const _PARSER_INPUT: &str = r"
### Hello world
## #Tab
# Group #
Super-a | test
Super-g | test2
# Group 2
Shift-a | capital a
";

#[test]
fn test_parsing_hard() {
    let mut lex = Token::lexer(_PARSER_INPUT);
    assert_eq!(lex.next(), Some(Token::WindowTitle(String::from("Hello world"))));
    assert_eq!(lex.next(), Some(Token::TabTitle(String::from("#Tab"))));
    assert_eq!(lex.next(), Some(Token::GroupTitle(String::from("Group #"))));
    assert_eq!(lex.next(), Some(Token::Binding(Binding {
        keys: String::from("Super-a"), action: String::from("test")
    })));
    assert_eq!(lex.next(), Some(Token::Binding(Binding {
        keys: String::from("Super-g"), action: String::from("test2")
    })));

    assert_eq!(lex.next(), Some(Token::GroupTitle(String::from("Group 2"))));
    assert_eq!(lex.next(), Some(Token::Binding(Binding {
        keys: String::from("Shift-a"), action: String::from("capital a")
    })));
    assert_eq!(lex.next(), None);
}

#[test]
fn test_parse_bindings() {
    let result = parse_bindings(_PARSER_INPUT);
    assert_eq!(result.title, "Hello world");
    assert_eq!(result.tabs.len(), 1);
    let tabs = result.tabs.first().unwrap();
    assert_eq!(tabs.title, "#Tab");
    assert_eq!(tabs.groups.len(), 2);
    let groups = &tabs.groups;
    assert_eq!(groups[0].title, "Group #");
    assert_eq!(groups[0].bindings.len(), 2);
    assert_eq!(groups[0].bindings[0].keys, "Super-a");
    assert_eq!(groups[0].bindings[1].keys, "Super-g");
    assert_eq!(groups[0].bindings[0].action, "test");
    assert_eq!(groups[0].bindings[1].action, "test2");

    assert_eq!(groups[1].title, "Group 2");
    assert_eq!(groups[1].bindings.len(), 1);
    assert_eq!(groups[1].bindings[0].keys, "Shift-a");
    assert_eq!(groups[1].bindings[0].action, "capital a");
}
