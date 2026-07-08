use std::fmt;

use crate::molecule::Molecule;

const FIELD_SEPARATOR: char = '|';
const HEADER: &str = "record_id|molecule_name|formula|atom_count|bond_count|notes";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabRecord {
    pub record_id: String,
    pub molecule_name: String,
    pub formula: String,
    pub atom_count: usize,
    pub bond_count: usize,
    pub notes: String,
}

impl LabRecord {
    pub fn from_molecule(
        record_id: &str,
        molecule_name: &str,
        molecule: &Molecule,
        notes: &str,
    ) -> Self {
        Self {
            record_id: record_id.to_string(),
            molecule_name: molecule_name.to_string(),
            formula: molecule.formula(),
            atom_count: molecule.atom_count(),
            bond_count: molecule.bond_count(),
            notes: notes.to_string(),
        }
    }

    pub fn header() -> &'static str {
        HEADER
    }

    pub fn to_record_line(&self) -> Result<String, LabRecordParseError> {
        for value in [
            &self.record_id,
            &self.molecule_name,
            &self.formula,
            &self.notes,
        ] {
            if value.contains(FIELD_SEPARATOR) {
                return Err(LabRecordParseError::UnsupportedSeparator);
            }
        }

        Ok(format!(
            "{}|{}|{}|{}|{}|{}",
            self.record_id,
            self.molecule_name,
            self.formula,
            self.atom_count,
            self.bond_count,
            self.notes
        ))
    }

    pub fn from_record_line(line: &str) -> Result<Self, LabRecordParseError> {
        let fields: Vec<&str> = line.split(FIELD_SEPARATOR).collect();

        if fields.len() != 6 {
            return Err(LabRecordParseError::WrongFieldCount {
                expected: 6,
                actual: fields.len(),
            });
        }

        Ok(Self {
            record_id: parse_non_empty(fields[0], "record_id")?.to_string(),
            molecule_name: parse_non_empty(fields[1], "molecule_name")?.to_string(),
            formula: parse_non_empty(fields[2], "formula")?.to_string(),
            atom_count: parse_usize(fields[3], "atom_count")?,
            bond_count: parse_usize(fields[4], "bond_count")?,
            notes: fields[5].to_string(),
        })
    }

    pub fn to_markdown_row(&self) -> String {
        format!(
            "| {} | {} | {} | {} | {} | {} |",
            self.record_id,
            self.molecule_name,
            self.formula,
            self.atom_count,
            self.bond_count,
            self.notes
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LabRecordParseError {
    WrongFieldCount { expected: usize, actual: usize },
    EmptyField { field: &'static str },
    InvalidNumber { field: &'static str, value: String },
    UnsupportedSeparator,
}

impl fmt::Display for LabRecordParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongFieldCount { expected, actual } => {
                write!(formatter, "expected {expected} fields, found {actual}")
            }
            Self::EmptyField { field } => write!(formatter, "field is empty: {field}"),
            Self::InvalidNumber { field, value } => {
                write!(formatter, "invalid number in {field}: {value}")
            }
            Self::UnsupportedSeparator => {
                write!(formatter, "field values cannot contain '{FIELD_SEPARATOR}'")
            }
        }
    }
}

fn parse_non_empty<'a>(
    value: &'a str,
    field: &'static str,
) -> Result<&'a str, LabRecordParseError> {
    if value.trim().is_empty() {
        Err(LabRecordParseError::EmptyField { field })
    } else {
        Ok(value)
    }
}

fn parse_usize(value: &str, field: &'static str) -> Result<usize, LabRecordParseError> {
    value
        .parse()
        .map_err(|_| LabRecordParseError::InvalidNumber {
            field,
            value: value.to_string(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::molecule::{ethanol, water};

    #[test]
    fn creates_record_from_molecule_summary() {
        let record = LabRecord::from_molecule("rec-001", "water", &water(), "starter example");

        assert_eq!(record.formula, "H2O");
        assert_eq!(record.atom_count, 3);
        assert_eq!(record.bond_count, 2);
    }

    #[test]
    fn round_trips_record_line() {
        let record = LabRecord::from_molecule("rec-002", "ethanol", &ethanol(), "class demo");
        let line = record.to_record_line().unwrap();

        assert_eq!(line, "rec-002|ethanol|C2H6O|9|8|class demo".to_string());
        assert_eq!(LabRecord::from_record_line(&line), Ok(record));
    }

    #[test]
    fn exposes_stable_header() {
        assert_eq!(
            LabRecord::header(),
            "record_id|molecule_name|formula|atom_count|bond_count|notes"
        );
    }

    #[test]
    fn renders_markdown_row_for_reports() {
        let record = LabRecord::from_molecule("rec-001", "water", &water(), "starter example");

        assert_eq!(
            record.to_markdown_row(),
            "| rec-001 | water | H2O | 3 | 2 | starter example |"
        );
    }

    #[test]
    fn rejects_lines_with_missing_fields() {
        assert_eq!(
            LabRecord::from_record_line("rec-001|water|H2O"),
            Err(LabRecordParseError::WrongFieldCount {
                expected: 6,
                actual: 3,
            })
        );
    }

    #[test]
    fn rejects_empty_required_fields() {
        assert_eq!(
            LabRecord::from_record_line("rec-001||H2O|3|2|missing name"),
            Err(LabRecordParseError::EmptyField {
                field: "molecule_name",
            })
        );
    }

    #[test]
    fn rejects_invalid_number_fields() {
        assert_eq!(
            LabRecord::from_record_line("rec-001|water|H2O|three|2|bad number"),
            Err(LabRecordParseError::InvalidNumber {
                field: "atom_count",
                value: "three".to_string(),
            })
        );
    }

    #[test]
    fn rejects_separator_inside_field_values() {
        let record = LabRecord::from_molecule("rec-001", "water", &water(), "bad | separator");

        assert_eq!(
            record.to_record_line(),
            Err(LabRecordParseError::UnsupportedSeparator)
        );
    }
}
