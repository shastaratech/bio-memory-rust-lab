use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConcentrationUnit {
    Nanomolar,
    Micromolar,
    Millimolar,
}

impl ConcentrationUnit {
    pub fn symbol(self) -> &'static str {
        match self {
            Self::Nanomolar => "nM",
            Self::Micromolar => "uM",
            Self::Millimolar => "mM",
        }
    }

    fn multiplier_to_micromolar(self) -> f64 {
        match self {
            Self::Nanomolar => 0.001,
            Self::Micromolar => 1.0,
            Self::Millimolar => 1000.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Concentration {
    value: f64,
    unit: ConcentrationUnit,
}

impl Concentration {
    pub fn new(value: f64, unit: ConcentrationUnit) -> Result<Self, MeasurementError> {
        if !value.is_finite() {
            return Err(MeasurementError::NonFiniteValue);
        }

        if value < 0.0 {
            return Err(MeasurementError::NegativeValue);
        }

        Ok(Self { value, unit })
    }

    pub fn value(self) -> f64 {
        self.value
    }

    pub fn unit(self) -> ConcentrationUnit {
        self.unit
    }

    pub fn as_micromolar(self) -> f64 {
        self.value * self.unit.multiplier_to_micromolar()
    }

    pub fn convert_to(self, unit: ConcentrationUnit) -> Self {
        let micromolar = self.as_micromolar();
        Self {
            value: micromolar / unit.multiplier_to_micromolar(),
            unit,
        }
    }
}

impl fmt::Display for Concentration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}", self.value, self.unit.symbol())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssayObservation {
    pub candidate_name: String,
    pub concentration: Concentration,
    pub response_percent: f64,
}

impl AssayObservation {
    pub fn new(
        candidate_name: &str,
        concentration: Concentration,
        response_percent: f64,
    ) -> Result<Self, MeasurementError> {
        if !response_percent.is_finite() {
            return Err(MeasurementError::NonFiniteValue);
        }

        if !(0.0..=100.0).contains(&response_percent) {
            return Err(MeasurementError::ResponseOutOfRange);
        }

        Ok(Self {
            candidate_name: candidate_name.to_string(),
            concentration,
            response_percent,
        })
    }

    pub fn normalized_response(&self) -> f64 {
        self.response_percent / 100.0
    }

    pub fn concentration_micromolar(&self) -> f64 {
        self.concentration.as_micromolar()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MeasurementError {
    NegativeValue,
    NonFiniteValue,
    ResponseOutOfRange,
}

impl fmt::Display for MeasurementError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NegativeValue => formatter.write_str("measurement value cannot be negative"),
            Self::NonFiniteValue => formatter.write_str("measurement value must be finite"),
            Self::ResponseOutOfRange => {
                formatter.write_str("response percent must be between 0 and 100")
            }
        }
    }
}

pub fn sort_observations_by_concentration(
    observations: &[AssayObservation],
) -> Vec<AssayObservation> {
    let mut sorted = observations.to_vec();

    sorted.sort_by(|left, right| {
        left.concentration_micromolar()
            .partial_cmp(&right.concentration_micromolar())
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| left.candidate_name.cmp(&right.candidate_name))
    });

    sorted
}

pub fn mean_response(observations: &[AssayObservation]) -> Option<f64> {
    if observations.is_empty() {
        return None;
    }

    let total: f64 = observations
        .iter()
        .map(|observation| observation.response_percent)
        .sum();

    Some(total / observations.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_concentration_units_to_micromolar() {
        let nanomolar = Concentration::new(500.0, ConcentrationUnit::Nanomolar).unwrap();
        let micromolar = Concentration::new(0.5, ConcentrationUnit::Micromolar).unwrap();
        let millimolar = Concentration::new(0.001, ConcentrationUnit::Millimolar).unwrap();

        assert_eq!(nanomolar.as_micromolar(), 0.5);
        assert_eq!(micromolar.as_micromolar(), 0.5);
        assert_eq!(millimolar.as_micromolar(), 1.0);
    }

    #[test]
    fn converts_between_units() {
        let concentration = Concentration::new(2.0, ConcentrationUnit::Micromolar).unwrap();
        let nanomolar = concentration.convert_to(ConcentrationUnit::Nanomolar);

        assert_eq!(nanomolar.value(), 2000.0);
        assert_eq!(nanomolar.unit(), ConcentrationUnit::Nanomolar);
    }

    #[test]
    fn rejects_invalid_concentration_values() {
        assert_eq!(
            Concentration::new(-1.0, ConcentrationUnit::Micromolar),
            Err(MeasurementError::NegativeValue)
        );
        assert_eq!(
            Concentration::new(f64::NAN, ConcentrationUnit::Micromolar),
            Err(MeasurementError::NonFiniteValue)
        );
    }

    #[test]
    fn creates_assay_observation_with_normalized_response() {
        let concentration = Concentration::new(1.0, ConcentrationUnit::Micromolar).unwrap();
        let observation = AssayObservation::new("candidate-a", concentration, 75.0).unwrap();

        assert_eq!(observation.normalized_response(), 0.75);
        assert_eq!(observation.concentration_micromolar(), 1.0);
    }

    #[test]
    fn rejects_invalid_response_percent() {
        let concentration = Concentration::new(1.0, ConcentrationUnit::Micromolar).unwrap();

        assert_eq!(
            AssayObservation::new("candidate-a", concentration, 125.0),
            Err(MeasurementError::ResponseOutOfRange)
        );
    }

    #[test]
    fn sorts_observations_by_common_unit() {
        let observations = vec![
            AssayObservation::new(
                "candidate-b",
                Concentration::new(1.0, ConcentrationUnit::Micromolar).unwrap(),
                20.0,
            )
            .unwrap(),
            AssayObservation::new(
                "candidate-a",
                Concentration::new(500.0, ConcentrationUnit::Nanomolar).unwrap(),
                40.0,
            )
            .unwrap(),
            AssayObservation::new(
                "candidate-c",
                Concentration::new(0.01, ConcentrationUnit::Millimolar).unwrap(),
                80.0,
            )
            .unwrap(),
        ];

        let sorted = sort_observations_by_concentration(&observations);

        assert_eq!(
            sorted
                .iter()
                .map(|observation| observation.candidate_name.as_str())
                .collect::<Vec<_>>(),
            vec!["candidate-a", "candidate-b", "candidate-c"]
        );
    }

    #[test]
    fn computes_mean_response_for_observations() {
        let concentration = Concentration::new(1.0, ConcentrationUnit::Micromolar).unwrap();
        let observations = vec![
            AssayObservation::new("candidate-a", concentration, 20.0).unwrap(),
            AssayObservation::new("candidate-a", concentration, 60.0).unwrap(),
        ];

        assert_eq!(mean_response(&observations), Some(40.0));
        assert_eq!(mean_response(&[]), None);
    }
}
