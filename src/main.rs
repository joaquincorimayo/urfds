use polars::prelude::*;

fn main() {
    let df: DataFrame = get_data();
    let column_name: &str = "string";
    let out: DataFrame = get_by_column_name(&df, column_name);

    println!("{}", df);
    println!("\nColumn name: {} - results {}", column_name, out);
}

fn get_by_column_name(df: &DataFrame, column_name: &str) -> DataFrame {
    df.clone().lazy().select([col(column_name)]).collect().unwrap()
}

fn get_data() -> DataFrame {
    df!(
        "integer" => &[1, 2, 3],
        "float" => &[4.0, 5.0, 6.0],
        "string" => &["a", "b", "c"],
   ).unwrap().with_row_index("index", None).unwrap()
}
