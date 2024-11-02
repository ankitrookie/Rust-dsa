pub fn palindromenum(mut n: usize) -> bool {
    let mut digit = 0;
    let dup = n;

    while n > 0 {
        let ld = n % 10;

        digit = (digit * 10) + ld;

        n = n / 10;
    }
    if digit == dup {
        return true;
    } else {
        return false;
    }
}
// Code Explanation

/* First Iteration:
n = 123
ld = 123 % 10 → ld = 3
digit = (0 * 10) + 3 → digit = 3
n = 123 / 10 → n = 12

Second Iteration:
n = 12
ld = 12 % 10 → ld = 2
digit = (3 * 10) + 2 → digit = 32
n = 12 / 10 → n = 1

Third Iteration:
n = 1
ld = 1 % 10 → ld = 1
digit = (32 * 10) + 1 → digit = 321
n = 1 / 10 → n = 0 */
