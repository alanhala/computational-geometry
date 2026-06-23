use std::ops::Sub;

#[derive(Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn cross(&self, other: &Self) -> f64 {
        self.x * other.y - self.y * other.x
    }

    pub fn signed_angle(&self, other: &Vector) -> f64 {
        self.cross(other).atan2(self.dot(other))
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl Sub for &Point {
    type Output = Vector;
    fn sub(self: Self, other: Self) -> Vector {
        Vector::new(other.x - self.x, other.y - self.y)
    }
}

pub struct Edge<'a> {
    pub from: &'a Point,
    pub to: &'a Point,
}
impl<'a> Edge<'a> {
    pub fn new(from: &'a Point, to: &'a Point) -> Self {
        Self { from, to }
    }
}

pub fn slow_convex_hull<'a>(points: &'a [Point]) -> Vec<&'a Point> {
    let mut edges: Vec<Edge> = vec![];
    for p in points {
        for q in points {
            if p != q {
                let mut is_edge = true;
                let pq = q - p;
                for r in points {
                    if r != q && r != p {
                        let pr = r - p;
                        if pq.signed_angle(&pr) > 0.0 {
                            is_edge = false;
                        }
                    }
                }
                if is_edge {
                    edges.push(Edge::new(p, q));
                }
            }
        }
    }
    let mut hull: Vec<&Point> = vec![];
    match edges.pop() {
        None => (),
        Some(first_edge) => {
            hull.push(first_edge.from);
            hull.push(first_edge.to);
            let mut last_point = first_edge.to;
            while !edges.is_empty() {
                let next_edge_position = edges.iter().position(|e| e.from == last_point);
                let next_edge = edges.remove(next_edge_position.unwrap());
                if next_edge.to != first_edge.from {
                    hull.push(next_edge.to);
                    last_point = next_edge.to;
                }
            }
        }
    }
    hull
}

pub fn convex_hull(points: &[Point]) -> Vec<Point> {
    let mut sorted = points.to_vec();
    sorted.sort_by(|a, b| a.x.total_cmp(&b.x).then_with(|| a.y.total_cmp(&b.y)));
    let mut upper: Vec<Point> = vec![];
    upper.push(sorted[0]);
    upper.push(sorted[1]);
    for i in 2..sorted.len() {
        let p3 = sorted[i];
        while upper.len() >= 2 {
            let &[p1, p2] = upper.last_chunk::<2>().unwrap();
            let edge1 = &p3 - &p1;
            let edge2 = &p2 - &p1;
            if edge1.signed_angle(&edge2) >= 0.0 {
                upper.pop();
            } else {
                break;
            }
        }
        upper.push(p3);
    }
    let mut lower: Vec<Point> = vec![];
    lower.push(sorted[sorted.len() - 1]);
    lower.push(sorted[sorted.len() - 2]);
    for i in (0..sorted.len() - 3).rev() {
        let p3 = sorted[i];
        while lower.len() >= 2 {
            let &[p1, p2] = lower.last_chunk::<2>().unwrap();
            let edge1 = &p3 - &p1;
            let edge2 = &p2 - &p1;
            if edge1.signed_angle(&edge2) >= 0.0 {
                lower.pop();
            } else {
                break;
            }
        }
        lower.push(p3);
    }
    lower.remove(0);
    lower.remove(lower.len() - 1);

    [upper, lower].concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convex_hull_test() {
        // Outer square corners = the true hull.
        // Everything else is strictly inside and must be deleted,
        // exactly like the "points deleted" arrows in the figure.
        let points = vec![
            Point::new(0.0, 0.0), // corner
            Point::new(6.0, 0.0), // corner
            Point::new(6.0, 6.0), // corner
            Point::new(0.0, 6.0), // corner
            Point::new(3.0, 3.0), // interior
            Point::new(2.0, 4.0), // interior
            Point::new(4.0, 2.0), // interior
            Point::new(1.0, 1.0), // interior
            Point::new(5.0, 5.0), // interior
            Point::new(3.0, 5.0), // interior (just below top edge)
        ];

        let hull = convex_hull(&points);

        let expected = vec![
            Point::new(0.0, 0.0),
            Point::new(6.0, 0.0),
            Point::new(6.0, 6.0),
            Point::new(0.0, 6.0),
        ];

        assert_eq!(
            hull, expected,
            "hull should be exactly the 4 square corners"
        );
    }

    #[test]
    fn convex_hull_collinear_test() {
        // A square whose every edge has an extra point sitting exactly on it.
        // Those midpoints are collinear with the corners, so per the book the
        // middle point "should not occur on the convex hull" — only the 4
        // corners should survive.
        let points = vec![
            Point::new(0.0, 0.0), // corner
            Point::new(4.0, 0.0), // corner
            Point::new(4.0, 4.0), // corner
            Point::new(0.0, 4.0), // corner
            Point::new(2.0, 0.0), // on bottom edge
            Point::new(2.0, 4.0), // on top edge
            Point::new(0.0, 2.0), // on left edge
            Point::new(4.0, 2.0), // on right edge
            Point::new(2.0, 2.0), // dead center (interior)
        ];

        let hull = convex_hull(&points);

        let expected = vec![
            Point::new(0.0, 0.0),
            Point::new(4.0, 0.0),
            Point::new(4.0, 4.0),
            Point::new(0.0, 4.0),
        ];

        assert_eq!(
            hull, expected,
            "collinear edge points must be dropped; only corners remain"
        );
    }

    #[test]
    fn slow_convex_hull_test() {
        let points = vec![
            Point { x: 3.0, y: 2.5 }, // interior
            Point { x: 0.5, y: 1.8 }, // hull (leftmost)
            Point { x: 3.5, y: 1.8 }, // interior
            Point { x: 4.2, y: 3.5 }, // hull (top-right)
            Point { x: 4.8, y: 2.0 }, // hull (rightmost)
            Point { x: 2.5, y: 2.0 }, // interior
            Point { x: 1.8, y: 2.8 }, // interior
            Point { x: 2.2, y: 0.5 }, // hull (bottom)
            Point { x: 1.8, y: 3.8 }, // hull (top)
        ];

        let hull: Vec<Point> = slow_convex_hull(&points).into_iter().copied().collect();

        let expected = vec![
            Point::new(1.8, 3.8),
            Point::new(4.2, 3.5),
            Point::new(4.8, 2.0),
            Point::new(2.2, 0.5),
            Point::new(0.5, 1.8),
        ];

        assert_eq!(
            hull, expected,
            "hull should be exactly the 5 extreme points"
        );
    }
}
