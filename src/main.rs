use computational_geometry::chapter5::BalancedBinarySearchTree;

fn main() {
    let points: Vec<usize> = vec![49, 23, 80, 10, 37, 62, 89, 3, 19, 30, 59, 70, 100];
    let tree = BalancedBinarySearchTree::new(points);
    tree.print();
    println!("{:?}", tree.range_query(18, 77));

    // let mut tree = BalancedBinarySearchTree::new();

    // // level order so a plain BST gets the right shape
    // tree.insert(49);
    // tree.insert(23);
    // tree.insert(80);
    // tree.insert(10);
    // tree.insert(37);
    // tree.insert(62);
    // tree.insert(89);
    // tree.insert(3);
    // tree.insert(19);
    // tree.insert(30);
    // tree.insert(59);
    // tree.insert(70);
    // tree.insert(100);

    // match tree.search(23) {
    //     None => println!("Not found"),
    //     Some(node) => println!("Node found {}", node.value),
    // }
    // match tree.minimum() {
    //     None => println!("Empty!"),
    //     Some(node) => println!("Minimum {}", node.value),
    // }
    // match tree.maximum() {
    //     None => println!("Empty!"),
    //     Some(node) => println!("Maximum {}", node.value),
    // }

    // println!("{}", tree);
    // tree.rotate_left();
    // println!("{}", tree);
    // tree.rotate_right();
    // println!("{}", tree);
}
