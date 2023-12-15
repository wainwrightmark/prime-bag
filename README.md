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
    fn into_prime_index(&self) -> usize {
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