use std::collections::HashSet;

use toy_ufdb::Ufdb;

pub fn render(ufdb: &mut Ufdb) -> String {
    let mut groups: Vec<Vec<String>> = ufdb
        .groups()
        .into_values()
        .map(|members| members.iter().map(|s| s.to_string()).collect())
        .collect();

    for group in &mut groups {
        group.sort();
    }

    groups.sort_by(|a, b| b.len().cmp(&a.len()));

    let body: String = groups
        .iter()
        .map(|members| render_group(ufdb, members))
        .collect();

    format!("<!DOCTYPE html><html><body>{body}</body></html>")
}

pub fn render_group(Ufdb: &Ufdb, members: &[String]) -> String {
    let key = &members[0];


    format!("render_group")
}

fn render_node(Ufdb: &Ufdb, key: &str, visited: &mut HashSet<String>) -> String {
    format!("render_node")
}
