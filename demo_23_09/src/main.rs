use std::fmt::Error;



fn get_shortest (names: Vec<&str>) -> Option<&str> {
    if names.len() > 0 {
        let mut name = names[0];
        for i in names {
            if i.len() < name.len() {
                name = i;
            }
        }
        Some(name)
    } else {
        None
    }
}

fn show_shortest(names: Vec<&str>) -> &str {
    get_shortest(names).unwrap_or("Not Found")
}
fn main() {
    // assert_eq!(show_shortest(vec!["hello", "world", "yes"]), "yes");
    assert_eq!(show_shortest(Vec::new()), "Not Found");
    // show_shortest(vec!["hello", "world", "yes"])
}   

