pub mod constants;
pub mod datatypes;
pub mod elements;

#[cfg(test)]
mod tests {
    use crate::{
        constants::pi,
        datatypes::radian::Radian,
        elements::{line::Line, plane::Plane, point::Point},
    };

    #[test]
    fn point() {
        let p = Point::new(3.0, 4.0);

        assert_eq!(p.angle(), Radian::new(0.9272952180016122));
    }

    #[test]
    fn line() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(3.0, 4.0);

        let line = Line::from_points(a, b);

        assert!(Plane::is_intersecting(line.into(), b.into()));

        let l1 = Line::new(1.0, 5.0);
        let l2 = Line::new(1.0, 10.0);

        assert_eq!(Plane::distance(l1.into(), l2.into()), 3.5355339059327373);
    }

    #[test]
    fn intersection() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(3.0, 4.0);
        let c = Point::new(6.0, 8.0);

        let line = Line::from_points(a, b);

        assert!(Plane::is_intersecting(c.into(), line.into()));
    }

    #[test]
    fn rotation() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(3.0, 4.0);

        let line = Line::from_points(a, b);

        b.rotate(pi() / 2);
        line.rotate(pi() / 2, Point::origin());
    }
}
