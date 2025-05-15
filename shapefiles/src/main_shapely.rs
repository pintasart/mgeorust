use shapefile::Reader;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let shapefile_path =
        "/data/data/com.termux/files/home/rustProjects/mgeorust/data/toponimia/Toponimia200k.shp";

    //abrir fich shapefile
    let mut reader = Reader::from_path(shapefile_path)?;

    //obter atributos e tipos
    for result in reader.iter_shapes_and_records() {
        let (shape, record) = result.unwrap();
        println!("Shape:{}, records: ", shape);
        for (name, field) in record {
            println!("\t{}: {:?}", name, field);
        }
        break; //apenas o primeiro registo
        println!();
    }
    Ok(())
}
