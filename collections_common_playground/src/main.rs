mod collections;
use collections::string::Strings;
use collections::vector::Vectors;
use collections::HashMaps;

fn main() {
    Vectors::method_callers();
    Strings::method_callers();
    HashMaps::method_callers();
}
