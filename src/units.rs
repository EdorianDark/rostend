use ini::ini::Properties;
use ini::Ini;
use std::collections::HashMap;

/// the properties of a unit
struct Unit {
    description: String,
    before: String, // if both units are started, this is started before
    after: String,  // if both units are started, this is started after
    wants: String,  // depends on other service
}

/// the type of a service
pub enum ServiceType {
    Simple,
    Forking,
    OneShot,
    Notify,
    DBus,
    Idle,
}

/// the properties of a service
pub struct Service {
    unit: Unit,
    service_type: ServiceType,
    exec_start: String,
}

fn parse_unit(mut properties: Properties) -> Unit {
    let description = properties.remove("Description").unwrap_or("".to_string());
    let before = properties.remove("Before").unwrap_or("".to_string());
    let after = properties.remove("After").unwrap_or("".to_string());
    let wants = properties.remove("Wants").unwrap_or("".to_string());

    if !properties.is_empty() {
        panic!("Unit has unrecognized options {:?}", properties);
    }

    Unit {
        description,
        before,
        after,
        wants,
    }
}

fn parse_service(mut properties: Properties, unit: Unit) -> Service {
    let exec_start = properties.remove("ExecStart").unwrap_or("".to_string());
    let service_type = properties.remove("Type").unwrap_or("Simple".to_string());
    let service_type = match service_type.as_ref() {
        "Simple" => ServiceType::Simple,
        "Forking" => ServiceType::Forking,
        "OneShot" => ServiceType::OneShot,
        "Notify" => ServiceType::Notify,
        "DBus" => ServiceType::DBus,
        "Idle" => ServiceType::Idle,
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

    let section = conf.section(Some("Unit")).unwrap();
    let unit = parse_unit(section.clone());

    let service = conf.section(Some("Service")).unwrap();
    parse_service(service.clone(), unit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_unit_empty() {
        let properties = HashMap::new();
        let unit = parse_unit(properties);
        assert_eq!(unit.description, "");
    }

    #[test]
    fn test_parse_unit_description() {
        let mut properties = HashMap::new();
        properties.insert("Description".to_string(), "test".to_string());
        let unit = parse_unit(properties);
        assert_eq!(unit.description, "test");
    }

    #[test]
    #[should_panic]
    fn test_parse_unit_invalid() {
        let mut properties = HashMap::new();
        properties.insert("Invalid".to_string(), "test".to_string());
        let _unit = parse_unit(properties);
    }
}
