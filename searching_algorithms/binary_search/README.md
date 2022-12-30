# Binary Search

This is an implementation of the binary search algorithm in Rust. It takes in a slice of data and an item to search for, and returns a boolean indicating whether the item is present in the data.

The time complexity of the binary search algorithm is O(log n), which means that the running time grows logarithmically with the size of the input data. This is significantly faster than the O(n) time complexity of a linear search, which means that the running time grows linearly with the size of the input data.

<br/>

![O-Notation of Binary Search](https://upload.wikimedia.org/wikipedia/commons/thumb/7/7f/Graph_of_common_logarithm.svg/200px-Graph_of_common_logarithm.svg.png)

## Usage

To use this implementation, you can call the `find` method on the `BinarySearch` struct:

```rust
let data = [1, 3, 4, 5, 6];
let is_found = BinarySearch::find(&data, 4);
```

The `find` method takes two arguments:

- `data`: A slice of data to search through. The data must be sorted in ascending order.
- `item`: The item to search for. The item must be of the same type as the items in the data slice, and must implement the `PartialEq` and `PartialOrd` traits.

The method returns a boolean indicating whether the item was found in the data.

## Testing

There are several tests included in the code to ensure that the implementation is correct. To run these tests, use the following command:

```rust
cargo test
```

Here is a list of the tests and a brief description of what each test does:

- `empty`: Tests the case where the data slice is empty. The expected result is `false`.
- `specific_string`: Tests searching for a specific string in a slice of strings. The expected result is `true`.
- `specific_integer`: Tests searching for a specific integer in a slice of integers. The expected result is `true`.
- `specific_integer_to_left`: Tests searching for an integer that is to the left of the middle element in a slice of integers. The expected result is `true`.
- `specific_integer_to_right`: Tests searching for an integer that is to the right of the middle element in a slice of integers. The expected result is `true`.
- `specific_integer_in_middle`: Tests searching for an integer that is the middle element in a slice of integers. The expected result is `true`.
- `specific_float`: Tests searching for a specific floating point number in a slice of floating point numbers. The expected result is `true`.
- `does_not_exist`: Tests searching for an item that is not present in the data slice. The expected result is `false`.

## Additional Notes

This implementation uses a recursive approach to the binary search algorithm. It first finds the middle element of the data slice, and then compares the item to be searched for with the middle element. If the item is equal to the middle element, it returns `true`. If the item is smaller than the middle element, it searches the left half of the data slice. If the item is larger than the middle element, it searches the right half of the data slice. This process continues until the item is found or it is determined that the item is not present in the data.

- The binary search algorithm has a time complexity of O(log n), making it much faster than a linear search for large data sets. However, it does require the data to be sorted, which can add additional time and complexity to the overall process.

- This implementation only works for types that implement the `PartialEq` and `PartialOrd` traits. This means that it can be used with primitive types such as integers and floating point numbers, as well as some types of custom structs and enums. However, it may not work with all types.

- The `find` method returns a boolean value indicating whether the item was found in the data. However, it is also possible to modify the method to return the index of the item in the data, or to return an option type (`Some` if the item is found, and `None` if it is not).

- This implementation uses a recursive approach to the binary search algorithm, but it is also possible to implement the algorithm using a loop.

- There are many variations on the binary search algorithm, including interpolation search, exponential search, and ternary search. of these variations may have different performance characteristics and may be more or less suitable for different types of data and search scenarios.

- Depending on the use case, it may also be necessary to consider additional factors such as memory usage and code readability when choosing an algorithm for searching through data.
