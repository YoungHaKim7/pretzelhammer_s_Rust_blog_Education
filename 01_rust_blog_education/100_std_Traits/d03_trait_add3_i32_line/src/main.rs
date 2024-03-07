trait Add<Rhs, Output> {
    fn add(self, rhs: Rhs) -> Output;
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add<Point, Point> for Point {
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<i32, Point> for Point {
    fn add(self, rhs: i32) -> Point {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

struct Line {
    start: Point,
    end: Point,
}

impl Add<Point, Line> for Point {
    fn add(self, rhs: Point) -> Line {
        Line {
            start: self,
            end: rhs,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3: Point = p1.add(p2);
    assert!(p3.x == 4 && p3.y == 6);

    let p1 = Point { x: 1, y: 4 };
    let int2 = 2;
    let p3 = p1.add(int2);
    assert_eq!(p3.x, 3);
    assert_eq!(p3.y, 6);

    let p1 = Point { x: 3, y: 5 };
    let p2 = Point { x: 4, y: 6 };
    let l: Line = p1.add(p2);
    println!(
        "l.start.x : {} \n l.start.y :{} \n l.end.x : {} \n l.end.y:{}",
        l.start.x, l.start.y, l.end.x, l.end.y
    );
}
