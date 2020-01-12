use std::thread;

fn main() {
    let data = "86967897737416471853297327050364959
                11861322575564723963297542624962850
                70856234701860851907960690014725639
                38397966707106094172783238747669219
                52380795257888236525459303330302837
                58495327135744041048897885734297812
                69920216438980873548808413720956532
                16278424637452589860345374828574668";

    let mut handles = Vec::new();

    for (idx, data_segment) in data.split_whitespace().enumerate() {
        handles.push(thread::spawn(move || -> u32 {
            let res = data_segment
                .chars()
                .map(|c| c.to_digit(10).expect("not a digit"))
                .sum();

            println!("Segment {} processed with intermediate sum {}", idx, res);
            res
        }));
    }

    let mut total = 0;
    for handle in handles {
        total += handle.join().unwrap();
    }

    println!("Total = {}", total);
}
