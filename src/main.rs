use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Write},
    iter,
    process::{Command, Stdio},
    sync::LazyLock,
};

use itertools::Itertools;
use petgraph::prelude::*;
use rustc_hash::FxHashSet;

static G: LazyLock<DiGraph<u32, u32>> = LazyLock::new(|| {
    let graph_data = include_str!("../graph.txt");
    DiGraph::from_edges(graph_data.lines().skip(1).map(|line| {
        let (from, to, weight) = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap();
        (from, to, weight)
    }))
});

fn get_inputs() -> impl Iterator<Item = FxHashSet<u32>> {
    let mut reader = BufReader::new(File::open("locs.txt").unwrap());
    let mut buf = String::new();
    iter::from_fn(move || {
        buf.clear();
        let n = reader.read_line(&mut buf).unwrap();
        if n > 0 {
            Some(
                buf.split_ascii_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect(),
            )
        } else {
            None
        }
    })
}

fn main() {
    let cmd_args = env::args().skip(1).collect_vec();
    'inputs_loop: for mut input in get_inputs() {
        let mut cmd = Command::new(&cmd_args[0])
            .args(&cmd_args[1..])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let mut stdin = cmd.stdin.take().unwrap();
        let mut stdout = BufReader::new(cmd.stdout.take().unwrap());
        let mut count = 0;
        let mut cost = 0;
        let mut curr_node = G.node_indices().find(|&nx| G[nx] == 0).unwrap();
        let mut buf = String::new();
        while count < input.len() && cmd.try_wait().unwrap().is_none() {
            buf.clear();
            let n = stdout.read_line(&mut buf).unwrap();
            if n == 0 {
                break;
            }
            let next_node_weight = buf.parse().unwrap();
            let next_node = G
                .node_indices()
                .find(|&nx| G[nx] == next_node_weight)
                .unwrap();
            if let Some(edge) = G.find_edge(curr_node, next_node) {
                curr_node = next_node;
                let mut found = 0;
                if input.remove(&G[curr_node]) {
                    found = 1;
                    count += 1;
                }
                cost += G[edge];
                writeln!(&mut stdin, "{found} {count} {cost}").unwrap();
            } else {
                eprintln!(
                    "disqualified trying to move from {} to {}",
                    G[curr_node], G[next_node]
                );
                cmd.kill().unwrap();
                break 'inputs_loop;
            }
        }
        if count == input.len() {
            println!("{} => {cost}", input.iter().join(" "));
        } else {
            eprintln!("subprocess gave up before completing the input");
            cmd.kill().unwrap();
            break 'inputs_loop;
        }
    }
}
