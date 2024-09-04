use std::{fmt::Debug, ptr};

///
/// A tree struct which implements 2-3-4 tree.
///
/// # The benefits of 2-3-4 tree
/// - The height of the tree is always O(log n).
/// - The tree is always balanced.
/// - The tree is always sorted.
/// - Search, insert, and delete operations are O(log n).
///
/// # Example
/// ```
/// use algorithm::Tree234;
/// let mut tree = Tree234::new();
/// tree.insert(1);
/// tree.insert(2);
/// tree.insert(3);
/// assert(tree.find(&1));
/// assert(tree.find(&2));
/// assert(!tree.find(&4));
/// tree.delete(&2);
/// assert(!tree.find(&2));
/// ```
///
///
#[derive(Debug, Clone)]
pub struct Tree234<T> {
    /// 
    /// array which contains node's data.
    /// 
    pub(crate) data: [Option<T>; 4],
    /// 
    /// array which contains node's children.
    /// 
    pub(crate) children: [Option<Box<Tree234<T>>>; 4],
    /// 
    /// size of node.
    /// 0 <= size <= 3
    /// 
    pub(crate) size: usize,
}

impl<T: PartialEq> PartialEq for Tree234<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }
        for i in 0..self.size {
            if self.data[i].as_ref().unwrap() != other.data[i].as_ref().unwrap() {
                return false;
            }
        }
        for i in 0..=self.size {
            if self.children[i].as_ref().unwrap() != other.children[i].as_ref().unwrap() {
                return false;
            }
        }
        true
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl<T: Eq> Eq for Tree234<T> {}

impl<T: Ord + Debug> From<Vec<T>> for Tree234<T> {
    ///
    /// a method to make a Tree234 from a Vec.
    /// ```
    /// use algorithm::Tree234;
    /// let tree = Tree234::from(vec![1, 2, 3]);
    /// assert(tree.find(&1));
    /// ```
    ///
    fn from(v: Vec<T>) -> Self {
        let mut tree = Tree234::new();
        for i in v {
            tree.insert(i);
        }
        tree
    }
}

impl<T> Tree234<T> {
    pub fn new() -> Self {
        Self {
            data: [None, None, None, None],
            children: [None, None, None, None],
            size: 0,
        }
    }

    ///
    /// clear the tree.
    ///
    pub fn clear(&mut self) {
        self.data = [None, None, None, None];
        self.children = [None, None, None, None];
        self.size = 0;
    }

    ///
    /// check if the tree is empty.
    ///
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl<T: Ord> Tree234<T> {
    ///
    /// append all value in values to self.
    /// ```rust
    /// use algorithm::Tree234;
    /// let mut tree = Tree234::new();
    /// tree.append(vec![1, 2, 3]);
    /// assert(tree.find(&1));
    /// assert(tree.find(&2));
    /// assert(tree.find(&3));
    /// ```
    ///
    pub fn append(&mut self, values: Vec<T>) {
        for i in values {
            self.insert(i);
        }
    }
    ///
    /// insert value to self
    /// ```rust
    /// use algorithm::Tree234;
    /// let mut tree = Tree234::new();
    /// tree.insert(1);
    /// tree.insert(2);
    /// ```
    ///
    pub fn insert(&mut self, value: T) {
        if self.size == 3 {
            // 自分がrootかつsizeが3の場合のみ if以下はすべてinsertが呼び出される前に分割されるのでここには入らない.
            let mid = self.data[1].take();
            let left = Box::new(Self {
                data: [self.data[0].take(), None, None, None],
                children: [self.children[0].take(), self.children[1].take(), None, None],
                size: 1,
            });
            let right = Box::new(Self {
                data: [self.data[2].take(), None, None, None],
                children: [self.children[2].take(), self.children[3].take(), None, None],
                size: 1,
            });
            self.data = [mid, None, None, None];
            self.children = [Some(left), Some(right), None, None];
            self.size = 1;
        }

        let mut pos = self.find_index(&value);
        if self.is_leaf() {
            insert_to_array(&mut self.data, pos, Some(value));
            self.size += 1;
            return;
        }

        if self.children[pos].as_ref().unwrap().size == 3 {
            let mut child = delete_from_array(&mut self.children, pos).unwrap();
            let mid = child.data[1].take();
            let left = Box::new(Self {
                data: [child.data[0].take(), None, None, None],
                children: [
                    child.children[0].take(),
                    child.children[1].take(),
                    None,
                    None,
                ],
                size: 1,
            });
            let right = Box::new(Self {
                data: [child.data[2].take(), None, None, None],
                children: [
                    child.children[2].take(),
                    child.children[3].take(),
                    None,
                    None,
                ],
                size: 1,
            });
            let pos_ = self.find_index(mid.as_ref().unwrap());
            insert_to_array(&mut self.data, pos_, mid);
            insert_to_array(&mut self.children, pos_, Some(right));
            insert_to_array(&mut self.children, pos_, Some(left));
            self.size += 1;
            pos = self.find_index(&value);
        }
        self.children[pos].as_mut().unwrap().insert(value);
    }
    /// 
    /// if value is in self, delete it and return true. else return false
    /// ```rust
    /// use algorithm::Tree234;
    /// let mut tree = Tree234::new();
    /// tree.insert(1);
    /// assert(tree.delete(&1));
    /// assert(!tree.delete(&1));
    /// ```
    /// 
    pub fn delete(&mut self, value: &T) -> bool {
        if self.is_leaf() {
            let pos = self.find_index(value);
            if pos < self.size && self.data[pos].as_ref().unwrap() == value {
                delete_from_array(&mut self.data, pos);
                self.size -= 1;
                return true;
            }
            return false;
        }
        let pos = self.find_index(value);
        let is_internal = pos < self.size && self.data[pos].as_ref().unwrap() == value;
        if !is_internal && self.children[pos].as_ref().unwrap().size > 1 {
            // いつかはここに引っかかるはず
            // 子ノードの大きさが2以上の場合, それを起点に再帰的に削除を行う.
            return self.children[pos].as_mut().unwrap().delete(value);
        }
        if !is_internal {
            // internalでないかつ子ノードの大きさが1の場合
            self.delete_balance(pos);
            return self.delete(value);
        }

        self.delete_balance(pos);
        // 内部ノードである場合, 通り道全体にrotateとかしてから左側最大と交換して削除する
        let mut current = self.children[pos].as_mut().unwrap();
        while !current.is_leaf() {
            current.delete_balance(current.size);
            current = current.children[current.size].as_mut().unwrap();
        }
        let pos = self.find_index(value);
        let is_internal = pos < self.size && self.data[pos].as_ref().unwrap() == value;
        if is_internal {
            let mut current = self.children[pos].as_mut().unwrap();
            while !current.is_leaf() {
                current.delete_balance(current.size);
                current = current.children[current.size].as_mut().unwrap();
            }
            std::mem::swap(&mut self.data[pos], &mut current.data[current.size - 1]);
            return current.delete(value);
        }
        self.delete(value)
    }

    /// 
    /// make the node balanced for deletion.
    /// 
    fn delete_balance(&mut self, pos: usize) {
        // 隣接兄弟ノードの大きさが2以上の場合, 回転を行う
        if (pos > 0 && self.children[pos - 1].as_ref().unwrap().size > 1)
            || (pos < self.size && self.children[pos + 1].as_ref().unwrap().size > 1)
        {
            self.rotate(pos);
        } else if self.size > 1 {
            // 隣接兄弟ノードの大きさが1で親要素の大きさが2以上の場合, マージを行う
            self.merge(pos);
        } else {
            // 高さを1下げる.
            self.shrink();
        }
    }

    /// 
    /// check if the tree contains value.
    /// ```rust
    /// use algorithm::Tree234;
    /// let mut tree = Tree234::new();
    /// tree.insert(1);
    /// assert(tree.find(&1));
    /// assert(!tree.find(&2));
    /// ```
    /// 
    pub fn find(&self, value: &T) -> bool {
        let pos = self.find_index(value);
        if pos < self.size && self.data[pos].as_ref().unwrap() == value {
            return true;
        }
        if self.is_leaf() {
            return false;
        }
        return self.children[pos].as_ref().unwrap().find(value);
    }

    /// make self.children[pos] contains more than 1 element.
    /// this should only be called when one of its sibling has more than 1 element.
    fn rotate(&mut self, pos: usize) {
        if pos > 0 && self.children[pos - 1].as_ref().unwrap().size > 1 {
            // 左の兄弟から値を持ってくる
            let brother_pos = pos - 1;
            let parent_pos = pos - 1;
            // data = [parent_data, self.data, None];
            // children = [*brother.children, *self.children];
            let parent_data = delete_from_array(&mut self.data, parent_pos);
            insert_to_array(
                &mut self.children[pos].as_mut().unwrap().data,
                0,
                parent_data,
            );
            let brother_size = self.children[brother_pos].as_mut().unwrap().size;
            let brother_data = delete_from_array(
                &mut self.children[brother_pos].as_mut().unwrap().data,
                brother_size - 1,
            );
            insert_to_array(&mut self.data, parent_pos, brother_data);
            let brother_child = delete_from_array(
                &mut self.children[brother_pos].as_mut().unwrap().children,
                brother_size,
            );
            let children = &mut self.children[pos].as_mut().unwrap().children;
            insert_to_array(children, 0, brother_child);
            self.children[brother_pos].as_mut().unwrap().size -= 1;
            self.children[pos].as_mut().unwrap().size = 2;
        } else {
            // 右の兄弟から値を持ってくる
            let brother_pos = pos + 1;
            let parent_pos = pos;
            // data = [self.data, parent_data, None];
            // children = [*brother.children, *self.children];
            let parent_data = delete_from_array(&mut self.data, parent_pos);
            insert_to_array(
                &mut self.children[pos].as_mut().unwrap().data,
                1,
                parent_data,
            );
            let brother_data =
                delete_from_array(&mut self.children[brother_pos].as_mut().unwrap().data, 0);
            insert_to_array(&mut self.data, parent_pos, brother_data);
            let brother_child = delete_from_array(
                &mut self.children[brother_pos].as_mut().unwrap().children,
                0,
            );
            let size = self.children[pos].as_mut().unwrap().size;
            let children = &mut self.children[pos].as_mut().unwrap().children;
            insert_to_array(children, size, brother_child);
            self.children[brother_pos].as_mut().unwrap().size -= 1;
            self.children[pos].as_mut().unwrap().size = 2;
        };
    }

    /// 
    /// make self.children[pos] contains more than 1 element.
    /// this should only be called when self.size > 1 and all of its sibling has only 1 element.
    /// 
    fn merge(&mut self, pos: usize) {
        // 兄弟要素の値と親要素のいい感じの値を自分のdataとし, 兄弟要素の子要素を自分の子要素とする.
        self.size -= 1;
        self.children[pos].as_mut().unwrap().size = 3;
        if pos == 3 {
            let brother_pos = pos - 1;
            let parent_pos = pos - 1;
            let parent_data = delete_from_array(&mut self.data, parent_pos);
            let brother_data =
                delete_from_array(&mut self.children[brother_pos].as_mut().unwrap().data, 0);
            let data = &mut self.children[pos].as_mut().unwrap().data;
            insert_to_array(data, 0, parent_data);
            insert_to_array(data, 0, brother_data);
            let brother_child0 = delete_from_array(
                &mut self.children[brother_pos].as_mut().unwrap().children,
                0,
            );
            let brother_child1 = delete_from_array(
                &mut self.children[brother_pos].as_mut().unwrap().children,
                0,
            );
            let children = &mut self.children[pos].as_mut().unwrap().children;
            insert_to_array(children, 0, brother_child1);
            insert_to_array(children, 0, brother_child0);
        } else {
            let brother_pos = pos + 1;
            let parent_pos = pos;
            let parent_data = delete_from_array(&mut self.data, parent_pos);
            let brother_data =
                delete_from_array(&mut self.children[brother_pos].as_mut().unwrap().data, 0);
            let data = &mut self.children[pos].as_mut().unwrap().data;
            data[2] = parent_data;
            data[3] = brother_data;
            let brother_child0 = delete_from_array(
                &mut self.children[brother_pos].as_mut().unwrap().children,
                0,
            );
            let brother_child1 = delete_from_array(
                &mut self.children[brother_pos].as_mut().unwrap().children,
                0,
            );
            let children = &mut self.children[pos].as_mut().unwrap().children;
            children[2] = brother_child0;
            children[3] = brother_child1;
        }
    }

    /// 
    /// make self.size == 1 and all of its sibling has only 1 element.
    /// this should only be called when self.size == 1 and all of its sibling has only 1 element.
    /// 
    fn shrink(&mut self) {
        // self.data = [left.data, self.data, right.data, None];
        // self.children = [*left.children, *right.children];
        self.size = 3;
        insert_to_array(
            &mut self.data,
            0,
            self.children[0].as_mut().unwrap().data[0].take(),
        );
        self.data[2] = self.children[1].as_mut().unwrap().data[0].take();
        assert!(self.children[1].as_mut().unwrap().children[0].is_some());
        self.children[3] = self.children[1].as_mut().unwrap().children[1].take();
        self.children[2] = self.children[1].as_mut().unwrap().children[0].take();
        self.children[1] = self.children[0].as_mut().unwrap().children[1].take();
        self.children[0] = self.children[0].as_mut().unwrap().children[0].take();
    }

    /// 
    /// find the index of self's child which may contains value
    /// if value is in self.data, return the index of self.data
    /// 
    fn find_index(&self, value: &T) -> usize {
        // 挿入する場合, どの位置に挿入するべきかを返す.
        for i in 0..self.size {
            if value <= self.data[i].as_ref().unwrap() {
                return i;
            }
        }
        self.size
    }
}

impl<T: Ord + Clone> Tree234<T> {
    /// 
    /// search the value and return (lower_bound, upper_bound)
    /// lower_bound <= value <= upper_bound
    /// if lower_bound is None, value is smaller than all of the values in the tree.
    /// if upper_bound is None, value is larger than all of the values in the tree.
    /// 
    pub fn search_and_get_range(&self, value: &T) -> (Option<&T>, Option<&T>) {
        if self.is_empty() {
            return (None, None);
        }
        if self.is_leaf() {
            let mut left = None;
            let mut right = None;
            for i in 0..self.size {
                if self.data[i].as_ref().unwrap() <= value {
                    left = Some(self.data[i].as_ref().unwrap());
                }
                if self.data[i].as_ref().unwrap() >= value {
                    right = Some(self.data[i].as_ref().unwrap());
                    break;
                }
            }
            return (left, right);
        }

        for i in 0..self.size {
            if value < self.data[i].as_ref().unwrap() {
                let mut ans = self.children[i]
                    .as_ref()
                    .unwrap()
                    .search_and_get_range(value);
                if ans.1.is_none() {
                    ans.1 = self.data[i].as_ref();
                }
                return ans;
            }
        }
        let mut ans = self.children[self.size]
            .as_ref()
            .unwrap()
            .search_and_get_range(value);
        if ans.0.is_none() {
            ans.0 = self.data[self.size - 1].as_ref();
        }
        return ans;
    }
}

impl<T> Tree234<T> {
    /// 
    /// check if self is a leaf node.
    /// 
    pub(crate) fn is_leaf(&self) -> bool {
        self.children[0].is_none()
    }
}

/// 
/// Insert value to array[index], shifting the rest of the array to the right.
/// 
fn insert_to_array<S>(array: &mut [S], index: usize, value: S) {
    #[cold]
    #[cfg_attr(not(feature = "panic_immediate_abort"), inline(never))]
    #[track_caller]
    fn assert_failed(index: usize, len: usize) -> ! {
        panic!("insertion index (is {index}) should be <= len (is {len})");
    }
    let len = 3;

    if index > len {
        assert_failed(index, len);
    }

    unsafe {
        // infallible
        // The spot to put the new value
        let p = array.as_mut_ptr().add(index);
        // Shift everything over to make space. (Duplicating the
        // `index`th element into two consecutive places.)
        ptr::copy(p, p.add(1), len - index);
        // Write it in, overwriting the first copy of the `index`th
        // element.
        ptr::write(p, value);
    }
}

/// 
/// remove array[index], shifting the rest of the array to the left.
/// 
fn delete_from_array<S>(array: &mut [S], index: usize) -> S {
    #[cold]
    #[cfg_attr(not(feature = "panic_immediate_abort"), inline(never))]
    #[track_caller]
    fn assert_failed(index: usize, len: usize) -> ! {
        panic!("insertion index (is {index}) should be <= len (is {len})");
    }
    let len = 3;

    if index > len {
        assert_failed(index, len);
    }

    unsafe {
        // infallible
        // The spot to put the new value
        let p = array.as_mut_ptr().add(index);
        let ret = ptr::read(p);
        // Shift everything over to make space. (Duplicating the
        // `index`th element into two consecutive places.)
        ptr::copy(p.add(1), p, len - index);
        return ret;
    }
}
