pub fn bsmath(mut n: usize) -> i32 {
    let mut _count = 0; // _ this is not nesseccery here, it just conventation I found plesure.
    let original_number = n;

    while n > 0 {
        let digit = n % 10; // Get  the last digit
        n = n / 10; // Remove the last digit

        if digit != 0 && original_number % digit == 0 {
            _count += 1;
        }
    }

    return _count;
}
