use proc_macro::TokenStream;

/// Given a token stream of day IDs (e.g., `1, 2, 7, 25`), create a `vec!` invocation
/// containing runners for each of those days.
#[proc_macro]
pub fn days(item: TokenStream) -> TokenStream {
    let days: Vec<_> = item
        .to_string()
        .split_terminator(",")
        .map(|s| {
            // Take a day id (e.g., 1, 2, 10, 25) and create a string representing a
            // token stream to create a runner for that particular day.
            let day_num = s.trim().parse::<u8>().unwrap();
            format!(
                "Day {{
                    num: {id},
                    p1: days::d{id:0>2}::part1,
                    p2: days::d{id:0>2}::part2,
                    input: \"{crate_name}/input/{id}\"
                }}",
                id = day_num,
                crate_name = std::env::var("CARGO_PKG_NAME").unwrap()
            )
        })
        .collect();

    format!("vec![{}]", days.join(",")).parse().unwrap()
}
