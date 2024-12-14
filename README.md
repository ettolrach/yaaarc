# yaaarc

Yet Another Abstract Algebra Rust Crate

More specifically, I enjoyed studying commutative algebra at uni and wanted to try my hand at making
a computer algebra program.

## Another one?

Yes! This is mostly a hobby project because there are already many great crates out there for this
stuff. Here's a short comparison:

* [Symbolica](https://symbolica.io/) - probably the most complete computer algebra library out there
  that's written in Rust. Can be used in both Rust and Python. Supports Jupyter notebooks. Free for
  non-commercial use.

* [feanor-math](https://github.com/FeanorTheElf/feanor-math) - a library for "computational number
  theory", though a large focus seems to be commutative algebra too.

* [Nœther](https://github.com/warlock-labs/noether) - provides traits for algebraic structures
  without implementing these on provided structs. Great to use if you just want some better traits
  as part of a bigger program.

* [alga](https://github.com/dimforge/alga) - abandoned abstract algebra library. Since it's no
  longer maintained, I would recommend using one of the two above alternatives, though its code is
  very good to learn form.

I take inspiration from these three in my approach, but will have my own unique take on things. For
example, I plan to have more mathematical structures represented; I want to program some results
from group theory and implement boolean algebras.

## Goals and plans

While I do want to just have some fun doing this, I really should set some goals to motivate me to
leave this in a somewhat finished state. I'm planning to implement the following features for a 1.0.0 release.

* [ ] Compute Gröbner basis of an ideal of a polynomial ring.
* [ ] Use said Gröbner basis to check if an ideal is in a polynomial ring.
  
The crate should be easy to use and have excellent documentation. These will be the main goals. Performance is, of course, important, and so I'll compare my implementation against Symbolica and feanor-math as is appropriate.

### Non-goals

I'm not personally interested in linear algebra. While I might get to developing some linear algebra
concepts, the focus will be on rings, fields, and polynomial rings, and less so on modules and
vector spaces.
