trait Add<Rhs> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add<Point> for Point {
    type Output = Self;
    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<i32> for Point {
    type Output = Point;
    fn add(self, rhs: i32) -> Point {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1.add(p2);

    assert_eq!(p3, Point { x: 4, y: 6 });
    assert_eq!(p3.x, 4);
    assert_eq!(p3.y, 6);

    let p1 = Point { x: 1, y: 4 };
    let int2 = 2;
    let p3 = p1.add(int2);
    assert_eq!(p3.x, 3);
    assert_eq!(p3.y, 6);
}
