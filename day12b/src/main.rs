struct Graph<'a> {
    graph: Vec<(&'a str, Vec<&'a str>)>
}

impl<'a> Graph<'a> {
    fn new() -> Graph<'a> {
        Graph{graph: Vec::new()}
    }

    fn add_edge(&mut self, origin: &'a str, dest: &'a str) {
        let mut found_origin = false;
        let mut found_dest = false;
        for vertex in &mut self.graph {
            if vertex.0 == origin {
                found_origin = true;
                vertex.1.push(dest);
            }
            if vertex.0 == dest {
                found_dest = true;
                vertex.1.push(origin);
            }
        }
        if !found_origin {
            self.graph.push((origin, vec![dest]))
        }
        if !found_dest {
            self.graph.push((dest, vec![origin]))
        }
    }

    fn get_edges(&self, vertex: &str) -> Option<&Vec<&str>> {
        for v in &self.graph {
            if vertex.eq(v.0) {
                return Some(&v.1)
            }
        }
        return None
    }

    fn get_all_paths(self) -> Vec<Vec<String>> {
        let mut paths: Vec<Vec<String>> = Vec::new();
        let mut path_stack: Vec<(Vec<String>, bool)> = Vec::new();

        for n in self.get_edges("start").unwrap().to_vec() {
            path_stack.push((vec!["start".to_owned(), n.to_owned()], false))
        }

        while !path_stack.is_empty() {
            let path_pop = path_stack.pop().unwrap();
            let current_path = path_pop.0;
            let has_revisited = path_pop.1;
            let current_vertex = current_path.last().unwrap();
            for edge in self.get_edges(&current_vertex).unwrap().to_vec() {
                let mut new_path = current_path.to_vec();
                new_path.push(edge.to_owned());
                let is_small = edge.chars().all(char::is_lowercase);
                let allow_revisit = !is_small || !has_revisited;
                if edge == "end" {
                    paths.push(new_path);
                } else if !current_path.contains(&edge.to_owned()) && edge != "start" {
                    path_stack.push((new_path, has_revisited));
                } else if current_path.contains(&edge.to_owned()) && edge != "start" {
                    if allow_revisit {
                        if is_small {
                            path_stack.push((new_path, true));
                        } else {
                            path_stack.push((new_path, has_revisited));
                        }

                    }
                }
            }
        }


        return paths
    }
}

fn main() {
    let pairs: Vec<&str> = include_str!("../input.txt")
        .lines().collect();

    let mut g = Graph::new();

    for edge in pairs {
        let verticies = edge.split("-").collect::<Vec<&str>>();
        g.add_edge(verticies[0], verticies[1])
    }

    println!("{:?}", g.get_all_paths().len())
}
