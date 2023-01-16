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

pub struct TriCurve;

impl TriCurve {
    fn select_fold_rule(kind: &ChartKind) -> Vec<FoldRule> {
        let radian = 90.0_f64.to_radians();

        let trans_rule = vec![FoldRule {
            folds: vec![
                Fold {
                    length: 1.0 / 2.0,
                    radian,
                    from_end: None,
                    from_start: None,
                },
                Fold {
                    length: 1.0 / 2.0,
                    radian: 0.0,
                    from_end: None,
                    from_start: Some(true),
                },
                Fold {
                    length: 1.0 / 2.0,
                    radian,
                    from_end: Some(true),
                    from_start: None,
                },
            ],
        }];
        let cis_rule = vec![FoldRule {
            folds: vec![
                Fold {
                    length: 1.0 / 2.0,
                    radian,
                    from_end: None,
                    from_start: None,
                },
                Fold {
                    length: 1.0 / 2.0,
                    radian: 0.0,
                    from_end: None,
                    from_start: Some(true),
                },
                Fold {
                    length: 1.0 / 2.0,
                    radian: -radian,
                    from_end: Some(true),
                    from_start: None,
                },
            ],
        }];

        match kind {
            ChartKind::TriCis => cis_rule,
            ChartKind::TriTrans => trans_rule,
            _ => panic!("Unsupported chart kind!"),
        }
    }
}

impl ShapesGenerator for TriCurve {
    fn generate_base_points(self: &Self, config: BaseChartConfig) -> Vec<Point> {
        let rules = Self::select_fold_rule(&config.kind);
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
    fn test_tri_cis_curve_generation() {
        let chart = TriCurve {};
        let actual = chart.generate_base_points(BaseChartConfig {
            kind: ChartKind::TriCis,
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
            Point { x: -1.0, y: -0.25 },
            Point { x: -1.25, y: 0.0 },
            Point { x: -1.5, y: -0.25 },
            Point { x: -1.5, y: 0.0 },
            Point { x: -1.75, y: 0.25 },
            Point { x: -1.25, y: 0.25 },
            Point { x: -1.25, y: 0.75 },
            Point { x: -1.0, y: 0.5 },
            Point { x: -1.25, y: 0.25 },
            Point { x: -1.25, y: 0.75 },
            Point { x: -1.75, y: 0.75 },
            Point { x: -1.5, y: 1.0 },
            Point { x: -1.5, y: 1.25 },
            Point { x: -1.25, y: 1.0 },
            Point { x: -1.0, y: 1.25 },
            Point { x: -1.0, y: 1.0 },
            Point { x: -1.25, y: 1.25 },
            Point { x: -0.75, y: 1.25 },
            Point { x: -0.75, y: 1.75 },
            Point { x: -0.5, y: 1.5 },
            Point { x: 0.0, y: 1.5 },
            Point { x: -0.5, y: 1.0 },
            Point { x: 0.0, y: 0.5 },
            Point { x: -0.5, y: 0.5 },
            Point { x: -0.5, y: 1.0 },
            Point { x: 0.0, y: 0.5 },
            Point { x: 0.5, y: 1.0 },
            Point { x: 0.5, y: 0.5 },
            Point { x: 0.75, y: 0.25 },
            Point { x: 0.25, y: 0.25 },
            Point { x: 0.25, y: -0.25 },
            Point { x: 0.0, y: 0.0 },
            Point { x: -0.25, y: -0.25 },
            Point { x: -0.25, y: 0.25 },
            Point { x: -0.75, y: 0.25 },
            Point { x: -0.5, y: 0.5 },
            Point { x: -0.5, y: 1.0 },
            Point { x: 0.0, y: 0.5 },
            Point { x: 0.5, y: 1.0 },
            Point { x: 0.5, y: 0.5 },
            Point { x: 0.0, y: 0.5 },
            Point { x: 0.5, y: 1.0 },
            Point { x: 0.0, y: 1.5 },
            Point { x: 0.5, y: 1.5 },
            Point { x: 0.75, y: 1.75 },
            Point { x: 0.75, y: 1.25 },
            Point { x: 1.25, y: 1.25 },
            Point { x: 1.0, y: 1.0 },
            Point { x: 1.0, y: 1.25 },
            Point { x: 1.25, y: 1.0 },
            Point { x: 1.5, y: 1.25 },
            Point { x: 1.5, y: 1.0 },
            Point { x: 1.75, y: 0.75 },
            Point { x: 1.25, y: 0.75 },
            Point { x: 1.25, y: 0.25 },
            Point { x: 1.0, y: 0.5 },
            Point { x: 1.25, y: 0.75 },
            Point { x: 1.25, y: 0.25 },
            Point { x: 1.75, y: 0.25 },
            Point { x: 1.5, y: 0.0 },
            Point { x: 1.5, y: -0.25 },
            Point { x: 1.25, y: 0.0 },
            Point { x: 1.0, y: -0.25 },
            Point { x: 1.0, y: 0.0 },
        ];
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_tri_trans_curve_generation() {
        let chart = TriCurve {};
        let actual = chart.generate_base_points(BaseChartConfig {
            kind: ChartKind::TriTrans,
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
            Point { x: -1.0, y: -0.25 },
            Point { x: -1.25, y: 0.0 },
            Point { x: -1.5, y: 0.25 },
            Point { x: -1.5, y: 0.0 },
            Point { x: -1.75, y: 0.25 },
            Point { x: -1.25, y: 0.25 },
            Point { x: -0.75, y: 0.25 },
            Point { x: -1.0, y: 0.5 },
            Point { x: -1.25, y: 0.75 },
            Point { x: -0.75, y: 0.75 },
            Point { x: -0.25, y: 0.75 },
            Point { x: -0.5, y: 1.0 },
            Point { x: -0.5, y: 0.75 },
            Point { x: -0.75, y: 1.0 },
            Point { x: -1.0, y: 1.25 },
            Point { x: -1.0, y: 1.0 },
            Point { x: -1.25, y: 1.25 },
            Point { x: -0.75, y: 1.25 },
            Point { x: -0.25, y: 1.25 },
            Point { x: -0.5, y: 1.5 },
            Point { x: 0.0, y: 1.5 },
            Point { x: -0.5, y: 1.0 },
            Point { x: -1.0, y: 0.5 },
            Point { x: -0.5, y: 0.5 },
            Point { x: 0.0, y: 0.5 },
            Point { x: -0.5, y: 0.0 },
            Point { x: -1.0, y: -0.5 },
            Point { x: -0.5, y: -0.5 },
            Point { x: -0.75, y: -0.25 },
            Point { x: -0.25, y: -0.25 },
            Point { x: 0.25, y: -0.25 },
            Point { x: 0.0, y: 0.0 },
            Point { x: -0.25, y: 0.25 },
            Point { x: 0.25, y: 0.25 },
            Point { x: 0.75, y: 0.25 },
            Point { x: 0.5, y: 0.5 },
            Point { x: 1.0, y: 0.5 },
            Point { x: 0.5, y: 0.0 },
            Point { x: 0.0, y: -0.5 },
            Point { x: 0.5, y: -0.5 },
            Point { x: 1.0, y: -0.5 },
            Point { x: 0.5, y: -1.0 },
            Point { x: 0.0, y: -1.5 },
            Point { x: 0.5, y: -1.5 },
            Point { x: 0.25, y: -1.25 },
            Point { x: 0.75, y: -1.25 },
            Point { x: 1.25, y: -1.25 },
            Point { x: 1.0, y: -1.0 },
            Point { x: 1.0, y: -1.25 },
            Point { x: 0.75, y: -1.0 },
            Point { x: 0.5, y: -0.75 },
            Point { x: 0.5, y: -1.0 },
            Point { x: 0.25, y: -0.75 },
            Point { x: 0.75, y: -0.75 },
            Point { x: 1.25, y: -0.75 },
            Point { x: 1.0, y: -0.5 },
            Point { x: 0.75, y: -0.25 },
            Point { x: 1.25, y: -0.25 },
            Point { x: 1.75, y: -0.25 },
            Point { x: 1.5, y: 0.0 },
            Point { x: 1.5, y: -0.25 },
            Point { x: 1.25, y: 0.0 },
            Point { x: 1.0, y: 0.25 },
            Point { x: 1.0, y: 0.0 },
        ];
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_orders_generation() {
        let chart = TriCurve {};
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
