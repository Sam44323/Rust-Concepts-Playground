mod res;
use res::Hosting;
use res::Serving;

pub fn eat_at_restaurant() {
    // Absolute path
    Hosting::add_to_waitlist();

    // Relative path
    Hosting::add_to_waitlist();
}
