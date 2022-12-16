use std::fs::File;
use std::io::BufRead;
use std::clone::Clone;
use rand::prelude::SliceRandom;
use rand::distributions::Uniform;
use rand::thread_rng;
use rand::Rng;

fn read_file(path: &str) -> Vec<(String, Vec<String>)> {
    let file = File::open(path).expect("Could not open file");
    let reader = std::io::BufReader::new(file).lines();
    let mut result: Vec<(String, Vec<String>)> = Vec::new();
    let range = Uniform::new(1, 1300000);
    let sample: Vec<usize> = thread_rng().sample_iter(&range).take(99).collect();
    for (i, line) in reader.enumerate() {
        // if i == 1 || i == 2 || i == 3 { // use this line and uncomment line 17 for tests
        if sample.contains(&i) { 
            let mut tags: Vec<String> = Vec::new();
            let line_str = line.expect("Error reading");
            let v: Vec<&str> = line_str.trim().split(",").collect();
            let artist = v[2].parse::<String>().unwrap();
            if artist != "" {
                let tags_str = v[6].parse::<String>().unwrap();
                let v2: Vec<&str> = tags_str.trim().split("; ").collect();
                for i in v2 {
                    tags.push(i.parse::<String>().unwrap());
                }
                result.push((artist, tags))
            }
        }
    }
    return result;
}

fn artists_only(data: &Vec<(String, Vec<String>)>) -> Vec<String> {
    let mut artists: Vec<String> = Vec::new();
    for (a, _t) in data {
        artists.push(a.clone());
    }
    artists
}

fn get_edges(data: &Vec<(String, Vec<String>)>) -> Vec<(String, String)> {
    let mut edges: Vec<(String, String)> = Vec::new();
    for a in data.clone() {
        let artista = a.0.clone(); 
        for x in data.clone() {
            let artistx = x.0.clone(); 
            if artista != artistx {
                let pair: &(String, String) = &(artista.clone(), artistx.clone());
                for b in a.1.clone() {
                    for y in x.1.clone() {
                        if b == y {
                            if edges.contains(pair) == false {
                                edges.push((artista.clone(), artistx.clone()));
                            }
                        }
                    }
                }
            }
        }
    }
    edges
}

fn get_adjlist(edges: &Vec<(String, String)>) -> Vec<(String, Vec<String>)> {
    let mut artists: Vec<String> = Vec::new();
    for (a, _b) in edges.clone() {
        if artists.contains(&a) == false {
            artists.push(a)
        }
    }
    let mut adj: Vec<(String, Vec<String>)> = Vec::new();
    for a1 in artists {
        let mut l: Vec<String> = Vec::new();
        for (x, y) in edges.clone() {
            if a1 == x {
                l.push(y)
            }
        }
        adj.push((a1, l))
    }
    adj
}

fn find_distance(start: String, end: String, mut queue: Vec<String>, adjlist: Vec<(String, Vec<String>)>) -> (String, String, i32) { // finds distance between two given artists 
    let found = 0;
    let mut distance: i32 = 1;
    let mut visited: Vec<String> = Vec::new();
    let mut search: Vec<String> = Vec::new();
    while found == 0 {
        for a in &queue {
            if visited.contains(&a) == false {
                if a == &end {
                    return (start, end, distance);
                }
                visited.push(a.clone());
                for (x, y) in &adjlist {
                    if a == x {
                        for yi in y {
                            if visited.contains(&yi) == false && queue.contains(&yi) == false {
                                search.push(yi.clone());
                            }
                        }
                    }
                }
            }
        }
        distance += 1;
        queue.clear();
        for s in &search {
            if visited.contains(&s) == false {
                if s == &end {
                    return (start, end, distance);
                }
                visited.push(s.clone());
                for (i, j) in &adjlist {
                    if s == i {
                        for ji in j {
                            if visited.contains(&ji) == false && queue.contains(&ji) == false {
                                queue.push(ji.clone());
                            }
                        }
                    }
                }
            }
        }
        distance += 1;
        search.clear();
        if queue.len() == 0 && search.len() == 0 {
            return (start, end, 99999);
        }
    }
    return (start, end, distance);
}

fn average_distance(adjlist: Vec<(String, Vec<String>)>) -> (f32, Vec<i32>, i32, Vec<(String, String)>) { // finds the average distance for all nodes in a graph
    let mut sum: f32 = 0.0;
    let mut no_edge: f32 = 0.0;
    let mut n: f32 = 0.0;
    let mut dist: Vec<i32> = Vec::new(); 
    let mut bad_pairs: Vec<(String, String)> = Vec::new();
    for a1 in adjlist.clone() {
        for a2 in adjlist.clone() {
            if a1 != a2 {
                let d = find_distance(a1.clone().0, a2.0, a1.clone().1, adjlist.clone());
                if d.2 == 99999 {
                    no_edge += 0.5;
                    bad_pairs.push((d.0, d.1));
                }
                else {
                    sum += d.2 as f32;
                    n += 1.0;
                    dist.push(d.2);
                }
            }
        }
    }
    let ave: f32 = sum / n;
    return (ave, dist, no_edge as i32, bad_pairs)
}

fn random_pair(adjlist: &Vec<(String, Vec<String>)>) -> (String, String, i32) { // find distance between two randomly selected artists 
    let artist1 = adjlist.choose(&mut rand::thread_rng()).unwrap();
    let start = artist1.clone().0;
    let search = artist1.clone().1;
    let mut adjlist2 = adjlist.clone();
    adjlist2.retain(|x| *x != artist1.clone());
    let end = adjlist2.choose(&mut rand::thread_rng()).unwrap().clone().0;
    let d = find_distance(start, end, search, adjlist.to_vec());
    return d;
}

fn random_average(adjlist: &Vec<(String, Vec<String>)>, n: i32) -> (f32, i32) { // for computing usual distance for bigger graphs to save time 
    let mut n = n; 
    let mut sum: f32 = 0.0;
    let mut no_edge: i32 = 0;
    for _i in 0..n {
        let r = random_pair(adjlist).2;
        if r == 99999 {
            no_edge += 1;
            n -= 1;
        }
        else {
            sum += r as f32;
        }
    }
    let ave: f32 = sum / n as f32;
    return (ave, no_edge);
}

#[test] 
fn test_read_file() {
    let test = read_file("artists test.csv");
    let tags1 = vec![String::from("rock"), String::from("alternative")];
    let tags2 = vec![String::from("alternative"), String::from("alternative rock")];
    let tags3 = vec![String::from("alternative rock"), String::from("Funk Rock")];
    let result = vec![(String::from("Coldplay"), tags1), (String::from("Radiohead"), tags2), (String::from("Red Hot Chili Peppers"), tags3)];
    assert_eq!(result, test);
}
#[test]
fn test_artists_only() {
    let data = read_file("artists test.csv");
    let test = artists_only(&data);
    let result = vec![String::from("Coldplay"), String::from("Radiohead"), String::from("Red Hot Chili Peppers")];
    assert_eq!(result, test);
}
#[test] 
fn test_get_edges() {
    let data = read_file("artists test.csv");
    let test = get_edges(&data);
    let result = vec![(String::from("Coldplay"), String::from("Radiohead")), (String::from("Radiohead"), String::from("Coldplay")), (String::from("Radiohead"), String::from("Red Hot Chili Peppers")), (String::from("Red Hot Chili Peppers"), String::from("Radiohead"))];
    assert_eq!(result, test);
}
#[test] 
fn test_get_adjlist() {
    let data = read_file("artists test.csv");
    let edges = get_edges(&data);
    let test = get_adjlist(&edges);
    let adj1 = vec![String::from("Radiohead")];
    let adj2 = vec![String::from("Coldplay"), String::from("Red Hot Chili Peppers")];
    let adj3 = vec![String::from("Radiohead")];
    let result = vec![(String::from("Coldplay"), adj1), (String::from("Radiohead"), adj2), (String::from("Red Hot Chili Peppers"), adj3)];
    assert_eq!(result, test);
}
#[test]
fn test_find_distance() {
    let data = read_file("artists test.csv");
    let edges = get_edges(&data);
    let adjlist = get_adjlist(&edges);
    let search = vec![String::from("Radiohead")];
    let test = find_distance(String::from("Coldplay"), String::from("Red Hot Chili Peppers"), search, adjlist);
    let result = (String::from("Coldplay"), String::from("Red Hot Chili Peppers"), 2);
    assert_eq!(result, test);
}

fn main() {
    let data = read_file("artists.csv");
    println!("There are {:?} artists in this sample.", data.len());
    let artists = artists_only(&data);
    println!("The artists in this sample are {:?}.", artists);
    let edges = get_edges(&data);
    let adjlist = get_adjlist(&edges);
    // let example = random_pair(&adjlist);
    // println!("The distance between {:?} and {:?} is {:?}.", example.0, example.1, example.2);
    // let average = average_distance(adjlist);
    // println!("The average distance between artists in this sample is {:?}", average.0);
    // println!("The distances between each possible pair are {:?}", average.1);
    // println!("{:?} pairs of artists cannot be connected.", average.2);
    // println!("The pairs that can't be connected are {:?}.", average.3);
    let n = 1000;
    let random_average = random_average(&adjlist, n);
    println!("For {:?} random pairs:", n);
    println!("The average distance between randomly selected pairs of artists in this sample is {:?}", random_average.0);
    println!("{:?} pairs of artists cannot be connected.", random_average.1);
}
