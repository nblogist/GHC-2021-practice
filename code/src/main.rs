#![allow(
    dead_code,
    unused_variables,
    unused_parens,
    unused_imports,
    unused_mut,
    unused_must_use
)]
use num_format::{Locale, ToFormattedString};
use simple_error::bail;
use std::cmp::min;
use std::collections::HashSet;
use std::time::Instant;
use std::{env, error::Error, fs::File, io::BufRead, io::BufReader, io::Read, io::Write};

// Utility Functions
fn get_filename() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file_number: u16 = args[1].parse()?;

    match file_number {
        0 => Ok("../../data/input/a_example.txt".to_owned()),
        1 => Ok("../../data/input/b_little_bit_of_everything.txt".to_owned()),
        2 => Ok("../../data/input/c_many_ingredients.txt".to_owned()),
        3 => Ok("../../data/input/d_many_pizzas.txt".to_owned()),
        4 => Ok("../../data/input/e_many_teams.txt".to_owned()),
        _ => bail!("Incorrect file number specified."),
    }
}

fn read_input(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(file_path).expect("file not found");
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim().parse()?;
        lines.push(line);
    }

    Ok(lines)
}

fn parse_input_data(input_data: &Vec<String>) -> Result<(PizzasInput), Box<dyn Error>> {
    let mut line01 = input_data[0].split_whitespace();

    let total_pizzas = line01.next().unwrap().parse()?;
    let t2 = line01.next().unwrap().parse()?;
    let t3 = line01.next().unwrap().parse()?;
    let t4 = line01.next().unwrap().parse()?;

    // vector of [pizzaNumber:u32 as string,totalIngredients:i32 as string, ...ingredient names as strings separated]
    let mut pizzas_ingredients: Vec<Vec<String>> = vec![];
    for index in 1..total_pizzas as usize {
        // leaving out first index as it tells about total pizzas and number of teams
        let mut ingredients: Vec<String> = input_data[index]
            .split_whitespace()
            .map(|x| x.to_string())
            .collect();
        ingredients.insert(0, (index - 1).to_string()); //giving numbers to pizzas useful to identify in output
        pizzas_ingredients.push(ingredients);
    }

    let pizzas_input = PizzasInput {
        total_pizzas,
        pizzas_ingredients,
        t2,
        t3,
        t4,
    };

    Ok((pizzas_input))
}

fn write_output(file_path: &str, pizza_outputs: &PizzasOutput) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(file_path).expect("unable to create file");
    let total_deliveries = pizza_outputs.total_deliveries;
    let delivery_details = pizza_outputs.delivery_details.clone();

    file.write_all(format!("{}\n", total_deliveries).as_bytes())?;
    for delivery_number in 0..pizza_outputs.total_deliveries as usize {
        let pizzas_delivered: Vec<String> = delivery_details[delivery_number]
            .pizza_code_numbers
            .iter()
            .map(|x| x.to_string())
            .collect();
        file.write_all(
            format!(
                "{} {}\n",
                delivery_details[delivery_number].team_type.to_string(),
                pizzas_delivered.join(" ")
            )
            .as_bytes(),
        )?;
    }
    Ok(())
}

#[derive(Clone, Debug)]
struct PizzasInput {
    total_pizzas: u32,
    pizzas_ingredients: Vec<Vec<String>>,
    t2: u32,
    t3: u32,
    t4: u32,
}
#[derive(Clone, Debug)]
struct PizzasOutput {
    total_deliveries: u32,
    delivery_details: Vec<Deliveries>,
}
#[derive(Clone, Debug)]
struct Deliveries {
    team_type: u32,
    pizza_code_numbers: Vec<u32>,
}

fn main() {
    let start = Instant::now();
    let filename = get_filename().unwrap();
    let input_data = read_input(&filename).unwrap();
    let (pizzas_input) = parse_input_data(&input_data).unwrap();

    println!("{:?}", pizzas_input);

    // !TODO processing

    // Saving Data
    let deliveries = Deliveries {
        team_type: 2,
        pizza_code_numbers: vec![5, 5],
    };
    let deliveries2 = Deliveries {
        team_type: 3,
        pizza_code_numbers: vec![5, 5, 9],
    };
    let pizzas_output = PizzasOutput {
        total_deliveries: 2,
        delivery_details: vec![deliveries, deliveries2],
    };

    // Output file
    let filename_output = filename.replace(".txt", &format!("_OUTPUT.txt"));
    write_output(&filename_output, &pizzas_output).expect("Failed to write output.");

    let end = Instant::now();
    println!("Benchmark = {:?}", (end - start).as_millis());
}
