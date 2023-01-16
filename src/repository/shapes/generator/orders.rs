use crate::model::shape::Order;

pub enum OrderKind {
    Loop,
    StartEnd2xFaster,
    End2xFaster,
    Linear,
}

pub struct OrderConfig {
    pub kind: OrderKind,
    pub point_count: u64,
}

pub trait OrdersGenerator {
    fn generate(config: OrderConfig) -> Vec<Order>
    where
        Self: Sized;
}

pub struct OrdersGeneratorImpl;

impl OrdersGenerator for OrdersGeneratorImpl {
    fn generate(config: OrderConfig) -> Vec<Order> {
        let OrderConfig { kind, point_count } = config;

        match kind {
            OrderKind::Linear => (0..point_count - 1)
                .map(|i| Order { link: (i, i + 1) })
                .collect(),
            OrderKind::Loop => (0..point_count)
                .map(|i| Order { link: (i, i + 1) })
                .collect(),
            OrderKind::StartEnd2xFaster => (0..point_count)
                .map(|i| Order {
                    link: ((2 * i) % point_count, (2 * (i + 1)) % point_count),
                })
                .collect(),
            OrderKind::End2xFaster => (0..point_count)
                .map(|i| Order {
                    link: (i, (2 * (i + 1)) % point_count),
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_linear_generation() {
        let expect: Vec<Order> = vec![
            Order { link: (0, 1) },
            Order { link: (1, 2) },
            Order { link: (2, 3) },
            Order { link: (3, 4) },
            Order { link: (4, 5) },
            Order { link: (5, 6) },
            Order { link: (6, 7) },
        ];
        let actual = OrdersGeneratorImpl::generate(OrderConfig {
            kind: OrderKind::Linear,
            point_count: 8,
        });

        assert_eq!(expect, actual);
    }

    #[test]
    fn test_order_loop_generation() {
        let expect: Vec<Order> = vec![
            Order { link: (0, 1) },
            Order { link: (1, 2) },
            Order { link: (2, 3) },
            Order { link: (3, 4) },
            Order { link: (4, 5) },
            Order { link: (5, 6) },
        ];
        let actual = OrdersGeneratorImpl::generate(OrderConfig {
            kind: OrderKind::Loop,
            point_count: 6,
        });

        assert_eq!(expect, actual);
    }

    #[test]
    fn test_order_start_end_2x_faster_even_generation() {
        let expect: Vec<Order> = vec![
            Order { link: (0, 2) },
            Order { link: (2, 4) },
            Order { link: (4, 0) },
            Order { link: (0, 2) },
            Order { link: (2, 4) },
            Order { link: (4, 0) },
        ];
        let actual = OrdersGeneratorImpl::generate(OrderConfig {
            kind: OrderKind::StartEnd2xFaster,
            point_count: 6,
        });

        assert_eq!(expect, actual);
    }

    #[test]
    fn test_order_start_end_2x_faster_odd_generation() {
        let expect: Vec<Order> = vec![
            Order { link: (0, 2) },
            Order { link: (2, 4) },
            Order { link: (4, 6) },
            Order { link: (6, 1) },
            Order { link: (1, 3) },
            Order { link: (3, 5) },
            Order { link: (5, 0) },
        ];
        let actual = OrdersGeneratorImpl::generate(OrderConfig {
            kind: OrderKind::StartEnd2xFaster,
            point_count: 7,
        });

        assert_eq!(expect, actual);
    }

    #[test]
    fn test_order_end_2x_faster_even_generation() {
        let expect: Vec<Order> = vec![
            Order { link: (0, 2) },
            Order { link: (1, 4) },
            Order { link: (2, 6) },
            Order { link: (3, 0) },
            Order { link: (4, 2) },
            Order { link: (5, 4) },
            Order { link: (6, 6) },
            Order { link: (7, 0) },
        ];
        let actual = OrdersGeneratorImpl::generate(OrderConfig {
            kind: OrderKind::End2xFaster,
            point_count: 8,
        });

        assert_eq!(expect, actual);
    }

    #[test]
    fn test_order_end_2x_faster_odd_generation() {
        let expect: Vec<Order> = vec![
            Order { link: (0, 2) },
            Order { link: (1, 4) },
            Order { link: (2, 6) },
            Order { link: (3, 1) },
            Order { link: (4, 3) },
            Order { link: (5, 5) },
            Order { link: (6, 0) },
        ];
        let actual = OrdersGeneratorImpl::generate(OrderConfig {
            kind: OrderKind::End2xFaster,
            point_count: 7,
        });

        assert_eq!(expect, actual);
    }
}
