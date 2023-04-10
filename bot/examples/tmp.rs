use bot::types::{START_MA1_INDEX, END_MA1_INDEX};


fn main() {
    for i in START_MA1_INDEX..END_MA1_INDEX+1 {
        println!("{}", i);
    }
}