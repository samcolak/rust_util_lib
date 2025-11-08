
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Deref;


#[derive(Debug, Clone)]
pub struct Btree <T> 
where T: Clone + Debug
{
    pub delim: String,
    pub head: String,
    pub var: Vec<T>,
    parts: HashMap<String, Btree<T>>
}


impl <T: Clone + Debug> Deref for Btree<T> {

    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.var
    }

}


impl <T: Clone + Debug> Btree<T> {


    pub fn new(delim: &str) -> Self {
        Self { delim: delim.to_string(), head: "".to_string(), var: Vec::new(), parts: HashMap::new() }
    }


    // recurse the binary tree enumerating all items..
    pub fn items(&self) -> Vec<T> {

        let mut _out = Vec::new();

        if !self.var.is_empty() {
            _out.extend(self.var.clone());
        }

        for _part in self.parts.values() {
            _out.extend(_part.items());
        }

        _out

    }


    pub fn insert(&mut self, mut path: &str, val: T) -> bool {

        // strip any preceding delimiter from the path JIC
        if path.starts_with(&self.delim) {
            path = path.trim_start_matches(&self.delim);
        }        

        if path.is_empty() {
            
            self.var.push(val);
            true

        } else {

            let _pieces = path.split(&self.delim).collect::<Vec<&str>>();
            
            match _pieces.len() {

                0 => false,

                n => { // start the moving around...
                    
                    let _newpath = _pieces[1..].join(&self.delim);
                    let _subpath = _pieces[0].to_string();

                    let mut _tree = match self.parts.get(&_subpath) {
                        Some(_t) => _t.to_owned(),
                        _ => { 
                            Btree::<T> { 
                                delim: self.delim.clone(), 
                                head: _pieces[0].to_string(), 
                                var: Vec::new(), 
                                parts: HashMap::new() 
                            } 
                        }
                    };

                    let _success = _tree.insert(&_newpath, val);
                    self.parts.insert(_subpath, _tree);

                    _success

                }

            }

        }

    }


    pub fn enumerate(&self) -> Vec<String> {
        self.parts.keys().cloned().collect()
    }


    pub fn nodes(&self) -> Vec<Btree<T>> {
        self.parts.values().cloned().collect()
    }


    pub fn remove(&mut self, mut path: &str) -> bool {

        // strip any preceding delimiter from the path JIC
        if path.starts_with(&self.delim) {
            path = path.trim_start_matches(&self.delim);
        }

        if path.is_empty() {
            
            false

        } else {

            let _pieces = path.split(&self.delim).collect::<Vec<&str>>();

            match _pieces.len() {
                0 => false,
                1 => {
                    let _subpath = _pieces[0].to_string();                    
                    if self.parts.contains_key(&_subpath) {
                        self.parts.remove(&_subpath);
                        true
                    } else {
                        false
                    }
                },
                n => {

                    let _newpath = _pieces[1..].join(&self.delim);
                    let _subpath = _pieces[0].to_string();                    

                    if let Some(mut _t) = self.parts.get(&_subpath).cloned() {                        
                        let _success = _t.remove(&_newpath);
                        self.parts.insert(_subpath, _t);
                        _success
                    } else {
                        false // not found
                    }
                    
                }
            }

        }

    }


    pub fn node_for(&self, mut path: &str) -> Option<Btree<T>> {

        // strip any preceding delimiter from the path JIC
        if path.starts_with(&self.delim) {
            path = path.trim_start_matches(&self.delim);
        }        

        if path.is_empty() {
            Some(self.clone())
        } else {

            let _pieces = path.split(&self.delim).collect::<Vec<&str>>();

            match _pieces.len() {

                0 => Some(self.clone()),

                n => {

                    let _newpath = _pieces[1..].join(&self.delim);
                    let _subpath = _pieces[0].to_string();                    

                    match self.parts.get(&_subpath) {
                        Some(_t) => {
                            _t.node_for(&_newpath)
                        }
                        _ => None
                    }

                }

            }

        }

    }


    pub fn fetch(&self, mut path: &str) -> Vec<T> {

        // strip any preceding delimiter from the path JIC
        if path.starts_with(&self.delim) {
            path = path.trim_start_matches(&self.delim);
        }

        if path.is_empty() {
            
            if self.var.is_empty() {
                Vec::new()
            } else {
                self.var.clone()
            }

        } else {

            let _pieces = path.split(&self.delim).collect::<Vec<&str>>();

            match _pieces.len() {

                0 => {
                    if self.var.is_empty() {
                        Vec::new()
                    } else {
                        self.var.clone()
                    }
                },

                n => {

                    let _newpath = _pieces[1..].join(&self.delim);
                    let _subpath = _pieces[0].to_string();                    

                    match self.parts.get(&_subpath) {
                        
                        Some(_t) => {
                            _t.fetch(&_newpath)
                        },

                        _ => Vec::new()

                    }

                }

            }

        }

    }

}




#[cfg(test)]
mod test {

    use super::Btree;

    #[test]
    fn btree_tests() {

        let mut _newtree: Btree<String> = Btree::new("/");

        _newtree.insert("/12/456/10", "12".to_string());
        _newtree.insert("/12/456/10", "9".to_string());
        _newtree.insert("12/456/10", "13".to_string());
        _newtree.insert("/12/456/13", "17".to_string());
        _newtree.insert("/13/457/15", "19".to_string());
        _newtree.insert("/15/458/17", "20".to_string());

        println!("Tree = {_newtree:?}");

        let _find = _newtree.fetch("13/457/15");

        println!("Found = {_find:?}");

        let _node = _newtree.node_for("13");

        println!("node = {_node:?}");

        if let Some(_n) = _node {

            println!("keys = {:?}", _n.enumerate());
            println!("children = {:?}", _n.nodes());

        }

        println!("Items = {:?}", _newtree.items());

    }

}
