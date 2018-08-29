use std::collections::HashMap;

type Parent = usize;
type Child = usize;
type Index = usize;


#[derive(Debug)]
pub struct Hierarchy<T> {
    /// contains nodes
    node_list: Vec<T>,
    /// contains relations parent to child
    tree: HashMap<Parent, Vec<Child>>
}

impl<T> Hierarchy<T> {
    pub fn new() -> Hierarchy<T> {
        Hierarchy {
            tree: HashMap::new(),
            node_list: Vec::new()
        }
    }

    pub fn add_root_node(&mut self, node: T) -> Index {
        let ind = self.node_list.len();
        self.node_list.push(node);
        ind
    }

    pub fn add_sub_node(&mut self, parent: Index, node: T) -> Index {
        let ind = self.add_root_node(node);
        self.attach_child(parent, ind);
        ind
    }

    fn attach_child(&mut self, parent: Parent, child: Child) {
        self.tree.entry(parent).or_insert_with(Vec::new).push(child);
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.node_list.len()
    }

    /// return `T` by it index
    pub fn get(&self, i: Index) -> Option<&T> {
        self.node_list.get(i)
    }

    pub fn iter_child(&self, parent: Index) -> TreeIterator<T> {
        TreeIterator::new(&self.node_list, &self.tree, parent)
    }

    pub fn iter(&self) -> impl Iterator<Item=&T>{
        self.node_list.iter()
    }
}

impl<T> ::std::ops::Index<usize> for Hierarchy<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.node_list[index]
    }
}

#[derive(Debug)]
pub struct TreeIterator<'a, T: 'a> {
    node_list: &'a [T],
    tree: &'a HashMap<Parent, Vec<Child>>,
    /// root node, we should iterate over it children
    parent: usize,
    /// nodes indexes that will be iterate over
    nodes_for_iter: Vec<usize>,
    /// current index of our iterator
    index: usize
}

impl<'a, T: 'a> TreeIterator<'a, T> {
    fn new(node_list: &'a [T], tree: &'a HashMap<Parent, Vec<Child>>, parent: usize) -> TreeIterator<'a, T> {
        TreeIterator {
            node_list,
            tree,
            parent,
            nodes_for_iter: collect_nodes(tree, parent),
            index: 0,
        }
    }
}

fn collect_nodes(tree: &HashMap<Parent, Vec<Child>>, parent: usize) -> Vec<usize> {

    let mut out: Vec<usize>  = vec![];
    let mut need_to_visit: Vec<usize> = vec![parent];

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
        if let Some(index_to_visit) = self.nodes_for_iter.get(self.index) {
            self.index+=1;
            self.node_list.get(*index_to_visit)
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {

    use super::Hierarchy;

    #[test]
    fn test_hierarchy() {

        let mut a = Hierarchy::new();
        let root = a.add_root_node(1i32);
        let sub1 = a.add_sub_node(root, 8);
        let sub2 = a.add_sub_node(root, 9);
        let _sub3 = a.add_sub_node(sub1, 11);

        assert_eq!(4, a.len());
        assert_eq!(Some(&8), a.get(sub1));
        assert_eq!(Some(&9), a.get(sub2));
        assert_eq!(9, a[sub2]);
        assert_eq!(None, a.get(5 ));

        assert_eq!(vec![&1, &8, &9, &11], a.iter().collect::<Vec<&i32>>());
        assert_eq!(vec![&8, &9, &11], a.iter_child(root).collect::<Vec<&i32>>());
        assert_eq!(vec![&11], a.iter_child(sub1).collect::<Vec<&i32>>());

    }

}
