use super::Element;

pub struct Plane {}

impl Plane {
    pub fn is_intersecting(a: Element, b: Element) -> bool {
        match (a, b) {
            (Element::Point(a), Element::Point(b)) => a == b,
            (Element::Point(p), Element::Line(line)) | (Element::Line(line), Element::Point(p)) => {
                p.y == line.slope * p.x + line.y_intercept
            }
            (Element::Line(l1), Element::Line(l2)) => {
                !(l1.slope == l2.slope && l1.y_intercept != l2.y_intercept)
            }
        }
    }

    pub fn distance(a: Element, b: Element) -> f64 {
        match (a, b) {
            (Element::Point(a), Element::Point(b)) => {
                ((a.x - b.x).powf(2.0) + (a.y - b.y).powf(2.0)).sqrt()
            }
            (Element::Point(p), Element::Line(line)) | (Element::Line(line), Element::Point(p)) => {
                (line.slope * p.x + line.y_intercept - p.y).abs()
                    / (1.0 + line.slope.powf(2.0)).sqrt()
            }
            (Element::Line(l1), Element::Line(l2)) => {
                if !(Plane::is_intersecting(l1.into(), l2.into())) {
                    (l1.y_intercept - l2.y_intercept).abs() / (1.0 + l1.slope.powf(2.0)).sqrt()
                } else {
                    0.0
                }
            }
        }
    }
}
