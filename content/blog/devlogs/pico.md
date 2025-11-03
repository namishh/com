---
title: Pico 8 Adventures 
date: 2 November 2025
draft: true
---

## Introduction

While on an failed attempt to 100%-ing [Celeste](https://www.celestegame.com/), I was reminded that PICO-8 exists. I had to finish a mini-game, which was a PICO-8 version of celeste. 

![https://u.cubeupload.com/namishhhh/maxresdefault.jpg](https://u.cubeupload.com/namishhhh/maxresdefault.jpg)

After digging into PICO-8 a bit more, and I could not resist grabbing a copy of my own to experiment with it. And shortly after exploring around the editor, I noticed some things. There were only 16 colors, you had a 8192 character limit on your code and you are limited to only 8x8 pixel art (which, in hindsight should have been obvious from the name). Oh, and no shaders, which is a bummer because they really help enhance the game looks. These are some really tight limitations and things I never even noticed them while playing Celeste's pico-8 version. This really seemed kind of insane to me, because I had been working on my little [top down dungeon crawler](https://x.com/namishh__/status/1978791461174100430) and it is already over 16k lines of code, let alone 8192 characters.

<br>

And so, I set out on a small goal for myself, squeezing the most game juice (and a game) out of these limitations. The game I have in mind is kinda like space invaders, kinda like a undertale-style bullet hell, kinda like vampire survivors upgrade system. We will see down the line how much I strayed from the original goal, but this is a very rough progression/game idea i have.

## The Player and The Sky

![https://u.cubeupload.com/namishhhh/Screenshot20251103at.png](https://u.cubeupload.com/namishhhh/Screenshot20251103at.png)

So above was the sprite I was able to draw with the 64 pixels I was given. This, will be my ship. PICO-8 has really easy inbuilt functions like `SPR()` to draw a sprite. Drawing and creating a basic character controller, was barely a minute of work.

![https://u.cubeupload.com/namishhhh/180Screenshot20251103at.png](https://u.cubeupload.com/namishhhh/180Screenshot20251103at.png)

But, it looks sad and lonely and it feels boring to move around. So the first fix was to draw the spaceship's exhaust below it. Using my very primitive art skills, I was able to whip out this spritesheet for the exhaust.

![https://u.cubeupload.com/namishhhh/27cScreenshot20251103at.png](https://u.cubeupload.com/namishhhh/27cScreenshot20251103at.png)

Let us also sprinkle some "life" into the space where our ship is flying around. The easiest way is to add some stars. We run a loop 100 times, we create a new star with a random `x` and `y` and we draw a `1x1` rectangle.

![https://u.cubeupload.com/namishhhh/dc8Screenshot20251103at.png](https://u.cubeupload.com/namishhhh/dc8Screenshot20251103at.png)

We are actually not "drawing rectangles" here, PICO-8 gives us this built-in called `PXSET` which can be used to set the color of a specific pixel. It is not possilbe to draw `1x1` rectangles using PICO-8's rectangle function

Now to add life to these stars, we can apply some techniques: 

- First, we can infinitely pan them downwards, which will give the illusion that WE are going up.
- Next we can apply a parallax effect, and make it so that each star has a random rate of falling down.
- On top of that, we can give the slower stars a different dimmer color, to give the illusion of depth (stars far away will move down slower).
- The last trick is that, using the principles of relative velocity, I can slow down or speed up the stars when I move up/down. Similarly I can also give them a horizontal displacement on moving left/right.

And with just these simple tricks, and slapping some static HUD on top, the movement feels far more juicy. 

<div align="center">

![https://u.cubeupload.com/namishhhh/20251103112002online.gif](https://u.cubeupload.com/namishhhh/20251103112002online.gif)

</div>
