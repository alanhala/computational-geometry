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

}

fn fmt_bst_node<T: fmt::Display>(
    arena: &Arena<T>,
    f: &mut fmt::Formatter<'_>,
    index: usize,
    prefix: &str,
    is_left: bool,
) -> fmt::Result {
    let node = &arena.nodes[index];
    let connector = if is_left { "├── " } else { "└── " };
    writeln!(f, "{}{}{}", prefix, connector, node.value)?;
    let new_prefix = format!("{}{}", prefix, if is_left { "│   " } else { "    " });
    if let Some(left) = node.left {
        fmt_bst_node(arena, f, left, &new_prefix, true)?;
    }
    if let Some(right) = node.right {
        fmt_bst_node(arena, f, right, &new_prefix, false)?;
    }
    Ok(())
}

fn fmt_kd_node<T: fmt::Debug, const D: usize>(
    arena: &Arena<Point<T, D>>,
    f: &mut fmt::Formatter<'_>,
    index: usize,
    prefix: &str,
    is_left: Option<bool>,
) -> fmt::Result {
    let node = &arena.nodes[index];
    let connector = match is_left {
        None => "",
        Some(true) => "├── ",
        Some(false) => "└── ",
    };
    writeln!(f, "{}{}{:?}", prefix, connector, node.value)?;
    let new_prefix = match is_left {
        None => String::new(),
        Some(true) => format!("{}│   ", prefix),
        Some(false) => format!("{}    ", prefix),
    };
    if let Some(left) = node.left {
        fmt_kd_node(arena, f, left, &new_prefix, Some(true))?;
    }
    if let Some(right) = node.right {
        fmt_kd_node(arena, f, right, &new_prefix, Some(false))?;
    }
    Ok(())
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

impl<T> fmt::Display for BalancedBinarySearchTree<T>
where
    T: Ord + Copy + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.arena.nodes[self.root].value)?;
        let root = &self.arena.nodes[self.root];
        if let Some(left) = root.left {
            fmt_bst_node(&self.arena, f, left, "", true)?;
        }
        if let Some(right) = root.right {
            fmt_bst_node(&self.arena, f, right, "", false)?;
        }
        Ok(())
    }
}

type SortedAxes<T, const D: usize> = Vec<Vec<Point<T, D>>>;
type Point<T, const D: usize> = [T; D];

#[derive(Debug)]
pub struct KdTree<T, const D: usize>
where
    T: PartialOrd,
    T: Clone,
    T: Copy,
{
    arena: Arena<Point<T, D>>,
    root: usize,
}

impl<const D: usize, T> KdTree<T, D>
where
    T: PartialOrd,
    T: Clone,
    T: Copy,
{
    pub fn new(points: Vec<Point<T, D>>) -> Self {
        assert!(!points.is_empty(), "points must not be empty");
        let mut arena = Arena::new();
        let sorted_by_axes: Vec<Vec<Point<T, D>>> = (0..D)
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

    pub fn search(&self, min: Point<T, D>, max: Point<T, D>) -> Vec<Point<T, D>> {
        let mut reported_values: Vec<Point<T, D>> = vec![];
        Self::report_tree(&self.arena, &mut reported_values, self.root, 0, min, max);
        reported_values
    }

    fn report_tree(
        arena: &Arena<Point<T, D>>,
        reported_values: &mut Vec<Point<T, D>>,
        node: usize,
        depth: usize,
        min: Point<T, D>,
        max: Point<T, D>,
    ) {
        let axis = depth % D;
        if arena.nodes[node].is_leaf() {
            if Self::point_contained(arena.nodes[node].value, min, max) {
                reported_values.push(arena.nodes[node].value);
            }
        } else if arena.nodes[node].value[axis] >= min[axis] && arena.nodes[node].value[axis] < max[axis] {
            Self::report_tree(
                arena,
                reported_values,
                arena.nodes[node].left.unwrap(),
                depth + 1,
                min,
                max,
            );
            Self::report_tree(
                arena,
                reported_values,
                arena.nodes[node].right.unwrap(),
                depth + 1,
                min,
                max,
            );
        } else if arena.nodes[node].value[axis] < min[axis] {
            Self::report_tree(
                arena,
                reported_values,
                arena.nodes[node].right.unwrap(),
                depth + 1,
                min,
                max,
            );
        } else if arena.nodes[node].value[axis] >= max[axis] {
            Self::report_tree(
                arena,
                reported_values,
                arena.nodes[node].left.unwrap(),
                depth + 1,
                min,
                max,
            );
        }
    }

    fn point_contained(point: Point<T, D>, min: Point<T, D>, max: Point<T, D>) -> bool {
        point
            .iter()
            .zip(min.iter())
            .zip(max.iter())
            .all(|((vi, lo), hi)| vi >= lo && vi <= hi)
    }

    fn build(arena: &mut Arena<Point<T, D>>, sorted_by_axes: Vec<Vec<Point<T, D>>>, depth: usize) -> Option<usize> {
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
                            let (left, right): (Vec<Point<T, D>>, Vec<Point<T, D>>) =
                                sorted_by_axes[i].iter().partition(|p| {
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
                arena.nodes[node].left = Self::build(arena, left, depth + 1);
                arena.nodes[node].right = Self::build(arena, right, depth + 1);
                Some(node)
            }
        }
    }
}

impl<T, const D: usize> fmt::Display for KdTree<T, D>
where
    T: PartialOrd + Clone + Copy + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_kd_node(&self.arena, f, self.root, "", None)
    }
}
