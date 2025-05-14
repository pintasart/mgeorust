use std::io;
use std::path::Path;
use gdal::Dataset;

fn listar_atributos_shapefile(caminho_ficheiro: &str) -> Result<(),String>{
    
    let path = Path::new(caminho_ficheiro);
    let dataset = Dataset::open(path).map_err(|e| format!("Erro ao abrir ficheiro {}",e));

    if dataset.layer_count() == 0 {
        return Err("Shapefila não contém nehuma camada.".to_string())}

    let layer = dataset.layer(0).mao_err(|e| format!("Erro a aceder à camada {}",e))?;
    let feature_dfn = layer.dfn();
    let attribute_count = feature_dfn.field_count();

    println!("Atributos e Tipo de Ficheiro Shapefile:");
    for i in 0..atribute_count {
        let field_dfn = feature_dfn.field_dfn(i);
        let attribute_name = field_dfn.name();
        let attribute_type = field_dfn.field_type();
        println!("- {}: {:?}", attrinute_name, attribute_type);
    }

    Ok(())
}

fn main(){
    
    println!("Intriduzir caminho do shp:");
    let mut caminho= String::new();
    io::stdin()
        .read_line(&mut caminho)
        .expect("Falha a ler a linha");
    let caminho= caminho.trim();

    match listar_atributos_shapefile(caminho){
    
        Ok(_) => (),
        Err(e) => eprintln("Erro: {}",e),
    }

}
