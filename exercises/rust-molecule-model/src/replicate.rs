use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct ReplicateSeries {
    pub candidate_name: String,
    responses_percent: Vec<f64>,
}

impl ReplicateSeries {
    pub fn new(candidate_name: &str, responses_percent: Vec<f64>) -> Result<Self, ReplicateError> {
        if responses_percent.is_empty() {
            return Err(ReplicateError::EmptySeries);
        }

        for &response in &responses_percent {
            if !response.is_finite() {
                return Err(ReplicateError::NonFiniteValue);
            }

            if !(0.0..=100.0).contains(&response) {
                return Err(ReplicateError::ResponseOutOfRange);
            }
        }

        Ok(Self {
            candidate_name: candidate_name.to_string(),
            responses_percent,
        })
    }

    pub fn responses_percent(&self) -> &[f64] {
        &self.responses_percent
    }

    pub fn replicate_count(&self) -> usize {
        self.responses_percent.len()
    }

    pub fn mean_response(&self) -> f64 {
        self.responses_percent.iter().sum::<f64>() / self.replicate_count() as f64
    }

    pub fn min_response(&self) -> f64 {
        self.responses_percent
            .iter()
            .copied()
            .fold(f64::INFINITY, f64::min)
    }

    pub fn max_response(&self) -> f64 {
        self.responses_percent
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max)
    }

    pub fn response_range(&self) -> f64 {
        self.max_response() - self.min_response()
    }

    pub fn sample_variance(&self) -> Option<f64> {
        if self.replicate_count() < 2 {
            return None;
        }

        let mean = self.mean_response();
        let sum_squared_error: f64 = self
            .responses_percent
            .iter()
            .map(|response| (response - mean).powi(2))
            .sum();

        Some(sum_squared_error / (self.replicate_count() - 1) as f64)
    }

    pub fn sample_standard_deviation(&self) -> Option<f64> {
        self.sample_variance().map(f64::sqrt)
    }

    pub fn coefficient_of_variation(&self) -> Option<f64> {
        let mean = self.mean_response();

        if mean == 0.0 {
            None
        } else {
            self.sample_standard_deviation()
                .map(|standard_deviation| standard_deviation / mean)
        }
    }

    pub fn is_consistent(&self, max_standard_deviation: f64) -> bool {
        match self.sample_standard_deviation() {
            Some(standard_deviation) => standard_deviation <= max_standard_deviation,
            None => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplicateError {
    EmptySeries,
    NonFiniteValue,
    ResponseOutOfRange,
}

impl fmt::Display for ReplicateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySeries => formatter.write_str("replicate series cannot be empty"),
            Self::NonFiniteValue => formatter.write_str("replicate values must be finite"),
            Self::ResponseOutOfRange => {
                formatter.write_str("response percent must be between 0 and 100")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_replicate_series_with_valid_responses() {
        let series = ReplicateSeries::new("candidate-a", vec![40.0, 44.0, 46.0]).unwrap();

        assert_eq!(series.candidate_name, "candidate-a");
        assert_eq!(series.responses_percent(), &[40.0, 44.0, 46.0]);
        assert_eq!(series.replicate_count(), 3);
    }

    #[test]
    fn rejects_empty_or_invalid_series() {
        assert_eq!(
            ReplicateSeries::new("candidate-a", vec![]),
            Err(ReplicateError::EmptySeries)
        );
        assert_eq!(
            ReplicateSeries::new("candidate-a", vec![f64::NAN]),
            Err(ReplicateError::NonFiniteValue)
        );
        assert_eq!(
            ReplicateSeries::new("candidate-a", vec![101.0]),
            Err(ReplicateError::ResponseOutOfRange)
        );
    }

    #[test]
    fn computes_mean_min_max_and_range() {
        let series = ReplicateSeries::new("candidate-a", vec![40.0, 44.0, 46.0]).unwrap();

        assert_eq!(series.mean_response(), 130.0 / 3.0);
        assert_eq!(series.min_response(), 40.0);
        assert_eq!(series.max_response(), 46.0);
        assert_eq!(series.response_range(), 6.0);
    }

    #[test]
    fn computes_sample_variance_and_standard_deviation() {
        let series = ReplicateSeries::new("candidate-a", vec![40.0, 44.0, 46.0]).unwrap();

        assert!((series.sample_variance().unwrap() - 9.333).abs() < 0.01);
        assert!((series.sample_standard_deviation().unwrap() - 3.055).abs() < 0.01);
    }

    #[test]
    fn single_replicate_has_no_sample_variance() {
        let series = ReplicateSeries::new("candidate-a", vec![40.0]).unwrap();

        assert_eq!(series.sample_variance(), None);
        assert_eq!(series.sample_standard_deviation(), None);
        assert_eq!(series.coefficient_of_variation(), None);
        assert!(series.is_consistent(1.0));
    }

    #[test]
    fn computes_coefficient_of_variation() {
        let series = ReplicateSeries::new("candidate-a", vec![40.0, 44.0, 46.0]).unwrap();

        assert!((series.coefficient_of_variation().unwrap() - 0.0705).abs() < 0.001);
    }

    #[test]
    fn checks_consistency_against_standard_deviation_limit() {
        let consistent = ReplicateSeries::new("candidate-a", vec![40.0, 41.0, 42.0]).unwrap();
        let noisy = ReplicateSeries::new("candidate-b", vec![20.0, 50.0, 80.0]).unwrap();

        assert!(consistent.is_consistent(2.0));
        assert!(!noisy.is_consistent(2.0));
    }
}
