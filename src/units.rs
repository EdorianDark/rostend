use ini::ini::Properties;
use ini::Ini;
use std::path::Path;
use std::io;
use std::fs::{self, DirEntry};
use std::fs::File;
use std::cmp::Ordering;

/// the properties of a unit
struct Unit {
    name: String,
    description: Option<String>,
    before: Option<String>, // if both units are started, this is started before
    after: Option<String>,  // if both units are started, this is started after
    wants: Option<String>,  // depends on other service
}

/// the type of a service
pub enum ServiceType {
    Simple,
    //Forking,
    //OneShot,
    //Notify,
    //DBus,
    //Idle,
}

/// the properties of a service
pub struct Service {
    unit: Unit,
    service_type: ServiceType,
    exec_start: Option<String>,
}

fn parse_unit(mut properties: Properties, name: &str) -> Unit {
    let description = properties.remove("Description");
    let before = properties.remove("Before");
    let after = properties.remove("After");
    let wants = properties.remove("Wants");

    if !properties.is_empty() {
        panic!("Unit has unrecognized options {:?}", properties);
    }

    let name = name.to_string();
    Unit {
        description,
        before,
        after,
        wants,
        name,
    }
}

fn parse_service(mut properties: Properties, unit: Unit) -> Service {
    let exec_start = properties.remove("ExecStart");
    let service_type = properties.remove("Type").unwrap_or("Simple".to_string());
    let service_type = match service_type.as_ref() {
        "Simple" => ServiceType::Simple,
        //"Forking" => ServiceType::Forking,
        //"OneShot" => ServiceType::OneShot,
        //"Notify" => ServiceType::Notify,
        //"DBus" => ServiceType::DBus,
        //"Idle" => ServiceType::Idle,
        _ => panic!("Service type is unrecognized {}", service_type),
    };

    if !properties.is_empty() {
        panic!("Service has unrecognized options {:?}", properties);
    }

    Service {
        unit,
        exec_start,
        service_type,
    }
}

pub fn parse(file: &str) -> Service {
    let conf = Ini::load_from_str(file).unwrap();
    let name : String = file.split(".").collect::<Vec<&str>>().get(1).unwrap().to_string();

    let section = conf.section(Some("Unit")).unwrap();
    let unit = parse_unit(section.clone(), &name);

    let service = conf.section(Some("Service")).unwrap();
    parse_service(service.clone(), unit)
}

fn sort_services(a: &Service, b: &Service)-> Ordering{
    /*if Some(&a.unit.name) == b.unit.before{
        return Ordering::Less;
    }
    if Some(&b.unit.name) == a.unit.before{
        return Ordering::Greater;
    }
    if Some(&a.unit.name) == b.unit.after{
        return Ordering::Greater;
    }
    if Some(&b.unit.name) == a.unit.after{
        return Ordering::Less;
    }*/
    a.unit.name.cmp(&b.unit.name)
}

pub fn parse_dir(path: &str) -> Vec<Service> {
    let dir = Path::new(path);
    let mut vec = Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
        let dir = entry.unwrap();
        dir.file_name().to_str().unwrap().ends_with(".service");
        let file: String = fs::read_to_string(dir.path()).ok().unwrap();
        vec.push(parse(&file));
    }
    if vec.is_empty(){
        panic!("could not find .service files in {}", dir.display());
    }
    vec.sort_by(sort_services);
    vec
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_parse_unit_empty() {
        let properties = HashMap::new();
        let unit = parse_unit(properties, "");
        assert_eq!(unit.description, None);
    }

    #[test]
    fn test_parse_unit_description() {
        let mut properties = HashMap::new();
        properties.insert("Description".to_string(), "test".to_string());
        let unit = parse_unit(properties, "");
        assert_eq!(unit.description, Some("test".to_string()));
    }

    #[test]
    #[should_panic]
    fn test_parse_unit_invalid() {
        let mut properties = HashMap::new();
        properties.insert("Invalid".to_string(), "test".to_string());
        let _unit = parse_unit(properties, "");
    }
}
