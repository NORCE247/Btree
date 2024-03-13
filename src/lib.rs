use  std::collections::VecDeque;

#[derive(Debug)]
pub struct Node{
    keys: VecDeque<i32>,
    edges: VecDeque<Node>,
}
pub struct BTree{ root : Node }

impl Node{

    fn new() -> Node{
        Node{
            keys : VecDeque::new(),
            edges: VecDeque::new(),
        }
    }

    fn insert_key(&mut self, key : i32){

        //Sjekk om det finnes edges
        match self.edges.len() {
            //ingen barn
            0 => {
                //self sort
                let len_key = self.keys.len();
                if self.keys.is_empty(){

                    self.keys.push_back(key);

                } else {

                    for i in 0..len_key {
                        if key < self.keys[i] {
                            self.keys.insert(i, key);
                            break;
                        }
                    }

                    if key > self.keys[len_key - 1] {
                        self.keys.push_back(key);
                    }

                }
            },

            //To barn
            2 => {
                //sjekk enhverbarn sin keys.
                if key < self.keys[0] {

                    //sjekk antall keys i barna
                    match self.edges[0].keys.len() {
                        3 => {
                            //flytt median opp
                            self.keys.insert(0, self.edges[0].keys.remove(1).unwrap());
                            // og Splitt node
                            let key_0 = self.edges[0].keys.pop_front().unwrap();
                            self.edges.insert(0,Node::new());
                            self.edges[0].keys.push_front(key_0);
                            //insert key
                            self.edges[0].insert_key(key);
                        },
                        _ => {
                            self.edges[0].insert_key(key);
                        },
                    }

                } else {

                    //sjekk antall keys i barna
                    match self.edges[1].keys.len() {
                        3 => {
                            //flytt median opp
                            self.keys.push_back(self.edges[1].keys.remove(1).unwrap());
                            // og Splitt node
                            let key_1 = self.edges[1].keys.pop_back().unwrap();
                            self.edges.insert(2,Node::new());
                            self.edges[1].keys.push_front(key_1);
                            //insert key
                            self.edges[1].insert_key(key);
                        },
                        _ => {
                            self.edges[1].insert_key(key);
                        },
                    }
                }
            },
            //todo 3 barn og 4 barn
            _ => (),
        }
    }
}


impl BTree{

    fn new() -> BTree{ BTree { root : Node::new() } }

    pub fn add(&mut self, key : i32){

        if self.root.keys.len()< 3 {

            self.root.insert_key(key);

        } else {

            match self.root.edges.len() {
                0 => {
                    let mut child0 = Node::new();
                    let mut child1 = Node::new();

                    child0.keys.push_back(self.root.keys.pop_front().unwrap());
                    child1.keys.push_back(self.root.keys.pop_back().unwrap());

                    self.root.edges.push_back(child0);
                    self.root.edges.push_back(child1);

                    self.root.insert_key(key);
                },
                //todo 2,3 og 4 barn
                _ => println!("ll"),
            }
        }
    }
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        let mut tree = BTree::new();
        for i in (0..=5).rev()  {
            tree.add(i);
        }

        println!("{:?}", tree.root.keys);
        let len = tree.root.edges.len();
        for i in 0..len  {
            println!("Egde {}: {:?}", i, tree.root.edges[i].keys);

        }


    }
}
