
const PRIMES_TO: usize = 10000;

///
/// Calculate all primes up to a max usize
/// Uses the Sieve of Eratosthenes algorithm 
/// 
/// #val - the max usize
/// 
/// #Panic - val must be greater than 1
/// 
fn all_primes_to(val: usize) -> Vec<usize>
{
    // panic if val does not fit parameters
    if val <= 1
    {
        panic!("Value must be greater than 1.")
    }

    // Prime list, which is returned at the end of function
    let mut primes = Vec::<usize>::new();

    // A list of boolean values, which represent each number being
    // checked. A true value means the number is NOT prime
    let mut checked_values = Vec::<bool>::with_capacity(val);
    for _ in 0..val { checked_values.push(false); }

    // Set max threshold, which is the sqrt of val. Numbers
    // after max do not need to be checked since all non-primes have
    // already been flagged in previous iterations.
    let max = (val as f32).sqrt() as usize;

    for i in 2..=max
    {
        // If number has already been flagged as not prime, continue
        if checked_values[i] { continue; }
        else
        {
            primes.push(i);
            // Set a base, which index aggregates by
            // and a second counter (i^2), which will be
            // increase until val is reached
            let mut j = i * i;
            while j < val
            {
                // set each value to true, ie. non-prime
                checked_values[j] = true;
                j += i;
            } 
        }  
    }

    // Finally, take all non-flagged bools, pushing
    // their indices onto returned prime collection
    for i in max..val
    {
        if !checked_values[i] 
        {
            primes.push(i);
        }
    }

    primes
}

fn main() 
{
    let primes = all_primes_to(PRIMES_TO);
    println!("All primes to {}: {:?}, len = {}", PRIMES_TO, primes, primes.len());
}
