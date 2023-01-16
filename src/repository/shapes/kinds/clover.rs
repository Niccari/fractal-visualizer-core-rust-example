use std::f64::consts::PI;

use super::traits::ShapesGenerator;
use crate::model::shape::{BaseChartConfig, Order, Point};
use crate::repository::shapes::generator::orders::{
    OrderConfig, OrderKind, OrdersGenerator, OrdersGeneratorImpl,
};

pub struct Clover;

impl Clover {
    fn point_count(complexity: u64) -> u64 {
        return 40 * complexity;
    }
}

impl ShapesGenerator for Clover {
    fn generate_base_points(self: &Self, config: BaseChartConfig) -> Vec<Point> {
        let point_count = Clover::point_count(config.complexity);
        (0..point_count)
            .map(|i| {
                let angle = (2.0 * PI * i as f64) / point_count as f64;
                let amplitude = (config.complexity as f64 * angle).sin();
                Point {
                    x: amplitude * (angle - PI).cos(),
                    y: amplitude * (angle - PI).sin(),
                }
            })
            .collect()
    }

    fn generate_orders(self: &Self, complexity: u64) -> Vec<Order> {
        let point_count = Clover::point_count(complexity);
        return OrdersGeneratorImpl::generate(OrderConfig {
            kind: OrderKind::Loop,
            point_count,
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::model::shape::ChartKind;

    use super::*;

    #[test]
    fn test_points_generation() {
        let chart = Clover {};
        let actual = chart.generate_base_points(BaseChartConfig {
            complexity: 3,
            kind: ChartKind::Clover,
            mutation: None,
            randomizer: None,
        });
        let expect = &vec![
            Point { x: -0.0, y: -0.0 },
            Point {
                x: -0.1562200770427064,
                y: -0.008187147317233846,
            },
            Point {
                x: -0.3073241669467797,
                y: -0.03230107154560245,
            },
            Point {
                x: -0.44840112333371024,
                y: -0.07101976096010312,
            },
            Point {
                x: -0.5749407342765973,
                y: -0.12220742564187133,
            },
            Point {
                x: -0.6830127018922192,
                y: -0.1830127018922195,
            },
            Point {
                x: -0.7694208842938134,
                y: -0.25000000000000006,
            },
        ][..];
        assert_eq!(expect, &actual[..7]);
    }

    #[test]
    fn test_orders_generation() {
        let chart = Clover {};
        let actual = chart.generate_orders(3);
        let expect: Vec<Order> = (0..3 * 40).map(|i| Order { link: (i, i + 1) }).collect();
        assert_eq!(expect, actual);
    }
}
