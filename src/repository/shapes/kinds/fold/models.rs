pub struct Fold {
    pub length: f64,
    pub radian: f64,
    pub from_start: Option<bool>,
    pub from_end: Option<bool>,
}

pub struct FoldRule {
    pub folds: Vec<Fold>,
}
