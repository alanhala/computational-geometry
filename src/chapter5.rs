use std::fmt;

#[derive(Debug)]
pub struct Arena<T> {
    nodes: Vec<Node<T>>,
}

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    left: Option<usize>,
    right: Option<usize>,
}

impl<T> Node<T> {
    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Arena { nodes: vec![] }
    }

    pub fn add(&mut self, value: T) -> usize {
        self.nodes.push(Node {
            value,
            left: None,
            right: None,
        });
        self.nodes.len() - 1
    }

    fn print_tree(&self, index: usize, prefix: &str, is_left: bool)
    where
        T: fmt::Display,
    {
        let node = &self.nodes[index];
        let connector = if is_left { "├── " } else { "└── " };
        println!("{}{}{}", prefix, connector, node.value);
        let new_prefix = format!("{}{}", prefix, if is_left { "│   " } else { "    " });
        if let Some(left) = node.left {
            self.print_tree(left, &new_prefix, true);
        }
        if let Some(right) = node.right {
            self.print_tree(right, &new_prefix, false);
        }
    }
}

#[derive(Debug)]
pub struct BalancedBinarySearchTree<T: Ord> {
    arena: Arena<T>,
    root: usize,
}

impl<T> BalancedBinarySearchTree<T>
where
    T: Ord,
    T: Copy,
{
    pub fn print(&self)
    where
        T: fmt::Display,
    {
        let root = &self.arena.nodes[self.root];
        println!("{}", root.value);
        if let Some(left) = root.left {
            self.arena.print_tree(left, "", true);
        }
        if let Some(right) = root.right {
            self.arena.print_tree(right, "", false);
        }
    }

    pub fn new(mut points: Vec<T>) -> Self {
        points.sort();
        let mut arena = Arena::new();
        let root = Self::build(&mut arena, &points).expect("points must not be empty");
        Self { arena, root }
    }

    pub fn find_split_node(&self, min: T, max: T) -> usize {
        let mut v = self.root;
        while !self.arena.nodes[v].is_leaf() && (max <= self.arena.nodes[v].value || min > self.arena.nodes[v].value) {
            v = if max <= self.arena.nodes[v].value {
                self.arena.nodes[v].left.unwrap()
            } else {
                self.arena.nodes[v].right.unwrap()
            }
        }
        v
    }

    pub fn range_query(&self, min: T, max: T) -> Vec<T> {
        let mut reported_values: Vec<T> = vec![];
        let split_node = self.find_split_node(min, max);
        Self::report_tree(&self.arena, &mut reported_values, split_node, min, max);
        reported_values
    }

    fn report_tree(arena: &Arena<T>, reported_values: &mut Vec<T>, node: usize, min: T, max: T) {
        let value = arena.nodes[node].value;
        if arena.nodes[node].is_leaf() {
            if value >= min && value <= max {
                reported_values.push(value);
            }
        } else if value >= min && value < max {
            Self::report_tree(arena, reported_values, arena.nodes[node].left.unwrap(), min, max);
            Self::report_tree(arena, reported_values, arena.nodes[node].right.unwrap(), min, max);
        } else if value < min {
            Self::report_tree(arena, reported_values, arena.nodes[node].right.unwrap(), min, max);
        } else if value >= max {
            Self::report_tree(arena, reported_values, arena.nodes[node].left.unwrap(), min, max);
        };
    }

    pub fn value(&self, node: usize) -> Option<T> {
        match self.arena.nodes.get(node) {
            None => None,
            Some(node) => {
                if node.is_leaf() {
                    Some(node.value)
                } else {
                    None
                }
            }
        }
    }

    fn build(arena: &mut Arena<T>, points: &[T]) -> Option<usize> {
        match points.len() {
            0 => None,
            1 => Some(arena.add(points[0])),
            n => {
                let split = (n - 1) / 2;
                let left = &points[..=split];
                let right = &points[split + 1..];
                let node = arena.add(points[split]);
                let left = Self::build(arena, left);
                let right = Self::build(arena, right);
                arena.nodes[node].left = left;
                arena.nodes[node].right = right;
                Some(node)
            }
        }
    }
}

type SortedAxes<T, const D: usize> = Vec<Vec<[T; D]>>;

#[derive(Debug)]
pub struct KdTree<T, const D: usize>
where
    T: PartialOrd,
    T: Clone,
    T: Copy,
{
    arena: Arena<[T; D]>,
    root: usize,
}

impl<const D: usize, T> KdTree<T, D>
where
    T: PartialOrd,
    T: Clone,
    T: Copy,
{
    pub fn new(points: Vec<[T; D]>) -> Self {
        assert!(!points.is_empty(), "points must not be empty");
        let mut arena = Arena::new();
        let sorted_by_axes: Vec<Vec<[T; D]>> = (0..D)
            .map(|i| {
                let mut ax = points.clone();
                ax.sort_by(|a, b| {
                    a[i].partial_cmp(&b[i])
                        .unwrap()
                        .then_with(|| a.iter().partial_cmp(b).unwrap())
                });
                ax
            })
            .collect();
        let root = Self::build(&mut arena, sorted_by_axes, 0).expect("points must not be empty");
        Self { arena, root }
    }

    pub fn print(&self)
    where
        T: fmt::Debug,
    {
        if self.arena.nodes[self.root].is_leaf() {
            println!("{:?}", self.arena.nodes[self.root].value);
        } else {
            println!("*");
        }
        if let Some(left) = self.arena.nodes[self.root].left {
            self.print_node(left, "", true);
        }
        if let Some(right) = self.arena.nodes[self.root].right {
            self.print_node(right, "", false);
        }
    }

    fn print_node(&self, index: usize, prefix: &str, is_left: bool)
    where
        T: fmt::Debug,
    {
        let node = &self.arena.nodes[index];
        let connector = if is_left { "├── " } else { "└── " };
        let new_prefix = format!("{}{}", prefix, if is_left { "│   " } else { "    " });
        if node.is_leaf() {
            println!("{}{}{:?}", prefix, connector, node.value);
        } else {
            println!("{}{}*", prefix, connector);
        }
        if let Some(left) = node.left {
            self.print_node(left, &new_prefix, true);
        }
        if let Some(right) = node.right {
            self.print_node(right, &new_prefix, false);
        }
    }

    fn build(arena: &mut Arena<[T; D]>, sorted_by_axes: Vec<Vec<[T; D]>>, depth: usize) -> Option<usize> {
        let axis = depth % D;
        match sorted_by_axes[axis].len() {
            0 => None,
            1 => Some(arena.add(sorted_by_axes[axis][0])),
            n => {
                let split = (n - 1) / 2;
                let median = &sorted_by_axes[axis][split];
                let node = arena.add(*median);
                let (left, right): (SortedAxes<T, D>, SortedAxes<T, D>) = (0..D)
                    .map(|i| {
                        if i == axis {
                            let left = &sorted_by_axes[i][0..=split];
                            let right = &sorted_by_axes[i][split + 1..];
                            (left.to_vec(), right.to_vec())
                        } else {
                            let (left, right): (Vec<[T; D]>, Vec<[T; D]>) = sorted_by_axes[i].iter().partition(|p| {
                                p[axis]
                                    .partial_cmp(&median[axis])
                                    .unwrap()
                                    .then_with(|| p.partial_cmp(&median).unwrap())
                                    .is_le()
                            });
                            (left, right)
                        }
                    })
                    .unzip();
                let left = Self::build(arena, left, depth + 1);
                let right = Self::build(arena, right, depth + 1);
                arena.nodes[node].left = left;
                arena.nodes[node].right = right;
                Some(node)
            }
        }
    }
}
