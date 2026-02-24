
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

    fn normalize_path<'a>(path: &'a str, delim: &str) -> &'a str {
        if path.starts_with(delim) {
            path.trim_start_matches(delim)
        } else {
            path
        }
    }

    fn split_path<'a>(path: &'a str, delim: &str) -> (&'a str, &'a str) {
        if delim.is_empty() {
            return (path, "");
        }

        match path.split_once(delim) {
            Some((head, tail)) => (head, tail),
            None => (path, ""),
        }
    }

    fn collect_items(&self, out: &mut Vec<T>) {
        out.extend(self.var.iter().cloned());
        for part in self.parts.values() {
            part.collect_items(out);
        }
    }


    pub fn new(delim: &str) -> Self {
        Self { delim: delim.to_string(), head: "".to_string(), var: Vec::new(), parts: HashMap::new() }
    }


    // recurse the binary tree enumerating all items..
    pub fn items(&self) -> Vec<T> {
        let mut out = Vec::new();
        self.collect_items(&mut out);
        out

    }


    pub fn replace(&mut self, mut path: &str, vals: &Vec<T>) -> bool {
        path = Self::normalize_path(path, &self.delim);

        if path.is_empty() {
            
            self.var = vals.clone();
            true

        } else {
            let (subpath, newpath) = Self::split_path(path, &self.delim);
            if subpath.is_empty() {
                return false;
            }

            if let Some(tree) = self.parts.get_mut(subpath) {
                tree.replace(newpath, vals)
            } else {
                let mut tree = Btree::<T> {
                    delim: self.delim.clone(),
                    head: subpath.to_string(),
                    var: vals.clone(),
                    parts: HashMap::new(),
                };

                let success = tree.replace(newpath, vals);
                self.parts.insert(subpath.to_string(), tree);
                success
            }

        }

    }


    pub fn insert(&mut self, mut path: &str, val: T) -> bool {
        path = Self::normalize_path(path, &self.delim);

        if path.is_empty() {
            
            self.var.push(val);
            true

        } else {
            let (subpath, newpath) = Self::split_path(path, &self.delim);
            if subpath.is_empty() {
                return false;
            }

            if let Some(tree) = self.parts.get_mut(subpath) {
                tree.insert(newpath, val)
            } else {
                let mut tree = Btree::<T> {
                    delim: self.delim.clone(),
                    head: subpath.to_string(),
                    var: Vec::new(),
                    parts: HashMap::new(),
                };

                let success = tree.insert(newpath, val);
                self.parts.insert(subpath.to_string(), tree);
                success
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
        path = Self::normalize_path(path, &self.delim);

        if path.is_empty() {
            
            false

        } else {
            let (subpath, newpath) = Self::split_path(path, &self.delim);
            if subpath.is_empty() {
                return false;
            }

            if newpath.is_empty() {
                self.parts.remove(subpath).is_some()
            } else {
                match self.parts.get_mut(subpath) {
                    Some(tree) => tree.remove(newpath),
                    None => false,
                }
            }

        }

    }


    pub fn node_for(&self, path: &str) -> Option<Btree<T>> {
        self.node_for_ref(path).cloned()
    }


    pub fn node_for_ref(&self, mut path: &str) -> Option<&Btree<T>> {
        path = Self::normalize_path(path, &self.delim);

        if path.is_empty() {
            Some(self)
        } else {
            let (subpath, newpath) = Self::split_path(path, &self.delim);
            if subpath.is_empty() {
                return Some(self);
            }

            match self.parts.get(subpath) {
                Some(t) => t.node_for_ref(newpath),
                None => None,
            }

        }

    }


    pub fn fetch(&self, path: &str) -> Vec<T> {
        self.fetch_ref(path).map(|vals| vals.to_vec()).unwrap_or_default()
    }


    pub fn fetch_ref(&self, path: &str) -> Option<&[T]> {
        self.node_for_ref(path).map(|node| node.var.as_slice())
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
        let _find_ref = _newtree.fetch_ref("13/457/15");

        println!("Found = {_find:?}");
        println!("Found Ref = {_find_ref:?}");

        assert_eq!(_find_ref, Some(["19".to_string()].as_slice()));

        let _node = _newtree.node_for("13");
        let _node_ref = _newtree.node_for_ref("13");

        println!("node = {_node:?}");
        println!("node_ref = {_node_ref:?}");

        assert!(_node_ref.is_some());

        if let Some(_n) = _node {

            println!("keys = {:?}", _n.enumerate());
            println!("children = {:?}", _n.nodes());

        }

        println!("Items = {:?}", _newtree.items());

    }

}
