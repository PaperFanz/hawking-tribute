extern crate rand;

use rand::Rng;

fn main() {

    let quotes = [ "Intelligence is the ability to adapt to change.", 
                    "The greatest enemy of knowledge is not ignorance, it is the illusion of knowledge.", 
                    "I have noticed even people who claim everything is predestined, and that we can do nothing to change it, look before they cross the road.", 
                    "We are just an advanced breed of monkeys of a minor planet of a very average star. But we can understand the Univers. That makes us something very special.", 
                    "My goal is simple. It is a complete understanding of the universe, why it is as it is and why it exists at all.", 
                    "Life would be tragic if it weren't funny.", 
                    "I have no idea. People who boast about their IQ are losers.",
                    "People won't have time for you if you are always angry or complaining.",
                    "We only have to look at ourselves to see how intelligent life might develop into somehting we shouldn't want to meet.",
                    "Not only does God play dice, but... sometimes he throws them where they cannot be seen."];

    let q_num = rand::thread_rng().gen_range(1,quotes.len());

    println!("{quote} -Stephen Hawking, 1942-2018", quote = quotes[q_num]);

}
