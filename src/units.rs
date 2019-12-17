use ini::ini::Properties;
use ini::Ini;
use std::path::Path;
use std::io;
use std::fs::{self, DirEntry};
use std::fs::File;
use std::cmp::Ordering;

/// the properties of a unit
#[derive(Eq, PartialEq)]
struct Unit {
    name: String,
    description: Option<String>,
    before: Option<String>, // if both units are started, this is started before
    after: Option<String>,  // if both units are started, this is started after
    wants: Option<String>,  // depends on other service
}

impl PartialOrd for Unit {
    fn partial_cmp(&self, other: &Unit) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Unit {
    fn cmp(&self, other: &Unit) -> Ordering {
        if self.after.is_some()
            && self.after.as_ref().unwrap() == &other.name{
                Ordering::Less
        }
        else if other.after.is_some()
            && other.after.as_ref().unwrap() == &self.name {
                Ordering::Greater
        }
        else if self.before.is_some()
            && self.before.as_ref().unwrap() == &other.name {
                Ordering::Greater
        }
        else if other.before.is_some()
            && other.before.as_ref().unwrap() == &self.name{
                Ordering::Less
        }
        else if self.wants.is_some()
            && self.wants.as_ref().unwrap() == &other.name {
                Ordering::Greater
        }
        else if other.wants.is_some()
            && other.wants.as_ref().unwrap() == &self.name{
                Ordering::Less
        }
        else {
            self.name.cmp(&other.name)
        }
    }
}

/// the type of a service
#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum ServiceType {
    Simple,
    //Forking,
    //OneShot,
    //Notify,
    //DBus,
    //Idle,
}

/// the properties of a service
#[derive(Ord, PartialOrd, Eq, PartialEq)]
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
    vec.sort();
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
    fn new_service(name: String) -> Service{
        Service{
            service_type: ServiceType::Simple,
            exec_start: None,
            unit : Unit{
                after: None,
                before: None,
                name,
                description: None,
                wants: None,
            }
        }
    }

    #[test]
    fn test_service_order_before(){
        let a = new_service("A".to_string());
        let mut b = new_service("B".to_string());
        b.unit.before = Some("A".to_string());
        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(b.cmp(&a), Ordering::Greater);
    }

    #[test]
    fn test_service_order_after(){
        let a = new_service("A".to_string());
        let mut b = new_service("B".to_string());
        b.unit.after = Some("A".to_string());
        assert_eq!(a.cmp(&b), Ordering::Greater);
        assert_eq!(b.cmp(&a), Ordering::Less);
    }

    #[test]
    fn test_service_order_wants(){
        let a = new_service("A".to_string());
        let mut b = new_service("B".to_string());
        b.unit.wants = Some("A".to_string());
        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(b.cmp(&a), Ordering::Greater);
    }
}
