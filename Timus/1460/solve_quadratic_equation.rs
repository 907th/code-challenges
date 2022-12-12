// Algorithm for solving quadration equation

const EPS: f64 = 1e-12;

fn eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPS
}

fn zero(a: f64) -> bool {
    eq(a, 0.0)
}

fn lt(a: f64, b: f64) -> bool {
    !eq(a, b) && a < b
}

fn neg(a: f64) -> bool {
    lt(a, 0.0)
}

pub fn solve_quadratic_equation(a: f64, b: f64, c: f64) -> Vec<f64> {
    if zero(a) {
        if zero(b) {
            if zero(c) {
                panic!("This equation has an infinite number of solutions!");
            } else {
                vec![]
            }
        } else {
            vec![-c / b]
        }
    } else {
        let d = b.powi(2) - 4.0 * a * c;
        if zero(d) {
            vec![-b / (2.0 * a)]
        } else if neg(d) {
            vec![]
        } else {
            vec![
                (-b - d.sqrt()) / (2.0 * a),
                (-b + d.sqrt()) / (2.0 * a)
            ]
        }
    }
}

#[cfg(test)]
mod test {
    use super::solve_quadratic_equation;

    #[test]
    fn test0() {
        let z = solve_quadratic_equation(1.0, 0.0, 0.0);
        println!("{:?}", z);
        assert!(z.len() == 1);
        assert!(z[0] == 0.0);
    }

    #[test]
    fn test1() {
        let z = solve_quadratic_equation(1.0, -1.0, 0.0);
        println!("{:?}", z);
        assert!(z.len() == 2);
        assert!(z[0] == 0.0);
        assert!(z[1] == 1.0);
    }

    #[test]
    fn test2() {
        let z = solve_quadratic_equation(2.0, -5.0, -3.0);
        println!("{:?}", z);
        assert!(z.len() == 2);
        assert!(z[0] == -0.5);
        assert!(z[1] == 3.0);
    }

    #[test]
    fn test3() {
        let z = solve_quadratic_equation(0.0, 3.0, -6.0);
        println!("{:?}", z);
        assert!(z.len() == 1);
        assert!(z[0] == 2.0);
    }

    #[test]
    fn test4() {
        let z = solve_quadratic_equation(0.0, 0.0, 5.0);
        println!("{:?}", z);
        assert!(z.len() == 0);
    }

    #[test]
    #[should_panic(expected = "This equation has an infinite number of solutions!")]
    fn test5() {
        solve_quadratic_equation(0.0, 0.0, 0.0);
    }
}
