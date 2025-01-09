use std::{error::Error, result};
use csv::Reader;
fn main()->Result<(), Box<dyn Error>> {

    let mut reader=Reader::from_path("./src/Automobile.csv")?;

    let mut count=0;
    for result in reader.records()
    {
        let record = result?;
        // println!("{:?}",record);
        // println!("Car Name is {}",record[0].to_string());
        // println!("Carr name {} and Model Year is 19{}",record[0].to_string(),record[7].to_string());

       
        // if record[7].to_string()<"71".to_string()
        // {
        //     println!("Carr name {} and Model Year is 19{}",record[0].to_string(),record[7].to_string());
        //     count=count+1;
        // }

        if record[8].to_string() == "usa"
        {
            println!("Car From USA {}",record[0].to_string());
            count=count+1;
        }
    }
    println!("Total Records are {}",count);

    Ok(())
}


