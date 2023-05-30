## Rust's collections can be grouped into four major categories:
- Sequences: Vec, VecDeque, LinkedList
- Maps: HashMap, BTreeMap
- Sets: HashSet, BTreeSet
- Misc: BinaryHeap

## When Should You Use Which Collection?
#### Use a Vec when:
- You want to collect items up to be processed or sent elsewhere later, and don't care about any properties of the actual values being stored.
- You want a sequence of elements in a particular order, and will only be appended to (or near) the end.
- You want a stack.
- You want a resizable array.
- You want a heap-allocated array.

#### Use a VecDeque when:
- You want a Vec that supports efficient insertion at both ends of the sequence.
- You want a queue.
- You want a double-ended queue (deque).

#### Use a LinkedList when:
- You want a Vec or VecDeque of unknown size, and can't tolerate amortization.
- You want to efficiently split and append lists.
- You are absolutely certain you really, truly, want a doubly linked list.

#### Use a HashMap when:
- You want to associate arbitrary keys with an arbitrary value.
- You want a cache.
- You want a map, with no extra functionality.

#### Use a BTreeMap when:
- You want a map sorted by its keys.
- You want to be able to get a range of entries on-demand.
- You're interested in what the smallest or largest key-value pair is.
- You want to find the largest or smallest key that is smaller or larger than something.

#### Use the Set variant of these Maps when:
- You just want to remember which keys you've seen.
- There is no meaningful value to associate with your keys.
- You just want a set.

#### Use a BinaryHeap when:
- You want to store a bunch of elements, but only ever want to process the "biggest" or "most important" one at any given time.
- You want a priority queue.

## Performance
|            | get(i)         | insert(i)       | remove(i)      | append | split_off(i)   |
|------------|----------------|-----------------|----------------|--------|----------------|
| Vec        | O(1)           | O(n-i)*         | O(n-i)         | O(m)*  | O(n-i)         |
| VecDeque   | O(1)           | O(min(i, n-i))* | O(min(i, n-i)) | O(m)*  | O(min(i, n-i)) |
| LinkedList | O(min(i, n-i)) | O(min(i, n-i))  | O(min(i, n-i)) | O(1)   | O(min(i, n-i)) |
Note that where ties occur, Vec is generally going to be faster than VecDeque, and VecDeque is generally going to be faster than LinkedList.


|          | get       | insert    | remove    | range     | append |
|----------|-----------|-----------|-----------|-----------|--------|
| HashMap  | O(1)~     | O(1)~*    | O(1)~     | N/A       | N/A    |
| BTreeMap | O(log(n)) | O(log(n)) | O(log(n)) | O(log(n)) | O(n+m) |


