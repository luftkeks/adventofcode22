#[derive(Copy, Clone, Debug)]
struct Compartment {
    start: i32,
    end: i32,
}

#[derive(Copy, Clone, Debug)]
struct Etage {
    comp1: Compartment,
    comp2: Compartment,
}

fn does_include(eta: Etage) -> bool {
    (eta.comp1.start <= eta.comp2.start && eta.comp1.end >= eta.comp2.end)
        || (eta.comp1.start >= eta.comp2.start && eta.comp1.end <= eta.comp2.end)
}

fn is_in_compartment(comp: &Compartment, number: i32) -> bool {
    return number >= comp.start && number <= comp.end;
}

fn number_overlap(eta: Etage) -> i32 {
    let mut count = 0;
    for ii in eta.comp1.start..=eta.comp1.end {
        if is_in_compartment(&eta.comp2, ii) {
            count += 1;
        }
    }
    return count;
}

pub fn day4() {
    let input = std::fs::read_to_string("input4.txt").unwrap();
    let mut counter = 0;
    let mut overlap = 0;
    for line in input.lines() {
        let comparts: Vec<&str> = line.split(",").collect();

        let one: Vec<&str> = comparts[0].split("-").collect();
        let two: Vec<&str> = comparts[1].split("-").collect();

        let eta = Etage {
            comp1: Compartment {
                start: one[0].parse::<i32>().unwrap(),
                end: one[1].parse::<i32>().unwrap(),
            },
            comp2: Compartment {
                start: two[0].parse::<i32>().unwrap(),
                end: two[1].parse::<i32>().unwrap(),
            },
        };
        if does_include(eta) {
            counter += 1;
        }
        if number_overlap(eta) > 0 {
            overlap += 1
        }
    }
    println!("Day4: Number of including Comps is {counter}");
    println!("Day4: Number of overlapping Comp is {overlap}");
}
