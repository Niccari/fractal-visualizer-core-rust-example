use std::f64::consts::PI;

use crate::model::shape::Point;
use rand::Rng;

pub enum PointKind {
    Circle,
    Random,
}

pub struct PointsConfig {
    pub kind: PointKind,
    pub length: u64,
}

pub trait PointsGenerator {
    fn generate(config: PointsConfig) -> Vec<Point>
    where
        Self: Sized;
}

pub struct PointsGeneratorImpl;

impl PointsGenerator for PointsGeneratorImpl {
    fn generate(config: PointsConfig) -> Vec<Point> {
        let PointsConfig { kind, length } = config;
        let mut rng = rand::thread_rng();

        match kind {
            PointKind::Circle => {
                let radians = (0..length).map(|i| (2.0 * PI * i as f64) / length as f64);
                radians
                    .map(|radian| Point {
                        x: radian.sin(),
                        y: radian.cos(),
                    })
                    .collect()
            }
            PointKind::Random => (0..length)
                .map(|_| Point {
                    x: rng.gen::<f64>() - 0.5,
                    y: rng.gen::<f64>() - 0.5,
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_generation() {
        let expect = vec![
            Point { x: 0.0, y: 1.0 },
            Point {
                x: 0.5,
                y: 0.8660254037844387,
            },
            Point {
                x: 0.8660254037844387,
                y: 0.5,
            },
            Point { x: 1.0, y: 0.0 },
            Point {
                x: 0.8660254037844387,
                y: -0.5,
            },
            Point {
                x: 0.5,
                y: -0.8660254037844387,
            },
            Point { x: 0.0, y: -1.0 },
            Point {
                x: -0.5,
                y: -0.8660254037844388,
            },
            Point {
                x: -0.8660254037844384,
                y: -0.5,
            },
            Point { x: -1.0, y: -0.0 },
            Point {
                x: -0.8660254037844386,
                y: 0.5,
            },
            Point {
                x: -0.5,
                y: 0.8660254037844384,
            },
        ];
        let actual = PointsGeneratorImpl::generate(PointsConfig {
            kind: PointKind::Circle,
            length: 12,
        });

        assert_eq!(expect, actual);
    }
}
