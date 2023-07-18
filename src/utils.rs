/// Returns the current through an ion channel (in -> out),
/// given the voltage, conductance, and reversal potential.
pub fn get_ion_current(v: f64, g: f64, e: f64) -> f64 {
    g * (v - e)
}

/// Append a string slice to a mutable reference of another string,
/// starting from a new line.
///
/// If neither of the strings is empty, a new line "\n" is inserted between them.
/// Otherwise, the two strings are concatenated simply.
/// This function is private and is used only in this crate.
pub fn append_line(s: &mut String, l: &str) {
    if !s.is_empty() && !l.is_empty() {
        s.push('\n');
    }
    s.push_str(l);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ion_current() {
        let v = -52.0;
        let g = 8.0;
        let e = -84.0;

        let expected = 256.0;
        let actual = get_ion_current(v, g, e);
        assert_eq!(expected, actual);
    }
}
