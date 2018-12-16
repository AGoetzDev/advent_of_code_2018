
#[derive(Debug)]
pub struct Node {
    nodes: Vec<Node>,
    metadata: Vec<u32>,
}
impl Node {

    fn traverse_bfs(&self) -> Vec<&Node>{
        let mut stack: Vec<&Node> = Vec::new();
        let mut result = Vec::new();
    
        stack.push(self);
        
        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            for n in node.nodes.iter() {
                stack.push(n);
            }
            result.push(node);
        }
        
        result
    }
    fn sum(&self) -> u32 {
        self.metadata.iter().sum()
    }
    fn sum2(&self) -> u32 {
        if self.nodes.len() == 0 {
            self.metadata.iter().sum()
        } else {
            let mut sum = 0;
            for i in self.metadata.iter() {
                let index = (i-1) as usize;
                if index < self.nodes.len() {
                    sum+= self.nodes[index].sum2();
                }
                
            }
            sum
        }
        
    }

}

impl AsRef<Node> for Node {
    fn as_ref(&self) -> &Node {
        &self
    }
}

fn generate_tree(vals: &[u32], current_pos: usize) -> (Node, usize) {
    let children_size = vals[current_pos];
    let metadata_size = vals[current_pos+1];
    let mut nodes: Vec<Node> = Vec::new();
    let mut metadata: Vec<u32> = Vec::new();
    let mut tree_pos = current_pos+2;
    
    
    for _i in 0..children_size {
        
        let (n, p) = generate_tree(vals, tree_pos);
        nodes.push(n);
        tree_pos = p;
    }
    for _i in 0..metadata_size {
        metadata.push(vals[tree_pos]);
        tree_pos +=1;
    }
    (Node {
        nodes: nodes,
        metadata: metadata,
    }, tree_pos)
}



#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Node {
    let split: Vec<u32> = input.split_whitespace().map(|c| c.parse::<u32>().unwrap_or(0)).collect();
    let (root, _pos) = generate_tree(&split, 0);
    
    root
        
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Node) -> u32 {
    let mut sum = 0;
    for n in input.traverse_bfs().iter(){
        
        sum+=n.sum();
        
    }
    sum
        
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Node) -> u32 {
    input.sum2()
        
}