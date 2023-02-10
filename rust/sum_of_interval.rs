fn process(mut vec: Vec<(i32,i32)>) -> Vec<(i32,i32)> {
    let mut intervals = Vec::new();
    
    vec.dedup();
    vec.sort();
    
    for interval in &vec {
        println!("{:?}",interval);
    }
    println!("hi");
    
    for i in 0..vec.len() {
        let mut interval = vec[i].clone();
        
        for j in i..vec.len() {
            let interval2 = &vec[j];

//             println!("C: {:?} | {:?} ", interval2, interval);
            if i != j && (interval.1 > interval2.0) {
                println!("B: {:?} | {:?}",interval, interval2 );

                interval.0 = if interval.0 <= interval2.0 { interval.0} else {interval2.0};
                interval.1 = if interval.1 <= interval2.1 { interval2.0} else {interval.0};
                
                println!("A: {:?} | {:?}", interval, interval2);
            }
        }
        intervals.push(interval);
    }

    for interval in &intervals {
        println!("{:?}",interval);
    }
    println!("done");

    intervals    
}

fn sum_intervals(intervals: &[(i32, i32)]) -> i32 {
    let interval_input = intervals.to_vec();
    
    let output = process(interval_input);
    
    let mut result = 0;

    for interval in &output {
        let l = interval.1 - interval.0;
        result += l;
    }
    
    result
}