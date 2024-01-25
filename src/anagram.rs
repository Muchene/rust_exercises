
use std::fs::read_to_string;
use std::collections::HashMap;

pub fn sort_string(unsorted : &String) -> String {
    let mut chars: Vec<char> = unsorted.chars().collect();
    chars.sort();
    chars.into_iter().collect()
}

pub fn read_into_ht(fname: &str) -> Result<HashMap<String, Vec<String>>, Box<dyn std::error::Error>> {
    let mut anagram_hash:HashMap<String, Vec<String>> = HashMap::new();
    for line in read_to_string(fname).unwrap().lines(){
        let sorted_string = sort_string(&line.to_string());
        match anagram_hash.get_mut(&sorted_string) {
            Some(current) => {
                current.push(line.to_string());
            }
            _ =>{
                anagram_hash.insert(sorted_string, vec![line.to_string()]).into_iter();
            }
        }
    }
    Ok(anagram_hash)
}
