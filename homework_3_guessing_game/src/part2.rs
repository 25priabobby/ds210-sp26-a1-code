use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        fn recursive(player: &mut Player, low: u32, high: u32) -> u32 {
            let mid = (high + low) / 2;
            match player.ask_to_compare(mid) {
                0 => mid,
                -1 => recursive(player, low, mid - 1),
                1 => recursive(player, mid + 1, high),
                _ => unreachable!("You're cheating!"),
            }
        }
        recursive(player, min, max)
    }
}
