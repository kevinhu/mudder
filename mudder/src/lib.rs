use num_bigint::BigInt;
use num_traits::cast::ToPrimitive;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::num::NonZeroUsize;

/// Performs long division on a vector of numerators with a single denominator.
/// Element-wise, each numerator is divided by the denominator, with any remainder carried to the next iteration.
///
/// # Arguments
///
/// * `numerator` - A vector of unsigned integers representing the numerators.
///
/// * `denominator` - An unsigned integer representing the denominator.
///
/// * `base` - The base to be multiplied with the remainder. This resembles the number base in positional numeral systems.
///
/// # Returns
///
/// * A tuple consisting of a vector of results from each numerator divided by the denominator and
///   the final remainder after iterating over all numerators.
pub fn long_div(numerator: Vec<usize>, denominator: usize, base: usize) -> (Vec<usize>, usize) {
    let mut result = Vec::new();
    let mut remainder = 0;

    for current in numerator {
        // Calculate the temporary value for the current iteration
        let temp_value = current + remainder * base;

        // Perform the division and store the result
        result.push(temp_value / denominator);

        // Calculate the remainder for the current iteration
        remainder = temp_value % denominator;
    }

    // Return the result and the remainder
    (result, remainder)
}

#[derive(Debug)]
pub struct DivisionResult {
    result: Vec<usize>,
    remainder: usize,
    denominator: usize,
}

/// Performs the subtraction of two equal-length vectors in the context of a specific number base, and handles the borrowing as needed.
/// This function operates on a digit by digit basis and each digit represents a value in a given base.
///
/// # Arguments
///
/// * `a` - A vector of unsigned integers, representing the minuend.
///
/// * `b` - A vector of unsigned integers, of equal length to 'a', representing the subtrahend.
///
/// * `base` - The base of the number system being used, resembles the number base in positional numeral systems.
///
/// * `remainder` - An optional tuple of unsigned integers. If provided, these values are appended to vectors 'a' and 'b' respectively.
///
/// * `denominator` - An unsigned integer representing the denominator in the case the remainder is involved in calculations.
///
/// # Returns
///
/// * A result type:
///   - `Ok` variant contains a tuple consisting of the resulting vector after the subtraction operation and a single unsigned integer remainder.
///   - `Err` variant contains a string message indicating an error (Mismatch length, negative result, or failure to borrow).
fn long_sub_same_len(
    a: &Vec<usize>,
    b: &Vec<usize>,
    base: usize,
    remainder: Option<(usize, usize)>,
    denominator: usize,
) -> Result<(Vec<usize>, usize), &'static str> {
    if a.len() != b.len() {
        return Err("a and b should have same length");
    }

    let mut a = a.clone();
    let mut b = b.clone();

    if let Some(rem) = remainder {
        a.push(rem.0);
        b.push(rem.1);
    }

    let mut ret = vec![0; a.len()];

    for i in (0..a.len()).rev() {
        if a[i] >= b[i] {
            ret[i] = a[i] - b[i];
            continue;
        }
        if i == 0 {
            return Err("Cannot go negative");
        }
        let mut do_break = false;
        // look for a digit to the left to borrow from
        for j in (0..i).rev() {
            if a[j] > 0 {
                // found a non-zero digit. Decrement it
                a[j] -= 1;
                // increment all digits to the right by `base-1`
                for k in j + 1..i {
                    a[k] += base - 1;
                }
                // until you reach the digit you couldn't subtract
                ret[i] =
                    a[i] + if remainder.is_some() && i == a.len() - 1 {
                        denominator
                    } else {
                        base
                    } - b[i];
                do_break = true;
                break;
            }
        }
        if do_break {
            continue;
        }
        return Err("Failed to find digit to borrow from");
    }
    if remainder.is_some() {
        // result, remainder
        Ok((ret[..ret.len() - 1].to_vec(), ret[ret.len() - 1]))
    } else {
        Ok((ret, 0))
    }
}

/// Performs addition on two equal-length vectors and handles carrying over values based on the base of the numeric system.
///
/// # Arguments
///
/// * `a` - A vector of unsigned integers to be added, representing the first operand.
///
/// * `b` - A vector of unsigned integers to be added, of equal length to 'a', representing the second operand.
///
/// * `base` - The base of the number system being used. This resembles the number base in positional numeral systems.
///
/// * `remainder` - An unsigned integer. If it's greater or equal to the denominator, it creates a carry for the addition operation.
///
/// * `denominator` - An unsigned integer representing the denominator used to check if the remainder should be carried.
///
/// # Returns
///
/// * A result type:
///    - `Ok` variant contains an instance of `DivisionResult` with fields `result` (the resulting vector after addition), `remainder` and `denominator`.
///    - `Err` variant contains an error message indicating a mismatch in the lengths of the input vectors.
pub fn long_add_same_len(
    a: &Vec<usize>,
    b: &Vec<usize>,
    base: usize,
    remainder: usize,
    denominator: usize,
) -> Result<DivisionResult, &'static str> {
    if a.len() != b.len() {
        return Err("a and b should have same length");
    }

    let mut _remainder = remainder;

    let mut carry = remainder >= denominator;
    if carry {
        _remainder -= denominator;
    }

    let mut res = b.clone();

    for (i, ai) in a.iter().enumerate().rev() {
        let result = ai + b[i] + carry as usize;
        carry = result >= base;
        res[i] = if carry { result - base } else { result };
    }

    Ok(DivisionResult {
        result: res,
        remainder: _remainder,
        denominator,
    })
}

/// Resizes the given vector to a provided length by appending a specific padding value. If the vector is already longer, it does nothing.
///
/// # Arguments
///
/// * `vector` - The input mutable vector of unsigned integers to be resized.
///
/// * `to_length` - The desired length of the input vector after the operation.
///
/// * `pad_value` - The unsigned integer value to append to the vector if resizing is needed.
///
/// # Note
///
/// This is a mutating function, the passed vector will be directly updated.
pub fn pad_right(vector: &mut Vec<usize>, to_length: usize, pad_value: usize) {
    if to_length > vector.len() {
        vector.resize(to_length, pad_value);
    }
}

/// Resizes the given vector to a provided length by prepending a specific padding value. If the vector is already longer, it does nothing.
///
/// # Arguments
///
/// * `arr` - The input vector of unsigned integers.
///
/// * `to_length` - The desired length of the vector after the operation.
///
/// * `val` - The unsigned integer value to prepend to the vector if resizing is needed.
///
/// # Returns
///
/// * Returns a new vector of unsigned integers that has been resized and possibly padded based on the inputs.
pub fn pad_left(arr: Vec<usize>, to_length: usize, val: usize) -> Vec<usize> {
    // Calculate the number of padding elements needed
    let pad_len = to_length.saturating_sub(arr.len());

    // Create a new vector from the original array
    let mut result = arr;

    // If padding is needed, insert the padding at the start of the array
    if pad_len > 0 {
        result.splice(0..0, vec![val; pad_len]);
    }

    result
}

/// Creates a range of values between two vectors `a` and `b` spread evenly over `n` steps.
///
/// # Arguments
///
/// * `a` - A vector of unsigned integers, the starting point of the range.
///
/// * `b` - A vector of unsigned integers, the ending point of the range. Must be of the same length as `a`.
///
/// * `base` - The base of the number system being used. This represents the number of possible values each digit can have.
///
/// * `n` - A `NonZeroUsize` that specifies the number of steps. The values will be generated such that they lie evenly between `a` and `b`.
///
/// * `m` - An unsigned integer used as denominator in internal division operations.
///
/// # Returns
///
/// * Returns a Result type:
///   - `Ok` variant that contains a vector of `DivisionResult`, which includes evenly spaced calculated values.
///   - `Err` variant that contains an error message if `a` and `b` are not separable.
pub fn long_linspace(
    a: Vec<usize>,
    b: Vec<usize>,
    base: usize,
    n: NonZeroUsize,
    m: usize,
) -> Result<Vec<DivisionResult>, &'static str> {
    let mut a = a.clone();
    let mut b = b.clone();

    if a.len() < b.len() {
        pad_right(&mut a, b.len(), 0);
    } else if b.len() < a.len() {
        pad_right(&mut b, a.len(), 0);
    }

    if a == b {
        return Err("Start and end strings are lexicographically inseperable");
    }
    let (a_div, a_div_rem) = long_div(a.clone(), m, base);
    let (b_div, b_div_rem) = long_div(b.clone(), m, base);

    let (mut a_prev, mut a_prev_rem) =
        long_sub_same_len(&a, &a_div, base, Some((0, a_div_rem)), m)?;
    let (mut b_prev, mut b_prev_rem) = (b_div.clone(), b_div_rem);

    let mut ret = Vec::new();
    for _i in 1..=(n.into()) {
        let result = long_add_same_len(&a_prev, &b_prev, base, a_prev_rem + b_prev_rem, m)?;
        ret.push(result);

        let (a_prev_temp, a_prev_rem_temp) =
            long_sub_same_len(&a_prev, &a_div, base, Some((a_prev_rem, a_div_rem)), m)?;
        a_prev = a_prev_temp;
        a_prev_rem = a_prev_rem_temp;

        let b_temp = long_add_same_len(&b_prev, &b_div, base, b_prev_rem + b_div_rem, m)?;
        b_prev = b_temp.result;
        b_prev_rem = b_temp.remainder;
    }
    Ok(ret)
}

/// Returns a slice of `water` up to the first index `i` where `water[i]` is not zero and
/// `water[i]` is not equal to `rock[i]`. If no such index exists, returns a copy of `water`.
pub fn chop_digits(rock: &[usize], water: &[usize]) -> Vec<usize> {
    // Iterate over the indices and values of `water`
    for (i, &value) in water.iter().enumerate() {
        // If `value` is not zero and `value` is not equal to the corresponding element in `rock`
        if let Some(&rock_value) = rock.get(i) {
            if value != 0 && value != rock_value {
                // Return a slice of `water` up to and including the current index
                return water[..=i].to_vec();
            }
        } else if value != 0 {
            // If `i` exceeds the length of `rock` and `value` is not zero
            // Return a slice of `water` up to and including the current index
            return water[..=i].to_vec();
        }
    }
    // If none of the elements in `water` meet the conditions, return a copy of `water`
    water.to_vec()
}

/// Compare two arrays lexicographically.
///
/// Returns true if `a` is lexicographically less than `b`.
pub fn lexicographic_less_than_array(a: &[usize], b: &[usize]) -> bool {
    // Iterate over both arrays simultaneously
    for (&x, &y) in a.iter().zip(b.iter()) {
        match x.cmp(&y) {
            // If elements are equal, continue to the next iteration
            Ordering::Equal => continue,
            // If a[i] is less than b[i], return true
            Ordering::Less => return true,
            // If a[i] is greater than b[i], return false
            Ordering::Greater => return false,
        }
    }

    // If all elements are equal, compare the lengths of the arrays
    a.len() < b.len()
}

/// Takes a mutable vector of vectors (`strings`), and manipulates the vectors in it by chopping the successive digits using the `chop_digits` function.
/// The sequences are processed based on their lexicographic order.
///
/// # Arguments
///
/// * `strings` - A mutable vector of vectors of unsigned integers.
///
/// # Returns
///
/// A vector of vectors of unsigned integers, where each vector has been modified by chopping off its successive digits.
pub fn chop_successive_digits(strings: &mut Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let reversed = !lexicographic_less_than_array(&strings[0], &strings[1]);
    if reversed {
        strings.reverse();
    }
    let mut result = Vec::new();
    result.push(strings[0].clone());

    for i in 1..strings.len() {
        let new = chop_digits(result.last().unwrap(), &strings[i]);
        result.push(new);
    }

    if reversed {
        result.reverse();
    }
    result
}

/// A table for generating lexicographically spaced strings.
pub struct SymbolTable {
    num2sym: Vec<char>,
    sym2num: HashMap<char, usize>,
    base: usize,
}

impl SymbolTable {
    /// Create a new symbol table given a vector of characters.
    pub fn new(symbols: Vec<char>) -> Self {
        let mut sym2num = HashMap::new();
        for (i, c) in symbols.iter().enumerate() {
            sym2num.insert(c.clone(), i);
        }

        let base = symbols.len();

        SymbolTable {
            num2sym: symbols,
            sym2num,
            base,
        }
    }

    /// Create a new symbol table from a string.
    pub fn from_str(symbols: &str) -> Self {
        Self::new(symbols.chars().collect())
    }

    /// Creates a decimal symbol table (0-9).
    pub fn decimal() -> Self {
        Self::new(('0'..='9').collect())
    }

    /// Creates an alphabetic symbol table (a-z).
    pub fn alphabetic() -> Self {
        Self::new(('a'..='z').collect())
    }

    /// Creates a base36 symbol table (0-9, a-z).
    pub fn base36() -> Self {
        Self::new(('0'..='9').chain('a'..='z').collect())
    }

    /// Creates a base62 symbol table (0-9, A-Z, a-z).
    pub fn base62() -> Self {
        Self::new(('0'..='9').chain('A'..='Z').chain('a'..='z').collect())
    }

    /// Creates a hexadecimal symbol table (0-9, a-f).
    pub fn hex() -> Self {
        Self::new(('0'..='9').chain('a'..='f').collect())
    }

    pub fn number_to_digits(&self, mut num: usize) -> Vec<usize> {
        // Unwrap the base, defaulting to self.base if None.
        let mut digits = Vec::new();

        // Convert the number to the specified base by repeatedly dividing it by the base and pushing the remainder onto the digits vector.
        while num >= 1 {
            digits.push(num % self.base);
            num /= self.base;
        }

        // Reverse the digits vector because the digits were pushed on in least-significant to most-significant order.
        digits.reverse();

        // If the digits vector is empty (i.e., if the input number was 0), return a vector containing a single 0. Otherwise, return the digits vector.
        if digits.is_empty() {
            vec![0]
        } else {
            digits
        }
    }

    pub fn digits_to_string(&self, digits: &[usize]) -> String {
        digits
            .iter()
            .map(|&n| self.num2sym.get(n).expect("Invalid digit provided").clone())
            .collect::<String>()
    }

    pub fn chars_to_digits(&self, string: Vec<char>) -> Vec<usize> {
        string
            .iter()
            .filter_map(|c| self.sym2num.get(c))
            .cloned()
            .collect()
    }

    pub fn digits_to_number(&self, digits: &[usize]) -> usize {
        // Initialize the current base and the result
        let mut current_base = 1;
        let mut result = 0;

        // Iterate over the digits in reverse order
        for &digit in digits.iter().rev() {
            // Add the current digit times the current base to the result
            result += digit * current_base;

            // Multiply the current base by the base
            current_base *= self.base;
        }

        // Return the result
        result
    }

    pub fn number_to_string(&self, num: usize) -> String {
        self.digits_to_string(&self.number_to_digits(num))
    }

    pub fn string_to_number(&self, num: Vec<char>) -> usize {
        self.digits_to_number(&self.chars_to_digits(num))
    }

    pub fn round_fraction(
        &self,
        numerator: usize,
        denominator: usize,
        base: Option<usize>,
    ) -> Result<Vec<usize>, &'static str> {
        let base = base.unwrap_or(self.base);
        let places = (denominator as f64).log(base as f64).ceil() as usize;
        let scale = base.pow(places as u32);

        let numerator_big = BigInt::from(numerator);
        let denominator_big = BigInt::from(denominator);
        let scale_big = BigInt::from(scale);

        let scaled = ((numerator_big * scale_big) / denominator_big)
            .to_biguint()
            .unwrap()
            .to_usize()
            .unwrap();
        let digits = self.number_to_digits(scaled);

        Ok(pad_left(digits, places, 0))
    }

    /// Generate a sequence of `n` intermediary strings between `start` and `end` that are approximately
    /// equally spaced in lexicographic order as determined by the symbols in the SymbolTable. Each string
    /// is represented in the current base of the SymbolTable.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of intermediary strings to generate.
    /// * `start` - An optional string as a lower bound for the resulting strings. If None,
    /// this will default to the first character in the SymbolTable.
    /// * `end` - An optional string as an upper bound for the resulting strings. If None,
    /// this will default to a string consisting of k+7 instances of the last character in the SymbolTable
    /// where k is the length of the `start` string.
    ///
    /// # Returns
    ///
    /// A result containing a vector of `n` strings if successful, or an error message if `n` is 0.
    pub fn mudder(
        &self,
        n: usize,
        start: Option<&str>,
        end: Option<&str>,
    ) -> Result<Vec<String>, &'static str> {
        let n = NonZeroUsize::new(n).ok_or("n must be greater than 0")?;

        let start_vec = match start {
            Some(start) => start.chars().collect(),
            None => vec![self.num2sym[0].clone()],
        };

        let end_vec = match end {
            Some(end) => end.chars().collect(),
            None => vec![self.num2sym[self.num2sym.len() - 1].clone(); start_vec.len() + 6],
        };

        assert!(
            !start_vec.is_empty(),
            "Empty start string not supported. Use None instead."
        );
        assert!(
            !end_vec.is_empty(),
            "Empty end string not supported. Use None instead."
        );

        let num_divisions = n.get() + 1;

        let ad = self.chars_to_digits(start_vec);
        let bd = self.chars_to_digits(end_vec);
        let mut intermediate_digits =
            long_linspace(ad.clone(), bd.clone(), self.base, n, num_divisions)?;

        let mut final_digits = vec![];

        for digit_items in &mut intermediate_digits {
            let mut v = digit_items.result.clone();
            v.extend(self.round_fraction(
                digit_items.remainder,
                digit_items.denominator,
                Some(self.base),
            )?);
            final_digits.push(v);
        }

        final_digits.insert(0, ad);
        final_digits.push(bd);

        final_digits = chop_successive_digits(&mut final_digits);

        let slice = &final_digits[1..final_digits.len() - 1];

        let strings = slice
            .iter()
            .map(|v| self.digits_to_string(&v))
            .collect::<Vec<_>>();

        Ok(strings)
    }

    /// Generate a single intermediary string between `start` and `end` that is centrally placed
    /// in lexicographic order as determined by the symbols in the SymbolTable. The string
    /// is represented in the current base of the SymbolTable.
    ///
    /// # Arguments
    ///
    /// * `start` - An optional string as a lower bound for the resulting string. If None,
    /// this will default to the first character in the SymbolTable.
    /// * `end` - An optional string as an upper bound for the resulting string. If None,
    /// this will default to a string consisting of k+6 instances of the last character in the SymbolTable
    /// where k is the length of the `start` string.
    ///
    /// # Returns
    ///
    /// A result containing a single string if successful, or an error message if unable to create a string.
    pub fn mudder_one(
        &self,
        start: Option<&str>,
        end: Option<&str>,
    ) -> Result<String, &'static str> {
        let strings = self.mudder(1, start, end)?;
        Ok(strings[0].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn all_less_than(strings: &Vec<String>) {
        for i in 0..strings.len() - 1 {
            assert!(strings[i] < strings[i + 1]);
        }
    }

    #[test]
    fn numeric_basics() {
        let table = SymbolTable::decimal();
        let res = table.mudder_one(Some("1"), Some("2")).unwrap();

        assert_eq!(res, "15");
    }

    #[test]
    fn repeated_subdivision() {
        let right = 'z';

        for _ in 0..50 {
            let newr = SymbolTable::alphabetic()
                .mudder_one(Some("a"), Some(&right.to_string()))
                .unwrap();
            assert_ne!('a', newr.chars().next().unwrap());
            assert_ne!(right, newr.chars().next().unwrap());
        }
    }

    #[test]
    fn no_start() {
        let table = SymbolTable::alphabetic();
        let res = table.mudder_one(None, Some("z")).unwrap();

        assert_eq!(res, "m");
    }

    #[test]
    fn no_end() {
        let table = SymbolTable::alphabetic();
        let res = table.mudder_one(Some("z"), None).unwrap();

        assert_eq!(res, "zm");
    }

    #[test]
    fn alphabetic_basics() {
        let table = SymbolTable::alphabetic();
        let res = table.mudder_one(Some("a"), Some("b")).unwrap();

        assert_eq!(res, "an");
    }

    #[test]
    fn base36_basics() {
        let table = SymbolTable::base36();
        let res = table.mudder_one(Some("1"), Some("2")).unwrap();

        assert_eq!(res, "1i");
    }

    #[test]
    fn base62_basics() {
        let table = SymbolTable::base62();
        let res = table.mudder_one(Some("1"), Some("2")).unwrap();

        assert_eq!(res, "1V");
    }

    #[test]
    fn empty_start_and_end() {
        let table = SymbolTable::decimal();
        let res = table.mudder_one(None, None).unwrap();

        assert_eq!(res, "4");
    }

    #[test]
    fn same_start_and_end() {
        let table = SymbolTable::decimal();
        let res = table.mudder_one(Some("1"), Some("1"));

        assert!(res.is_err());
    }

    #[test]
    fn chars_to_digits_test() {
        let table = SymbolTable::alphabetic();
        let res = table.chars_to_digits(vec!['a', 'b', 'c']);

        assert_eq!(res, vec![0, 1, 2]);
    }

    #[test]
    fn digits_to_string_test() {
        let table = SymbolTable::alphabetic();
        let res = table.digits_to_string(&[0, 1, 2]);

        assert_eq!(res, "abc");
    }

    #[test]
    fn number_to_string_test() {
        let table = SymbolTable::decimal();
        let res = table.number_to_string(123);

        assert_eq!(res, "123");
    }

    #[test]
    fn string_to_number_test() {
        let table = SymbolTable::decimal();
        let res = table.string_to_number(vec!['1', '2', '3']);

        assert_eq!(res, 123);
    }

    #[test]
    fn number_to_digits_test() {
        let table = SymbolTable::decimal();
        let res = table.number_to_digits(123);

        assert_eq!(res, vec![1, 2, 3]);
    }

    #[test]
    fn test_single_digit_to_string() {
        let table = SymbolTable::decimal();
        let res = table.digits_to_string(&[5]);

        assert_eq!(res, "5");
    }

    #[test]
    fn test_zero_number_to_string() {
        let table = SymbolTable::decimal();
        let res = table.number_to_string(0);

        assert_eq!(res, "0");
    }

    #[test]
    fn test_single_digit_string_to_number() {
        let table = SymbolTable::decimal();
        let res = table.string_to_number(vec!['5']);

        assert_eq!(res, 5);
    }

    #[test]
    fn test_zero_number_to_digits() {
        let table = SymbolTable::decimal();
        let res = table.number_to_digits(0);

        assert_eq!(res, vec![0]);
    }

    #[test]
    fn test_invalid_chars_to_digits() {
        let table = SymbolTable::decimal();
        let res = table.chars_to_digits(vec!['a']);

        assert_eq!(res, vec![]);
    }

    #[test]
    fn test_empty_digit_to_string() {
        let table = SymbolTable::decimal();
        let res = table.digits_to_string(&[]);

        assert_eq!(res, "");
    }

    #[test]
    fn test_empty_number_to_string() {
        let table = SymbolTable::decimal();
        let res = table.number_to_string(0);

        assert_eq!(res, "0");
    }

    #[test]
    fn test_new() {
        let symbols = vec!['a', 'b', 'c'];
        let table = SymbolTable::new(symbols.clone());
        assert_eq!(table.num2sym, symbols);
        assert_eq!(
            table.sym2num,
            [('a', 0), ('b', 1), ('c', 2)].iter().cloned().collect()
        );
        assert_eq!(table.base, 3);
    }

    #[test]
    fn test_mudder_multiple_strings() {
        let table = SymbolTable::base62();
        let res = table.mudder(3, Some("1"), Some("z")).unwrap();
        assert_eq!(res, vec!["G", "V", "k"]);
    }

    #[test]
    fn test_fine_divisions() {
        let fine = SymbolTable::decimal().mudder(100, Some("9"), None).unwrap();
        let partial_fine = SymbolTable::decimal().mudder(101, Some("9"), None).unwrap();
        let coarse = SymbolTable::decimal().mudder(5, Some("9"), None).unwrap();

        all_less_than(&fine);
        all_less_than(&partial_fine);
        all_less_than(&coarse);
    }

    #[test]
    fn test_default_end() {
        let a = SymbolTable::base36()
            .mudder_one(Some(&"z".repeat(10)), None)
            .unwrap();
        let b = SymbolTable::base36()
            .mudder_one(Some(&"z".repeat(15)), None)
            .unwrap();
        assert!(a < b);
    }
}
