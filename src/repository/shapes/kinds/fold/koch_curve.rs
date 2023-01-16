use crate::{
    model::shape::{BaseChartConfig, Order, Point},
    repository::shapes::{
        generator::orders::{OrderConfig, OrderKind, OrdersGenerator, OrdersGeneratorImpl},
        kinds::traits::ShapesGenerator,
    },
};

use super::{
    generator::FoldCurveGenerator,
    models::{Fold, FoldRule},
};

pub struct KochCurve;

impl KochCurve {
    fn get_fold_rule() -> Vec<FoldRule> {
        vec![FoldRule {
            folds: vec![
                Fold {
                    length: 1.0 / 3.0,
                    radian: 0.0,
                    from_end: None,
                    from_start: None,
                },
                Fold {
                    length: 1.0 / 3.0,
                    radian: 60.0_f64.to_radians(),
                    from_end: None,
                    from_start: None,
                },
                Fold {
                    length: 1.0 / 3.0,
                    radian: -60.0_f64.to_radians(),
                    from_end: None,
                    from_start: None,
                },
            ],
        }]
    }
}

impl ShapesGenerator for KochCurve {
    fn generate_base_points(self: &Self, config: BaseChartConfig) -> Vec<Point> {
        let rules = Self::get_fold_rule();
        FoldCurveGenerator::generate_points(config, rules)
    }

    fn generate_orders(self: &Self, complexity: u64) -> Vec<Order> {
        let point_count = 2_u64.pow(2 * (complexity as u32 - 1)) + 1;
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
    fn test_koch_curve_generation() {
        let chart = KochCurve {};
        let actual = chart.generate_base_points(BaseChartConfig {
            kind: ChartKind::KochCurve,
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
            Point { x: -1.0, y: 0.0 },
            Point {
                x: -0.7777777777777778,
                y: 0.0,
            },
            Point {
                x: -0.6666666666666666,
                y: 0.19245008972987523,
            },
            Point {
                x: -0.5555555555555555,
                y: 0.0,
            },
            Point {
                x: -0.33333333333333337,
                y: 0.0,
            },
            Point {
                x: -0.22222222222222227,
                y: 0.19245008972987523,
            },
            Point {
                x: -0.3333333333333333,
                y: 0.3849001794597505,
            },
            Point {
                x: -0.1111111111111111,
                y: 0.3849001794597505,
            },
            Point {
                x: 0.0,
                y: 0.5773502691896257,
            },
            Point {
                x: 0.11111111111111112,
                y: 0.3849001794597505,
            },
            Point {
                x: 0.3333333333333333,
                y: 0.3849001794597505,
            },
            Point {
                x: 0.22222222222222227,
                y: 0.1924500897298752,
            },
            Point {
                x: 0.33333333333333337,
                y: 0.0,
            },
            Point {
                x: 0.5555555555555556,
                y: 0.0,
            },
            Point {
                x: 0.6666666666666667,
                y: 0.19245008972987523,
            },
            Point {
                x: 0.7777777777777779,
                y: 0.0,
            },
            Point { x: 1.0, y: 0.0 },
        ];
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_orders_generation() {
        let chart = KochCurve {};
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
        ];
        assert_eq!(expect, actual);
    }
}
