extern crate ferris_says;

use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let width = 50;
    let out = "\nHello fellow Rustacean! \nIt's been a while. \nNot too long ago you came in with a really strong hold on various intreating stuff which always blew my mind away. \nThe most amazing thing about you is the sheer passion to keep finding new stuff and pick them up and apply it. \nYour commitment to solve a problem in the most efficient way or come up or build your own way though research and implementation is just one of the things i admire about and you not just say but DO IT that's always put you up. \nIf i had a chance to takes one skill of yours then i would take you reading skill, cause the way you read through documentation and book just amazes me and i wish i could have that resolve to just sit and read. I wish you the very best!!! \nWith Love, Respect and Admiration \n- Mahesh";

    let mut writer = BufWriter::new(stdout());
    say(&out, width, &mut writer).unwrap();
}
