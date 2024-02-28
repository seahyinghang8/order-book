use slab::Slab;

struct SlabNode<T> {
    pub value: T,
    pub prev_id: Option<usize>,
    pub next_id: Option<usize>,
}

pub struct SlabLinkedList<T> {
    front_id: Option<usize>,
    back_id: Option<usize>,
    len: usize,
    slab: Slab<SlabNode<T>>,
}

impl<T> SlabLinkedList<T> {
    pub fn new() -> SlabLinkedList<T> {
        SlabLinkedList {
            front_id: None,
            back_id: None,
            len: 0,
            slab: Slab::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn front(&self) -> Option<&T> {
        Some(&self.slab[self.front_id?].value)
    }

    pub fn back(&self) -> Option<&T> {
        Some(&self.slab[self.back_id?].value)
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match (self.front_id, self.back_id) {
            (None, None) => None,
            (Some(front_id), Some(back_id)) => {
                let front_node = self.slab.remove(front_id);
                if front_id == back_id {
                    // LinkedList has one last element left
                    self.front_id = None;
                    self.back_id = None
                } else {
                    let next_id = front_node.next_id.unwrap(); // next id always valid since front != back
                    let next_node = &mut self.slab[next_id];
                    self.front_id = Some(next_id);
                    next_node.prev_id = None;
                }

                self.len -= 1;
                Some(front_node.value)
            }
            _ => panic!("Something is wrong with implementation"),
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match (self.front_id, self.back_id) {
            (None, None) => None,
            (Some(front_id), Some(back_id)) => {
                let back_node = self.slab.remove(back_id);
                if front_id == back_id {
                    // LinkedList has one last element left
                    self.front_id = None;
                    self.back_id = None
                } else {
                    let prev_id = back_node.prev_id.unwrap(); // prev id always valid since front != back
                    let prev_node = &mut self.slab[prev_id];
                    self.back_id = Some(prev_id);
                    prev_node.next_id = None;
                }

                self.len -= 1;
                Some(back_node.value)
            }
            _ => panic!("Something is wrong with implementation"),
        }
    }

    pub fn push_front(&mut self, value: T) -> usize {
        self.len += 1;

        match self.front_id {
            Some(front_id) => {
                let node_id = self.slab.insert(SlabNode {
                    value,
                    prev_id: None,
                    next_id: Some(front_id),
                });
                self.slab[front_id].prev_id = Some(node_id);
                self.front_id = Some(node_id);
                node_id
            }
            None => {
                let node_id = self.slab.insert(SlabNode {
                    value,
                    prev_id: None,
                    next_id: None,
                });
                self.front_id = Some(node_id);
                self.back_id = Some(node_id);
                node_id
            }
        }
    }

    pub fn push_back(&mut self, value: T) -> usize {
        self.len += 1;

        match self.back_id {
            Some(back_id) => {
                let node_id = self.slab.insert(SlabNode {
                    value,
                    prev_id: Some(back_id),
                    next_id: None,
                });
                self.slab[back_id].next_id = Some(node_id);
                self.back_id = Some(node_id);
                node_id
            }
            None => {
                let node_id = self.slab.insert(SlabNode {
                    value,
                    prev_id: None,
                    next_id: None,
                });
                self.front_id = Some(node_id);
                self.back_id = Some(node_id);
                node_id
            }
        }
    }

    pub fn remove(&mut self, node_id: usize) -> Option<T> {
        match (self.front_id, self.back_id) {
            (Some(front_id), Some(back_id)) => {
                if front_id == node_id {
                    self.pop_front()
                } else if back_id == node_id {
                    self.pop_back()
                } else {
                    match self.slab.try_remove(node_id) {
                        Some(removed_node) => {
                            // Node to be removed is not at the front or back of linkedlist
                            let next_id = removed_node.next_id.unwrap(); // Next id must be valid
                            let prev_id = removed_node.prev_id.unwrap(); // Prev id must be valid

                            let prev_node = &mut self.slab[prev_id];
                            prev_node.next_id = Some(next_id);

                            let next_node = &mut self.slab[next_id];
                            next_node.prev_id = Some(prev_id);

                            self.len -= 1;
                            Some(removed_node.value)
                        }
                        None => None,
                    }
                }
            }
            _ => None,
        }
    }

    // TODO: Needs testing
    pub fn get_mut(&mut self, node_id: usize) -> Option<&mut T> {
        match self.slab.get_mut(node_id) {
            Some(node) => {
                Some(&mut node.value)
            }
            None => None,
        }
    }

    // TODO: Needs testing
    pub fn get(&mut self, node_id: usize) -> Option<&T> {
        match self.slab.get(node_id) {
            Some(node) => {
                Some(&node.value)
            }
            None => None,
        }
    }

    pub fn iter(&self) -> SlabLinkedListIter<T> {
        SlabLinkedListIter {
            next_id: self.front_id,
            next_back_id: self.back_id,
            slab: &self.slab,
        }
    }
}

// Forward iterator
pub struct SlabLinkedListIter<'a, T> {
    next_id: Option<usize>,
    next_back_id: Option<usize>,
    slab: &'a Slab<SlabNode<T>>,
}

impl<'a, T> Iterator for SlabLinkedListIter<'a, T> {
    type Item = (usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.next_id, self.next_back_id) {
            (Some(node_id), Some(next_back_id)) => {
                let node = &self.slab[node_id];
                if node_id == next_back_id {
                    self.next_id = None;
                    self.next_back_id = None;
                } else {
                    self.next_id = node.next_id;
                }
                Some((node_id, &node.value))
            }
            _ => None,
        }
    }
}

// Reverse iterator
impl<'a, T> DoubleEndedIterator for SlabLinkedListIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match (self.next_back_id, self.next_id) {
            (Some(node_id), Some(next_id)) => {
                let node = &self.slab[node_id];
                if node_id == next_id {
                    self.next_id = None;
                    self.next_back_id = None;
                } else {
                    self.next_back_id = node.prev_id;
                }
                Some((node_id, &node.value))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty() {
        let mut list = SlabLinkedList::new();
        assert!(list.is_empty());
        list.push_front(1);
        assert!(!list.is_empty());
    }

    #[test]
    fn test_front_and_back() {
        let mut list = SlabLinkedList::new();
        assert_eq!(list.front(), None);
        assert_eq!(list.back(), None);

        list.push_front(1);
        assert_eq!(list.front(), Some(&1));
        assert_eq!(list.back(), Some(&1));

        list.push_back(2);
        assert_eq!(list.front(), Some(&1));
        assert_eq!(list.back(), Some(&2));
    }

    #[test]
    fn test_pop_front_and_pop_back() {
        let mut list = SlabLinkedList::new();
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);

        list.push_front(1);
        list.push_back(2);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_back(), Some(2));
        assert!(list.is_empty());
    }

    #[test]
    fn test_push_front_and_push_back() {
        let mut list = SlabLinkedList::new();
        let id1 = list.push_front(1);
        let id2 = list.push_back(2);
        assert_eq!(list.front(), Some(&1));
        assert_eq!(list.back(), Some(&2));
        assert_eq!(list.remove(id1), Some(1));
        assert_eq!(list.remove(id2), Some(2));
        assert!(list.is_empty());
    }

    #[test]
    fn test_remove_node_id() {
        let mut list = SlabLinkedList::new();
        let id1 = list.push_front(1);
        let id2 = list.push_back(2);
        assert_eq!(list.remove(id1), Some(1));
        assert_eq!(list.remove(id2), Some(2));
        assert!(list.is_empty());

        // Removing a non-existent node_id should return None
        assert_eq!(list.remove(id1), None);
    }

    #[test]
    fn test_multiple_insertions_and_removals() {
        let mut list = SlabLinkedList::new();
        let mut ids = Vec::new();

        // Insert  100 elements at the front
        for i in 0..100 {
            ids.push(list.push_front(i));
        }
        assert_eq!(list.len(), 100);

        // Remove elements from the front
        for i in (0..100).rev() {
            assert_eq!(list.pop_front(), Some(i));
        }

        assert!(list.is_empty());

        // Insert  100 elements at the back
        for i in 0..100 {
            ids.push(list.push_back(i));
        }
        assert_eq!(list.len(), 100);

        // Remove elements from the back
        for i in (0..100).rev() {
            assert_eq!(list.pop_back(), Some(i));
        }
        assert!(list.is_empty());
    }

    #[test]
    fn test_random_insertions_and_removals() {
        let mut list = SlabLinkedList::new();
        let mut ids = Vec::new();

        // Insert elements randomly
        for i in 0..100 {
            if i % 2 == 0 {
                ids.push(list.push_front(i));
            } else {
                ids.push(list.push_back(i));
            }
        }
        assert_eq!(list.len(), 100);

        // Remove elements in reverse order of insertion
        for i in (0..100).rev() {
            assert_eq!(list.remove(ids[i]), Some(i));
        }
        assert!(list.is_empty());
    }

    #[test]
    fn test_list_near_capacity() {
        let mut list = SlabLinkedList::new();
        // Assuming the list has a capacity of  1000, this test might need adjustment based on the actual implementation
        for i in 0..999 {
            list.push_back(i);
        }
        assert_eq!(list.len(), 999);
        // Attempting to add one more element should not panic or cause undefined behavior
        list.push_back(999);
        assert_eq!(list.len(), 1000);

        // Removing elements should not cause issues
        for _ in 0..1000 {
            list.pop_front();
        }
        assert!(list.is_empty());
    }

    #[test]
    fn test_remove_non_existent_node_id() {
        let mut list = SlabLinkedList::new();
        let id1 = list.push_front(1);
        let id2 = list.push_back(2);
        // Removing a non-existent node_id should return None
        assert_eq!(list.remove(id1 + 1000), None); // Assuming the list's capacity is  1000
        assert_eq!(list.remove(id2 + 1000), None);
    }

    #[test]
    fn test_remove_all_elements() {
        let mut list = SlabLinkedList::new();
        for i in 0..100 {
            list.push_back(i);
        }
        assert_eq!(list.len(), 100);

        // Remove all elements
        for _ in 0..100 {
            list.pop_front();
        }
        assert!(list.is_empty());
    }

    #[test]
    fn test_forward_iterator() {
        let mut list = SlabLinkedList::new();

        let mut id_vec = Vec::new();
        // Insert elements into the list
        id_vec.push(list.push_back(1));
        id_vec.push(list.push_back(2));
        id_vec.push(list.push_back(3));

        // Use the forward iterator to traverse the list
        let mut iter = list.iter();
        let expected_values = vec![1, 2, 3];

        // Iterate over the list and check if the values are as expected
        for (expected_value, &expected_id) in expected_values.iter().zip(id_vec.iter()) {
            match iter.next() {
                Some(value) => assert_eq!(value, (expected_id, expected_value), "Value mismatch"),
                None => panic!("Iterator returned None unexpectedly"),
            }
        }

        // Check if the iterator is exhausted
        assert_eq!(iter.next(), None, "Iterator should be exhausted");
        assert_eq!(
            list.len(),
            expected_values.len(),
            "List should have the same number of elements before iter"
        );
    }

    #[test]
    fn test_rev_iterator() {
        let mut list = SlabLinkedList::new();

        let mut id_vec = Vec::new();
        // Insert elements into the list
        id_vec.push(list.push_back(1));
        id_vec.push(list.push_back(2));
        id_vec.push(list.push_back(3));

        // Use the reverse iterator to traverse the list
        let mut iter_rev = list.iter().rev();
        let expected_values = vec![1, 2, 3];

        // Iterate over the list in reverse and check if the values are as expected
        for (expected_value, &expected_id) in expected_values.iter().rev().zip(id_vec.iter().rev())
        {
            match iter_rev.next() {
                Some(value) => assert_eq!(value, (expected_id, expected_value), "Value mismatch"),
                None => panic!("Iterator returned None unexpectedly"),
            }
        }

        // Check if the iterator is exhausted
        assert_eq!(iter_rev.next(), None, "Iterator should be exhausted");
        assert_eq!(
            list.len(),
            expected_values.len(),
            "List should have the same number of elements before iter"
        );
    }

    #[test]
    fn test_iteration_both_directions() {
        let mut list = SlabLinkedList::new();

        let mut id_vec = Vec::new();
        // Insert elements into the list
        id_vec.push(list.push_back(1));
        id_vec.push(list.push_back(2));
        id_vec.push(list.push_back(3));

        let mut iter = list.iter();

        assert_eq!(iter.next(), Some((id_vec[0], &1)), "First value mismatch");
        assert_eq!(
            iter.next_back(),
            Some((id_vec[2], &3)),
            "Last value mismatch"
        );
        assert_eq!(
            iter.next_back(),
            Some((id_vec[1], &2)),
            "Middle value mismatch"
        );
        assert_eq!(iter.next(), None, "Iter should be exhausted");
        assert_eq!(iter.next_back(), None, "Iter should be exhausted");
    }
}
