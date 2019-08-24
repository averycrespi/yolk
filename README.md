<h1 align="center">
    <br>
    <img src="https://raw.githubusercontent.com/averycrespi/yolk/master/resources/yolk.png" width="150"</img>
    <br>
    Yolk
    <br>
</h1>

<h4 align="center">Numerical computing for Yolol.</h4>

<p align="center">
    <a href="#what-is-yolk">What</a> •
    <a href="#why-should-i-use-yolk">Why</a> •
    <a href="#how-do-i-get-started">How</a> •
    <a href="#development">Development</a> •
    <a href="#license">License</a> •
    <a href="#contribution">Contribution</a> •
    <a href="#credits">Credits</a>
</p>

**Warning: Yolk is currently in development. Things may change at any time!**

## What is Yolk?

Yolk is a [domain-specific language](https://en.wikipedia.org/wiki/Domain-specific_language) that transpiles to [Yolol](https://wiki.starbasegame.com/index.php/YOLOL).

Yolk specializes in working with numbers and arrays.

## Why should I use Yolk?

#### Convenience

Yolol doesn't support arrays, so each element must be a separate variable.

```
// Yolol
a_0=1 a_1=2 a_2=4 a_3=8
```

Yolk handles array expansion so that you don't have to.

```
// Yolk
let a = [1, 2, 4, 8]
```

In Yolol, applying element-wise operations to an array is repetitive and error-prone.

```
// Yolol
a_0=1 a_1=2 a_2=4 a_3=8
b_0=a_0^2 b_1=a_1^2 b_2=a_2^2 b_3=a_3^2
// Result: b_0 == 1, b_1 == 4, b_2 == 16, b_3 == 64
```

In Yolk, element-wise operations are powerful and concise.

```
// Yolk
let a = [1, 2, 4, 8]
let b = a ^ 2
// Result: b == [1, 4, 16, 64]
```

#### Simplicity

Yolk's syntax is simple and beginner-friendly.

```
// This is a comment!
import a
define b(c, d) = c + d
let e = 0
export f
```

Yolk provides the same operators and precedence as Yolol.

```
let number = 1 + 2 * 3
// Result: number == 7
```

#### Efficiency

Yolol chips are slow and have a limited amount of space.

Yolk uses [SSA form](https://en.wikipedia.org/wiki/Static_single_assignment_form) to aggressively optimizes your code, making it faster and smaller.

## How do I get started?

Learn the Yolk syntax by checking out some [examples](tests/corpus) or by reading the [language specification](docs/spec.md).

Try Yolk online with [Yolk Web](https://averycrespi.github.io/yolk-web/).

## Development

Requires Git and Rust nightly.

```bash
# Set nightly as default
rustup default nightly

# Clone the repository
git clone https://github.com/averycrespi/yolk.git && cd yolk

# Build and run tests
make
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Credits

Logo derived from: Egg by David from the [Noun Project](https://thenounproject.com/)
