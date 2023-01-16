use crate::{
    model::shape::{BaseChartConfig, ChartKind, Order, Point},
    repository::shapes::{
        generator::orders::{OrderConfig, OrderKind, OrdersGenerator, OrdersGeneratorImpl},
        kinds::traits::ShapesGenerator,
    },
};

use super::koch_curve::KochCurve;

pub struct KochTriangle;

impl ShapesGenerator for KochTriangle {
    fn generate_base_points(self: &Self, config: BaseChartConfig) -> Vec<Point> {
        let is_inner = match config.kind {
            ChartKind::KochTriangleInner => true,
            ChartKind::KochTriangleOuter => false,
            _ => {
                panic!("Unsupported chart kind!");
            }
        };
        let chart = KochCurve {};
        let mut base_points = chart.generate_base_points(config);
        let radian = 120.0_f64.to_radians();

        if is_inner {
            base_points = base_points
                .iter()
                .map(|p| Point { x: p.x, y: -p.y })
                .collect();
        }
        base_points = base_points
            .iter()
            .map(|p| Point {
                x: p.x,
                y: p.y + 1.0 / 3.0_f64.sqrt(),
            })
            .collect();

        let mut points: Vec<Point> = vec![];
        let origin = Point { x: 0.0, y: 0.0 };
        let mut base_points_120 = base_points
            .iter()
            .map(|p| origin.rotate_by(p, radian))
            .collect();
        let mut base_points_240 = base_points
            .iter()
            .map(|p| origin.rotate_by(p, 2.0 * radian))
            .collect();
        points.append(&mut base_points);
        points.append(&mut base_points_120);
        points.append(&mut base_points_240);
        points
    }

    fn generate_orders(self: &Self, complexity: u64) -> Vec<Order> {
        let point_count = 3 * (2_u64.pow(2 * (complexity as u32 - 1)) + 1);
        OrdersGeneratorImpl::generate(OrderConfig {
            kind: OrderKind::Linear,
            point_count,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::model::shape::{ChartKind, Mutation, Randomizer};

    use super::*;

    #[test]
    fn test_koch_triangle_generation() {
        let chart = KochTriangle {};
        let actual = chart.generate_base_points(BaseChartConfig {
            kind: ChartKind::KochTriangleInner,
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
            Point {
                x: -1.0,
                y: 0.5773502691896258,
            },
            Point {
                x: -0.7777777777777778,
                y: 0.5773502691896258,
            },
            Point {
                x: -0.6666666666666666,
                y: 0.38490017945975064,
            },
            Point {
                x: -0.5555555555555555,
                y: 0.5773502691896258,
            },
            Point {
                x: -0.33333333333333337,
                y: 0.5773502691896258,
            },
            Point {
                x: -0.22222222222222227,
                y: 0.38490017945975064,
            },
            Point {
                x: -0.3333333333333333,
                y: 0.19245008972987532,
            },
            Point {
                x: -0.1111111111111111,
                y: 0.19245008972987532,
            },
            Point { x: 0.0, y: 0.0 },
            Point {
                x: 0.11111111111111112,
                y: 0.19245008972987532,
            },
            Point {
                x: 0.3333333333333333,
                y: 0.19245008972987532,
            },
            Point {
                x: 0.22222222222222227,
                y: 0.38490017945975064,
            },
            Point {
                x: 0.33333333333333337,
                y: 0.5773502691896258,
            },
            Point {
                x: 0.5555555555555556,
                y: 0.5773502691896258,
            },
            Point {
                x: 0.6666666666666667,
                y: 0.38490017945975064,
            },
            Point {
                x: 0.7777777777777779,
                y: 0.5773502691896258,
            },
            Point {
                x: 1.0,
                y: 0.5773502691896258,
            },
            Point {
                x: 0.0,
                y: -1.1547005383792515,
            },
            Point {
                x: -0.11111111111111133,
                y: -0.9622504486493764,
            },
            Point {
                x: 0.0,
                y: -0.769800358919501,
            },
            Point {
                x: -0.2222222222222225,
                y: -0.7698003589195008,
            },
            Point {
                x: -0.3333333333333335,
                y: -0.5773502691896257,
            },
            Point {
                x: -0.22222222222222238,
                y: -0.3849001794597506,
            },
            Point {
                x: 0.0,
                y: -0.3849001794597505,
            },
            Point {
                x: -0.11111111111111122,
                y: -0.19245008972987526,
            },
            Point { x: 0.0, y: 0.0 },
            Point {
                x: -0.22222222222222227,
                y: 0.0,
            },
            Point {
                x: -0.33333333333333337,
                y: 0.19245008972987523,
            },
            Point {
                x: -0.4444444444444446,
                y: 0.0,
            },
            Point {
                x: -0.6666666666666667,
                y: 0.0,
            },
            Point {
                x: -0.7777777777777778,
                y: 0.19245008972987537,
            },
            Point {
                x: -0.6666666666666667,
                y: 0.3849001794597506,
            },
            Point {
                x: -0.888888888888889,
                y: 0.3849001794597507,
            },
            Point {
                x: -1.0,
                y: 0.577350269189626,
            },
            Point {
                x: 1.0000000000000004,
                y: 0.5773502691896252,
            },
            Point {
                x: 0.8888888888888891,
                y: 0.38490017945974997,
            },
            Point {
                x: 0.666666666666667,
                y: 0.38490017945975,
            },
            Point {
                x: 0.7777777777777779,
                y: 0.1924500897298747,
            },
            Point {
                x: 0.6666666666666667,
                y: 0.0,
            },
            Point {
                x: 0.44444444444444453,
                y: 0.0,
            },
            Point {
                x: 0.3333333333333335,
                y: 0.192450089729875,
            },
            Point {
                x: 0.22222222222222227,
                y: 0.0,
            },
            Point { x: 0.0, y: 0.0 },
            Point {
                x: 0.11111111111111105,
                y: -0.19245008972987535,
            },
            Point {
                x: 0.0,
                y: -0.3849001794597505,
            },
            Point {
                x: 0.2222222222222221,
                y: -0.38490017945975075,
            },
            Point {
                x: 0.33333333333333304,
                y: -0.577350269189626,
            },
            Point {
                x: 0.22222222222222188,
                y: -0.7698003589195013,
            },
            Point {
                x: 0.0,
                y: -0.769800358919501,
            },
            Point {
                x: 0.1111111111111106,
                y: -0.9622504486493765,
            },
            Point {
                x: 0.0,
                y: -1.1547005383792515,
            },
        ];
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_orders_generation() {
        let chart = KochTriangle {};
        let actual = chart.generate_orders(3);
        let expect: Vec<Order> = vec![
            Order { link: (0, 1) },
            Order { link: (1, 2) },
            Order { link: (2, 3) },
            Order { link: (3, 4) },
            Order { link: (4, 5) },
            Order { link: (5, 6) },
            Order { link: (6, 7) },
            Order { link: (7, 8) },
            Order { link: (8, 9) },
            Order { link: (9, 10) },
            Order { link: (10, 11) },
            Order { link: (11, 12) },
            Order { link: (12, 13) },
            Order { link: (13, 14) },
            Order { link: (14, 15) },
            Order { link: (15, 16) },
            Order { link: (16, 17) },
            Order { link: (17, 18) },
            Order { link: (18, 19) },
            Order { link: (19, 20) },
            Order { link: (20, 21) },
            Order { link: (21, 22) },
            Order { link: (22, 23) },
            Order { link: (23, 24) },
            Order { link: (24, 25) },
            Order { link: (25, 26) },
            Order { link: (26, 27) },
            Order { link: (27, 28) },
            Order { link: (28, 29) },
            Order { link: (29, 30) },
            Order { link: (30, 31) },
            Order { link: (31, 32) },
            Order { link: (32, 33) },
            Order { link: (33, 34) },
            Order { link: (34, 35) },
            Order { link: (35, 36) },
            Order { link: (36, 37) },
            Order { link: (37, 38) },
            Order { link: (38, 39) },
            Order { link: (39, 40) },
            Order { link: (40, 41) },
            Order { link: (41, 42) },
            Order { link: (42, 43) },
            Order { link: (43, 44) },
            Order { link: (44, 45) },
            Order { link: (45, 46) },
            Order { link: (46, 47) },
            Order { link: (47, 48) },
            Order { link: (48, 49) },
            Order { link: (49, 50) },
        ];
        assert_eq!(expect, actual);
    }
}
