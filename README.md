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

Once you've got the compiler working, compile and run the server:

    RUST_LOG=logger,karkinos,error cargo run

You can then view the site at <http://localhost:8344>.


## Licenses

Copyright © 2016 Chris Wong

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

`symbola-crab.ttf` and `symbola-crab.woff` are derived from the Symbola typeface by [George Douros].

`icon.png` is a part of [Noto Emoji], and is licensed under the Apache License version 2.0.

[George Douros]: http://users.teilar.gr/~g1951d/
[Noto Emoji]: https://github.com/googlei18n/noto-emoji
