use std::collections::{HashMap, HashSet};

pub fn part_two(input: &str) -> String {
    let mut connection_map: HashMap<&str, HashSet<&str>> = HashMap::new();

    input.lines().for_each(|line| {
        let mut parts = line.split('-');
        let (comp1, comp2): (&str, &str) = (parts.next().unwrap(), parts.next().unwrap());
        connection_map
            .entry(comp1)
            .or_insert_with(HashSet::new)
            .insert(comp2);

        connection_map
            .entry(comp2)
            .or_insert_with(HashSet::new)
            .insert(comp1);
    });
    let mut cliques: Vec<&str> = get_cliques(&connection_map);
    cliques.sort();
    cliques.join(",")
}

fn get_cliques<'a>(connection_map: &'a HashMap<&'a str, HashSet<&'a str>>) -> Vec<&'a str> {
    let mut biggest_clique: HashSet<&str> = HashSet::new();

    let mut previously_seen: HashSet<&str> = HashSet::new();

    for &computer in connection_map.keys() {
        if !previously_seen.contains(computer) {
            let mut current_clique: HashSet<&str> = HashSet::new();
            dfs(
                connection_map,
                computer,
                &mut previously_seen,
                &mut current_clique,
            );
            if current_clique.len() >= biggest_clique.len() {
                biggest_clique = current_clique;
            }
        }
    }

    biggest_clique.iter().cloned().collect::<Vec<&str>>()
}

fn dfs<'a>(
    connection_map: &'a HashMap<&'a str, HashSet<&'a str>>,
    computer: &'a str,
    previously_seen: &mut HashSet<&'a str>,
    current_clique: &mut HashSet<&'a str>,
) {
    previously_seen.insert(computer);
    current_clique.insert(computer);

    if let Some(connected_computers) = connection_map.get(computer) {
        for connected_computer in connected_computers {
            if !previously_seen.contains(connected_computer) {
                if current_clique.iter().all(|&clique_computer| {
                    connection_map[connected_computer].contains(clique_computer)
                }) {
                    dfs(
                        connection_map,
                        connected_computer,
                        previously_seen,
                        current_clique,
                    );
                }
            }
        }
    }
}
