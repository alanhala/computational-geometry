use computational_geometry::chapter5::KdTree;

fn main() {
    let points: Vec<[f64; 2]> = vec![
        [4.0, 1.0],
        [4.0, 9.0],
        [6.0, 5.0],
        [7.0, 8.0],
        [9.0, 9.0],
        [9.0, 11.0],
        [9.0, 2.0],
        [10.0, 5.0],
        [11.0, 3.0],
        [12.0, 8.0],
        [15.0, 1.0],
    ];
    let kdtree = KdTree::new(points);
    kdtree.print();
}
