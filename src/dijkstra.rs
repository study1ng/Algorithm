use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, start: usize) -> Vec<usize> {
    let n = graph.len();
    let mut dist = vec![usize::MAX; n];

    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), start));
    dist[start] = 0;

    while let Some((Reverse(d), u)) = pq.pop() {
        if d > dist[u] {
            continue;
        }
        dist[u] = d;
        for &(v, c) in &graph[u] {
            if dist[u] == usize::MAX {
                continue;
            }
            if dist[v] > dist[u] + c {
                dist[v] = dist[u] + c;
                pq.push((Reverse(dist[v]), v));
            }
        }
    }

    dist
}