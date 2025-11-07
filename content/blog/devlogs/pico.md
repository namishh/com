---
title: Pico 8 Adventures 
date: 2 November 2025
draft: true
---

## Introduction

While on an failed attempt to 100% [Celeste](https://www.celestegame.com/), I encountered a PICO-8 version of Celeste, inside Celeste itself. I had heard of it before, but never fully ventured in, and I got a bit intruiged by it.

![https://u.cubeupload.com/namishhhh/maxresdefault.jpg](https://u.cubeupload.com/namishhhh/maxresdefault.jpg)

After digging into PICO-8 a bit more, I could not resist grabbing a copy of my own to experiment with it. Shortly after exploring around the editor, I noticed some things. There were only 16 colors, you had a 8192 tokens limit on your code and you are limited to only 8x8 pixel art (which, in hindsight should have been obvious from the name). Oh, and no shaders, which is a bummer because they really help enhance the looks of my abysmal pixel art. These are some really tight limitations and things I never even noticed while playing Celeste's PICO-8 version. This really seemed kind of insane to me, because I had been working on my little [top down dungeon crawler](https://x.com/namishh__/status/1978791461174100430) and it is already over 16k lines of code, let alone 8192 tokens.

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

And basically if the two ojbects are withing 8 pixels of one another (since the sprites are `8x8`), we count it as collision. It’s a rough check that’s fast and simple for pixel games. I also made the enemy flash white, when get hit. PICO-8 gives us a function `PAL` which can be used to replace colors on a sprite. So on bullet collision with enemy, we add a small flash timer for the enemy and replace all the 16 colors with white. Sprite can be brought back to its original form by calling `PAL` without any arguements.

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