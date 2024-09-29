use tree::Tree;
use colored::Colorize;

fn main() {
    let tmp = tree::BinaryTree::rand(15);


    println!("{}", "To String".red().bold());
    println!("{}", tmp.to_nice_str());
}
