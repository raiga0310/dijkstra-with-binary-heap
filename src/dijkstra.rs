use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn dijkstra(graph: Vec<Vec<usize>>, start: usize, goal: usize) -> (Vec<usize>, Vec<usize>) {
    let node_num = graph.len();
    let mut distance = vec![usize::MAX; node_num];
    let mut queue = BinaryHeap::new();
    let mut prev_nodes = vec![None; node_num];

    //init
    distance[start] = 0;
    queue.push(Reverse((start, 0)));

    //Solve
    while let Some(Reverse((u, sd))) = queue.pop() {
        // 訪問済みならスキップ
        if sd > distance[u] {
            continue;
        }

        for (v, &weight) in graph[u].iter().enumerate() {
            // 辺が存在しないのでスキップ
            if weight == 0 {
                continue;
            }

            println!("現在のノード: {}, 最短距離: {}", u, sd);
            println!("計算するノード: {}", v);
            println!("各ノードと始点sとの最短距離");
            for i in distance.clone().into_iter() {
                print!(
                    "{}",
                    if i == usize::MAX {
                        String::from("-, ")
                    } else {
                        format!("{}, ", i)
                    }
                );
            }
            println!();

            let alt = sd + weight;
            if alt < distance[v] {
                distance[v] = alt;
                prev_nodes[v] = Some(u);
                queue.push(Reverse((v, alt)));

                println!("更新: ノード {} の最短距離を {} に変更", v, alt);
            }
        }

        println!("===========================================================")
    }

    // path solve
    let mut path = Vec::new();
    let mut current = goal;
    while current != start {
        path.push(current);
        if let Some(prev) = prev_nodes[current] {
            current = prev;
        } else {
            println!("Path not found");
            return (Vec::new(), distance);
        }
    }
    path.push(start);
    path.reverse();

    (path, distance)
}
