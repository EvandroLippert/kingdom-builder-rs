# kingdom-builder-rs

Design a Kingdom in Rust. In such Kingdom we have people that live in it. These people can be classified into a few categories:
- King
- Nobles
- Warrior
- Peasants

These people are allowed to have children (hint: children generation will generate another person of the same category), for simplicity only people from the same category can have kids together.
People can die. We also need a way to keep track of the number of people and their title in the kingdom. It would be nice to have a function that returns the name of all of them, their titles, if they're alive or dead, and if they have parents, or children.