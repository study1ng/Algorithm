use std::{fmt::Debug, ptr};

#[derive(Debug, Clone)]
pub struct Tree234<T> {
    pub(crate) data: [Option<T>; 4],
    pub(crate) children: [Option<Box<Tree234<T>>>; 4],
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

    pub fn clear(&mut self) {
        self.data = [None, None, None, None];
        self.children = [None, None, None, None];
        self.size = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl<T: Ord + Debug> Tree234<T> {
    pub fn contains(&self, value: &T) -> bool {
        if self.size == 0 {
            return false;
        }
        for i in 0..self.size {
            if value == self.data[i].as_ref().unwrap() {
                return true;
            }
            if value > self.data[i].as_ref().unwrap() {
                return self.children[i].as_ref().unwrap().contains(value);
            }
        }
        return false;
    }

    pub fn append(&mut self, values: Vec<T>) {
        for i in values {
            self.insert(i);
        }
    }

    pub fn insert(&mut self, value: T) {
        if self.size == 3 {
            // 自分がrootかつsizeが3の場合 if以下はすべてinsertが呼び出される前に分割されるのでここには入らない.
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

    /**
     * 値を探す.
     */
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

    fn shrink(&mut self) {
        // self.data = [left.data, self.data, right.data, None];
        // self.children = [*left.children, *right.children, None, None];
        self.size = 3;
        insert_to_array(&mut self.data, 0, self.children[0].as_mut().unwrap().data[0].take());
        self.data[2] = self.children[1].as_mut().unwrap().data[0].take();
        assert!(self.children[1].as_mut().unwrap().children[0].is_some());
        self.children[2] = self.children[1].as_mut().unwrap().children[0].take();
        self.children[3] = self.children[1].as_mut().unwrap().children[1].take();
        self.children[1] = self.children[0].as_mut().unwrap().children[1].take();
        self.children[0] = self.children[0].as_mut().unwrap().children[0].take();
    }

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
    /**
     * 指定した値を探し, (その値以下のもののうち一番大きい値, その値以上のもののうち一番小さい値)
     */
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
    pub(crate) fn is_leaf(&self) -> bool {
        self.children[0].is_none()
    }
}

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
