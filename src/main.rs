pub mod dijkstra;

fn main() {
    let graph = vec![
        vec![0, 3, 0, 0, 0, 6, 0, 0, 0, 9],
        vec![3, 0, 1, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 7, 0, 0, 0, 4, 0, 0],
        vec![0, 0, 7, 0, 2, 0, 0, 0, 0, 3],
        vec![0, 0, 0, 2, 0, 5, 0, 0, 0, 0],
        vec![6, 0, 0, 0, 5, 0, 8, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 8, 0, 9, 0, 0],
        vec![0, 0, 4, 0, 0, 0, 9, 0, 1, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 2],
        vec![9, 0, 0, 3, 0, 0, 0, 0, 2, 0],
    ];
    let start = 0;
    let goal = 8;
    let (path, dist) = dijkstra::dijkstra(graph, start, goal);
    println!("最短経路: {:?}", path);
    println!(
        "始点{}から終点{}までの最短距離: {}",
        start, goal, dist[goal]
    ); //node_iから始点までの最短距離: dist[i]
}
