use graph::GraphNode;

pub mod anagram;
pub mod graph;


fn main() {
    // let anagram_hash = anagram::read_into_ht("words.txt").unwrap();
    // let k = anagram::sort_string(&"god".to_string());
    // let anagrams = anagram_hash.get(&k).unwrap();
    // for an in anagrams {
    //     println!("{}", an);
    // }

    let mut g: GraphNode<&str> = GraphNode::new("a");
    let mut b: GraphNode<&str> = GraphNode::new("b");
    let mut c: GraphNode<&str> = GraphNode::new("c");
    let  d: GraphNode<&str> = GraphNode::new("d");

    g.add_child(&mut b);
    b.add_child(&mut c);
    c.add_child(&d);

    match g.top_sort() {
        Some(sorted) => {
            println!("{:?}", sorted);
        },
        None => {
            println!("No sort");
        }
    }

    
}