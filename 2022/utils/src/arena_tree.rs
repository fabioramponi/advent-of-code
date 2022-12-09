#[derive(Debug, Default)]
pub struct ArenaTree<T> 
where
    T: PartialEq
{
    pub arena: Vec<Node<T>>,
}

impl<T> ArenaTree<T>
where
    T: PartialEq
{
    pub fn node(&mut self, val: T) -> usize {
        //first see if it exists
        for node in &self.arena {
            if node.val == val {
                return node.idx;
            }
        }
        // Otherwise, add new node
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, val));
        idx
    }

    fn size(&self) -> usize {
        self.arena.len()
    }

    fn edges(&self) -> usize {
        self.arena.iter().fold(0, |acc, node| acc + node.children.len())
    }

    fn depth(&self, idx: usize) -> usize {
        match self.arena[idx].parent {
            Some(id) => 1 + self.depth(id),
            None => 0,
        }
    }

    pub fn insert_or_get(&mut self, new_node: T, parent_node_idx: usize) -> usize{
        let prev_sz = self.size();
        let new_node_idx = self.node(new_node);
        if (prev_sz<self.size()) {

            // set orbit
            match self.arena[new_node_idx].parent {
                Some(_) => panic!("Attempt to overwrite existing orbit"),
                None => self.arena[new_node_idx].parent = Some(parent_node_idx),
            }
            // set parents
            self.arena[parent_node_idx].children.push(new_node_idx);
        }
        new_node_idx
    }
}

#[derive(Debug)]
pub struct Node<T>
where
    T: PartialEq
{
    pub idx: usize,
    pub val: T,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

impl<T> Node<T>
where
    T: PartialEq
{
    fn new(idx: usize, val: T) -> Self {
        Self {
            idx,
            val,
            parent: None,
            children: vec![],
        }
    }
}