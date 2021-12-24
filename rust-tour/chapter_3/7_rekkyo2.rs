#![allow(dead_code)] 

enum Species { Crab, Octpus, Fish, Clam }
enum PoisonType { Adic, Painful, Lethal }
enum Size { Big, Small }
enum Weapon {
    Claw(i32, Size),
    Poison( PoisonType ),
    None
}