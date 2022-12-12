// Algorithm for finding intersection points of line (defined by two point) and circle (defined by center point and radius)

mod solve_quadratic_equation;

use solve_quadratic_equation::solve_quadratic_equation;

const EPS: f64 = 1e-12;

fn eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPS
}

fn zero(a: f64) -> bool {
    eq(a, 0.0)
}

#[derive(Copy, Clone, Debug)]
struct P {
    x: f64,
    y: f64,
}

impl P {
    fn new(x: f64, y: f64) -> P {
        P { x, y }
    }

    fn zero() -> P {
        P::new(0.0, 0.0)
    }

    fn eq(self, other: P) -> bool {
        eq(self.x, other.x) && eq(self.y, other.y)
    }
}

fn intersect_circle_and_line(p1: P, p2: P, o: P, r: f64) -> Vec<P> {
    assert!(!p1.eq(p2));
    let (a, b, c) = ((p2.y - p1.y), (p1.x - p2.x), (p1.y * p2.x - p1.x * p2.y));
    if zero(a) {
        let z = solve_quadratic_equation(
            1.0,
            -2.0 * o.x,
            o.x.powi(2) - r.powi(2) + (c / b + o.y).powi(2)
        );
        z.into_iter().map(|x| P::new(x, -c / b)).collect()
    } else {
        let t = b / a;
        let k = c / a + o.x;
        let z = solve_quadratic_equation(
            t.powi(2) + 1.0,
            2.0 * (t * k - o.y),
            k.powi(2) + o.y.powi(2) - r.powi(2)
        );
        z.into_iter().map(|y| P::new((-b / a) * y - (c / a), y)).collect()
    }
}

#[cfg(test)]
mod test {
    use super::{P, intersect_circle_and_line};

    #[test]
    fn test0() {
        let (p1, p2) = (P::zero(), P::new(1.0, 1.0));
        let (o, r) = (P::zero(), 1.0);
        let z = intersect_circle_and_line(p1, p2, o, r);
        println!("{:?}", z);
        assert!(z.len() == 2);
        let q = 2f64.sqrt() / 2.0;
        assert!(z[0].eq(P::new(-q, -q)));
        assert!(z[1].eq(P::new(q, q)));
    }

    #[test]
    fn test1() {
        let (p1, p2) = (P::new(0.0, 1.0), P::new(1.0, 1.0));
        let (o, r) = (P::zero(), 1.0);
        let z = intersect_circle_and_line(p1, p2, o, r);
        println!("{:?}", z);
        assert!(z.len() == 1);
        assert!(z[0].eq(P::new(0.0, 1.0)));
    }

    #[test]
    fn test2() {
        let (p1, p2) = (P::new(1.0, 0.0), P::new(1.0, 1.0));
        let (o, r) = (P::zero(), 1.0);
        let z = intersect_circle_and_line(p1, p2, o, r);
        println!("{:?}", z);
        assert!(z.len() == 1);
        assert!(z[0].eq(P::new(1.0, 0.0)));
    }

    #[test]
    fn test3() {
        let (p1, p2) = (P::new(2.0, 0.0), P::new(0.0, 2.0));
        let (o, r) = (P::zero(), 1.0);
        let z = intersect_circle_and_line(p1, p2, o, r);
        println!("{:?}", z);
        assert!(z.len() == 0);
    }

    #[test]
    fn test4() {
        let (p1, p2) = (P::new(-1.0, 2.0), P::new(2.0, -1.0));
        let (o, r) = (P::zero(), 1.0);
        let z = intersect_circle_and_line(p1, p2, o, r);
        println!("{:?}", z);
        assert!(z.len() == 2);
        assert!(z[0].eq(P::new(1.0, 0.0)));
        assert!(z[1].eq(P::new(0.0, 1.0)));
    }

    #[test]
    fn test5() {
        let (p1, p2) = (P::new(0.5, -2.0), P::new(0.5, 3.0));
        let (o, r) = (P::zero(), 1.0);
        let z = intersect_circle_and_line(p1, p2, o, r);
        println!("{:?}", z);
        assert!(z.len() == 2);
        assert!(z[0].eq(P::new(0.5, -3f64.sqrt() / 2.0)));
        assert!(z[1].eq(P::new(0.5, 3f64.sqrt() / 2.0)));
    }
}
