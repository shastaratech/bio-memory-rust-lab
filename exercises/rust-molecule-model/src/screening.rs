use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExperimentalLabel {
    Active,
    Inactive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreeningDecision {
    Test,
    Defer,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Prediction {
    pub candidate_name: String,
    pub score: f64,
    pub label: Option<ExperimentalLabel>,
}

impl Prediction {
    pub fn new(candidate_name: &str, score: f64, label: Option<ExperimentalLabel>) -> Self {
        Self {
            candidate_name: candidate_name.to_string(),
            score,
            label,
        }
    }

    pub fn decision(&self, threshold: f64) -> ScreeningDecision {
        if self.score >= threshold {
            ScreeningDecision::Test
        } else {
            ScreeningDecision::Defer
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ConfusionMatrix {
    pub true_positive: usize,
    pub false_positive: usize,
    pub true_negative: usize,
    pub false_negative: usize,
}

impl ConfusionMatrix {
    pub fn from_predictions(predictions: &[Prediction], threshold: f64) -> Self {
        let mut matrix = Self::default();

        for prediction in predictions {
            let Some(label) = prediction.label else {
                continue;
            };

            match (prediction.decision(threshold), label) {
                (ScreeningDecision::Test, ExperimentalLabel::Active) => matrix.true_positive += 1,
                (ScreeningDecision::Test, ExperimentalLabel::Inactive) => {
                    matrix.false_positive += 1
                }
                (ScreeningDecision::Defer, ExperimentalLabel::Inactive) => {
                    matrix.true_negative += 1
                }
                (ScreeningDecision::Defer, ExperimentalLabel::Active) => matrix.false_negative += 1,
            }
        }

        matrix
    }

    pub fn precision(&self) -> Option<f64> {
        let predicted_positive = self.true_positive + self.false_positive;

        if predicted_positive == 0 {
            None
        } else {
            Some(self.true_positive as f64 / predicted_positive as f64)
        }
    }

    pub fn recall(&self) -> Option<f64> {
        let actual_positive = self.true_positive + self.false_negative;

        if actual_positive == 0 {
            None
        } else {
            Some(self.true_positive as f64 / actual_positive as f64)
        }
    }

    pub fn accuracy(&self) -> Option<f64> {
        let total =
            self.true_positive + self.false_positive + self.true_negative + self.false_negative;

        if total == 0 {
            None
        } else {
            Some((self.true_positive + self.true_negative) as f64 / total as f64)
        }
    }
}

pub fn testing_queue(predictions: &[Prediction], threshold: f64) -> VecDeque<Prediction> {
    let mut selected: Vec<Prediction> = predictions
        .iter()
        .filter(|prediction| prediction.decision(threshold) == ScreeningDecision::Test)
        .cloned()
        .collect();

    selected.sort_by(|left, right| {
        right
            .score
            .partial_cmp(&left.score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| left.candidate_name.cmp(&right.candidate_name))
    });

    selected.into()
}

pub fn uncertain_predictions(
    predictions: &[Prediction],
    threshold: f64,
    margin: f64,
) -> Vec<Prediction> {
    let mut selected: Vec<Prediction> = predictions
        .iter()
        .filter(|prediction| (prediction.score - threshold).abs() <= margin)
        .cloned()
        .collect();

    selected.sort_by(|left, right| {
        (left.score - threshold)
            .abs()
            .partial_cmp(&(right.score - threshold).abs())
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| left.candidate_name.cmp(&right.candidate_name))
    });

    selected
}

#[cfg(test)]
mod tests {
    use super::*;

    fn toy_predictions() -> Vec<Prediction> {
        vec![
            Prediction::new("candidate-a", 0.92, Some(ExperimentalLabel::Active)),
            Prediction::new("candidate-b", 0.81, Some(ExperimentalLabel::Inactive)),
            Prediction::new("candidate-c", 0.44, Some(ExperimentalLabel::Inactive)),
            Prediction::new("candidate-d", 0.39, Some(ExperimentalLabel::Active)),
            Prediction::new("candidate-e", 0.74, None),
        ]
    }

    #[test]
    fn converts_scores_to_screening_decisions() {
        let high = Prediction::new("high", 0.8, None);
        let low = Prediction::new("low", 0.2, None);

        assert_eq!(high.decision(0.7), ScreeningDecision::Test);
        assert_eq!(low.decision(0.7), ScreeningDecision::Defer);
    }

    #[test]
    fn builds_confusion_matrix_from_labeled_predictions() {
        let matrix = ConfusionMatrix::from_predictions(&toy_predictions(), 0.7);

        assert_eq!(
            matrix,
            ConfusionMatrix {
                true_positive: 1,
                false_positive: 1,
                true_negative: 1,
                false_negative: 1,
            }
        );
    }

    #[test]
    fn computes_precision_recall_and_accuracy() {
        let matrix = ConfusionMatrix::from_predictions(&toy_predictions(), 0.7);

        assert_eq!(matrix.precision(), Some(0.5));
        assert_eq!(matrix.recall(), Some(0.5));
        assert_eq!(matrix.accuracy(), Some(0.5));
    }

    #[test]
    fn handles_metric_denominators_that_are_zero() {
        let matrix = ConfusionMatrix::from_predictions(
            &[Prediction::new(
                "candidate-a",
                0.1,
                Some(ExperimentalLabel::Inactive),
            )],
            0.7,
        );

        assert_eq!(matrix.precision(), None);
        assert_eq!(matrix.recall(), None);
        assert_eq!(matrix.accuracy(), Some(1.0));
    }

    #[test]
    fn builds_priority_queue_for_testing() {
        let queue = testing_queue(&toy_predictions(), 0.7);

        assert_eq!(queue.len(), 3);
        assert_eq!(queue[0].candidate_name, "candidate-a");
        assert_eq!(queue[1].candidate_name, "candidate-b");
        assert_eq!(queue[2].candidate_name, "candidate-e");
    }

    #[test]
    fn finds_predictions_near_the_decision_threshold() {
        let uncertain = uncertain_predictions(&toy_predictions(), 0.7, 0.12);

        assert_eq!(
            uncertain
                .iter()
                .map(|prediction| prediction.candidate_name.as_str())
                .collect::<Vec<_>>(),
            vec!["candidate-e", "candidate-b"]
        );
    }
}
