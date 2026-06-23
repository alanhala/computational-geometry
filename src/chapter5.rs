use std::{fmt, usize};

use crate::chapter5::Color::Black;

#[derive(Debug)]
pub struct Arena<T> {
    nodes: Vec<Node<T>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    Red,
    Black,
}

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    color: Color,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Arena { nodes: vec![] }
    }

    pub fn add(&mut self, parent: Option<usize>, value: T) -> usize {
        self.nodes.push(Node {
            value: value,
            parent: parent,
            left: None,
            right: None,
            color: Color::Red,
        });
        self.nodes.len() - 1
    }
}

#[derive(Debug)]
pub struct BalancedBinarySearchTree<T: PartialOrd> {
    arena: Arena<T>,
    root: Option<usize>,
}

impl<T: Ord> BalancedBinarySearchTree<T> {
    pub fn new() -> Self {
        BalancedBinarySearchTree {
            arena: Arena::new(),
            root: None,
        }
    }

    pub fn insert(&mut self, value: T) {
        match self.root {
            None => {
                let index = self.arena.add(None, value);
                self.root = Some(index);
                self.fixup_node(index)
            }
            Some(root) => {
                self.insert_node(root, value);
            }
        }
    }

    pub fn rotate_left(&mut self) {
        self.rotate_left_node(self.root)
    }
    pub fn rotate_right(&mut self) {
        self.rotate_right_node(self.root)
    }

    pub fn search(&self, value: T) -> Option<&Node<T>> {
        self.search_node(self.root, value)
    }

    pub fn minimum(&self) -> Option<&Node<T>> {
        self.minimum_node(self.root)
    }

    fn minimum_node(&self, node: Option<usize>) -> Option<&Node<T>> {
        match node {
            None => None,
            Some(node) => match self.arena.nodes[node].left {
                None => Some(&self.arena.nodes[node]),
                Some(_) => self.minimum_node(self.arena.nodes[node].left),
            },
        }
    }

    fn insert_node(&mut self, node: usize, value: T) {
        if value == self.arena.nodes[node].value {
            return;
        }
        if value < self.arena.nodes[node].value {
            match self.arena.nodes[node].left {
                None => {
                    let index = self.arena.add(Some(node), value);
                    self.arena.nodes[node].left = Some(index);
                    self.fixup_node(index)
                }
                Some(left) => self.insert_node(left, value),
            }
        } else {
            match self.arena.nodes[node].right {
                None => {
                    let index = self.arena.add(Some(node), value);
                    self.arena.nodes[node].right = Some(index);
                    self.fixup_node(index)
                }
                Some(right) => self.insert_node(right, value),
            }
        }
    }

    fn fixup_node(&mut self, node: usize) {
        let mut z = node;
        while self.color(self.arena.nodes[z].parent) == Color::Red {
            let parent = self.arena.nodes[z].parent.unwrap(); // if parent is red it can't be None
            let grandparent = self.arena.nodes[parent].parent.unwrap(); // if parent is red, it can't be root
            if self.arena.nodes[grandparent].left == Some(parent) {
                let uncle = self.arena.nodes[grandparent].right;
                match self.color(uncle) {
                    Color::Red => {
                        self.arena.nodes[parent].color = Color::Black;
                        self.arena.nodes[uncle.unwrap()].color = Color::Black; // if uncle is red it is not nil
                        self.arena.nodes[grandparent].color = Color::Red;
                        z = grandparent;
                    }
                    Color::Black => {
                        if Some(z) == self.arena.nodes[parent].right {
                            z = parent;
                            self.rotate_left_node(Some(z));
                        }
                        let parent = self.arena.nodes[z].parent.unwrap();
                        let grandparent = self.arena.nodes[parent].parent.unwrap();
                        self.arena.nodes[parent].color = Color::Black;
                        self.arena.nodes[grandparent].color = Color::Red;
                        self.rotate_right_node(Some(grandparent));
                    }
                }
            } else {
                let uncle = self.arena.nodes[grandparent].left;
                match self.color(uncle) {
                    Color::Red => {
                        self.arena.nodes[parent].color = Color::Black;
                        self.arena.nodes[uncle.unwrap()].color = Color::Black; // if uncle is red it is not nil
                        self.arena.nodes[grandparent].color = Color::Red;
                        z = grandparent;
                    }
                    Color::Black => {
                        if Some(z) == self.arena.nodes[parent].left {
                            z = parent;
                            self.rotate_left_node(Some(z));
                        }
                        let parent = self.arena.nodes[z].parent.unwrap();
                        let grandparent = self.arena.nodes[parent].parent.unwrap();
                        self.arena.nodes[parent].color = Color::Black;
                        self.arena.nodes[grandparent].color = Color::Red;
                        self.rotate_right_node(Some(grandparent));
                    }
                }
            }
        }
        self.arena.nodes[self.root.unwrap()].color = Color::Black;
    }

    fn color(&self, node: Option<usize>) -> Color {
        match node {
            None => Color::Black,
            Some(idx) => self.arena.nodes[idx].color,
        }
    }

    fn search_node(&self, node: Option<usize>, value: T) -> Option<&Node<T>> {
        match node {
            None => None,
            Some(node) => {
                if self.arena.nodes[node].value == value {
                    Some(&self.arena.nodes[node])
                } else if value < self.arena.nodes[node].value {
                    self.search_node(self.arena.nodes[node].left, value)
                } else {
                    self.search_node(self.arena.nodes[node].right, value)
                }
            }
        }
    }

    pub fn maximum(&self) -> Option<&Node<T>> {
        self.maximum_node(self.root)
    }

    fn maximum_node(&self, node: Option<usize>) -> Option<&Node<T>> {
        match node {
            None => None,
            Some(node) => match self.arena.nodes[node].right {
                None => Some(&self.arena.nodes[node]),
                Some(_) => self.maximum_node(self.arena.nodes[node].right),
            },
        }
    }

    fn rotate_left_node(&mut self, node: Option<usize>) {
        match node {
            None => (),
            Some(x) => match self.arena.nodes[x].right {
                None => (),
                Some(y) => {
                    self.arena.nodes[x].right = self.arena.nodes[y].left;
                    match self.arena.nodes[y].left {
                        None => (),
                        Some(y_left) => self.arena.nodes[y_left].parent = Some(x),
                    }
                    self.arena.nodes[y].parent = self.arena.nodes[x].parent;
                    self.arena.nodes[y].left = Some(x);
                    match self.arena.nodes[x].parent {
                        None => self.root = Some(y),
                        Some(x_parent) => {
                            if self.arena.nodes[x_parent].right == Some(x) {
                                self.arena.nodes[x_parent].right = Some(y)
                            } else {
                                self.arena.nodes[x_parent].left = Some(y)
                            }
                        }
                    }
                    self.arena.nodes[x].parent = Some(y)
                }
            },
        }
    }

    fn rotate_right_node(&mut self, node: Option<usize>) {
        match node {
            None => (),
            Some(y) => match self.arena.nodes[y].left {
                None => (),
                Some(x) => {
                    self.arena.nodes[y].left = self.arena.nodes[x].right;
                    match self.arena.nodes[x].right {
                        None => (),
                        Some(x_right) => self.arena.nodes[x_right].parent = Some(y),
                    }
                    self.arena.nodes[x].parent = self.arena.nodes[y].parent;
                    self.arena.nodes[x].right = Some(y);
                    match self.arena.nodes[x].parent {
                        None => self.root = Some(x),
                        Some(y_parent) => {
                            if self.arena.nodes[y_parent].left == Some(y) {
                                self.arena.nodes[y_parent].left = Some(x)
                            } else {
                                self.arena.nodes[y_parent].right = Some(x)
                            }
                        }
                    }
                    self.arena.nodes[y].parent = Some(x)
                }
            },
        }
    }
}

impl<T: PartialOrd + fmt::Display> fmt::Display for BalancedBinarySearchTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.root {
            None => write!(f, "(empty)"),
            Some(root) => print_node(f, &self.arena, root, "", true, ""),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Builds CLRS Figure 13.4(a): the tree just after inserting z=4,
    // before fixup runs. Returns the tree and the arena index of z.
    //
    //         11(B)
    //        /     \
    //      2(R)   14(B)
    //     /    \      \
    //   1(B)  7(B)   15(R)
    //        /    \
    //      5(R)   8(R)  <- y (uncle)
    //      /
    //    4(R)  <- z
    fn build_clrs_13_4a() -> (BalancedBinarySearchTree<i32>, usize) {
        let mut tree = BalancedBinarySearchTree::new();

        let i11 = tree.arena.add(None, 11);
        let i2 = tree.arena.add(Some(i11), 2);
        let i14 = tree.arena.add(Some(i11), 14);
        let i1 = tree.arena.add(Some(i2), 1);
        let i7 = tree.arena.add(Some(i2), 7);
        let i15 = tree.arena.add(Some(i14), 15);
        let i5 = tree.arena.add(Some(i7), 5);
        let i8 = tree.arena.add(Some(i7), 8);
        let i4 = tree.arena.add(Some(i5), 4);

        tree.arena.nodes[i11].left = Some(i2);
        tree.arena.nodes[i11].right = Some(i14);
        tree.arena.nodes[i2].left = Some(i1);
        tree.arena.nodes[i2].right = Some(i7);
        tree.arena.nodes[i14].right = Some(i15);
        tree.arena.nodes[i7].left = Some(i5);
        tree.arena.nodes[i7].right = Some(i8);
        tree.arena.nodes[i5].left = Some(i4);

        tree.arena.nodes[i11].color = Color::Black;
        tree.arena.nodes[i14].color = Color::Black;
        tree.arena.nodes[i1].color = Color::Black;
        tree.arena.nodes[i7].color = Color::Black;
        // i2, i15, i5, i8, i4 stay Red (added as Red by Arena::add)

        tree.root = Some(i11);

        (tree, i4)
    }

    #[test]
    fn test_fixup_clrs_13_4a() {
        let (mut tree, z) = build_clrs_13_4a();
        tree.fixup_node(z);

        // Expected result: CLRS Figure 13.4(d)
        //       7(B)
        //      /     \
        //    2(R)    11(R)
        //   /    \   /   \
        // 1(B) 5(B) 8(B) 14(B)
        //      /              \
        //    4(R)             15(R)

        let root = tree.root.unwrap();
        assert_eq!(tree.arena.nodes[root].value, 7);
        assert_eq!(tree.arena.nodes[root].color, Color::Black);

        let i2 = tree.arena.nodes[root].left.unwrap();
        assert_eq!(tree.arena.nodes[i2].value, 2);
        assert_eq!(tree.arena.nodes[i2].color, Color::Red);

        let i1 = tree.arena.nodes[i2].left.unwrap();
        assert_eq!(tree.arena.nodes[i1].value, 1);
        assert_eq!(tree.arena.nodes[i1].color, Color::Black);

        let i5 = tree.arena.nodes[i2].right.unwrap();
        assert_eq!(tree.arena.nodes[i5].value, 5);
        assert_eq!(tree.arena.nodes[i5].color, Color::Black);

        let i4 = tree.arena.nodes[i5].left.unwrap();
        assert_eq!(tree.arena.nodes[i4].value, 4);
        assert_eq!(tree.arena.nodes[i4].color, Color::Red);

        let i11 = tree.arena.nodes[root].right.unwrap();
        assert_eq!(tree.arena.nodes[i11].value, 11);
        assert_eq!(tree.arena.nodes[i11].color, Color::Red);

        let i8 = tree.arena.nodes[i11].left.unwrap();
        assert_eq!(tree.arena.nodes[i8].value, 8);
        assert_eq!(tree.arena.nodes[i8].color, Color::Black);

        let i14 = tree.arena.nodes[i11].right.unwrap();
        assert_eq!(tree.arena.nodes[i14].value, 14);
        assert_eq!(tree.arena.nodes[i14].color, Color::Black);

        let i15 = tree.arena.nodes[i14].right.unwrap();
        assert_eq!(tree.arena.nodes[i15].value, 15);
        assert_eq!(tree.arena.nodes[i15].color, Color::Red);
    }
}

fn print_node<T: fmt::Display>(
    f: &mut fmt::Formatter,
    arena: &Arena<T>,
    idx: usize,
    prefix: &str,
    is_root: bool,
    connector: &str,
) -> fmt::Result {
    let node = &arena.nodes[idx];
    writeln!(f, "{}{}{}", prefix, connector, node.value)?;

    let child_prefix = if is_root {
        String::new()
    } else if connector == "├── " {
        format!("{}│   ", prefix)
    } else {
        format!("{}    ", prefix)
    };

    if let Some(right) = node.right {
        let right_connector = if node.left.is_some() {
            "├── "
        } else {
            "└── "
        };
        print_node(f, arena, right, &child_prefix, false, right_connector)?;
    }
    if let Some(left) = node.left {
        print_node(f, arena, left, &child_prefix, false, "└── ")?;
    }
    Ok(())
}
