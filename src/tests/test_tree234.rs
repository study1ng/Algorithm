#[cfg(test)]
mod tests {
    use super::super::super::tree234::*;
    use std::fmt::Debug;


    #[test]
    fn test_new() {
        let tree = Tree234::<usize>::new();
        assert_eq!(tree.size, 0);
        assert_eq!(tree.data, [None, None, None, None]);
        assert_eq!(tree.children, [None, None, None, None]);
    }

    #[test]
    fn test_clear() {
        let mut tree = Tree234::<usize>::new();
        tree.size = 3;
        tree.data = [Some(1), Some(2), Some(3), None];
        tree.children = [
            Some(Box::new(Tree234::<usize> {
                data: [Some(4), Some(5), None, None],
                children: [None, None, None, None],
                size: 2,
            })),
            Some(Box::new(Tree234::<usize> {
                data: [Some(6), Some(7), None, None],
                children: [None, None, None, None],
                size: 2,
            })),
            None,
            None,
        ];
        tree.clear();
        assert_eq!(tree.size, 0);
        assert_eq!(tree.data, [None, None, None, None]);
        assert_eq!(tree.children, [None, None, None, None]);
    }

    #[test]
    fn test_is_empty() {
        let mut tree = Tree234::<usize>::new();
        assert!(tree.is_empty());
        tree.size = 1;
        assert!(!tree.is_empty());
    }

    #[test]
    fn test_insert() {
        let mut tree = Tree234::<usize>::new();
        tree.insert(1);
        assert_eq!(tree.size, 1);
        assert_eq!(tree.data, [Some(1), None, None, None]);
        tree.insert(2);
        assert_eq!(tree.size, 2);
        assert_eq!(tree.data, [Some(1), Some(2), None, None]);
        tree.insert(3);
        assert_eq!(tree.size, 3);
        assert_eq!(tree.data, [Some(1), Some(2), Some(3), None]);
        tree.insert(4);
        assert_eq!(tree.size, 1);
        assert_eq!(tree.data, [Some(2), None, None, None]);
        assert_eq!(
            tree.children[0].as_ref().unwrap().data,
            [Some(1), None, None, None]
        );
        assert_eq!(tree.children[0].as_ref().unwrap().size, 1);
        assert_eq!(
            tree.children[1].as_ref().unwrap().data,
            [Some(3), Some(4), None, None]
        );
        assert_eq!(tree.children[1].as_ref().unwrap().size, 2);
        tree.clear();
        for i in vec![1, 2, 3, 4, 5, 6, 7, 8, 9] {
            tree.insert(i);
            check(&tree);
        }
        for i in vec![92, 40, 54, 53, 58, 24, 88, 59, 35, 30, 70, 42, 79, 96, 5, 49, 17, 43, 74, 82, 98, 13, 84, 16, 73, 63, 90] {
            tree.insert(i);
            check(&tree);
        }
    }

    #[test]
    fn test_find() {
        let mut tree = Tree234::new();
        assert!(!tree.find(&1));
        tree.append(vec![1, 2, 3, 4, 5]);
        assert!(tree.find(&2));
        assert!(tree.find(&1));
        assert!(tree.find(&3));
        assert!(tree.find(&4));
        assert!(tree.find(&5));
        assert!(!tree.find(&6));
        tree.clear();
        tree.append(vec![92, 40, 54, 53, 58, 24, 88, 59, 35, 30, 70, 42, 79, 96, 5, 49, 17, 43, 74, 82, 98, 13, 84, 16, 73, 63, 90]);
        assert!(tree.find(&92));
        assert!(tree.find(&30));
        assert!(!tree.find(&1));

    }

    #[test]
    fn test_delete() {
        let mut tree = Tree234::<usize>::new();
        assert!(!tree.delete(&1));
        tree.append(vec![1, 2, 3, 4, 5]);
        assert!(tree.delete(&2));
        assert!(!tree.delete(&2));
        check(&tree);
        assert!(tree.delete(&1));
        check(&tree);
        tree.clear();
        tree.append(vec![92, 40, 54, 53, 58, 24, 88, 59, 35, 30, 70, 42, 79, 96, 5, 49, 17, 43, 74, 82, 98, 13, 84, 16, 73, 63, 90]);
        assert!(tree.delete(&92));
        check(&tree);
        assert!(tree.delete(&30));
        check(&tree);
        assert!(!tree.delete(&1));
        check(&tree);
    }

    fn first_data<T: Clone>(tree: &Option<Box<Tree234<T>>>) -> T {
        let tree = tree.as_ref().unwrap();
        let tree = tree.as_ref();
        tree.data[0].clone().unwrap()
    }

    fn last_data<T: Clone>(tree: &Option<Box<Tree234<T>>>) -> T {
        let tree = tree.as_ref().unwrap();
        let tree = tree.as_ref();
        tree.data[tree.size - 1].clone().unwrap()
    }

    fn check<T: Ord + Debug + Clone>(tree: &Tree234<T>) {
        // if self.data.size == self.size && self.children.size == self.size + 1 && self.data.take(self.size) is all Some
        assert!(tree.size <= 3);
        assert_eq!(tree.size, tree.data.iter().filter(|x| x.is_some()).count());
        assert!(tree.data.iter().take(tree.size).all(|x| x.is_some()));
        if tree.is_leaf() {
            assert!(tree.children.iter().all(|x| x.is_none()));
        } else {
            assert_eq!(
                tree.size + 1,
                tree.children.iter().filter(|x| x.is_some()).count()
            );
            assert!(tree
                .children
                .iter()
                .take(tree.size + 1)
                .all(|x| x.is_some()));
            // if self.children[0].data.last < self.data[0] < self.children[1].data.first < self.children[1].data.last < self.data[1] < ...
            let mut v = vec![];
            for i in 0..tree.size {
                v.push(first_data(&tree.children[i]));
                v.push(last_data(&tree.children[i]));
                v.push(tree.data[i].clone().unwrap());
            }
            v.push(first_data(&tree.children[tree.size]));
            assert!(v.windows(2).all(|x| x[0] <= x[1]), "{:?}", tree);
            // all depth is equal
            check_depth(tree);
        }
    }

    fn check_depth<T: Debug>(tree: &Tree234<T>) -> usize {
        if tree.is_leaf() {
            return 1;
        }
        let depths = tree
            .children
            .iter()
            .filter_map(|x| x.as_ref().map(|x| check_depth(x)))
            .enumerate()
            .collect::<Vec<_>>();
        depths.windows(2).for_each(|x| assert_eq!(x[0].1, x[1].1, "{:?}, {:?}\n{:#?}", x[0], x[1], tree));
        depths[0].1 + 1
    }

}
