---
title: Pico 8 Adventures 
date: 2 November 2025
draft: false
---

<div align="center">


<iframe frameborder="0" src="https://itch.io/embed-upload/15635269?color=1e1e23" allowfullscreen="" width="600" height="450"><a href="https://namishh.itch.io/space-8">Play space-8 on itch.io</a></iframe>

</div>

## Introduction

While on an failed attempt to 100% [Celeste](https://www.celestegame.com/), I encountered a PICO-8 version of Celeste, inside Celeste itself. I had heard of it before, but never fully ventured in, and I got a bit intruiged by it.

![https://u.cubeupload.com/namishhhh/maxresdefault.jpg](https://u.cubeupload.com/namishhhh/maxresdefault.jpg)

After digging into PICO-8 a bit more, I could not resist grabbing a copy of my own to experiment with it. Shortly after exploring around the editor, I noticed some things. There were only 16 colors, you had a 8192 token limit on your code and you are limited to only 8x8 pixel art (which, in hindsight should have been obvious from the name). Oh, and no shaders, which is a bummer because they really help enhance the looks of my abysmal pixel art. These are some really tight limitations and things I never even noticed while playing Celeste's PICO-8 version. This really seemed kind of insane to me, because I had been working on my little [top down dungeon crawler](https://x.com/namishh__/status/1978791461174100430) and it is already over 16k lines of code, let alone 8192 tokens.

<br>

And so, I set out on a small goal for myself, squeezing the most game juice (and a game) out of these limitations. The game I have in mind is kinda like space invaders, kinda like a Undertale-style bullet hell, kinda like Vampire Survivors upgrade system. We will see down the line how much I strayed from the original goal, but this is a very rough progression/game idea I have.

## The Player and The Sky(box)

![https://u.cubeupload.com/namishhhh/Screenshot20251103at.png](https://u.cubeupload.com/namishhhh/Screenshot20251103at.png)

So above was the sprite I was able to draw with the 64 pixels I was given. This, will be my ship. PICO-8 has really a easy inbuilt function called `SPR()` to draw a sprite. Drawing and creating a basic character controller, was barely a minute of work.

![https://u.cubeupload.com/namishhhh/180Screenshot20251103at.png](https://u.cubeupload.com/namishhhh/180Screenshot20251103at.png)

But, it looks sad and lonely and it feels boring to move around. So the first fix was to draw the spaceship's exhaust below it. Using my very primitive art skills, I was able to whip out this spritesheet for the exhaust.

![https://u.cubeupload.com/namishhhh/27cScreenshot20251103at.png](https://u.cubeupload.com/namishhhh/27cScreenshot20251103at.png)

Let us also sprinkle some "life" into the space where our ship is flying around. The easiest way is to add some stars. We run a loop 100 times, we create a new star with a random `x` and `y` and we draw a `1x1` rectangle.

![https://u.cubeupload.com/namishhhh/dc8Screenshot20251103at.png](https://u.cubeupload.com/namishhhh/dc8Screenshot20251103at.png)

We are actually not "drawing rectangles" here, PICO-8 gives us this built-in called `PSET` which can be used to set the color of a specific pixel. It is not possilbe to draw `1x1` rectangles using PICO-8's rectangle function

Now to add life to these stars, we can apply some techniques: 

- First, we can infinitely pan them downwards, which will give the illusion that WE are going up.
- Next we can apply a parallax effect, and make it so that each star has a random rate of falling down.
- On top of that, we can give the slower stars a different dimmer color, to give the illusion of depth (stars far away will move down slower).

```lua
for i = 1, #stars do
    local scol = 7
    if stars[i].speed < 0.5 then scol = 5 -- too far, darker grey
    elseif stars[i].speed < 1 then scol = 13 --far, grey
    elseif stars[i].speed > 1.5 then scol = 7 -- near, white
    end
    pset(stars[i].x, stars[i].y, scol)  
end
```

- The last trick is that, using the principles of relative velocity, I can slow down or speed up the stars when I move up/down. Similarly I can also give them a horizontal displacement on moving left/right. The final code for updating the positions of stars comes down to

```lua
for i = 1, #stars do
    stars[i].y = (stars[i].y + stars[i].speed + 0.2 - vely * 0.1) % 128
    stars[i].x = (stars[i].x - velx * (0.1 + stars[i].speed * 0.1) + 128) % 128
end
```

And with just these simple tricks, the movement feels far more better. 

<div align="center">

![https://u.cubeupload.com/namishhhh/20251103112002online.gif](https://u.cubeupload.com/namishhhh/20251103112002online.gif)

</div>

## Satisfying Pew Pew

![img](https://u.cubeupload.com/namishhhh/Screenshot20251104at.png)

I plan to have two types of projectiles that can spawn from the player. One will be normal regular projectiles that hits one enemy. Other would be a special big fireball that does damage over an area. So the code to shoot a fireball starts off really simple.

```lua
bulls = {}
if btn(5) then
    -- add item to bulls table
	add(bulls, {
		x = posx, -- posx: player x position
		y = posy - 3 -- posy: player y position 
	})
end
```

If we now try to shoot, it will shoot 30 bullets per frame, which is not ideal at all. So we can add a small timer to regulate the frequency at which these guns shoot.

```lua
-- GLOBALS
bulls = {}
timer = 5

-- GAME UPDATE LOOP
if btn(5) then
    if timer <= 0 then
    -- add item to bulls table
	add(bulls, {
		x = posx, -- posx: player x position
		y = posy - 3 -- posy: player y position 
	})
    timer = 5
    end
end
timer -= 1
```

To actually move the bullets, in the update function, I just move them up, until they are out of screen, and use a built-in `DELI` to delete that bullet from the bullets table.

<br>

We have a simple shooter, but it still feels.... stale? A little visual indicator I did was to draw a muzzle flash for every shot. So the way the flash works is that it is a white circle that appears instanteously upon shooting and frame by frame we make it smaller.

```lua
-- GLOBALS
muzzle = 0

-- GAME PLAY LOOP
if btn(5) then
    if timer <= 0 then
        --- ...
        muzzle = 5
    end
end

if muzzle > 0 then
    muzzle = muzzle - 1
end
```

Then using this, we just draw a white circle of radius `muzzle` just a little above our spaceship.

![img](https://u.cubeupload.com/namishhhh/20251104001852online.gif)

Because I want this game to be in like a roguelike fashion, I need to nerf the player in the beginning so he can buy upgrades later. One of the ways I did that was to add a cooldown to the special attack. Now cooldown timer, works like any other timer I have showcased till now. It is also important to give some sort of visual indicator.

<br>

My visual indicator was a huge progress bar spanning across the screen with a white border. I also made the white border flash red and white for a short duration when cooldown ends. To make the progress bar more interesting I tried to replicate a diagonal striped pattern using

```lua
fillp(0b1100011000110001)
```

`FILLP` takes in a bitfield representing the fill pattern to use. It is a single number that represents a 4x4 pixel pattern. And after these changes, this is what my cooldown indicator looked like.

![img](https://u.cubeupload.com/namishhhh/20251104002006online.gif)

I wanted to nerf the primary attack in some way as well. Using cooldowns again felt kind of cheap, so I made it so that you can spam the primary attack but you will have to reload after certain amount of bullets.

This was really easy to code as all I did was check if the magazine is empty and if it is, over the reload time, disable shooting and keeping add bullets to the max capacity of the magazine.

<br>

The visual indicator for this was literally 
```lua
print(tostr(mag)..'/'..tostr(maxmag), 7)
```

But it looked really bad, so I made a couple of changes to it. First, as a visual indicator that the magazine is about to be empty, I change the text to yellow if magazine is less than 30% of capacity.

![img](https://u.cubeupload.com/namishhhh/d70Screenshot20251104at.png)

Next, I made these little indicators of how much bullets/mana is left according to capacity of magazine and cycle through it when shooting or reloading. Combining all of these, we end up with a pretty satisfying result with which we can conclude the core mechanics of your shooting system.

![img](https://u.cubeupload.com/namishhhh/20251104002123online.gif)

## Explosions and Particles

In order to prepare for explosions, I just added the most basic enemy and all it does it stand idle at one place. If a bullet `collides` with the enemy, we decrease it's health by 10. If health is depleted, it despawns. The collision to check between collision of two sprites is fairly easy

```lua
function collision(a,b)
  return (abs(a.x-b.x) + abs(a.y-b.y)) <= 8
end
```

And basically if the two ojbects are within 8 pixels of one another (since the sprites are `8x8`), we count it as collision. It’s a rough check that’s fast and simple for pixel games. I also made the enemy flash white, when get hit. PICO-8 gives us a function `PAL` which can be used to replace colors on a sprite. So on bullet collision with enemy, we add a small flash timer for the enemy and replace all the 16 colors with white. Sprite can be brought back to its original form by calling `PAL` without any arguements.

```lua
if e[i].flash and e[i].flash > 0 then
    for c=0,15 do
        pal(c, 7)
    end
    pal()
end
```

To make the game satisfying, it should also feel like out bullets have some impact on the enemy. The enemy should not just despawn when it's health goes to 0. So we need to add more particle effects to the game.

<br>

The process to make a big boom is fairly easy. When the enemy dies, I spawn a 25 circles of random sizes with random x and y velocities. But I also want to remove the particles from the screen. So I give them a random max_age, and an age timer. If age exceeds, max_age, the particle is removed from the particles table.

<br>

This is clearly a start but we can make it better. First, I do not want my particles to just disappear when they reach their max_age, so I changed it so that after they reach their max_age, they slowly decreases their size until they are gone and then I remove them from the table. Then I can also cycle the explosion through a bunch of colors to make it look more like an explosion. I also set the `age` to a random number instead of 0 to prevent the particles from changing colors at the same time.

The very last thing I did was to add one big particle before these random particles. This big particle had a very short max_age and was white. This represented the instantaneous "flash" of an explosion.

<br>

![img](https://u.cubeupload.com/namishhhh/simpleexplosion.gif)

Another way I can enhance this was by adding shockwaves. A shockwave i just a circle outline that grows bigger and bigger and then disappears. I made small shockwaves appear when I hit an enemy and big shockwave appears when the enemy dies.

```lua
function swave(ex, ey, mt)
	add(swaves, {
		x = ex,
		y = ey,
		r = 2,
		t = 0,
		mt = mt ~= nil and mt or 15
	})
end

function swave_draw()
	for s in all(swaves) do
		local alpha = 1 - (s.t / s.mt)
		local pc = alpha > 0.5 and 7 or 6
		circ(s.x, s.y, s.r, pc)
		s.r += 1.5
		s.t += 1
		if s.t > s.mt then
			del(swaves, s)
		end
	end
end
```

![image](https://u.cubeupload.com/namishhhh/swaves.gif)

After adding particles to the enemies, it only makes sense to explode and add particles to our ships as well. The first particle effect on our ship is the same explosion we use on the enemy ship when it dies, but on our ship it happens everytime we take damage. This explosion in blue in color and much much smaller.

<br>
The second and a new effect I did was releasing some smoke particles from our ship when we are low on lifes. It works by adding small grey circles that only go up, but they increase in size as they age and then they despawn when they reach the `max_age`.


![img](https://u.cubeupload.com/namishhhh/playerparticles.gif)

Pretty cool. Another thing I did at this point was to make the special attack, actually special. Just giving it a bigger sprite and more damage is kind of lame, which makes waiting for a cooldown for it, even lamer. I want the player to always be thinking when will they get the next chance at firing the special.

<br>

So I modified to be a spreadshot, basically five fireballs firing in an arc infront of the player (using some basic trignometry for which I obviously did not consult AI). 

![img](https://u.cubeupload.com/namishhhh/newsecondary.gif)

## Progression System

![img](https://u.cubeupload.com/namishhhh/image.jpg)

Now is the time to work on the actual roguelike mechanic of our game, for which I "took inspiration" from [Vampire Survivors](https://store.steampowered.com/app/1794680/Vampire_Survivors/). In that game, when you kill an enemy, it leaves a gem behind and collecting that you can increase your level. After a certain threshold, (when the bar on top fills), you get to select one of the three "boons" to upgrade your player or to upgrade existing weapons. The only change in mine would be that instead of gems, you will collect falling strawberries from dead enemies. The progression system can be broken into two parts.

### Juice Requirement

The straw berry part was easy to code. Spawn a berry where the enemy dies, and just move it downwards, add some horizontal sine wave like movement to it, and if it collides with the player, add to the "juice requirement".

<br>

More interesting part was coming up with the juice requirement and threshold. I did not want to waste tokens of setting up some really comprehensive requirement, instead I wanted a simple one liner formula for getting the requirement at `nth` iteration.

The most easiest would be:

```lua
function juice_req(n)
    return 5 * n
end
```
 which goes like

```
5
10
15
20
```

But most of these games work on exponential level ups. And I allowed claude to cook me up this formula

```lua
return flr(3 * (n ^ 1.6))
```

which went steep real quick so recalibrated it to be `n ^ 1.3`. Now the progression systems goes like

```
3
7
12
18
25
32
```

Now to complete the progression system we need to add in 

### Boons

![img](https://u.cubeupload.com/namishhhh/VampireSurvivorsleve.jpeg)

I want to have two types of boons, upgrades to the player and adding in spawns, like spawning a bomb. For now, I will only implement the first type, I will add spawns later when I have a comprehensive enemy system. 

So I introduced a bunch of global variables to control each parameter and for the sake of having cleaner code I split them into two tables.

```lua
mults = {
    v = 1,       -- player speed
    pbv = 1,     -- primary bullet velocity
    pbr = 1,     -- primary bullet rate
    pd = 1,      -- primary damage
    sd = 1,      -- secondary damage
    sbv = 1,     -- secondary bullet velocity
}

stats = {
    mmag = 30,       -- magazine size
    pr = 210,        -- reload time
    scd = 90,        -- secondary cooldown
    sc = 5,          -- secondary bullet count
    sa = deg(30),    -- secondary spread angle
    l = 4,           -- player lives
    ml = 5,          -- max lives
}
```

The different in these two stats are that each mult in mults table is used like

```lua
v.x = v * mult.v
```

while stats are used as it is and we add or subtract from it. Now each boon in my game is defined like

```lua
{id=12, f=function()
    stats.sc += 1
    scount += 1
end, t="bomb count +1", r=2,
c=function()
    return stats.sc < 9
end
},
```

This boon is used to increase the amount of bombs that in the special attack, which is done by the `f` method. The `c` method acts as a check to make it so that the player at maximum can only have 9 bombs. After that the boon stops appearing on the list. The `r` field indicates its rarity and `t` is the text that will be shown on the select menu.

<br>

The select menu in itself is not really hard to implement. It just gives you three boons at random whose `c` method returns true. Then I just show them on top of some overlay and lets the player navigate with arrow keys and select with the primary key. I did run into a lot of input handling bugs in this portion and I had to introduce some more global state variables to handle it.

![img](https://u.cubeupload.com/namishhhh/20251111002613online.gif)

## Enemies

Time to remove these cardboard cutouts and replace them with actual enemies. To start off, The first kind of enemy was really simple. Just follows the player, and rightfully so, called it the "follower". 

<br>

The system for creating enemies is really dead simple, I have defined enemies in a table where each entry includes some basic information about them.

```lua
follower = {
    h = 30,
    pts = 20,
    sprite=9,
    sprite_end = 12,
    speed = 0.5,
    update = update_follower,
    states = { "spawn", "active","dead"}
}
```

`states` define all the possible states the enemy can be in. The `spawn` and `dead` states are common to all enemies. During the `spawn` state we have the player spawn in some random place outside the screen and have it travel in, so it looks like he is coming to attack us. The `death` state just removes the enemy from the enemies table.

For the follower, we do not need any more state than "active" because all it does is.... follow us. We just use some basic maths to move the enemy towards the player

```lua
local dx = posx - enemy.x
local dy = posy - enemy.y
local dist = sqrt(dx*dx + dy*dy)
    
if dist > 0 then
    enemy.x += (dx / dist) * enemy.speed
    enemy.y += (dy / dist) * enemy.speed
end
```

Inspired from the follower, I added another type of enemy, which charges up and lunges at us, compared to just following us forever. This enemy had a slightly more complex state machine. It had:

```lua
states = {"spawn", "idle", "charging", "charging_up", "dead", "cooldown"}
```

I added a really small charging_up state, in which the charcter blinks between two alternate sprites, to indicate that it is gonna fire off soon. For the charging, we just move it in the player's direction for 15 frames, so by then its confirmed that it would have reached the player (real smart). To give it a more realistic feel, I made it's charge speed go up initially and then go down.

```lua
if enemy.charge_timer > 15 then
    enemy.charge_speed += 0.3
else
    enemy.charge_speed -= 0.2
    if enemy.charge_speed < 0 then enemy.charge_speed = 0 end
end

enemy.x += enemy.charge_dx * enemy.charge_speed
enemy.y += enemy.charge_dy * enemy.charge_speed
```

![img](https://u.cubeupload.com/namishhhh/20251124144129online.gif)


Now we need enemies, that can actually shoot back. The most basic version was an enemy that moves to and fro horizontally and randomly selected `y-axis` and shoots bullet downwards, pretty basic. Now I also wanted the same enemy but one that shoots at the player, so I added the ability to pass in custom properties when spawning a player.

<br>

With this I made on variant, where it shoots at the player instead of shooting straight, and the last variant where it shoots at the player but the bullets bounce like the DVD logo for 3 times. I have heard many people who have never looked into physics and game dev say bouncing seems pretty hard but it is just inverting the x and the y axis.

```lua
if b.x < 0 or b.x > 128 then 
    b.dx *= -1
    bounced = true
end
if b.y < 0 or b.y > playable_height then 
    b.dy *= -1
    bounced = true
end
```

Then to make this enemy even more hard, I included a "frenzy" state, and when it is this state, instead of shooting one bullet it shoots bursts of bullets rapidly for an amount of time and then goes back to being normal. Because it oscillates back and forth, this enemy is named the "oscillator".

<br>

I also crafted another variant of "oscillator" itself and instead of going back and fro, it roams randomly in the map, stopping at places for short breaks sometimes. And yes, it is named, the "roamer"

![img](https://u.cubeupload.com/namishhhh/roamer.gif)

I then wanted to create an enemy which would be inspired by our special attack. So one of them, called the wizard, has 5 fireballs revolving arround him, which he can shoot them outwards at any time. 

And then I edited some of wizard's code to create the merlin, which shoots A LOT of bullets in all directions, swirling them in a bit as we go.

```lua
if enemy.fire_cooldown <= 0 then
    for i = 0, 12 do
    local base_angle = (i / 8) * 1
    local swirl_angle = base_angle + enemy.swirl_offset
    local dx = cos(swirl_angle) * 2
    local dy = sin(swirl_angle) * 2
    add(ebul, {x = enemy.x + 4, y = enemy.y + 4, dx = dx, dy = dy, wizard = true})
    end
    enemy.muzzle = 5
    enemy.swirl_offset += 0.05
    enemy.fire_cooldown = 5
end
```

Both of them wait for their ealier bullets to despawn before they start roaming for their next attack.

![img](https://u.cubeupload.com/namishhhh/fireee.gif)

I was making my classmates play this game in the early stages and none of them really tried to check the fact that you can go in below and pop up from above like you can in snake. So my next enemy was all about teaching the player that.

Essentially all it does is spawn an entire row or column of bullets that start from either left, right, top, bottom most part of the screen and travel to the other side, and the only way to beat them is if you pop out from the other side (shoutout to [quantinium3](x.com/quantinium3) for still not being able to figure that out).

![img](https://u.cubeupload.com/namishhhh/serc.gif)

The last enemy has an attack that I just straight ripped from the Sans boss fight in Undertale. It has one attack where it flashes a zone in red as a telegraph and you have to just steer away from that area when it flashes white. I did the same, meet the "mercenary" which take 2-3 rectangles with random height, full screen width on the screen and turns them into death zones. Each death zone will flash for some time before there will be series of explosions in that area, if you caught in it, you will take damage.

![img](https://u.cubeupload.com/namishhhh/merc11.gif)

## Wrapping Up

And, by now I had used MORE THAN 90% of the game tokens, so it was time for me to deal with wrapping up the game, instead of adding new features. First I just created a simple Game Over and Start Game screen, nothign fancy at all, it is just really primitive navigation. For the start game however I learned that there is function called `SSPR` which helps us to draw scaled sprites, so I used that,  to create a giant version of our ship in the Start Game screen.

<br>

Even though, no one will reach more than 10k+, I did want to have a way to store large scores. PICO-8 has a 32k limitation, so I started to store digits in a table instead of storing it as a number

```lua
-- from
score = 5000
-- to
score = {0,0,0,5,0,0,0,0}
```

and I think it is pretty safe to say NO ONE is reaching 9,99,99,999. To add a number to this table, it starts with the rightmost digit (ones place) and adds the points there. If that digit exceeds 9, it keeps only the ones digit and "carries" the tens value to the next position. This carrying process repeats through each digit position moving left, stopping early if there's nothing left to carry.

```lua
function add_score(pts)
  local carry = pts
  for i = 1, 8 do
    score[i] += carry
    carry = flr(score[i] / 10)
    score[i] = score[i] % 10
    if carry == 0 then break end
  end
end
```

I also wrote a function to check if the current score is greater than highscore. If the first score's digit is larger, that score is greater and return true immediately. If it's smaller, that score is less and return false immediately. If they're equal, move to the next digit and repeat for 8 digits

```lua
function score_gt(s1, s2)
  for i = 8, 1, -1 do
    if s1[i] > s2[i] then return true end
    if s1[i] < s2[i] then return false end
  end
  return false
end
```

And... with that I have used 8180 of 8192 tokens available. There is so MUCH I wanted to add like sound effects, and spawn-type boons, which I mentioned earlier, but it seems I have run out of space.

<br>

This was a really fun project to make, and it took me a lot of time mainly because I had mid sems going on, so I was barely giving this 30 minutes a day, but I really enjoyed it. I know it seems like a really simple game but I really enjoyed the development process and PICO-8 felt similiar to what it felt like when I created my first HTML page six years ago. Until next time, goodbye!