use std::io::Read;
use std::fs::File;
use petgraph::graph::{NodeIndex};
use petgraph::algo::{dijkstra};
use petgraph::Graph;
use std::collections::HashMap;
use petgraph::visit::{EdgeRef};
use rand::Rng;
use petgraph::Incoming;

fn read() -> (Vec<usize>,Vec<(usize,usize)>) {
    let mut f = File::open("TwitterDataSet.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong");
    let lines: Vec<&str> = contents.lines().collect();
    let mut graph_edges: Vec<(usize,usize)> = vec![];
    let mut nodes: Vec<usize> = vec![];

    for line in lines{
        let l: Vec<&str> = line.split_whitespace().collect();
        if !nodes.contains(&l[0].parse().unwrap()){
            nodes.push(l[0].parse().unwrap());
        }
        let node_edge: (usize,usize) = (l[0].parse().unwrap(),l[1].parse().unwrap());
        graph_edges.push(node_edge);
    }
    return (nodes,graph_edges);
}

fn make_graph(edge: Vec<(usize,usize)>) -> Graph<usize,usize> {
    let mut graph = Graph::<usize,usize>::new();
    let mut indices = HashMap::<usize,NodeIndex>::new();
    
    for (x,y) in edge.iter(){
        if !indices.contains_key(x){
            let n1 = graph.add_node(*x);
            indices.insert(*x,n1);
        }
        if !indices.contains_key(y){
            let n2 = graph.add_node(*y);
            indices.insert(*y, n2);
        }
        graph.extend_with_edges(&[(*indices.get(x).unwrap(), *indices.get(y).unwrap())]);
    }
    return graph;
}

fn six_degrees(graph: Graph<usize,usize>, nodelist: Vec<usize>, iterations: usize) -> (f64,Vec<f64>){
    let mut avg: f64 = 0.0;
    let mut separations: Vec<f64> = vec![];
    for _i in 0..iterations{
        let start: usize = rand::thread_rng().gen_range(0..nodelist.len());
        let mut end = rand::thread_rng().gen_range(0..nodelist.len());
        if start ==  end{
            end = rand::thread_rng().gen_range(0..nodelist.len());
        }
        
        let mut sum: f64 = 0.0;

        let res = dijkstra(
                    &graph,
                    graph.node_indices().find(|i| graph[*i] == graph.raw_nodes()[start].weight)
                    .expect("node does not exist"),
                    Some(graph.node_indices().find(|i| graph[*i] == graph.raw_nodes()[end].weight)
                    .expect("no node")),
                    |_| 1);

        for (_key,value) in &res{
            sum += *value as f64;
        }
        let size = &res.keys().len();
        sum = sum / *size as f64;
        avg += sum;
        separations.push(sum);
    }
    return (avg/iterations as f64, separations);
}

fn friends(graph: Graph<usize,usize>, nodelist: Vec<usize>,iterations: usize) -> (f64, Vec<f64>){
    let mut vec:Vec<f64> = vec![];
    let mut avg: f64 = 0.0;

    for _i in 0..iterations{

        let mut friend_one: usize = rand::thread_rng().gen_range(0..nodelist.len());
        let mut friend_two = rand::thread_rng().gen_range(0..nodelist.len());

        let mut first =  graph.node_indices().
                                find(|i| graph[*i] == graph.raw_nodes()[friend_one].weight).unwrap();
        let mut second = graph.node_indices().
                                find(|i| graph[*i] == graph.raw_nodes()[friend_two].weight).unwrap();

        while !graph.contains_edge(first, second) && !graph.contains_edge(second, first){
            friend_one = rand::thread_rng().gen_range(0..nodelist.len());
            friend_two = rand::thread_rng().gen_range(0..nodelist.len());
            first = graph.node_indices().
                                find(|i| graph[*i] == graph.raw_nodes()[friend_one].weight).unwrap();
            second = graph.node_indices().
                                find(|i| graph[*i] == graph.raw_nodes()[friend_two].weight).unwrap();
        }
        
        let mut friend_count_one: usize = 0;
        let mut friend_count_two: usize = 0;

        for (_node,next) in graph.edges_directed(first, Incoming).into_iter().enumerate(){
            if graph.contains_edge(first, next.source()){
                friend_count_one += 1;
            }
        }

        for (_node,next) in graph.edges_directed(second, Incoming).into_iter().enumerate(){
            if graph.contains_edge(second, next.source()){
                friend_count_two += 1;
            }
        }
        
        if friend_count_two > friend_count_one{
            vec.push(1.0);
        }
        else{
            vec.push(0.0);
        }
    }
    
    for i in vec.iter(){
        avg += *i;
    }
    return (avg/vec.len() as f64,vec);
}


fn main(){
    let edge = read().1;
    let nodes: Vec<usize> = read().0;
    let dir_graph = make_graph(edge);
    let iterations:usize = 10;

    let algo = six_degrees(dir_graph.clone(), nodes.clone(), iterations);
    let friendship = friends(dir_graph.clone(),nodes.clone(),iterations);

    println!("Average distance from node to node: {}",algo.0);
    println!("Sample average for 10 different nodes: {:?}", algo.1);
    
    println!("Average percentage of time your friends have more friends than you: {}",friendship.0);
    println!("Sample of how many times your friends have more friends than you: {:?}",friendship.1);
}