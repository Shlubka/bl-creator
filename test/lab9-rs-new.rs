use rand::Rng;
use std::collections::{HashSet, VecDeque};
use std::io::{self, Write};
use std::time::Instant;

fn read_from_keyboard() -> u32 {
    io::stdout().flush().expect("flush error");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    if let Ok(i) = trimmed.parse::<u32>() {
        return i;
    } else {
        println!("\x1b[31m!!!No number in input, установлено значение по умолчанию(10)!!!\x1b[0m");
        return 10;
    }
}

fn create_graph(size: usize) -> Vec<Vec<u32>> {
    let mut rng = rand::thread_rng();
    let mut graf = vec![vec![0u32; size]; size];

    for i in 0..size {
        for j in i..size {
            if i == j {
                continue;
            }
            graf[i][j] = rng.gen::<u32>() % 2;
            graf[j][i] = graf[i][j];
        }
    }

    return graf;
}

fn create_adjacency_list(graf: &Vec<Vec<u32>>) -> Vec<Vec<usize>> {
    let size = graf.len();
    let mut adj_list = vec![vec![]; size];

    for i in 0..size {
        for j in 0..size {
            if graf[i][j] == 1 {
                adj_list[i].push(j);
            }
        }
    }

    return adj_list;
}

fn bfs_matrix(graf: &Vec<Vec<u32>>, start: usize) -> Vec<usize> {
    let num_vertices = graf.len();
    let mut visited = vec![false; num_vertices];
    let mut distances = vec![usize::MAX; num_vertices];
    let mut queue = VecDeque::new();
    queue.push_back(start);
    distances[start] = 0;

    while let Some(vertex) = queue.pop_front() {
        if !visited[vertex] {
            visited[vertex] = true;
            for neighbour in 0..num_vertices {
                if graf[vertex][neighbour] == 1 && !visited[neighbour] {
                    queue.push_back(neighbour);
                    distances[neighbour] = distances[vertex] + 1;
                }
            }
        }
    }

    return distances;
}

fn bfs_list(adj_list: &Vec<Vec<usize>>, start: usize) -> Vec<usize> {
    let num_vertices = adj_list.len();
    let mut visited = vec![false; num_vertices];
    let mut distances = vec![usize::MAX; num_vertices];
    let mut queue = VecDeque::new();
    queue.push_back(start);
    distances[start] = 0;

    while let Some(vertex) = queue.pop_front() {
        if !visited[vertex] {
            visited[vertex] = true;
            for &neighbour in &adj_list[vertex] {
                if !visited[neighbour] {
                    queue.push_back(neighbour);
                    distances[neighbour] = distances[vertex] + 1;
                }
            }
        }
    }

    return distances;
}

fn dfs_matrix(graf: &Vec<Vec<u32>>, start: usize) -> Vec<usize> {
    let num_vertices = graf.len();
    let mut visited = vec![false; num_vertices];
    let mut distances = vec![usize::MAX; num_vertices];
    let mut stack = vec![start];
    distances[start] = 0;

    while let Some(vertex) = stack.pop() {
        if !visited[vertex] {
            visited[vertex] = true;
            for neighbour in (0..num_vertices).rev() {
                if graf[vertex][neighbour] == 1 && !visited[neighbour] {
                    stack.push(neighbour);
                    distances[neighbour] = distances[vertex] + 1;
                }
            }
        }
    }

    return distances;
}

fn dfs_list(adj_list: &Vec<Vec<usize>>, start: usize) -> Vec<usize> {
    let num_vertices = adj_list.len();
    let mut visited = vec![false; num_vertices];
    let mut distances = vec![usize::MAX; num_vertices];
    let mut stack = vec![start];
    distances[start] = 0;

    while let Some(vertex) = stack.pop() {
        if !visited[vertex] {
            visited[vertex] = true;
            for &neighbour in adj_list[vertex].iter().rev() {
                if !visited[neighbour] {
                    stack.push(neighbour);
                    distances[neighbour] = distances[vertex] + 1;
                }
            }
        }
    }

    return distances;
}

fn main() {
    print!("Enter graph size, more than 0 (one number for line and column) > ");
    let size = read_from_keyboard();
    let graf = create_graph(size as usize);

    println!("Graph:");
    for row in &graf {
        println!("{:?}", row);
    }

    println!("BFS using matrix");
    let start = Instant::now();
    let distances = bfs_matrix(&graf, 0);
    let duration = start.elapsed();
    println!("Distances: {:?}", distances);
    println!("Time elapsed: {:?}", duration);

    let adj_list = create_adjacency_list(&graf);
    println!("Graph:");
    for row in &adj_list {
        println!("{:?}", row);
    }
    println!("BFS using adjacency list:");
    let start = Instant::now();
    let distances = bfs_list(&adj_list, 0);
    let duration = start.elapsed();
    println!("Distances: {:?}", distances);
    println!("Time elapsed: {:?}", duration);

    println!("DFS using matrix");
    let start = Instant::now();
    let distances = dfs_matrix(&graf, 0);
    let duration = start.elapsed();
    println!("Distances: {:?}", distances);
    println!("Time elapsed: {:?}", duration);

    println!("DFS using adjacency list:");
    let start = Instant::now();
    let distances = dfs_list(&adj_list, 0);
    let duration = start.elapsed();
    println!("Distances: {:?}", distances);
    println!("Time elapsed: {:?}", duration);
}
