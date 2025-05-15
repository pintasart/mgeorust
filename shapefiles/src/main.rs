use gdal::Dataset;
use gdal::vector::LayerAccess;
use std::io;
use std::path::Path;

fn listar_atributos_shapefile(caminho_ficheiro: &str) -> Result<(), String> {
    let path = Path::new(caminho_ficheiro);
    let dataset = Dataset::open(path).map_err(|e| format!("Erro ao abrir ficheiro {}", e));

    //if dataset.unwrap().layer_count()

    let dataset = dataset.unwrap();

    if dataset.layer_count() == 0 {
        return Err("Shapefila não contém nehuma camada.".to_string());
    }

    let layer = dataset
        .layer(0)
        .map_err(|e| format!("Erro a aceder à camada {}", e))?;
    let feature_defn = layer.defn();
    let attribute_count = feature_defn.fields().count();

    println!("Atributos y Tipos del Archivo Shapefile:");
    // for i in 0..attribute_count {
    //     let field_defn = feature_defn.fields();

    //        let attribute_name = field_defn..name();
    //     let attribute_type = field_defn.field_type();
    //     println!("- {}: {:?}", attribute_name, attribute_type);
    // }
    Ok(())
}

fn main() {
    println!("Intriduzir caminho do shp:");
    let mut caminho = String::new();
    io::stdin()
        .read_line(&mut caminho)
        .expect("Falha a ler a linha");
    let caminho = caminho.trim();

    match listar_atributos_shapefile(caminho) {
        Ok(_) => (),
        Err(e) => eprintln!("Erro: {}", e),
    }
}
