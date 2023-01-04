**Installation**

Requirements:
<ol>
<li>    Git 
<li>    Rust compiler
<li>    Cargo
</ol>
<br>
Installation instructions:

```sh
git clone https://github.com/Incorrectish/Conquer-World/ ./Conquer-World
cd Conquer-World
cargo run 
```

After running these three commands, you should see a screen like this:
![Tue Jan  3 03:49:58 PM PST 2023](https://user-images.githubusercontent.com/84337209/210460190-1b1d0d37-8cf3-4a56-94e8-b37bdacc4d3d.jpg)

You should also hear music playing. If you do, the process was successful. If not, you might have misentered one of the previous commands, or you may be missing either git, rustc, or cargo.



<br>


**Player Abilities**

| Ability | Description |
| --- | --- |
| Directions | Press WASD to change directions |
| Movement | Press the arrow keys to move in the corresponding direction |
| Melee | Press M to deal damage in the direction you are facing |
| Slam | Press Z to deal damage to all eight squares around you |
| Lightning | Click a tile and press L to summon lightning |
| Building | Click a tile within one tile of the player and press B to build a wall on that square |
| Fire | Press F to summon a wave of fire in the direction that the player is facing |
| Heal | Press H to heal |
| Teleport | Click a square and press T to teleport to that tile |
| Missiles | Press X to spawn a missile that tracks the closest enemies to it and deals massive damage |
<br>

**Warning: The rest of this file contains massive spoilers for the game and mechanics. Don't read on if you want the most interesting experience**
<br>
<br>
<br>
<br>
<br>
<br>
<br>
<br>
<br>
<br>
<br>
<br>

**General Game Model**

The game takes place on a 7x7 grid of "worlds", as shown below
![Tue Jan  3 03:54:11 PM PST 2023](https://user-images.githubusercontent.com/84337209/210460606-535e08dd-6611-427e-85a8-3ed29d330a16.jpg)
Where M marks a mini boss and F marks a final boss. 
The player field of vision is one "world", and crossing a boundary shifts the field of vision into the next world
Unlike most games, which are based on time, every mechanic in our game takes place around actions. Every single entity and object in the game takes one
action for every action that the player takes. So for example, using an ability or moving up would be one action, and each enemy would make one move
towards you and every projectile would make one move in it's intended direction.

**World Generation**

The program uses depth first search and probabalistic random generation to create unique lakes and mountains at runtime. Mountains are uncrossable by most 
enemies, and lakes are crossable by major enemies and projectiles. Further documentation is availible in the function `gen_lakes` in `src/world.rs`.

**Enemies**

There are 3 different types of enemies

| Enemy | Description |
| --- | --- |
| Chaser | Relentlessly chases the player, and attacks the player with melees |
| Bomber | Gets close to the player and explodes, dealing damage if the player is in the explosion radius |
| Major | Chases the player, can move through walls, mountains, and lakes. Deals a lot of damage, but is larger and easier to damage | 

<br>

**Player Abilities**

<br>

**Bosses and Mechanics**
<br>

