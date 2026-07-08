use std::fmt;

use crate::measurement::Concentration;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DosePoint {
    pub concentration: Concentration,
    pub response_percent: f64,
}

impl DosePoint {
    pub fn new(
        concentration: Concentration,
        response_percent: f64,
    ) -> Result<Self, DoseResponseError> {
        if !response_percent.is_finite() {
            return Err(DoseResponseError::NonFiniteResponse);
        }

        if !(0.0..=100.0).contains(&response_percent) {
            return Err(DoseResponseError::ResponseOutOfRange);
        }

        Ok(Self {
            concentration,
            response_percent,
        })
    }

    pub fn concentration_micromolar(&self) -> f64 {
        self.concentration.as_micromolar()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DoseResponseCurve {
    pub candidate_name: String,
    points: Vec<DosePoint>,
}

impl DoseResponseCurve {
    pub fn new(candidate_name: &str, points: Vec<DosePoint>) -> Result<Self, DoseResponseError> {
        if points.len() < 2 {
            return Err(DoseResponseError::TooFewPoints);
        }

        let mut sorted = points;
        sorted.sort_by(|left, right| {
            left.concentration_micromolar()
                .partial_cmp(&right.concentration_micromolar())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        for window in sorted.windows(2) {
            if window[0].concentration_micromolar() == window[1].concentration_micromolar() {
                return Err(DoseResponseError::DuplicateConcentration);
            }
        }

        Ok(Self {
            candidate_name: candidate_name.to_string(),
            points: sorted,
        })
    }

    pub fn points(&self) -> &[DosePoint] {
        &self.points
    }

    pub fn max_response(&self) -> f64 {
        self.points
            .iter()
            .map(|point| point.response_percent)
            .fold(f64::NEG_INFINITY, f64::max)
    }

    pub fn min_response(&self) -> f64 {
        self.points
            .iter()
            .map(|point| point.response_percent)
            .fold(f64::INFINITY, f64::min)
    }

    pub fn response_span(&self) -> f64 {
        self.max_response() - self.min_response()
    }

    pub fn crosses_response(&self, target_response: f64) -> bool {
        self.estimated_concentration_at_response(target_response)
            .is_some()
    }

    pub fn estimated_concentration_at_response(&self, target_response: f64) -> Option<f64> {
        if !target_response.is_finite() {
            return None;
        }

        for window in self.points.windows(2) {
            let left = window[0];
            let right = window[1];
            let low = left.response_percent.min(right.response_percent);
            let high = left.response_percent.max(right.response_percent);

            if target_response < low || target_response > high {
                continue;
            }

            if left.response_percent == right.response_percent {
                return Some(left.concentration_micromolar());
            }

            let response_fraction = (target_response - left.response_percent)
                / (right.response_percent - left.response_percent);
            let concentration = left.concentration_micromolar()
                + response_fraction
                    * (right.concentration_micromolar() - left.concentration_micromolar());

            return Some(concentration);
        }

        None
    }

    pub fn estimated_half_max_concentration(&self) -> Option<f64> {
        self.estimated_concentration_at_response(50.0)
    }

    pub fn is_monotonic_increasing(&self) -> bool {
        self.points
            .windows(2)
            .all(|window| window[1].response_percent >= window[0].response_percent)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoseResponseError {
    TooFewPoints,
    DuplicateConcentration,
    NonFiniteResponse,
    ResponseOutOfRange,
}

impl fmt::Display for DoseResponseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooFewPoints => {
                formatter.write_str("dose-response curve needs at least two points")
            }
            Self::DuplicateConcentration => {
                formatter.write_str("dose concentrations must be unique")
            }
            Self::NonFiniteResponse => formatter.write_str("response must be finite"),
            Self::ResponseOutOfRange => {
                formatter.write_str("response percent must be between 0 and 100")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::measurement::{Concentration, ConcentrationUnit};

    fn point(micromolar: f64, response_percent: f64) -> DosePoint {
        DosePoint::new(
            Concentration::new(micromolar, ConcentrationUnit::Micromolar).unwrap(),
            response_percent,
        )
        .unwrap()
    }

    #[test]
    fn creates_and_sorts_curve_by_concentration() {
        let curve = DoseResponseCurve::new(
            "candidate-a",
            vec![point(10.0, 80.0), point(0.1, 10.0), point(1.0, 50.0)],
        )
        .unwrap();

        assert_eq!(
            curve
                .points()
                .iter()
                .map(DosePoint::concentration_micromolar)
                .collect::<Vec<_>>(),
            vec![0.1, 1.0, 10.0]
        );
    }

    #[test]
    fn rejects_invalid_curve_inputs() {
        assert_eq!(
            DoseResponseCurve::new("candidate-a", vec![point(1.0, 50.0)]),
            Err(DoseResponseError::TooFewPoints)
        );
        assert_eq!(
            DoseResponseCurve::new("candidate-a", vec![point(1.0, 40.0), point(1.0, 50.0)]),
            Err(DoseResponseError::DuplicateConcentration)
        );
        assert_eq!(
            DosePoint::new(
                Concentration::new(1.0, ConcentrationUnit::Micromolar).unwrap(),
                120.0
            ),
            Err(DoseResponseError::ResponseOutOfRange)
        );
    }

    #[test]
    fn computes_response_span_and_monotonicity() {
        let curve = DoseResponseCurve::new(
            "candidate-a",
            vec![point(0.1, 10.0), point(1.0, 50.0), point(10.0, 80.0)],
        )
        .unwrap();

        assert_eq!(curve.min_response(), 10.0);
        assert_eq!(curve.max_response(), 80.0);
        assert_eq!(curve.response_span(), 70.0);
        assert!(curve.is_monotonic_increasing());
    }

    #[test]
    fn detects_non_monotonic_curve() {
        let curve = DoseResponseCurve::new(
            "candidate-a",
            vec![point(0.1, 10.0), point(1.0, 70.0), point(10.0, 60.0)],
        )
        .unwrap();

        assert!(!curve.is_monotonic_increasing());
    }

    #[test]
    fn estimates_concentration_at_target_response_by_linear_interpolation() {
        let curve = DoseResponseCurve::new(
            "candidate-a",
            vec![point(0.1, 10.0), point(1.0, 50.0), point(10.0, 80.0)],
        )
        .unwrap();

        assert_eq!(curve.estimated_half_max_concentration(), Some(1.0));
        assert!((curve.estimated_concentration_at_response(65.0).unwrap() - 5.5).abs() < 0.01);
    }

    #[test]
    fn returns_none_when_target_response_is_not_crossed() {
        let curve = DoseResponseCurve::new(
            "candidate-a",
            vec![point(0.1, 10.0), point(1.0, 20.0), point(10.0, 30.0)],
        )
        .unwrap();

        assert!(!curve.crosses_response(50.0));
        assert_eq!(curve.estimated_half_max_concentration(), None);
    }
}
