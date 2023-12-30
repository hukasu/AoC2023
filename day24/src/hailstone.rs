pub trait Vec3 {
    fn is_normal(&self) -> bool;
    fn grided(&self) -> Self;

    fn cross(&self, other: &Self) -> Self;
    fn dot(&self, other: &Self) -> f64;
    fn length(&self) -> f64;

    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn div(&self, other: &Self) -> Self;

    fn mul_scalar(&self, scalar: f64) -> Self;
    fn div_scalar(&self, scalar: f64) -> Self;
}

impl Vec3 for (f64, f64, f64) {
    fn is_normal(&self) -> bool {
        self.0.is_normal() && self.1.is_normal() && self.2.is_normal()
    }

    fn grided(&self) -> Self {
        (
            self.0 - self.0.fract(),
            self.1 - self.1.fract(),
            self.2 - self.2.fract(),
        )
    }

    fn cross(&self, other: &Self) -> Self {
        (
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    fn dot(&self, other: &Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    fn length(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }

    fn add(&self, other: &Self) -> Self {
        (self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }

    fn sub(&self, other: &Self) -> Self {
        (self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }

    fn mul(&self, other: &Self) -> Self {
        (self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }

    fn div(&self, other: &Self) -> Self {
        (self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }

    fn mul_scalar(&self, scalar: f64) -> Self {
        (self.0 * scalar, self.1 * scalar, self.2 * scalar)
    }

    fn div_scalar(&self, scalar: f64) -> Self {
        (self.0 / scalar, self.1 / scalar, self.2 / scalar)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Hailstone {
    pub position: (f64, f64, f64),
    pub velocity: (f64, f64, f64),
}

impl Hailstone {
    pub fn new(position: (f64, f64, f64), velocity: (f64, f64, f64)) -> Self {
        Self { position, velocity }
    }

    /// Creates a new [Hailstone] from the line between points in the path of two other [Hailstone]s.
    ///
    /// The new [Hailstone] will have it's start position on the path of the [Hailstone] with the smaller interpolation
    /// value.
    pub fn new_between_interpolated_hailstones(
        lhs: &Self,
        lhs_interpolation: f64,
        rhs: &Self,
        rhs_interpolation: f64,
    ) -> Self {
        if lhs_interpolation < rhs_interpolation {
            let moved = lhs.interpolate(lhs_interpolation);
            let moved2 = rhs.interpolate(rhs_interpolation);
            Hailstone::new(
                moved,
                moved2
                    .sub(&moved)
                    .div_scalar(rhs_interpolation - lhs_interpolation),
            )
        } else {
            let moved = rhs.interpolate(rhs_interpolation);
            let moved2 = lhs.interpolate(lhs_interpolation);
            Hailstone::new(
                moved,
                moved2
                    .sub(&moved)
                    .div_scalar(lhs_interpolation - rhs_interpolation),
            )
        }
    }

    fn a(&self) -> f64 {
        self.velocity.1
    }

    fn b(&self) -> f64 {
        -self.velocity.0
    }

    fn c(&self) -> f64 {
        self.velocity.1 * self.position.0 - self.velocity.0 * self.position.1
    }

    pub fn intersection_2d(&self, other: &Self) -> ((f64, f64), (f64, f64)) {
        let px = ((other.b() * self.c()) - (self.b() * other.c()))
            / (self.a() * other.b() - self.b() * other.a());
        let py = ((self.a() * other.c()) - (other.a() * self.c()))
            / (self.a() * other.b() - self.b() * other.a());
        let t = (px - self.position.0) / self.velocity.0;
        let u = (px - other.position.0) / other.velocity.0;

        ((px, py), (t, u))
    }

    pub fn skew_line_distance(&self, other: &Self) -> f64 {
        let throw_path_skew = self.skew_line_point(other);
        let stone_path_skew = other.skew_line_point(self);

        let dist = throw_path_skew.sub(&stone_path_skew).length();
        if dist.is_finite() {
            dist
        } else {
            f64::MAX
        }
    }

    pub fn skew_line_point(&self, other: &Self) -> (f64, f64, f64) {
        let cross = self.velocity.cross(&other.velocity);
        let points_dist = other.position.sub(&self.position);
        let n2 = other.velocity.cross(&cross);

        self.interpolate(points_dist.dot(&n2) / self.velocity.dot(&n2))
    }

    pub fn interpolate(&self, time: f64) -> (f64, f64, f64) {
        self.position.add(&self.velocity.mul_scalar(time))
    }

    pub fn move_hailstone(&self, time: f64) -> Self {
        Hailstone {
            position: self.interpolate(time),
            velocity: self.velocity,
        }
    }

    pub fn in_bounds(&self, bounds: (f64, f64)) -> bool {
        self.position.0 >= bounds.0
            && self.position.0 <= bounds.1
            && self.position.1 >= bounds.0
            && self.position.1 <= bounds.1
            && self.position.2 >= bounds.0
            && self.position.2 <= bounds.1
    }

    fn time_of_component_in_bounds(position: f64, velocity: f64, bounds: (f64, f64)) -> (f64, f64) {
        if position >= bounds.0 && position <= bounds.1 {
            if velocity.is_sign_positive() {
                (0., (bounds.1 - position) / velocity)
            } else {
                (0., (bounds.0 - position) / velocity)
            }
        } else if position <= bounds.0 && velocity.is_sign_positive() {
            (
                (bounds.0 - position) / velocity,
                (bounds.1 - position) / velocity,
            )
        } else if position >= bounds.0 && velocity.is_sign_negative() {
            (
                (bounds.1 - position) / velocity,
                (bounds.0 - position) / velocity,
            )
        } else {
            panic!("Some other case.");
        }
    }

    pub fn time_in_bounds(&self, bounds: (f64, f64)) -> (f64, f64) {
        let time_for_x_to_be_in_bounds =
            Self::time_of_component_in_bounds(self.position.0, self.velocity.0, bounds);
        let time_for_y_to_be_in_bounds =
            Self::time_of_component_in_bounds(self.position.1, self.velocity.1, bounds);
        let time_for_z_to_be_in_bounds =
            Self::time_of_component_in_bounds(self.position.2, self.velocity.2, bounds);
        (
            time_for_x_to_be_in_bounds
                .0
                .max(time_for_y_to_be_in_bounds.0)
                .max(time_for_z_to_be_in_bounds.0)
                .ceil(),
            time_for_x_to_be_in_bounds
                .1
                .min(time_for_y_to_be_in_bounds.1)
                .min(time_for_z_to_be_in_bounds.1)
                .floor(),
        )
    }
}

impl PartialOrd for Hailstone {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hailstone {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.position
            .partial_cmp(&other.position)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl Eq for Hailstone {}

#[cfg(test)]
mod tests {
    use crate::hailstone::Hailstone;

    #[test]
    fn move_test() {
        let origin = Hailstone::new((0., 0., 0.), (0., 0., 1.));
        assert_eq!(origin.move_hailstone(0.), origin);
        assert_eq!(
            origin.move_hailstone(1.),
            Hailstone::new((0., 0., 1.), (0., 0., 1.))
        );
        assert_eq!(
            origin.move_hailstone(-1.),
            Hailstone::new((0., 0., -1.), (0., 0., 1.))
        );

        let origin = Hailstone::new((0., 0., 0.), (0., 1., 1.));
        assert_eq!(origin.move_hailstone(0.), origin);
        assert_eq!(
            origin.move_hailstone(1.),
            Hailstone::new((0., 1., 1.), (0., 1., 1.))
        );
        assert_eq!(
            origin.move_hailstone(-1.),
            Hailstone::new((0., -1., -1.), (0., 1., 1.))
        );

        let origin = Hailstone::new((0., 0., 0.), (-1., 1., 1.));
        assert_eq!(origin.move_hailstone(0.), origin);
        assert_eq!(
            origin.move_hailstone(1.),
            Hailstone::new((-1., 1., 1.), (-1., 1., 1.))
        );
        assert_eq!(
            origin.move_hailstone(-1.),
            Hailstone::new((1., -1., -1.), (-1., 1., 1.))
        );
    }

    #[test]
    fn skew_line_point_should_not_change_because_start_changed() {
        let origin = Hailstone::new((0., 0., 0.), (0., 0., 1.));
        let target = Hailstone::new((1., 1., 0.), (0., 1., 0.));
        assert_eq!(
            origin.skew_line_point(&target),
            origin.move_hailstone(1.).skew_line_point(&target)
        );
        assert_eq!(
            origin.skew_line_point(&target),
            origin.move_hailstone(-1.).skew_line_point(&target)
        );
        assert_eq!(
            origin.skew_line_distance(&target),
            origin.move_hailstone(1.).skew_line_distance(&target)
        );
        assert_eq!(
            origin.skew_line_distance(&target),
            target.skew_line_distance(&origin)
        );
        assert_ne!(
            origin.skew_line_point(&target),
            target.skew_line_point(&origin)
        );
    }
}
