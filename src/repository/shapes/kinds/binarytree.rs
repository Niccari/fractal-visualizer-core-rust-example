use std::cmp::{self, Ordering};

use super::traits::ShapesGenerator;
use crate::{
    model::shape::{BaseChartConfig, Mutation, Order, Point},
    repository::shapes::generator::randomizer::RandomGenerator,
};

struct IndexedPoint {
    index: u64,
    point: Point,
}

pub struct BinaryTree;

impl BinaryTree {
    fn point_count(complexity: u64) -> u64 {
        let limited_compelixty: u32 = cmp::max(cmp::min(complexity as u32, 10), 2);
        2_u64.pow(limited_compelixty + 1)
    }

    fn divide_base_points(
        depth_threshold: u64,
        mutation: &Mutation,
        start: &IndexedPoint,
        end: &IndexedPoint,
        depth: u64,
        parent_length: f64,
        parent_angle: f64,
        length_randomizer: &mut RandomGenerator,
        angle_randomizer: &mut RandomGenerator,
    ) -> Vec<IndexedPoint> {
        let length_random = length_randomizer.generate();
        let angle_random = angle_randomizer.generate();
        let length = parent_length * (length_random + mutation.size);
        let angle = parent_angle * (angle_random + mutation.angle);
        if depth >= depth_threshold {
            return vec![];
        }
        let vector = Point {
            x: length * (end.point.x - start.point.x),
            y: length * (end.point.y - start.point.y),
        };

        let left_depth = 2 * depth;
        let right_depth = 2 * depth + 1;
        let left_point = IndexedPoint {
            index: left_depth,
            point: end.point.rotate_by(&vector, angle),
        };
        let right_point = IndexedPoint {
            index: right_depth,
            point: end.point.rotate_by(&vector, -angle),
        };

        let mut middle: Vec<IndexedPoint> = vec![];
        middle.append(&mut BinaryTree::divide_base_points(
            depth_threshold,
            mutation,
            &end,
            &left_point,
            left_depth,
            length,
            angle,
            length_randomizer,
            angle_randomizer,
        ));
        middle.append(&mut BinaryTree::divide_base_points(
            depth_threshold,
            mutation,
            &end,
            &right_point,
            right_depth,
            length,
            angle,
            length_randomizer,
            angle_randomizer,
        ));
        let mut result: Vec<IndexedPoint> = vec![left_point, right_point];
        result.append(&mut middle);
        result
    }
}

impl ShapesGenerator for BinaryTree {
    fn generate_base_points(self: &Self, config: BaseChartConfig) -> Vec<Point> {
        let randomizer = config.randomizer.unwrap();
        let mutation = config.mutation.unwrap();

        let mut points: Vec<IndexedPoint> = vec![
            IndexedPoint {
                index: 0,
                point: Point { x: 0.0, y: -1.0 },
            },
            IndexedPoint {
                index: 1,
                point: Point { x: 0.0, y: 0.0 },
            },
        ];
        let mut size_randomizer =
            RandomGenerator::new(randomizer.size_seed, randomizer.size_amplitude);
        let mut angle_randomizer =
            RandomGenerator::new(randomizer.angle_seed, randomizer.angle_amplitude);

        let depth_threshold = BinaryTree::point_count(config.complexity) / 2;
        points.append(&mut BinaryTree::divide_base_points(
            depth_threshold,
            &mutation,
            &points[0],
            &points[1],
            1,
            0.85,
            45.0_f64.to_radians(),
            &mut size_randomizer,
            &mut angle_randomizer,
        ));
        points.sort_by(|a, b| {
            if a.index > b.index {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
        points
            .iter()
            .map(|p| Point {
                x: p.point.x,
                y: p.point.y,
            })
            .collect()
    }

    fn generate_orders(self: &Self, complexity: u64) -> Vec<Order> {
        let max_depth = BinaryTree::point_count(complexity) - 1;
        let orders = (0..max_depth)
            .map(|i| Order {
                link: ((i + 1) / 2, i + 1),
            })
            .collect();
        orders
    }
}

#[cfg(test)]
mod tests {
    use crate::model::shape::{ChartKind, Randomizer};

    use super::*;

    #[test]
    fn test_points_generation() {
        let chart = BinaryTree {};
        let actual = chart.generate_base_points(BaseChartConfig {
            kind: ChartKind::BinaryTree,
            complexity: 3,
            mutation: Some(Mutation {
                size: 1.0,
                angle: 1.0,
            }),
            randomizer: Some(Randomizer {
                size_amplitude: 0.0,
                size_seed: 0,
                angle_amplitude: 0.0,
                angle_seed: 0,
            }),
        });
        let expect: Vec<Point> = vec![
            Point { x: 0.0, y: -1.0 },
            Point { x: 0.0, y: 0.0 },
            Point {
                x: -0.6010407640085653,
                y: 0.6010407640085654,
            },
            Point {
                x: 0.6010407640085653,
                y: 0.6010407640085654,
            },
            Point {
                x: -1.3235407640085652,
                y: 0.6010407640085655,
            },
            Point {
                x: -0.6010407640085653,
                y: 1.3235407640085655,
            },
            Point {
                x: 0.6010407640085653,
                y: 1.3235407640085655,
            },
            Point {
                x: 1.3235407640085652,
                y: 0.6010407640085655,
            },
            Point {
                x: -1.7577927160047537,
                y: 0.16678881201237722,
            },
            Point {
                x: -1.7577927160047537,
                y: 1.0352927160047538,
            },
            Point {
                x: -1.0352927160047538,
                y: 1.757792716004754,
            },
            Point {
                x: -0.16678881201237683,
                y: 1.757792716004754,
            },
            Point {
                x: 0.16678881201237683,
                y: 1.757792716004754,
            },
            Point {
                x: 1.0352927160047538,
                y: 1.757792716004754,
            },
            Point {
                x: 1.7577927160047537,
                y: 1.0352927160047538,
            },
            Point {
                x: 1.7577927160047537,
                y: 0.16678881201237722,
            },
        ];
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_orders_generation() {
        let chart = BinaryTree {};
        let actual = chart.generate_orders(3);
        let expect: Vec<Order> = vec![
            Order { link: (0, 1) },
            Order { link: (1, 2) },
            Order { link: (1, 3) },
            Order { link: (2, 4) },
            Order { link: (2, 5) },
            Order { link: (3, 6) },
            Order { link: (3, 7) },
            Order { link: (4, 8) },
            Order { link: (4, 9) },
            Order { link: (5, 10) },
            Order { link: (5, 11) },
            Order { link: (6, 12) },
            Order { link: (6, 13) },
            Order { link: (7, 14) },
            Order { link: (7, 15) },
        ];
        assert_eq!(expect, actual);
    }
}
