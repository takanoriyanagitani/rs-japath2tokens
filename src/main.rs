use std::io;

use rs_japath2tokens::stdin2paths2tokens2stdout;

fn main() -> Result<(), io::Error> {
    stdin2paths2tokens2stdout()
}
