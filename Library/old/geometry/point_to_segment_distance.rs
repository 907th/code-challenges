#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn dist(self, other: Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

fn point_to_segment_distance(p: Point, s: (Point, Point)) -> Point {
    let a = p.x - s.0.x;
    let b = p.y - s.0.y;
    let c = s.1.x - s.0.x;
    let d = s.1.y - s.0.y;
    let dot = a * c + b * d;
    let len_sq = c * c + d * d;
    let shift = if len_sq != 0.0 { dot / len_sq } else { -1.0 };
    if shift < 0.0 { s.0 } else if shift > 1.0 { s.1 } else { Point::new(s.0.x + shift * c, s.0.y + shift * d) }
}
