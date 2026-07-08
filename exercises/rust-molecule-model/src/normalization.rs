use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ControlSet {
    pub blank_response: f64,
    pub negative_control_response: f64,
    pub positive_control_response: f64,
}

impl ControlSet {
    pub fn new(
        blank_response: f64,
        negative_control_response: f64,
        positive_control_response: f64,
    ) -> Result<Self, NormalizationError> {
        for value in [
            blank_response,
            negative_control_response,
            positive_control_response,
        ] {
            if !value.is_finite() {
                return Err(NormalizationError::NonFiniteValue);
            }
        }

        if positive_control_response <= negative_control_response {
            return Err(NormalizationError::InvalidControlOrder);
        }

        Ok(Self {
            blank_response,
            negative_control_response,
            positive_control_response,
        })
    }

    pub fn blank_correct(&self, raw_response: f64) -> Result<f64, NormalizationError> {
        if !raw_response.is_finite() {
            return Err(NormalizationError::NonFiniteValue);
        }

        Ok(raw_response - self.blank_response)
    }

    pub fn control_window(&self) -> f64 {
        self.positive_control_response - self.negative_control_response
    }

    pub fn normalize_to_controls(&self, raw_response: f64) -> Result<f64, NormalizationError> {
        let corrected = self.blank_correct(raw_response)?;
        let corrected_negative = self.negative_control_response - self.blank_response;
        let corrected_positive = self.positive_control_response - self.blank_response;
        let window = corrected_positive - corrected_negative;

        if window <= 0.0 {
            return Err(NormalizationError::InvalidControlOrder);
        }

        Ok((corrected - corrected_negative) / window)
    }

    pub fn normalize_percent(&self, raw_response: f64) -> Result<f64, NormalizationError> {
        Ok(self.normalize_to_controls(raw_response)? * 100.0)
    }

    pub fn passes_quality_window(&self, minimum_window: f64) -> bool {
        self.control_window() >= minimum_window
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NormalizationError {
    NonFiniteValue,
    InvalidControlOrder,
}

impl fmt::Display for NormalizationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteValue => {
                formatter.write_str("control and response values must be finite")
            }
            Self::InvalidControlOrder => {
                formatter.write_str("positive control must be greater than negative control")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_valid_control_set() {
        let controls = ControlSet::new(5.0, 10.0, 90.0).unwrap();

        assert_eq!(controls.blank_response, 5.0);
        assert_eq!(controls.control_window(), 80.0);
    }

    #[test]
    fn rejects_non_finite_or_bad_control_order() {
        assert_eq!(
            ControlSet::new(f64::NAN, 10.0, 90.0),
            Err(NormalizationError::NonFiniteValue)
        );
        assert_eq!(
            ControlSet::new(5.0, 90.0, 10.0),
            Err(NormalizationError::InvalidControlOrder)
        );
    }

    #[test]
    fn subtracts_blank_response() {
        let controls = ControlSet::new(5.0, 10.0, 90.0).unwrap();

        assert_eq!(controls.blank_correct(42.0), Ok(37.0));
    }

    #[test]
    fn normalizes_raw_response_to_control_fraction() {
        let controls = ControlSet::new(5.0, 10.0, 90.0).unwrap();

        assert_eq!(controls.normalize_to_controls(10.0), Ok(0.0));
        assert_eq!(controls.normalize_to_controls(90.0), Ok(1.0));
        assert_eq!(controls.normalize_to_controls(50.0), Ok(0.5));
    }

    #[test]
    fn normalizes_raw_response_to_percent() {
        let controls = ControlSet::new(5.0, 10.0, 90.0).unwrap();

        assert_eq!(controls.normalize_percent(50.0), Ok(50.0));
    }

    #[test]
    fn checks_control_quality_window() {
        let strong = ControlSet::new(5.0, 10.0, 90.0).unwrap();
        let weak = ControlSet::new(5.0, 10.0, 25.0).unwrap();

        assert!(strong.passes_quality_window(50.0));
        assert!(!weak.passes_quality_window(50.0));
    }

    #[test]
    fn rejects_non_finite_raw_response() {
        let controls = ControlSet::new(5.0, 10.0, 90.0).unwrap();

        assert_eq!(
            controls.normalize_to_controls(f64::INFINITY),
            Err(NormalizationError::NonFiniteValue)
        );
    }
}
