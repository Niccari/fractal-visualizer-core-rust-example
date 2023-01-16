use crate::model::shape::{BaseChartConfig, Order, Point};

pub trait ShapesGenerator {
    fn generate_base_points(self: &Self, config: BaseChartConfig) -> Vec<Point>;
    fn generate_orders(self: &Self, complexity: u64) -> Vec<Order>;
}
