use std::collections::HashMap;

fn main() {

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores:HashMap<_, _> = 
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut ref_name = &field_name;
    let ref_name_2 = &field_name;
   
    ref_name.push_str("dfaf");


    let mut map = HashMap::new();
    map.insert(ref_name, field_value);
    // field_name and field_value are invalid at this point. 
    //println!("{}", field_value);
}
