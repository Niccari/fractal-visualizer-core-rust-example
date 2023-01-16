use crate::{
    model::shape::{BaseChartConfig, Mutation, Point},
    repository::shapes::generator::randomizer::RandomGenerator,
};

use super::models::{Fold, FoldRule};

pub struct FoldCurveGenerator;

impl FoldCurveGenerator {
    fn get_div_points(
        mutation: &Mutation,
        start: &Point,
        end: &Point,
        rule: &FoldRule,
        length_randomizer: &mut RandomGenerator,
        angle_randomizer: &mut RandomGenerator,
    ) -> Vec<Point> {
        let vector = Point {
            x: end.x - start.x,
            y: end.y - start.y,
        };
        let mut div_points: Vec<Point> = vec![];
        for (i, fold) in rule.folds.iter().enumerate() {
            let Fold {
                length,
                radian,
                from_end,
                from_start,
            } = *fold;
            let src = (|| -> &Point {
                if i == 0 || from_start == Some(true) {
                    return &start;
                }
                if from_end == Some(true) {
                    return &end;
                }
                &div_points[i - 1]
            })();
            let sign = if from_end == Some(true) { -1.0 } else { 1.0 };
            let length_random = length_randomizer.generate();
            let angle_random = angle_randomizer.generate();
            let new_length = sign * length * (length_random + mutation.size);
            let new_radian = radian * (angle_random + mutation.angle);
            let new_vector = Point {
                x: vector.x * new_length,
                y: vector.y * new_length,
            };
            div_points.push(src.rotate_by(&new_vector, new_radian));
        }
        div_points
    }

    fn recursive(
        complexity: u64,
        mutation: &Mutation,
        div: u64,
        depth: u64,
        start: &Point,
        end: &Point,
        rules: &Vec<FoldRule>,
        length_randomizer: &mut RandomGenerator,
        angle_randomizer: &mut RandomGenerator,
    ) -> Vec<Point> {
        let index = (div % rules.len() as u64) as usize;
        let rule = &rules[index];
        let mut div_points = FoldCurveGenerator::get_div_points(
            &mutation,
            &start,
            &end,
            rule,
            length_randomizer,
            angle_randomizer,
        );
        if depth == complexity {
            let mut points: Vec<Point> = vec![];
            if div == 0 {
                points.push(Point {
                    x: start.x,
                    y: start.y,
                });
            }
            points.append(&mut div_points);
            points.push(Point { x: end.x, y: end.y });
            return points;
        }
        let mut points: Vec<Point> = vec![];
        points.push(Point {
            x: start.x,
            y: start.y,
        });
        points.append(&mut div_points);
        points.push(Point { x: end.x, y: end.y });
        let fold_len = rule.folds.len() as u64;
        (0..points.len() - 1)
            .map(|i| (&points[i], &points[i + 1]))
            .enumerate()
            .flat_map(|(i, (div_start, div_end))| {
                let next_div = (fold_len + 1) * div + i as u64;
                FoldCurveGenerator::recursive(
                    complexity,
                    &mutation,
                    next_div,
                    depth + 1,
                    div_start,
                    div_end,
                    &rules,
                    length_randomizer,
                    angle_randomizer,
                )
            })
            .collect()
    }

    pub fn generate_points(config: BaseChartConfig, rules: Vec<FoldRule>) -> Vec<Point> {
        let randomizer = config.randomizer.unwrap();
        let complexity = config.complexity;
        let mutation = config.mutation.unwrap();

        let mut length_randomizer =
            RandomGenerator::new(randomizer.size_seed, randomizer.size_amplitude);
        let mut angle_randomizer =
            RandomGenerator::new(randomizer.angle_seed, randomizer.angle_amplitude);
        let start = Point { x: -1.0, y: 0.0 };
        let end = Point { x: 1.0, y: 0.0 };

        FoldCurveGenerator::recursive(
            complexity,
            &mutation,
            0,
            2,
            &start,
            &end,
            &rules,
            &mut length_randomizer,
            &mut angle_randomizer,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::shape::Mutation;

    #[test]
    fn test_get_div_points_generation() {
        let actual = FoldCurveGenerator::get_div_points(
            &Mutation {
                size: 1.0,
                angle: 1.0,
            },
            &Point { x: 0.0, y: 0.5 },
            &Point { x: 1.0, y: 0.5 },
            &FoldRule {
                folds: vec![Fold {
                    length: 1.0 / 2.0_f64.sqrt(),
                    radian: 45.0_f64.to_radians(),
                    from_start: None,
                    from_end: None,
                }],
            },
            &mut RandomGenerator::new(0, 0.0),
            &mut RandomGenerator::new(0, 0.0),
        );
        let expect: Vec<Point> = vec![Point { x: 0.5, y: 1.0 }];
        assert_eq!(expect, actual);
    }
}
