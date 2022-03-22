# Rusty Dungeon Design

A dungeon crawler with procedurally generated levels, monsters of increasing
difficulty, and turn-based movement.

## Story

The hero's hometown is suffering from a plague of monsters. Welling up from
the deep, they seem unstoppable. Legend tesll of the Amulet of Yala - Yet
Another Lost Amulet - that can be used to stem the tide. After a long night
at the tavern, the hero promises to save the day - and sets forth into the 
dungeon.

## Basic Game Loop

1. Enter the dungeon level
2. Explore, revealing the map
3. Encounter enemies whom the player fights or flees from
4. Find power-ups and use them to strengthen the player
5. Locate the exit to the level - goto [1].

## Minimum Viable Product

1. Create a basic dungeon map
2. Place the player and them walk around
3. Spawn monsters, draw them, and let the player attack them by walking into 
   them
4. Add health and a combat system that uses it
5. Add healing potions
6. Displaying a "Game Over" screen when the player dies
7. Add the Amulet of Yala to the level and let the player win by reaching it

## Stretch Goals

1. Add fields-of-view
2. Add more interesting dungeon designs
3. Add some dungeon themes
4. Add multiple layers to the dungeon, the the Amulet in the last layer
5. Add varied weapons to the game 
6. Move to a data-driven design for spawning enemies
7. Consider some visual effects to make combat more visceral
8. Consider keeping score
