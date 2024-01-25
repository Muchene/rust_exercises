use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
};
#[derive(Clone, Debug)]
pub struct GraphNode<'a, T> {
    obj: T,
    children: Vec<&'a GraphNode<'a, T>>,
}

impl<'a, T: Copy + Eq + PartialEq + Hash> GraphNode<'a, T> {
    pub fn new(obj: T) -> GraphNode<'a, T> {
        GraphNode {
            obj: obj,
            children: Vec::new(),
        }
    }
    pub fn add_child(&mut self, child: &'a GraphNode<'a, T>) {
        self.children.push(child);
    }

    pub fn top_sort(&self) -> Option<Vec<T>> {
        let mut stack = VecDeque::new();
        let mut visited: HashSet<&T> = HashSet::new();
        let mut visiting: HashSet<&T> = HashSet::new();
        let mut result = Vec::new();

        stack.push_front(self);
        while let Some(node) = stack.front_mut() {
            if visiting.contains(&node.obj) {
                //cycle detect
                let mut acyclic = true;
                for child in node.children.iter() {
                    acyclic &= visited.contains(&child.obj);
                }
                if !acyclic {
                    return None;
                }
                result.push(node.obj);
                visited.insert(&node.obj);
                visiting.remove(&node.obj);
                stack.pop_front();
            } else {
                visiting.insert(&node.obj);
                if node.children.len() == 0 {
                    result.push(node.obj);
                    visited.insert(&node.obj);
                    visiting.remove(&node.obj);
                    stack.pop_front();
                    continue;
                }
                for child in node.children.iter() {
                    if !visited.contains(&child.obj) {
                        stack.push_front(child);
                    }
                }
            }
        }
        result.reverse();
        Some(result)
    }
}
