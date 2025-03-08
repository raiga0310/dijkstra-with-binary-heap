use std::collections::BinaryHeap;

// Node構造体
#[derive(Clone, PartialEq, Eq)]
pub struct Node {
    // 識別番号
    idx: usize,
    // 始点までの最短経路
    sd: usize,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.sd.cmp(&self.sd)) // 逆順にして最小値を優先
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.sd.cmp(&self.sd) // 逆順にして最小値を優先
    }
}

pub fn dijkstra(graph: Vec<Vec<usize>>, start: usize, goal: usize) -> (Vec<usize>, Vec<usize>) {
    let node_num = graph.len();
    let mut distance = vec![usize::MAX; node_num];
    let mut queue = BinaryHeap::new();
    let mut prev_nodes = vec![None; node_num];

    //init
    distance[start] = 0;
    queue.push(Node { idx: start, sd: 0 });

    //Solve
    while let Some(Node { idx: u, sd }) = queue.pop() {
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
                queue.push(Node { idx: v, sd: alt });

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
