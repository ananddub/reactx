//! Compact encoding for widget payloads carried in custom attributes.
//!
//! The engine transports attributes as strings (`key:value` pairs inside a
//! `style` declaration separated by `;`), so widget data is encoded with
//! control characters that cannot collide with the style syntax:
//!
//! - [`FIELD`] separates values inside a record.
//! - [`ROW`] separates records.
//! - `;` inside a value becomes [`SEMI`] so the style parser keeps it intact.

/// Separator between fields of one record.
pub const FIELD: char = '\u{1f}';
/// Separator between records.
pub const ROW: char = '\u{1e}';
/// Stand-in for a literal `;`, which the style parser treats as a separator.
pub const SEMI: char = '\u{1d}';

fn escape(value: &str) -> String {
    value.replace(';', &SEMI.to_string())
}

fn unescape(value: &str) -> String {
    value.replace(SEMI, ";")
}

/// Encode one record from its fields.
pub fn join_fields(fields: &[String]) -> String {
    fields
        .iter()
        .map(|field| escape(field))
        .collect::<Vec<_>>()
        .join(&FIELD.to_string())
}

/// Decode the fields of one record. An empty string decodes to no fields.
pub fn split_fields(text: &str) -> Vec<String> {
    if text.is_empty() {
        return Vec::new();
    }
    text.split(FIELD).map(unescape).collect()
}

/// Encode a matrix as records of fields.
pub fn join_rows(rows: &[Vec<String>]) -> String {
    rows.iter()
        .map(|row| join_fields(row))
        .collect::<Vec<_>>()
        .join(&ROW.to_string())
}

/// Decode a matrix of records.
pub fn split_rows(text: &str) -> Vec<Vec<String>> {
    if text.is_empty() {
        return Vec::new();
    }
    text.split(ROW).map(split_fields).collect()
}

/// Encode a list of unsigned integers.
pub fn join_nums(values: &[u64]) -> String {
    values
        .iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join(&FIELD.to_string())
}

/// Decode a list of unsigned integers, skipping malformed entries.
pub fn split_nums(text: &str) -> Vec<u64> {
    text.split(FIELD)
        .filter_map(|value| value.parse().ok())
        .collect()
}

/// Encode `(x, y)` points as `"x,y"` values separated by `;`. Numeric data
/// contains no `;`, so the result survives a single field unchanged.
pub fn join_points(points: &[(f64, f64)]) -> String {
    points
        .iter()
        .map(|(x, y)| format!("{x},{y}"))
        .collect::<Vec<_>>()
        .join(";")
}

/// Decode `(x, y)` points, skipping malformed entries.
pub fn split_points(text: &str) -> Vec<(f64, f64)> {
    text.split(';')
        .filter_map(|point| {
            let (x, y) = point.split_once(',')?;
            Some((x.parse().ok()?, y.parse().ok()?))
        })
        .collect()
}

/// Encode an `(a, b)` pair of floats (for axis bounds).
pub fn join_pair(pair: (f64, f64)) -> String {
    format!("{},{}", pair.0, pair.1)
}

/// Decode an `(a, b)` pair of floats.
pub fn split_pair(text: &str) -> Option<(f64, f64)> {
    let (a, b) = text.split_once(',')?;
    Some((a.parse().ok()?, b.parse().ok()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rows_roundtrip_and_escape_semicolons() {
        let rows = vec![
            vec!["a;b".to_string(), "plain".to_string()],
            vec!["1".to_string(), "".to_string()],
        ];
        let encoded = join_rows(&rows);
        assert!(!encoded.contains(';'), "semicolons must be escaped");
        assert_eq!(split_rows(&encoded), rows);
    }

    #[test]
    fn empty_payloads_decode_empty() {
        assert!(split_rows("").is_empty());
        assert!(split_fields("").is_empty());
        assert!(split_nums("").is_empty());
        assert!(split_points("").is_empty());
    }

    #[test]
    fn numbers_and_points_roundtrip() {
        assert_eq!(split_nums(&join_nums(&[3, 0, 42])), vec![3, 0, 42]);
        let points = vec![(0.0, 1.5), (10.0, -2.25)];
        assert_eq!(split_points(&join_points(&points)), points);
        assert_eq!(
            split_pair(&join_pair((-180.0, 180.0))),
            Some((-180.0, 180.0))
        );
    }
}
