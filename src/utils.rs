use std::collections::HashMap;

pub fn get_classes() -> HashMap<usize, &'static str> {
    let classes = [
        "A10",
        "A400M",
        "AG600",
        "AV8B",
        "B1",
        "B2",
        "B52",
        "Be200",
        "C130",
        "C17",
        "C5",
        "E2",
        "EF2000",
        "F117",
        "F14",
        "F15",
        "F16",
        "F18",
        "F22",
        "F35",
        "F4",
        "J20",
        "JAS39",
        "MQ9",
        "Mig31",
        "Mirage2000",
        "RQ4",
        "Rafale",
        "SR71",
        "Su34",
        "Su57",
        "Tornado",
        "Tu160",
        "Tu95",
        "U2",
        "US2",
        "V22",
        "Vulcan",
        "XB70",
        "YF23",
    ];
    let mut classes_lu = HashMap::new();
    classes.iter().copied().enumerate().for_each(|(i, val)| {
        classes_lu.insert(i, val);
    });
    return classes_lu;
}
