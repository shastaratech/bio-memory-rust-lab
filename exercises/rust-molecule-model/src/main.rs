use rust_molecule_model::molecule::{molecule_by_name, molecule_names};
use rust_molecule_model::{Describe, Molecule};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() || args[0] == "help" || args[0] == "--help" {
        print_help();
        return;
    }

    if args[0] == "list" {
        println!("{}", molecule_names().join(", "));
        return;
    }

    let molecule_name = &args[0];
    let Some(molecule) = molecule_by_name(molecule_name) else {
        eprintln!("unknown molecule: {molecule_name}");
        eprintln!("available molecules: {}", molecule_names().join(", "));
        std::process::exit(1);
    };

    let command = args.get(1).map(String::as_str).unwrap_or("summary");

    match command {
        "summary" => print_summary(molecule_name, &molecule),
        "formula" => println!("{}", molecule.formula()),
        "atoms" => print_atoms(&molecule),
        "bonds" => print_bonds(&molecule),
        "neighbors" => {
            let atom_id = parse_atom_id(&args, 2);
            println!("{:?}", molecule.neighbors(atom_id));
        }
        "path" => {
            let start = parse_atom_id(&args, 2);
            let goal = parse_atom_id(&args, 3);
            match molecule.shortest_path(start, goal) {
                Some(path) => println!("{path:?}"),
                None => println!("no path"),
            }
        }
        "components" => println!("{:?}", molecule.connected_components()),
        "validate" => println!("{}", molecule.validate_bond_indices()),
        _ => {
            eprintln!("unknown command: {command}");
            print_help();
            std::process::exit(1);
        }
    }
}

fn print_help() {
    println!("Rust Molecule Model CLI");
    println!();
    println!("Usage:");
    println!("  cargo run -- list");
    println!("  cargo run -- <molecule> summary");
    println!("  cargo run -- <molecule> formula");
    println!("  cargo run -- <molecule> atoms");
    println!("  cargo run -- <molecule> bonds");
    println!("  cargo run -- <molecule> neighbors <atom_id>");
    println!("  cargo run -- <molecule> path <start_atom_id> <goal_atom_id>");
    println!("  cargo run -- <molecule> components");
    println!("  cargo run -- <molecule> validate");
    println!();
    println!("Molecules:");
    println!("  {}", molecule_names().join(", "));
    println!();
    println!("Examples:");
    println!("  cargo run -- water summary");
    println!("  cargo run -- ethanol formula");
    println!("  cargo run -- ethanol neighbors 1");
    println!("  cargo run -- ethanol path 3 8");
}

fn print_summary(name: &str, molecule: &Molecule) {
    println!("{name}: {}", molecule.describe());
    println!("formula: {}", molecule.formula());
    println!("atoms: {}", molecule.atom_count());
    println!("bonds: {}", molecule.bond_count());
    println!("valid bond indices: {}", molecule.validate_bond_indices());
}

fn print_atoms(molecule: &Molecule) {
    for (id, atom) in molecule.atoms().iter().enumerate() {
        println!("{id}: {}", atom.describe());
    }
}

fn print_bonds(molecule: &Molecule) {
    for (id, bond) in molecule.bonds().iter().enumerate() {
        println!("{id}: {}", bond.describe());
    }
}

fn parse_atom_id(args: &[String], index: usize) -> usize {
    let Some(raw) = args.get(index) else {
        eprintln!("missing atom id");
        std::process::exit(1);
    };

    match raw.parse() {
        Ok(atom_id) => atom_id,
        Err(_) => {
            eprintln!("atom id must be a non-negative integer: {raw}");
            std::process::exit(1);
        }
    }
}
