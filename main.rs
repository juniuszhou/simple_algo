fn main() {
}

// parse the graph, store in the vector use char's order as index 
fn parse_graph(input: String) -> [[u32; 26]; 26] {
    let mut graph = [[0; 26]; 26];

    let vec = input.trim().split(",").collect::<Vec<&str>>();
    let _ = vec.iter().map(|item| {
            let char_vec: Vec<char> = item.chars().collect();
            let first = *char_vec.get(0).unwrap() as u8 - 'A' as u8;
            let second = *char_vec.get(1).unwrap() as u8 - 'A' as u8;
            let distance = item[2..].parse::<u32>().unwrap();
            graph[first as usize][second as usize] = distance;
        }
    ).collect::<Vec<()>>();
    return graph;
}

fn compute_distance(input: String, graph: [[u32; 26]; 26]) -> u32 {
    let vec = input.trim().split("-").collect::<Vec<&str>>();
    let mut last_point = 0;
    let mut is_first = true;
    let mut distance = 0;

    for item in vec.iter() {
        let point = *item.chars().collect::<Vec<char>>().get(0).unwrap() as u8 - 'A' as u8;
        
        if is_first {
            is_first = false;
        } else {
            if graph[last_point as usize][point as usize] == 0 {
                return 0;
            }
            distance += graph[last_point as usize][point as usize];
        }
        last_point = point;
    }
    distance
}

// compute the distance 
fn path_distance(vec: Vec<u8>, graph: [[u32; 26]; 26]) -> u32 {
    let mut last_point: usize = 0;
    let mut is_first = true;
    let mut distance: u32 = 0;

    for item in vec.iter() {
        if is_first {
            is_first = false;
        } else {
            distance += graph[last_point][*item as usize];
        }
        last_point = *item as usize;
    }
    distance
}

// dfs according to depth or distance
fn dfs(vec: Vec<u8>, graph: [[u32; 26]; 26], end: u8, max: usize, is_distance: bool) -> u32{
    if !is_distance && vec.len() >= max {
        return 0;
    } else if is_distance && path_distance(vec.clone(), graph) >= max as u32 {
        return 0;
    }

    let mut result = 0; 

    let last_point = vec.get(vec.len() - 1).unwrap();
    if graph[*last_point as usize][end as usize] > 0 {
        let mut tmp = vec.clone();
        tmp.push(end);
        if !is_distance || path_distance(tmp.clone(), graph) < max as u32 {
            result += 1;
        }
    }

    for index in 0..26 {
        if graph[*last_point as usize][index] > 0 {
            let mut new_vec = vec.clone();
            new_vec.push(index as u8);
            result += dfs(new_vec, graph, end, max, is_distance);
        } 
    }
    return result;
}

fn bfs(vec: Vec<u8>, graph: [[u32; 26]; 26], end: u8, max: usize) -> u32 {
    let mut result = 0;
    let last_point = vec.get(vec.len() - 1).unwrap();

    if vec.len() == max {
        if graph[*last_point as usize][end as usize] > 0 {
            return 1;
        } else {
            return 0;
        }
    } 

    for index in 0..26 {
        if graph[*last_point as usize][index] > 0 {
            let mut new_vec = vec.clone();
            new_vec.push(index as u8);
            result += bfs(new_vec, graph, end, max);
        } 
    }
    return result;
}

// find shortest path, if it is a loop, one more step to compute
fn spfa(graph: [[u32; 26]; 26], start: u8, end: u8, is_loop: bool) -> u32 {
    let mut vec = vec![start];
    let mut result = [u32::MAX; 26];
    result[start as usize] = 0;
    let mut visit = [false; 26];

    while vec.len() > 0 {
        let point = vec.pop().unwrap();
        for index in 0..26 {
            if graph[point as usize][index] > 0 {
                if result[index] > result[point as usize] + graph[point as usize][index] {
                    result[index] = result[point as usize] + graph[point as usize][index];
                }
                if !visit[index] {
                    visit[index] = true;
                    vec.push(index as u8);
                }
            } 
        }
        visit[point as usize] = true;
    }

    if !is_loop {
        return result[end as usize];
    }
    
    let mut min = u32::MAX;
    for index in 0..26 {
        if graph[index][start as usize] > 0 && result[index] < u32::MAX {
            if graph[index][start as usize] + result[index] < min {
                min = graph[index][start as usize] + result[index];
            }
        } 
    }

    min
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_one() {
        let input = String::from("AB5,BC4,CD8,DC8,DE6,AD5,CE2,EB3,AE7");
        let graph = parse_graph(input);

        let distance = compute_distance(String::from("A-B-C"), graph);
        assert_eq!(distance, 9);

        let distance = compute_distance(String::from("A-D"), graph);
        assert_eq!(distance, 5);

        let distance = compute_distance(String::from("A-D-C"), graph);
        assert_eq!(distance, 13);

        let distance = compute_distance(String::from("A-E-B-C-D"), graph);
        assert_eq!(distance, 22);

        let distance = compute_distance(String::from("A-E-D"), graph);
        assert_eq!(distance, 0);

        // c to c with 3 stops
        let result = dfs(vec![2], graph, 2, 4, false);
        assert_eq!(result, 2);

        // a to c with 4 stops
        let result = bfs(vec![0], graph, 2, 4);
        assert_eq!(result, 3);

        // shortest route from a to c
        assert_eq!(spfa(graph, 0, 2, false), 9);

        // shortest route from b to b
        assert_eq!(spfa(graph, 1, 0, true), 9);

        // different route from c to c less 30
        assert_eq!(dfs(vec![2], graph, 2, 30, true), 7);
    }
}
