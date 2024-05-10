use crate::traits::{eps::Eps, is_equal::IsEqual, is_same::IsSame};

pub struct Point2 {
    pub x: f64,
    pub y: f64,
}

impl IsEqual for Point2 {
    fn is_equal(&self, other: &Self, eps: Option<Eps>) -> bool {
        let eps = eps.unwrap_or(Eps::default()).value;
        let diff_x = self.x - other.x;
        let diff_y = self.y - other.y;
        diff_x.abs() < eps && diff_y.abs() < eps
    }
}

impl IsSame for Point2 {
    fn is_same(&self, other: &Self, eps: Option<Eps>) -> bool {
        self.is_equal(other, eps)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        topo::point_2::Point2,
        traits::{eps::Eps, is_equal::IsEqual},
    };

    #[test]
    fn is_equal() {
        let p1 = Point2 { x: 1.0, y: 1.0 };
        let p2 = Point2 { x: 1.0, y: 1.0 };
        let result = p1.is_equal(&p2, None);
        assert_eq!(result, true);

        let p1 = Point2 { x: 1.0, y: 1.0 };
        let p2 = Point2 { x: 1.0, y: 1.1 };
        let result = p1.is_equal(&p2, None);
        assert_eq!(result, false);

        let p1 = Point2 { x: 1.0, y: 1.0 };
        let p2 = Point2 { x: 1.1, y: 1.0 };
        let result = p1.is_equal(&p2, None);
        assert_eq!(result, false);

        let p1 = Point2 { x: 1.0, y: 1.0 };
        let p2 = Point2 { x: 1.1, y: 1.1 };
        let result = p1.is_equal(&p2, Some(Eps::new(0.2)));
        assert_eq!(result, true);

        let p1 = Point2 { x: 1.0, y: 1.0 };
        let p2 = Point2 { x: 1.1, y: 1.1 };
        let result = p1.is_equal(&p2, Some(Eps::new(0.01)));
        assert_eq!(result, false);
    }
}
