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

fn main() {
    let bag = PrimeBag16::<usize>::try_from_iter([1, 2, 2]).unwrap();
    let bag2 = bag.try_extend([3, 3, 3]).unwrap();
}
```