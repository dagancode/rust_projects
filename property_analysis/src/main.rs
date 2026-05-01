use dotenvy::dotenv;
use property_analysis::services::csv::load_sales_history;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    println!("Welcome to the Property API\n");

    let sales_history_path = std::env::var("SALES_HISTORY_PATH").expect("Failed to load path from .env, ensure SALES_HISTORY_PATH is set. ");

    let sales_history = load_sales_history(&sales_history_path)?;

    println!("Properies with multiple sales: ");
    for detail in &sales_history {
        if detail.sales_history.len() > 1 {
            println!(
                "{:>3} {} = {}",
                detail.property.location.street_number,
                detail.property.location.street_name,
                detail
                    .sales_history
                    .iter()
                    .map(|sh| format!("[{} - R{}] ", sh.year, sh.price))
                    .collect::<String>()
            );
        }
    }

    Ok(())
}

