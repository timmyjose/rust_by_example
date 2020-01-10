// note how the types that collect uses affects the bahaviour of the entire operation

fn main() {
    normal_approach();
    filter_map_approach();
    fail_entire_operation_with_collect();
    collect_values_and_errors_separately();
}

fn normal_approach() {
    let strings = vec!["tofu", "1", "2", "3", "4", "5"];

    let numbers = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect::<Vec<_>>();

    println!("[normal approach] {:?}", numbers);
}

fn filter_map_approach() {
    let strings = vec!["tofu", "1", "2", "3", "4", "5"];

    let numbers = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    println!("[filter_map_approach] {:?}", numbers);
}

fn fail_entire_operation_with_collect() {
    let strings = vec!["tofu", "1", "2", "3", "4", "5"];

    let numbers = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<_>, _>>();

    println!("[fail_entire_operation_with_collect] {:?}", numbers);
}

fn collect_values_and_errors_separately() {
    let strings = vec!["tofu", "1", "2", "3", "4", "5"];

    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);

    let numbers = numbers.into_iter().map(Result::unwrap).collect::<Vec<_>>();
    let errors = errors
        .into_iter()
        .map(Result::unwrap_err)
        .collect::<Vec<_>>();

    println!(
        "[collect_values_and_errors_separately] Numbers = {:?}",
        numbers
    );
    println!(
        "[collect_values_and_errors_separately] Errors = {:?}",
        errors
    );
}
