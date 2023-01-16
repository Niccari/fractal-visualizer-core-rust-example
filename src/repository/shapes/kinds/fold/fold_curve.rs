use crate::{
    model::shape::{BaseChartConfig, ChartKind, Order, Point},
    repository::shapes::{
        generator::orders::{OrderConfig, OrderKind, OrdersGenerator, OrdersGeneratorImpl},
        kinds::traits::ShapesGenerator,
    },
};

use super::{
    generator::FoldCurveGenerator,
    models::{Fold, FoldRule},
};

pub struct FoldCurve;

impl FoldCurve {
    fn select_fold_rules(kind: &ChartKind) -> Vec<FoldRule> {
        let radian = 45.0_f64.to_radians();
        let left_fold = Fold {
            length: 1.0 / 2.0_f64.sqrt(),
            radian,
            from_start: None,
            from_end: None,
        };
        let right_fold = Fold {
            length: 1.0 / 2.0_f64.sqrt(),
            radian: -radian,
            from_start: None,
            from_end: None,
        };
        match kind {
            ChartKind::FoldCCurve => {
                vec![FoldRule {
                    folds: vec![right_fold],
                }]
            }
            ChartKind::FoldDragon => {
                vec![
                    FoldRule {
                        folds: vec![left_fold],
                    },
                    FoldRule {
                        folds: vec![right_fold],
                    },
                ]
            }
            _ => {
                panic!("Unsupported curve type!")
            }
        }
    }
}

impl ShapesGenerator for FoldCurve {
    fn generate_base_points(self: &Self, config: BaseChartConfig) -> Vec<Point> {
        let rules = Self::select_fold_rules(&config.kind);
        FoldCurveGenerator::generate_points(config, rules)
    }

    fn generate_orders(self: &Self, complexity: u64) -> Vec<Order> {
        let point_count = 2_u64.pow(complexity as u32 - 1) + 1;
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
    fn test_dragon_curve_generation() {
        let chart = FoldCurve {};
        let actual = chart.generate_base_points(BaseChartConfig {
            kind: ChartKind::FoldDragon,
            complexity: 6,
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
            Point { x: -1.25, y: -0.25 },
            Point { x: -1.5, y: 0.0 },
            Point { x: -1.25, y: 0.25 },
            Point { x: -1.5, y: 0.5 },
            Point { x: -1.25, y: 0.75 },
            Point { x: -1.0, y: 0.5 },
            Point { x: -0.75, y: 0.75 },
            Point { x: -1.0, y: 1.0 },
            Point { x: -0.75, y: 1.25 },
            Point { x: -0.5, y: 1.0 },
            Point { x: -0.75, y: 0.75 },
            Point { x: -0.5, y: 0.5 },
            Point { x: -0.25, y: 0.75 },
            Point { x: 0.0, y: 0.5 },
            Point { x: 0.25, y: 0.75 },
            Point { x: 0.0, y: 1.0 },
            Point { x: 0.25, y: 1.25 },
            Point { x: 0.5, y: 1.0 },
            Point { x: 0.25, y: 0.75 },
            Point { x: 0.5, y: 0.5 },
            Point { x: 0.25, y: 0.25 },
            Point { x: 0.0, y: 0.5 },
            Point { x: -0.25, y: 0.25 },
            Point { x: 0.0, y: 0.0 },
            Point { x: 0.25, y: 0.25 },
            Point { x: 0.5, y: 0.0 },
            Point { x: 0.25, y: -0.25 },
            Point { x: 0.5, y: -0.5 },
            Point { x: 0.75, y: -0.25 },
            Point { x: 1.0, y: -0.5 },
            Point { x: 1.25, y: -0.25 },
            Point { x: 1.0, y: 0.0 },
        ];
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_c_curve_generation() {
        let chart = FoldCurve {};
        let actual = chart.generate_base_points(BaseChartConfig {
            kind: ChartKind::FoldCCurve,
            complexity: 4,
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
            Point { x: -1.5, y: -0.5 },
            Point { x: -1.0, y: -1.0 },
            Point { x: -0.5, y: -1.5 },
            Point { x: 0.0, y: -1.0 },
            Point { x: 0.5, y: -1.5 },
            Point { x: 1.0, y: -1.0 },
            Point { x: 1.5, y: -0.5 },
            Point { x: 1.0, y: 0.0 },
        ];
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_orders_generation() {
        let chart = FoldCurve {};
        let actual = chart.generate_orders(6);
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
        ];
        assert_eq!(expect, actual);
    }
}
