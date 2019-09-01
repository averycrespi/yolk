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

```
// Calculate dot products

define dot(V, W) = sum(V * W)

let foo = dot([1, 2], [3, 4])

// Result: foo == 11
```

Yolk specializes in working with **numbers** and **arrays**. It doesn't support strings, conditionals, or gotos.

## Why should I use Yolk?

- **Minimal syntax**: has the same operators and precedence as Yolol
- **Interoperable design**: easy to integrate with your existing Yolol
- **Aggressive optimization**: saves space on your Yolol chips

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

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Credits

Logo derived from: Egg by David from the [Noun Project](https://thenounproject.com/)
