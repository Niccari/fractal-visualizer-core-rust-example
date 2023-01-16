use super::traits::ShapesGenerator;
use crate::model::shape::{BaseChartConfig, Order, Point};
use crate::repository::shapes::generator::orders::{
    OrderConfig, OrderKind, OrdersGenerator, OrdersGeneratorImpl,
};
use crate::repository::shapes::generator::points::{
    PointKind, PointsConfig, PointsGenerator, PointsGeneratorImpl,
};

pub struct Sunrise;

impl ShapesGenerator for Sunrise {
    fn generate_base_points(self: &Self, config: BaseChartConfig) -> Vec<Point> {
        return PointsGeneratorImpl::generate(PointsConfig {
            kind: PointKind::Circle,
            length: config.complexity,
        });
    }

    fn generate_orders(self: &Self, complexity: u64) -> Vec<Order> {
        return OrdersGeneratorImpl::generate(OrderConfig {
            kind: OrderKind::End2xFaster,
            point_count: complexity,
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::model::shape::ChartKind;

    use super::*;

    #[test]
    fn test_points_generation() {
        let chart = Sunrise {};
        let actual = chart.generate_base_points(BaseChartConfig {
            complexity: 3,
            kind: ChartKind::Sunrise,
            mutation: None,
            randomizer: None,
        });
        let expect = vec![
            Point { x: 0.0, y: 1.0 },
            Point {
                x: 0.8660254037844388,
                y: -0.5,
            },
            Point {
                x: -0.8660254037844384,
                y: -0.5,
            },
        ];
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_orders_generation() {
        let chart = Sunrise {};
        let actual = chart.generate_orders(5);
        let expect = vec![
            Order { link: (0, 2) },
            Order { link: (1, 4) },
            Order { link: (2, 1) },
            Order { link: (3, 3) },
            Order { link: (4, 0) },
        ];
        assert_eq!(expect, actual);
    }
}
