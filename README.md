# K A R K I N O S

**KARKINOS** is a database of people interested in the [Rust programming language][Rust]. It uses the same data as [rustaceans.org], but presents it through a different interface.

I created Karkinos for these reasons:

- To provide access for users who browse with JavaScript disabled;

- To rewrite the backend in Rust (instead of Node.js);

- As a proving ground for my template engine, [Maud];

- To screw around with CSS (this is the most important reason).

Karkinos is named after a very special [giant crab].

[Rust]: https://www.rust-lang.org
[rustaceans.org]: http://rustaceans.org
[Maud]: https://github.com/lfairy
[giant crab]: https://en.wikipedia.org/wiki/Cancer_(constellation)#Names


## Setting up

Karkinos requires a nightly version of the Rust compiler. The easiest way to install this is through [rustup]:

    rustup override set nightly

[rustup]: https://rustup.rs/

Once you've got the compiler working, first update the data:

    ./update-data.sh

Then run the server:

    RUST_LOG=logger,karkinos,error cargo run

You can then view the site at <http://localhost:8344>.
