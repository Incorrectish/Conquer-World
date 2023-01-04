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
| Projectiles | Press Space to spawn a projectile that travels in the direction that the player is facing |
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
<br>
<br>
Lightning
<br>
![lightning](https://user-images.githubusercontent.com/84337209/210485645-a814326b-6065-468a-aede-c7ee2aab7303.gif)
<br>
Projectiles
<br>
![projectiles](https://user-images.githubusercontent.com/84337209/210485696-897ba642-ba4a-4092-89d3-e56baf650107.gif)
<br>
Heal
<br>
![heal](https://user-images.githubusercontent.com/84337209/210509129-7c7df29a-f9ea-403a-9c3d-5bb42944d8ff.gif)
<br>
Invisibility
<br>
![invis](https://user-images.githubusercontent.com/84337209/210509134-28172cc0-939d-4234-ae75-dac77ff9c5d9.gif)
<br>
Missile
<br>
![missile](https://user-images.githubusercontent.com/84337209/210509136-e4d1fa7d-3acc-4ff3-affe-6db621039862.gif)
<br>
Teleport
<br>
![teleport](https://user-images.githubusercontent.com/84337209/210509138-5aba55b7-5321-4a4b-9bbc-d78e82d08b84.gif)
<br>
Fire
<br>
![fire](https://user-images.githubusercontent.com/84337209/210509140-dd42970a-f97f-45ca-a608-bde944748ab7.gif)
<br>



<br>

**Bosses and Mechanics**
<br>



Laser Grid Boss
<br>
 ![lasergrid](https://user-images.githubusercontent.com/84337209/210518932-ac623aa7-adc7-45cd-a200-6f2d1e148967.gif)
<br>
Spawns grid of lasers with short activation time. Stay away from the lasers to avoid damage. Two types of enemies in this room, majors and bombers. Kill all enemies in the room to activate a boss damage phase. Repeat until boss is dead.
<br>

Chasing Boss
<br>
![ChasingBoss](https://user-images.githubusercontent.com/84337209/210518924-c5cd450e-5063-445d-b236-c533be8fee41.gif)
<br>
Relentlessly chases the player, spawns stun wels around the room. Stepping in a stun well temporarily prevents a player from acting. Stepping in the path of the boss provokes it to do a charge attack, which kills you if you get caught. It leaves behind a trail of fire which should be avoided. Damage the boss once it has charged into a wall. No enemies in this room. 
<br>
<br>
SlidingLaser Boss
<br>
![slidinglaser](https://user-images.githubusercontent.com/84337209/210518929-689e15de-8227-4caa-891a-183c12b81412.gif)
<br>
The boss creates a full map range laser horizontally or vertically depending on the players position and creates a full map range laser and begins moving towards the player vertically or horizontally. Additionally, asteroids will spawn every few turns which linger for a couple of turns and do damage to the player if stood in. Once the boss has finished it's laser animation, a short damage phase will begin. Chasers and majors spawn in this room. 
<br>
<br>
Blackout Boss
<br>
![Blackout](https://user-images.githubusercontent.com/84337209/210518918-c663768e-7dba-43e2-ade5-b381e211aa38.gif)
<br>
Creates safe zones around the dungeon that the player must get into before the timer expires. If the player doesn't reach the zone by the time the blackout effect happens(timer expiring), the player will instantly die. Damage specific parts of the boss that glow yellow to activate this damage phase. Only chasers spawn in this room. 
<br>
<br>
<br>


<br>




