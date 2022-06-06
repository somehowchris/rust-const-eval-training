const fn power_of(base: u16, exponent: u32) -> u16 {
    base.pow(exponent)
}

const fn sort<'a, const LEN: usize>(mut numbers: [u8; LEN]) -> [u8; LEN] {
    let mut count = 0;
    loop {
        if count < LEN {
            if count > 0 && numbers[count] < numbers[count - 1] {
                (numbers[count], numbers[count - 1]) = (numbers[count - 1], numbers[count]);
                numbers = sort(numbers);
            }
            count+=1;
        } else {
            break;
        }
    }
    numbers
}

const fn sum<const LEN: usize>(numbers: &[u8; LEN]) -> u8 {
    let mut sum = 0;
    let mut count = 0;
    loop {
        if count < LEN {
            sum+=numbers[count];
            count+=1;
        } else {
            break;
        }
    }
    sum
}

const fn count<const LEN: usize>(_numbers: &[u8; LEN]) -> usize {
    let mut count = 0;
    loop {
        if count < LEN {
            count+=1;
        } else {
            break;
        }
    }
    count
}

const fn count_unique_numbers<const LEN: usize>(numbers: &[u8; LEN]) -> usize {
    let mut unique_values = [None; LEN]; 
    let mut unique_count = 0;
    let mut count = 0;
    'nominate: loop {
        if count < LEN {
            let mut inner_count = 0;
            'detect: loop {
                if inner_count < LEN {
                    if let Some(value)  = unique_values[inner_count] {
                        if value == numbers[count] {
                            break 'detect;
                        }
                    }
                    inner_count+=1;
                } else {
                    unique_values[unique_count] = Some(numbers[count]); 
                    unique_count+=1;
                    break 'detect;
                }
            }
            count+=1;
        } else {
            break 'nominate;
        }
    }
    unique_count
}


const fn unique_numbers<const LEN: usize, const FINAL_LEN: usize>(numbers: &[u8; LEN]) -> [u8; FINAL_LEN] {
    let mut unique_values = [None; LEN]; 
    let mut unique_count = 0;
    let mut count = 0;
    'nominate: loop {
        if count < LEN {
            let mut inner_count = 0;
            'detect: loop {
                if inner_count < LEN {
                    if let Some(value)  = unique_values[inner_count] {
                        if value == numbers[count] {
                            break 'detect;
                        }
                    }
                    inner_count+=1;
                } else {
                    unique_values[unique_count] = Some(numbers[count]); 
                    unique_count+=1;
                    break 'detect;
                }
            }
            count+=1;
        } else {
            break 'nominate;
        }
    }
    let mut data = [0; FINAL_LEN];

    let mut count = 0;
    loop {
        if count < FINAL_LEN {
            if let Some(value) = unique_values[count] {
                data[count] = value;
            }
            count+=1;
        } else {
            break;
        }
    }

    data
}

const DATA: [u8; 7] = [1u8, 12u8, 12u8, 9u8, 6u8, 2u8, 3u8];

const SORTED: [u8; DATA.len()] = sort(DATA);
const SUMMED: u8 = sum(&DATA);
const UNIQUE_NUMBERS_COUNT: usize = count_unique_numbers(&SORTED);
const UNIQUE_NUMBERS: [u8; UNIQUE_NUMBERS_COUNT] = unique_numbers(&SORTED);

fn main() {
    println!("4 ** 4: {}", power_of(4, 4));
    println!("Raw data array: {:?}", DATA);
    println!("sorted data array: {:?}", SORTED);
    println!("summed data:  {:?}", SUMMED);
    println!("# unique_values:  {:?}", UNIQUE_NUMBERS_COUNT);
    println!("unique_values:  {:?}", UNIQUE_NUMBERS);
}
