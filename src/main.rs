use computational_geometry::chapter5::KdTree;

fn main() {
    let points: Vec<[f64; 2]> = vec![
        [2.0, 5.0],  // p1
        [4.0, 8.0],  // p2
        [5.0, 3.0],  // p3
        [2.0, 10.0], // p4
        [5.0, 9.0],  // p5
        [7.0, 2.0],  // p6
        [6.0, 6.0],  // p7
        [8.0, 4.0],  // p8
        [7.0, 10.0], // p9
        [8.0, 8.0],  // p10
    ];
    let kdtree = KdTree::new(points);
    print!("{}", kdtree);
    println!("{:?}", kdtree.search([5.0, 3.0], [9.0, 6.0]));
}
