use std::f64::consts::PI;

use super::traits::ShapesGenerator;
use crate::model::shape::{BaseChartConfig, Order, Point};
use crate::repository::shapes::generator::orders::{
    OrderConfig, OrderKind, OrdersGenerator, OrdersGeneratorImpl,
};

pub struct Starmine;

impl Starmine {
    fn point_count(complexity: u64) -> u64 {
        return 2 * complexity;
    }
}

impl ShapesGenerator for Starmine {
    fn generate_base_points(self: &Self, config: BaseChartConfig) -> Vec<Point> {
        let point_count = Starmine::point_count(config.complexity);
        (0..point_count)
            .map(|i| {
                let angle = (2.0 * PI * i as f64) / point_count as f64 - PI;
                let amplitude = if i % 2 == 0 { 1.0 } else { 0.25 };
                Point {
                    x: amplitude * angle.cos(),
                    y: amplitude * angle.sin(),
                }
            })
            .collect()
    }

    fn generate_orders(self: &Self, complexity: u64) -> Vec<Order> {
        let point_count = Starmine::point_count(complexity);
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
        let chart = Starmine {};
        let actual = chart.generate_base_points(BaseChartConfig {
            complexity: 3,
            kind: ChartKind::Starmine,
            mutation: None,
            randomizer: None,
        });
        let expect = vec![
            Point {
                x: (-PI).cos(),
                y: (-PI).sin(),
            },
            Point {
                x: (-2.0 * PI / 3.0).cos() / 4.0,
                y: (-2.0 * PI / 3.0).sin() / 4.0,
            },
            Point {
                x: (-1.0 * PI / 3.0).cos(),
                y: (-1.0 * PI / 3.0).sin(),
            },
            Point {
                x: 1.0 / 4.0,
                y: 0.0,
            },
            Point {
                x: (1.0 * PI / 3.0).cos(),
                y: (1.0 * PI / 3.0).sin(),
            },
            Point {
                x: (2.0 * PI / 3.0).cos() / 4.0,
                y: (2.0 * PI / 3.0).sin() / 4.0,
            },
        ];
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_orders_generation() {
        let chart = Starmine {};
        let actual = chart.generate_orders(3);
        let expect = vec![
            Order { link: (0, 1) },
            Order { link: (1, 2) },
            Order { link: (2, 3) },
            Order { link: (3, 4) },
            Order { link: (4, 5) },
            Order { link: (5, 6) },
        ];
        assert_eq!(expect, actual);
    }
}
