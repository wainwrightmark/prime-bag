# prime bag

![GITHUB](https://img.shields.io/github/last-commit/wainwrightmark/prime_bag)

A bag datatype that used unsigned integers for storage.
This works by assigning each possible item a prime number.
The contents of the bag is represented by the product of those prime numbers.
This works best if the set of possible items is constrained and some items are much more common than others.
To maximize the possible size of the bag, assign lower prime numbers to more common items.

Using prime bags, certain operations can be done very efficiently:
Adding an element or combining two bags is achieved by multiplication.
Removing an element or bag of elements is achieved by division.
Testing for the presence of an element is achieved by modulus.

|    Set Operation    |     Math Operation     |
| :-----------------: | :--------------------: |
|   Insert / Extend   |     Multiplication     |
|       Remove        |        Division        |
| Contains / Superset |        Modulus         |
|    Intersection     | Greatest Common Factor |

Elements of the Bag must implement `PrimeBagElement`

## Getting started

```rust
use prime_bag::*;

#[derive(Debug)]
pub struct MyElement(usize);

impl PrimeBagElement for MyElement {
    fn to_prime_index(&self) -> usize {
        self.0
    }

    fn from_prime_index(value: usize) -> Self {
        Self(value)
    }
}

fn main() {
    let bag = PrimeBag16::<MyElement>::try_from_iter([MyElement(1), MyElement(2), MyElement(2)]).unwrap();
    let bag2 = bag.try_extend([MyElement(3), MyElement(3), MyElement(3)]).unwrap();

    let items : Vec<(MyElement, core::num::NonZeroUsize)> = bag2.iter_groups().collect();
    let inner_items: Vec<(usize, usize)> = items.into_iter().map(|(element, count)|(element.0, count.get())).collect();

    assert_eq!(inner_items, vec![(1,1), (2,2), (3,3)])
}
```

### Bits used per element

| Index | Prime | Bits Used | Capacity of 128 bit Prime Bag |
| ----- | ----- | --------- | ----------------------------- |
| 0     | 2     | 1.00      | 128                           |
| 1     | 3     | 1.58      | 80                            |
| 2     | 5     | 2.32      | 55                            |
| 3     | 7     | 2.81      | 45                            |
| 4     | 11    | 3.46      | 37                            |
| 5     | 13    | 3.70      | 34                            |
| 6     | 17    | 4.09      | 31                            |
| 7     | 19    | 4.25      | 30                            |
| 8     | 23    | 4.52      | 28                            |
| 9     | 29    | 4.86      | 26                            |
| 10    | 31    | 4.95      | 25                            |
| 11    | 37    | 5.21      | 24                            |
| 14    | 47    | 5.55      | 23                            |
| 15    | 53    | 5.73      | 22                            |
| 18    | 67    | 6.07      | 21                            |
| 22    | 83    | 6.38      | 20                            |
| 26    | 103   | 6.69      | 19                            |
| 32    | 137   | 7.10      | 18                            |
| 41    | 181   | 7.50      | 17                            |
| 53    | 251   | 7.97      | 16                            |
| 72    | 367   | 8.52      | 15                            |
| 102   | 563   | 9.14      | 14                            |
| 156   | 919   | 9.84      | 13                            |
| 255   | 1619  | 10.66     | 12                            |
