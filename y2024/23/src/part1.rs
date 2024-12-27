use std::collections::{BTreeSet, HashMap, HashSet};

pub fn part_one(input: &str) -> usize {
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
    let triplets: HashSet<BTreeSet<&str>> = get_triplets(&connection_map);
    triplets
        .iter()
        .filter(|triplet| {
            for &computer in triplet.iter() {
                if computer.starts_with('t') {
                    return true;
                }
            }
            false
        })
        .count()
}

fn get_triplets<'a>(
    connection_map: &'a HashMap<&'a str, HashSet<&'a str>>,
) -> HashSet<BTreeSet<&'a str>> {
    let mut triplet_set: HashSet<BTreeSet<&str>> = HashSet::new();

    for &computer in connection_map.keys() {
        let connections: Vec<_> = connection_map[computer].iter().collect();

        for (i, &connected_computer_1) in connections.iter().enumerate() {
            for &connected_computer_2 in connections.iter().skip(i + 1) {
                if connection_map[connected_computer_1].contains(connected_computer_2) {
                    let triplet: BTreeSet<&str> =
                        BTreeSet::from([computer, connected_computer_1, connected_computer_2]);
                    triplet_set.insert(triplet);
                }
            }
        }
    }
    triplet_set
}
