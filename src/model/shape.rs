#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn rotate_by(self: &Self, vector: &Point, radian: f64) -> Point {
        let sin = radian.sin();
        let cos = radian.cos();
        return Point {
            x: self.x + vector.x * cos - vector.y * sin,
            y: self.y + vector.x * sin + vector.y * cos,
        };
    }
}

pub struct Size {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, PartialEq)]
pub struct Order {
    pub link: (u64, u64),
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < f64::EPSILON * 1e1
            && (self.y - other.y).abs() < f64::EPSILON * 1e1
    }
}

pub struct Mutation {
    pub size: f64,
    pub angle: f64,
}

pub struct Randomizer {
    pub size_amplitude: f64,
    pub size_seed: u64,
    pub angle_amplitude: f64,
    pub angle_seed: u64,
}

pub enum ChartKind {
    BinaryTree,
    Clover,
    FoldCCurve,
    FoldDragon,
    KochCurve,
    KochTriangleInner,
    KochTriangleOuter,
    Star,
    Starmine,
    Sunrise,
    Sunset,
    TriCis,
    TriTrans,
}

pub struct BaseChartConfig {
    pub kind: ChartKind,
    pub complexity: u64,
    pub mutation: Option<Mutation>,
    pub randomizer: Option<Randomizer>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_origin_rotation() {
        let angles = [
            30.0_f64.to_radians(),
            45.0_f64.to_radians(),
            60.0_f64.to_radians(),
            90.0_f64.to_radians(),
            -30.0_f64.to_radians(),
            -45.0_f64.to_radians(),
            -60.0_f64.to_radians(),
            -90.0_f64.to_radians(),
        ];
        let expect = angles.map(|angle| Point {
            x: angle.cos(),
            y: angle.sin(),
        });
        let actual = angles
            .map(|angle| Point { x: 0.0, y: 0.0 }.rotate_by(&Point { x: 1.0, y: 0.0 }, angle));

        assert_eq!(expect, actual);
    }

    #[test]
    fn test_point_offset_rotation() {
        let angles = [
            30.0_f64.to_radians(),
            45.0_f64.to_radians(),
            60.0_f64.to_radians(),
            90.0_f64.to_radians(),
            -30.0_f64.to_radians(),
            -45.0_f64.to_radians(),
            -60.0_f64.to_radians(),
            -90.0_f64.to_radians(),
        ];
        let origin = Point { x: 1.5, y: 2.5 };
        let expect = angles.map(|angle| Point {
            x: origin.x + angle.cos(),
            y: origin.y + angle.sin(),
        });
        let actual = angles.map(|angle| origin.rotate_by(&Point { x: 1.0, y: 0.0 }, angle));

        assert_eq!(expect, actual);
    }
}
