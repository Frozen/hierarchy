

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

type Parent = usize;
type Child = usize;

pub struct Index<T> {
    tree: Rc<RefCell<Tree<T>>>,
    index: usize
}

impl<T> Index<T> {
    fn new(ref_to_inner: Rc<RefCell<Tree<T>>>, index: usize) -> Index<T> {
        Index {
            tree: ref_to_inner,
            index
        }
    }

    pub fn add_node(&self, node: T) -> Index<T> {
        let idx = (*self.tree).borrow_mut().add_node_without_parent(node);
        Index::new(self.tree.clone(), idx)
    }
}


struct Tree<T> {
    node_list: Vec<T>,
    tree: HashMap<Parent, Vec<Child>>
}

impl<T> Tree<T> {
    fn new() -> Tree<T> {
        Tree {
            tree: HashMap::new(),
            node_list: Vec::new(),
        }
    }

    fn add_node_without_parent(&mut self, node: T) -> usize {
        let ind = self.node_list.len();
        self.node_list.push(node);
        ind
    }

    fn add_node_with_parent(&mut self, parent: Parent, node: T) -> usize {
        let ind = self.add_node_without_parent(node);
        self.attach_child(parent, ind);
        ind
    }

    fn attach_child(&mut self, parent: Parent, child: Child) {
        self.tree.entry(parent).or_insert_with(Vec::new).push(child);
    }

    fn len(&self) -> usize {
        self.node_list.len()
    }

}


pub struct Hierarchy<T> {
    tree: Rc<RefCell<Tree<T>>>
}

impl<T> Hierarchy<T> {
    pub fn new() -> Hierarchy<T> {
        Hierarchy {
            tree: Rc::new(RefCell::new(Tree::new()))
        }
    }

    pub fn add_node(&self, node: T) -> Index<T> {
        let idx = (*self.tree).borrow_mut().add_node_without_parent(node);
        Index::new(self.tree.clone(), idx)
    }

    pub fn len(&self) -> usize {
        (*self.tree).borrow().len()
    }

    pub fn get<'a, 'b>(&self, i: &'b Index<T>) -> &'a T {
        (*self.tree).borrow().node_list.get(i.index).unwrap()
    }
}


struct TreeIterator<'a, T: 'a> {
    node_list: &'a [T],
    tree: &'a HashMap<Parent, Vec<Child>>,
    parent: usize,
    nodes_for_iter: Vec<usize>,
    index: usize
}

impl<'a, T: 'a> TreeIterator<'a, T> {
    fn new(node_list: &'a [T], tree: &'a HashMap<Parent, Vec<Child>>, parent: usize) -> TreeIterator<'a, T> {

        TreeIterator {
            node_list,
            tree,
            parent,
            nodes_for_iter: aaaaa(tree, parent),
            index: 0,
        }
    }
}

fn aaaaa(tree: &HashMap<Parent, Vec<Child>>, parent: usize) -> Vec<usize> {

    let mut out: Vec<usize>  = vec![];
    let mut need_to_visit: Vec<usize> = vec![parent];//tree.get(&parent).unwrap_or_else(Vec::new);

    loop {

        if need_to_visit.len() == 0 {
            break
        }

        let cur = need_to_visit.pop().unwrap();

        if let Some(ref t) = tree.get(&cur) {
            out.extend_from_slice(t);
            need_to_visit.extend_from_slice(t);
        }
    }

    return out

}

impl<'a, T> Iterator for TreeIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let node = self.node_list.get(self.index);
        self.index+=1;
        node
    }
}
