// 06/09/2019
//
// Given an array of N sorted linked lists. Merge them and return it as one
// sorted linked list.
//
//   /**
//    * Definition for singly-linked list.
//    */
//    function ListNode(val) {
//        this.val = val;
//        this.next = null;
//    }
//    /**
//     * @param {ListNode[]} lists
//     * @return {ListNode}
//     */
//    var mergeNLists = function(lists) {
//
//    }/
//
// Example:
//  Input:
//  [
//    1 -> 10 -> 20,
//    4 -> 11 -> 13,
//    3 -> 8 -> 9
//  ]
//
//  Output: 1->3->4->8->9->10->11->13->20
//
//  Input:
//  [
//    1->4->5,
//    2->6
//  ]
//
//  Output: 1->2->4->5->6

use std::collections::{BinaryHeap, LinkedList};

fn merge_linked_lists(lists: &Vec<LinkedList<i32>>) -> LinkedList<i32> {
    let mut heap = BinaryHeap::new();
    for list in lists {
        for &item in list.iter() {
            heap.push(item);
        }
    }
    let mut merged = LinkedList::new();
    while let Some(item) = heap.pop() {
        merged.push_front(item);
    }

    return merged;
}

fn main() {
    let lists = vec![
      [1, 10, 20].into_iter().map(|x| *x).collect::<LinkedList<i32>>(),
      [4, 11, 13].into_iter().map(|x| *x).collect::<LinkedList<i32>>(),
      [3, 8, 9].into_iter().map(|x| *x).collect::<LinkedList<i32>>()
    ];

    let merged = merge_linked_lists(&lists);
    for i in merged.iter() {
        print!("{} ", i);
    }
    println!("");

    let lists = vec![
      [1, 4, 5].into_iter().map(|x| *x).collect::<LinkedList<i32>>(),
      [2, 6].into_iter().map(|x| *x).collect::<LinkedList<i32>>()
    ];

    let merged = merge_linked_lists(&lists);
    for i in merged.iter() {
        print!("{} ", i);
    }
    println!("");
}
