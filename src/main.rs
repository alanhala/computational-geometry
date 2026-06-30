use std::collections::HashMap;
use computational_geometry::chapter5::KdTree;

fn main() {
    let points: HashMap<usize, [f64; 2]> = HashMap::from([
        (1,  [2.0, 5.0]),
        (2,  [4.0, 8.0]),
        (3,  [5.0, 3.0]),
        (4,  [2.0, 10.0]),
        (5,  [5.0, 9.0]),
        (6,  [7.0, 2.0]),
        (7,  [6.0, 6.0]),
        (8,  [8.0, 4.0]),
        (9,  [7.0, 10.0]),
        (10, [8.0, 8.0]),
    ]);
    let kdtree = KdTree::new(points);
    print!("{}", kdtree);
    println!("{:?}", kdtree.search([5.0, 3.0], [9.0, 6.0]));
}
