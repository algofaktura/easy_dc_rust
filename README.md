# easy dc solver
An algorithm for solving the Hamiltonian cycle problem deterministically and in linear time on all instances of discocube graphs which are:
3-dimensional grid graphs derived from: a polycube of an octahedron | a Hauy construction of an octahedron using cubes as identical building blocks | the accretion of cubes around a central cube forming an octahedron at the limit...

![Planar embedding of Cube and Discocubes](imgs/planar_emb.png?raw=true "Planar embedding of Cube and Discocubes")
*Planar embedding of a cube and a discocube. from the set of all graphs G, where the order of G is of the ***Uncentered octahedral numbers*** [A130809](https://oeis.org/A130809), only the first two instances shown above; n[0] and n[1] are planarly embeddable i.e., it can be represented on a two-dimensional surface without any of its edges crossing.*

![First 11 discocubes and their order (number of nodes)](imgs/rect5857.png?raw=true "Discocubes orders")
*The first eleven discocubes and their respective orders (number of nodes)*

Why weave()? Finding the solution to the problem reminded me of macramé, of tying knots, weaving and how repeating certain movements resulted in certain patterns. I followed the thought further and asked myself if there was a 'weave' I could use to expose underlying unit structure and repeat this throughout to get an initial solution which could later be mutated to produce a more polished solution. 

The focus of this work is to apply all that I know about this graph, not as a discrete mathematician, but as an artists with an eye towards visual aesthetics. Inspiration was the driving force behind the work (a bit of obsession I confess). 

From expressing the desire to sculpt a 3-dimensional contour drawing of an object to reformulating this desire mathematically as searching for a Hamiltonian cycle in specific yet unidentified graph, sketches are indistinguishable from mini algorithms.  An artist uses language and forms that language to communicate their vision to others, taking part in a process of translation from one medium to another, from vision to object, from words to movement, just as a programmer might transform ideas into an orchestra of processing with the perfect score.

This is a tiny result of that artistic investigative process and I hope it will be useful. I've grown so obsessed with the discocube object, really not unlike an obsessive artist's muse to the point of being a stalker. 

The goal wasn't to write a fast algorithm that finds always turning hamiltonian cycles in discocube graphs, and other stuff...  it was a constant moving of goalposts, of never being satisfied, of not knowing what, but of wanting more... until I could claim the discocube was my own (in my mind), as a painter would claim a portrait their own after having spent months realizing a vision.

Art studies forms, the curvature of the neck as it disappears into the back, the color in the foreground, so luminous, relegating all things beyond to irrelevance. So in this project, I studied the discocube as a body, where each turn was conceptualized not as a discrete math object but as movement of the body, resulting in more doodles and sketches than pages of equations.

I hope that I've shared some insights (through code) I've gained by solving this problem having taken an approach similar to that of sculpting the human body...After thousands of studies, drawings, a little math: this is a tiny glimpse into how moving towards a specific aethetic goal yields results. When a graph becomes an artist's muse, how does the artist go about rendering their vision as a painter paints a portrait?

Goal:
Optimize the algorithm so that it is able to solve instances of over a billion vertices:
``
TO BE CONTINUED TO: | 🇳  1000 | ⭕️  1_337_336_000 | 
``

![Discocubes](imgs/dcviews.png?raw=true "Discocubes")
*Discocubes 8 - 1760*

What started as a hack-your-own version of a depth-first-search-with-shortcuts for the discocube graph (solving up to 960 vertices), metastasized into pages of overgrown mixin classes mysteriously coupled to each another like overgrown vines pushing me deeper and deeper into the underbelly of its mutant tentacles. Although it was able to solve instances of over a million vertices, it had the clarity of primordial soup. So, as a sadistic gardener I painstakingly pruned my own unescapable web (all those letters you haven't opened yet?) of thorny vines into presentable tiny bonsai trees. So what is a bonsai if not a tree in intimate scope?

To paraphrase Hauy: 

*When solving problems that involve analyzing how nature progresses, we are led by very rapid methods to results that are not immediately obvious. These results may appear paradoxical and surprising. However, if we take the time to carefully examine the steps we took to reach these results, we will begin to understand the underlying principles that led to these outcomes. By going back over the process step by step, we can better understand the logic behind the final results.*

The result of this creative process is a family of algorithms developed specifically to solve various graph problems on the disoocube graph, 3d grid graph and hexprism honeycomb diamond graphs. 
The algorithm presented in this repository is the least complex, also making it the fastest. It does the job, solving the hamiltonian cycle problem for over millions of vertices in reasonable time (seconds vs. years), while others take longer but also have other objectives, like forming an always turning cycle with even edge distribution across all axes. But that's giving too much away... 

Eventually this repository will include other linear time algorithms for solving the hamiltonian cycle problem in 3d grid graphs and also in solid grid graphs, addressing some open issues raised in the graph theory research literature.
Execution time of each order (in millions):

![Hexprism Honeycomb Diamond](imgs/hexhoneydiamond.png?raw=true "Hexprism Honeycomb Diamond")
*Hexprism Honeycomb Diamond*

### digital discocubes
As each solution is as unique as a fingerprint, or a diamond it allows one to have their own digital version of a discocube, which is also an instruction for building your own.

![Discocube 3640 view](imgs/icy_cube.png?raw=true "icy cube") 
![Discocube 3640 view](imgs/icy_cube5.png?raw=true "icy cube")
![Discocube 3640 view](imgs/icy_cube3.png?raw=true "icy cube another view")
![Discocube 3640 view](imgs/icy_cube2.png?raw=true "icy cube another view")
![Always Turning Discocube 9120 view](imgs/always_turning_9120.png?raw=true "Always Turning Discocube 9120 view")
*Discocubes as glb, using different mirrored texture yields personalized results and unique reflections meaning each discocube has its own reflection/shadow fingerprint! With millions of combinations available (glass texture/image/color, mirror texture/image/color, edge texture/image/color), the possibilities are endless!*

### ps...
Please note: the hamiltonian cycle produced by this algorithm is the base form, without a high mutation rate. The polished versions available have no nonturns and all their edges are distributed evenly across the three axes.
The other algorithms I spoke of earlier accomplish this task.

## Command line usage
To use the package via the command line, navigate to the root directory of the project in your terminal and run the following command:
```
cargo run --release [N] [N_UPPER_INCLUSIVE] [REPEATS]
// Graph start instance | Graph end instance | Repeats
```
```
// run each graph 10 tens from the first instance to the 100th (32-1,373600)
cargo run --release 1 100 10
```

## Running times
![Running times from 8 to 68,085,920 vertices](imgs/8_to_68085920.png?raw=true "Runtimes up to 68 million")
8_to_68085920.png

### PYTHON VS. RUST:

### solve python profile 5,061,680 vertices:
![Profile of solve_np](imgs/profile_solve_np5.png?raw=true "Profile of solve_np")
### solve rust speed 5,061,680 vertices:
![Profile of solve_np](imgs/rust_speed_graph_5061680_verts.png?raw=true "Profile of solve_np")

#### Running times for the first 500 instances: graphs with 8 to 167_668_000 vertices (to be continued until 1000th order (over 1 billion)):
```
| 🇳    1 | ⭕️            8 | 🕗      0.0000010 | 📌 HamCycle |
| 🇳    2 | ⭕️           32 | 🕗      0.0000021 | 📌 HamCycle |
| 🇳    3 | ⭕️           80 | 🕗      0.0000207 | 📌 HamCycle |
| 🇳    4 | ⭕️          160 | 🕗      0.0000376 | 📌 HamCycle |
| 🇳    5 | ⭕️          280 | 🕗      0.0000457 | 📌 HamCycle |
| 🇳    6 | ⭕️          448 | 🕗      0.0000612 | 📌 HamCycle |
| 🇳    7 | ⭕️          672 | 🕗      0.0000849 | 📌 HamCycle |
| 🇳    8 | ⭕️          960 | 🕗      0.0001190 | 📌 HamCycle |
| 🇳    9 | ⭕️        1_320 | 🕗      0.0001546 | 📌 HamCycle |
| 🇳   10 | ⭕️        1_760 | 🕗      0.0001940 | 📌 HamCycle |
| 🇳   11 | ⭕️        2_288 | 🕗      0.0002370 | 📌 HamCycle |
| 🇳   12 | ⭕️        2_912 | 🕗      0.0003046 | 📌 HamCycle |
| 🇳   13 | ⭕️        3_640 | 🕗      0.0003566 | 📌 HamCycle |
| 🇳   14 | ⭕️        4_480 | 🕗      0.0004434 | 📌 HamCycle |
| 🇳   15 | ⭕️        5_440 | 🕗      0.0005282 | 📌 HamCycle |
| 🇳   16 | ⭕️        6_528 | 🕗      0.0006934 | 📌 HamCycle |
| 🇳   17 | ⭕️        7_752 | 🕗      0.0008181 | 📌 HamCycle |
| 🇳   18 | ⭕️        9_120 | 🕗      0.0009507 | 📌 HamCycle |
| 🇳   19 | ⭕️       10_640 | 🕗      0.0010603 | 📌 HamCycle |
| 🇳   20 | ⭕️       12_320 | 🕗      0.0013304 | 📌 HamCycle |
| 🇳   21 | ⭕️       14_168 | 🕗      0.0015334 | 📌 HamCycle |
| 🇳   22 | ⭕️       16_192 | 🕗      0.0018256 | 📌 HamCycle |
| 🇳   23 | ⭕️       18_400 | 🕗      0.0020479 | 📌 HamCycle |
| 🇳   24 | ⭕️       20_800 | 🕗      0.0023825 | 📌 HamCycle |
| 🇳   25 | ⭕️       23_400 | 🕗      0.0027736 | 📌 HamCycle |
| 🇳   26 | ⭕️       26_208 | 🕗      0.0029853 | 📌 HamCycle |
| 🇳   27 | ⭕️       29_232 | 🕗      0.0033606 | 📌 HamCycle |
| 🇳   28 | ⭕️       32_480 | 🕗      0.0038334 | 📌 HamCycle |
| 🇳   29 | ⭕️       35_960 | 🕗      0.0043816 | 📌 HamCycle |
| 🇳   30 | ⭕️       39_680 | 🕗      0.0050111 | 📌 HamCycle |
| 🇳   31 | ⭕️       43_648 | 🕗      0.0057679 | 📌 HamCycle |
| 🇳   32 | ⭕️       47_872 | 🕗      0.0060557 | 📌 HamCycle |
| 🇳   33 | ⭕️       52_360 | 🕗      0.0068013 | 📌 HamCycle |
| 🇳   34 | ⭕️       57_120 | 🕗      0.0077425 | 📌 HamCycle |
| 🇳   35 | ⭕️       62_160 | 🕗      0.0087764 | 📌 HamCycle |
| 🇳   36 | ⭕️       67_488 | 🕗      0.0093920 | 📌 HamCycle |
| 🇳   37 | ⭕️       73_112 | 🕗      0.0101394 | 📌 HamCycle |
| 🇳   38 | ⭕️       79_040 | 🕗      0.0122911 | 📌 HamCycle |
| 🇳   39 | ⭕️       85_280 | 🕗      0.0123574 | 📌 HamCycle |
| 🇳   40 | ⭕️       91_840 | 🕗      0.0136622 | 📌 HamCycle |
| 🇳   41 | ⭕️       98_728 | 🕗      0.0148989 | 📌 HamCycle |
| 🇳   42 | ⭕️      105_952 | 🕗      0.0162773 | 📌 HamCycle |
| 🇳   43 | ⭕️      113_520 | 🕗      0.0180751 | 📌 HamCycle |
| 🇳   44 | ⭕️      121_440 | 🕗      0.0194382 | 📌 HamCycle |
| 🇳   45 | ⭕️      129_720 | 🕗      0.0214285 | 📌 HamCycle |
| 🇳   46 | ⭕️      138_368 | 🕗      0.0229243 | 📌 HamCycle |
| 🇳   47 | ⭕️      147_392 | 🕗      0.0254875 | 📌 HamCycle |
| 🇳   48 | ⭕️      156_800 | 🕗      0.0280501 | 📌 HamCycle |
| 🇳   49 | ⭕️      166_600 | 🕗      0.0298669 | 📌 HamCycle |
| 🇳   50 | ⭕️      176_800 | 🕗      0.0325643 | 📌 HamCycle |
| 🇳   51 | ⭕️      187_408 | 🕗      0.0355896 | 📌 HamCycle |
| 🇳   52 | ⭕️      198_432 | 🕗      0.0389029 | 📌 HamCycle |
| 🇳   53 | ⭕️      209_880 | 🕗      0.0408493 | 📌 HamCycle |
| 🇳   54 | ⭕️      221_760 | 🕗      0.0440270 | 📌 HamCycle |
| 🇳   55 | ⭕️      234_080 | 🕗      0.0482504 | 📌 HamCycle |
| 🇳   56 | ⭕️      246_848 | 🕗      0.0510917 | 📌 HamCycle |
| 🇳   57 | ⭕️      260_072 | 🕗      0.0539111 | 📌 HamCycle |
| 🇳   58 | ⭕️      273_760 | 🕗      0.0570619 | 📌 HamCycle |
| 🇳   59 | ⭕️      287_920 | 🕗      0.0626398 | 📌 HamCycle |
| 🇳   60 | ⭕️      302_560 | 🕗      0.0665345 | 📌 HamCycle |
| 🇳   61 | ⭕️      317_688 | 🕗      0.0696655 | 📌 HamCycle |
| 🇳   62 | ⭕️      333_312 | 🕗      0.0756095 | 📌 HamCycle |
| 🇳   63 | ⭕️      349_440 | 🕗      0.0812501 | 📌 HamCycle |
| 🇳   64 | ⭕️      366_080 | 🕗      0.0855361 | 📌 HamCycle |
| 🇳   65 | ⭕️      383_240 | 🕗      0.0897376 | 📌 HamCycle |
| 🇳   66 | ⭕️      400_928 | 🕗      0.0939994 | 📌 HamCycle |
| 🇳   67 | ⭕️      419_152 | 🕗      0.0992696 | 📌 HamCycle |
| 🇳   68 | ⭕️      437_920 | 🕗      0.1073214 | 📌 HamCycle |
| 🇳   69 | ⭕️      457_240 | 🕗      0.1160096 | 📌 HamCycle |
| 🇳   70 | ⭕️      477_120 | 🕗      0.1238002 | 📌 HamCycle |
| 🇳   71 | ⭕️      497_568 | 🕗      0.1302345 | 📌 HamCycle |
| 🇳   72 | ⭕️      518_592 | 🕗      0.1427145 | 📌 HamCycle |
| 🇳   73 | ⭕️      540_200 | 🕗      0.1481154 | 📌 HamCycle |
| 🇳   74 | ⭕️      562_400 | 🕗      0.1528944 | 📌 HamCycle |
| 🇳   75 | ⭕️      585_200 | 🕗      0.1619013 | 📌 HamCycle |
| 🇳   76 | ⭕️      608_608 | 🕗      0.1697858 | 📌 HamCycle |
| 🇳   77 | ⭕️      632_632 | 🕗      0.1802106 | 📌 HamCycle |
| 🇳   78 | ⭕️      657_280 | 🕗      0.1929411 | 📌 HamCycle |
| 🇳   79 | ⭕️      682_560 | 🕗      0.1991495 | 📌 HamCycle |
| 🇳   80 | ⭕️      708_480 | 🕗      0.2135235 | 📌 HamCycle |
| 🇳   81 | ⭕️      735_048 | 🕗      0.2184628 | 📌 HamCycle |
| 🇳   82 | ⭕️      762_272 | 🕗      0.2459208 | 📌 HamCycle |
| 🇳   83 | ⭕️      790_160 | 🕗      0.2592264 | 📌 HamCycle |
| 🇳   84 | ⭕️      818_720 | 🕗      0.2696866 | 📌 HamCycle |
| 🇳   85 | ⭕️      847_960 | 🕗      0.2710578 | 📌 HamCycle |
| 🇳   86 | ⭕️      877_888 | 🕗      0.2800946 | 📌 HamCycle |
| 🇳   87 | ⭕️      908_512 | 🕗      0.2869450 | 📌 HamCycle |
| 🇳   88 | ⭕️      939_840 | 🕗      0.3290268 | 📌 HamCycle |
| 🇳   89 | ⭕️      971_880 | 🕗      0.3456512 | 📌 HamCycle |
| 🇳   90 | ⭕️    1_004_640 | 🕗      0.3538976 | 📌 HamCycle |
| 🇳   91 | ⭕️    1_038_128 | 🕗      0.3681797 | 📌 HamCycle |
| 🇳   92 | ⭕️    1_072_352 | 🕗      0.3963305 | 📌 HamCycle |
| 🇳   93 | ⭕️    1_107_320 | 🕗      0.4033277 | 📌 HamCycle |
| 🇳   94 | ⭕️    1_143_040 | 🕗      0.4153683 | 📌 HamCycle |
| 🇳   95 | ⭕️    1_179_520 | 🕗      0.4387953 | 📌 HamCycle |
| 🇳   96 | ⭕️    1_216_768 | 🕗      0.4771417 | 📌 HamCycle |
| 🇳   97 | ⭕️    1_254_792 | 🕗      0.4902358 | 📌 HamCycle |
| 🇳   98 | ⭕️    1_293_600 | 🕗      0.4930317 | 📌 HamCycle |
| 🇳   99 | ⭕️    1_333_200 | 🕗      0.5267313 | 📌 HamCycle |
| 🇳  100 | ⭕️    1_373_600 | 🕗      0.5552125 | 📌 HamCycle |
| 🇳  101 | ⭕️    1_414_808 | 🕗      0.5823348 | 📌 HamCycle |
| 🇳  102 | ⭕️    1_456_832 | 🕗      0.5761076 | 📌 HamCycle |
| 🇳  103 | ⭕️    1_499_680 | 🕗      0.6100927 | 📌 HamCycle |
| 🇳  104 | ⭕️    1_543_360 | 🕗      0.6406497 | 📌 HamCycle |
| 🇳  105 | ⭕️    1_587_880 | 🕗      0.6618903 | 📌 HamCycle |
| 🇳  106 | ⭕️    1_633_248 | 🕗      0.6993168 | 📌 HamCycle |
| 🇳  107 | ⭕️    1_679_472 | 🕗      0.7392572 | 📌 HamCycle |
| 🇳  108 | ⭕️    1_726_560 | 🕗      0.7754343 | 📌 HamCycle |
| 🇳  109 | ⭕️    1_774_520 | 🕗      0.7944393 | 📌 HamCycle |
| 🇳  110 | ⭕️    1_823_360 | 🕗      0.8072736 | 📌 HamCycle |
| 🇳  111 | ⭕️    1_873_088 | 🕗      0.8542167 | 📌 HamCycle |
| 🇳  112 | ⭕️    1_923_712 | 🕗      0.8878529 | 📌 HamCycle |
| 🇳  113 | ⭕️    1_975_240 | 🕗      0.8812421 | 📌 HamCycle |
| 🇳  114 | ⭕️    2_027_680 | 🕗      0.9601020 | 📌 HamCycle |
| 🇳  115 | ⭕️    2_081_040 | 🕗      0.9935364 | 📌 HamCycle |
| 🇳  116 | ⭕️    2_135_328 | 🕗      1.0323677 | 📌 HamCycle |
| 🇳  117 | ⭕️    2_190_552 | 🕗      1.0316335 | 📌 HamCycle |
| 🇳  118 | ⭕️    2_246_720 | 🕗      1.1275588 | 📌 HamCycle |
| 🇳  119 | ⭕️    2_303_840 | 🕗      1.1763284 | 📌 HamCycle |
| 🇳  120 | ⭕️    2_361_920 | 🕗      1.2075880 | 📌 HamCycle |
| 🇳  121 | ⭕️    2_420_968 | 🕗      1.2717817 | 📌 HamCycle |
| 🇳  122 | ⭕️    2_480_992 | 🕗      1.2900746 | 📌 HamCycle |
| 🇳  123 | ⭕️    2_542_000 | 🕗      1.2972989 | 📌 HamCycle |
| 🇳  124 | ⭕️    2_604_000 | 🕗      1.3435471 | 📌 HamCycle |
| 🇳  125 | ⭕️    2_667_000 | 🕗      1.4070555 | 📌 HamCycle |
| 🇳  126 | ⭕️    2_731_008 | 🕗      1.4372251 | 📌 HamCycle |
| 🇳  127 | ⭕️    2_796_032 | 🕗      1.4794157 | 📌 HamCycle |
| 🇳  128 | ⭕️    2_862_080 | 🕗      1.5322369 | 📌 HamCycle |
| 🇳  129 | ⭕️    2_929_160 | 🕗      1.5380286 | 📌 HamCycle |
| 🇳  130 | ⭕️    2_997_280 | 🕗      1.6097608 | 📌 HamCycle |
| 🇳  131 | ⭕️    3_066_448 | 🕗      1.6794202 | 📌 HamCycle |
| 🇳  132 | ⭕️    3_136_672 | 🕗      1.7742593 | 📌 HamCycle |
| 🇳  133 | ⭕️    3_207_960 | 🕗      1.8197930 | 📌 HamCycle |
| 🇳  134 | ⭕️    3_280_320 | 🕗      1.8474405 | 📌 HamCycle |
| 🇳  135 | ⭕️    3_353_760 | 🕗      1.9404466 | 📌 HamCycle |
| 🇳  136 | ⭕️    3_428_288 | 🕗      1.9601974 | 📌 HamCycle |
| 🇳  137 | ⭕️    3_503_912 | 🕗      2.0513310 | 📌 HamCycle |
| 🇳  138 | ⭕️    3_580_640 | 🕗      2.0951188 | 📌 HamCycle |
| 🇳  139 | ⭕️    3_658_480 | 🕗      2.1660309 | 📌 HamCycle |
| 🇳  140 | ⭕️    3_737_440 | 🕗      2.2257590 | 📌 HamCycle |
| 🇳  141 | ⭕️    3_817_528 | 🕗      2.2943380 | 📌 HamCycle |
| 🇳  142 | ⭕️    3_898_752 | 🕗      2.3628054 | 📌 HamCycle |
| 🇳  143 | ⭕️    3_981_120 | 🕗      2.4565878 | 📌 HamCycle |
| 🇳  144 | ⭕️    4_064_640 | 🕗      2.4630859 | 📌 HamCycle |
| 🇳  145 | ⭕️    4_149_320 | 🕗      2.5294545 | 📌 HamCycle |
| 🇳  146 | ⭕️    4_235_168 | 🕗      2.6146848 | 📌 HamCycle |
| 🇳  147 | ⭕️    4_322_192 | 🕗      2.7125676 | 📌 HamCycle |
| 🇳  148 | ⭕️    4_410_400 | 🕗      2.7204847 | 📌 HamCycle |
| 🇳  149 | ⭕️    4_499_800 | 🕗      2.8164327 | 📌 HamCycle |
| 🇳  150 | ⭕️    4_590_400 | 🕗      2.9506035 | 📌 HamCycle |
| 🇳  151 | ⭕️    4_682_208 | 🕗      2.9311955 | 📌 HamCycle |
| 🇳  152 | ⭕️    4_775_232 | 🕗      2.9945934 | 📌 HamCycle |
| 🇳  153 | ⭕️    4_869_480 | 🕗      3.1270444 | 📌 HamCycle |
| 🇳  154 | ⭕️    4_964_960 | 🕗      3.1455088 | 📌 HamCycle |
| 🇳  155 | ⭕️    5_061_680 | 🕗      3.3308525 | 📌 HamCycle |
| 🇳  156 | ⭕️    5_159_648 | 🕗      3.3205125 | 📌 HamCycle |
| 🇳  157 | ⭕️    5_258_872 | 🕗      3.4157286 | 📌 HamCycle |
| 🇳  158 | ⭕️    5_359_360 | 🕗      3.5471230 | 📌 HamCycle |
| 🇳  159 | ⭕️    5_461_120 | 🕗      3.6002979 | 📌 HamCycle |
| 🇳  160 | ⭕️    5_564_160 | 🕗      3.8180778 | 📌 HamCycle |
| 🇳  161 | ⭕️    5_668_488 | 🕗      3.8757975 | 📌 HamCycle |
| 🇳  162 | ⭕️    5_774_112 | 🕗      3.9876018 | 📌 HamCycle |
| 🇳  163 | ⭕️    5_881_040 | 🕗      4.0251026 | 📌 HamCycle |
| 🇳  164 | ⭕️    5_989_280 | 🕗      4.0782704 | 📌 HamCycle |
| 🇳  165 | ⭕️    6_098_840 | 🕗      4.3708367 | 📌 HamCycle |
| 🇳  166 | ⭕️    6_209_728 | 🕗      4.4891286 | 📌 HamCycle |
| 🇳  167 | ⭕️    6_321_952 | 🕗      4.4396729 | 📌 HamCycle |
| 🇳  168 | ⭕️    6_435_520 | 🕗      4.7049131 | 📌 HamCycle |
| 🇳  169 | ⭕️    6_550_440 | 🕗      4.8034945 | 📌 HamCycle |
| 🇳  170 | ⭕️    6_666_720 | 🕗      4.8120036 | 📌 HamCycle |
| 🇳  171 | ⭕️    6_784_368 | 🕗      5.0999947 | 📌 HamCycle |
| 🇳  172 | ⭕️    6_903_392 | 🕗      5.1406999 | 📌 HamCycle |
| 🇳  173 | ⭕️    7_023_800 | 🕗      5.2226992 | 📌 HamCycle |
| 🇳  174 | ⭕️    7_145_600 | 🕗      5.4760885 | 📌 HamCycle |
| 🇳  175 | ⭕️    7_268_800 | 🕗      5.5385985 | 📌 HamCycle |
| 🇳  176 | ⭕️    7_393_408 | 🕗      5.6618280 | 📌 HamCycle |
| 🇳  177 | ⭕️    7_519_432 | 🕗      5.6368289 | 📌 HamCycle |
| 🇳  178 | ⭕️    7_646_880 | 🕗      5.9726715 | 📌 HamCycle |
| 🇳  179 | ⭕️    7_775_760 | 🕗      5.8712640 | 📌 HamCycle |
| 🇳  180 | ⭕️    7_906_080 | 🕗      6.3023825 | 📌 HamCycle |
| 🇳  181 | ⭕️    8_037_848 | 🕗      6.3492045 | 📌 HamCycle |
| 🇳  182 | ⭕️    8_171_072 | 🕗      6.5235395 | 📌 HamCycle |
| 🇳  183 | ⭕️    8_305_760 | 🕗      6.8535862 | 📌 HamCycle |
| 🇳  184 | ⭕️    8_441_920 | 🕗      6.8551383 | 📌 HamCycle |
| 🇳  185 | ⭕️    8_579_560 | 🕗      7.0290108 | 📌 HamCycle |
| 🇳  186 | ⭕️    8_718_688 | 🕗      7.2782393 | 📌 HamCycle |
| 🇳  187 | ⭕️    8_859_312 | 🕗      7.1627302 | 📌 HamCycle |
| 🇳  188 | ⭕️    9_001_440 | 🕗      6.5989094 | 📌 HamCycle |
| 🇳  189 | ⭕️    9_145_080 | 🕗      6.9484482 | 📌 HamCycle |
| 🇳  190 | ⭕️    9_290_240 | 🕗      7.1640368 | 📌 HamCycle |
| 🇳  191 | ⭕️    9_436_928 | 🕗      7.4092646 | 📌 HamCycle |
| 🇳  192 | ⭕️    9_585_152 | 🕗      7.0487099 | 📌 HamCycle |
| 🇳  193 | ⭕️    9_734_920 | 🕗      7.8608513 | 📌 HamCycle |
| 🇳  194 | ⭕️    9_886_240 | 🕗      7.5017881 | 📌 HamCycle |
| 🇳  195 | ⭕️   10_039_120 | 🕗      8.8736382 | 📌 HamCycle |
| 🇳  196 | ⭕️   10_193_568 | 🕗      9.1231308 | 📌 HamCycle |
| 🇳  197 | ⭕️   10_349_592 | 🕗      8.9380989 | 📌 HamCycle |
| 🇳  198 | ⭕️   10_507_200 | 🕗      9.3729649 | 📌 HamCycle |
| 🇳  199 | ⭕️   10_666_400 | 🕗      9.9800529 | 📌 HamCycle |
| 🇳  200 | ⭕️   10_827_200 | 🕗      9.6292152 | 📌 HamCycle |
| 🇳  201 | ⭕️   10_989_608 | 🕗     10.3535652 | 📌 HamCycle |
| 🇳  202 | ⭕️   11_153_632 | 🕗      9.9995518 | 📌 HamCycle |
| 🇳  203 | ⭕️   11_319_280 | 🕗     10.8522596 | 📌 HamCycle |
| 🇳  204 | ⭕️   11_486_560 | 🕗      9.9973278 | 📌 HamCycle |
| 🇳  205 | ⭕️   11_655_480 | 🕗     10.0661449 | 📌 HamCycle |
| 🇳  206 | ⭕️   11_826_048 | 🕗     10.5527630 | 📌 HamCycle |
| 🇳  207 | ⭕️   11_998_272 | 🕗     10.0613995 | 📌 HamCycle |
| 🇳  208 | ⭕️   12_172_160 | 🕗     10.3916283 | 📌 HamCycle |
| 🇳  209 | ⭕️   12_347_720 | 🕗     11.3493938 | 📌 HamCycle |
| 🇳  210 | ⭕️   12_524_960 | 🕗     11.0978727 | 📌 HamCycle |
| 🇳  211 | ⭕️   12_703_888 | 🕗     11.7553492 | 📌 HamCycle |
| 🇳  212 | ⭕️   12_884_512 | 🕗     11.7754831 | 📌 HamCycle |
| 🇳  213 | ⭕️   13_066_840 | 🕗     12.2666359 | 📌 HamCycle |
| 🇳  214 | ⭕️   13_250_880 | 🕗     11.4517202 | 📌 HamCycle |
| 🇳  215 | ⭕️   13_436_640 | 🕗     12.8435555 | 📌 HamCycle |
| 🇳  216 | ⭕️   13_624_128 | 🕗     12.4718933 | 📌 HamCycle |
| 🇳  217 | ⭕️   13_813_352 | 🕗     12.2922544 | 📌 HamCycle |
| 🇳  218 | ⭕️   14_004_320 | 🕗     13.0266390 | 📌 HamCycle |
| 🇳  219 | ⭕️   14_197_040 | 🕗     13.2950439 | 📌 HamCycle |
| 🇳  220 | ⭕️   14_391_520 | 🕗     13.9172525 | 📌 HamCycle |
| 🇳  221 | ⭕️   14_587_768 | 🕗     13.8073683 | 📌 HamCycle |
| 🇳  222 | ⭕️   14_785_792 | 🕗     15.7910948 | 📌 HamCycle |
| 🇳  223 | ⭕️   14_985_600 | 🕗     16.9914913 | 📌 HamCycle |
| 🇳  224 | ⭕️   15_187_200 | 🕗     16.2843609 | 📌 HamCycle |
| 🇳  225 | ⭕️   15_390_600 | 🕗     17.7109051 | 📌 HamCycle |
| 🇳  226 | ⭕️   15_595_808 | 🕗     17.9329987 | 📌 HamCycle |
| 🇳  227 | ⭕️   15_802_832 | 🕗     16.9582157 | 📌 HamCycle |
| 🇳  228 | ⭕️   16_011_680 | 🕗     19.2418537 | 📌 HamCycle |
| 🇳  229 | ⭕️   16_222_360 | 🕗     19.1698818 | 📌 HamCycle |
| 🇳  230 | ⭕️   16_434_880 | 🕗     19.6889629 | 📌 HamCycle |
| 🇳  231 | ⭕️   16_649_248 | 🕗     19.8991928 | 📌 HamCycle |
| 🇳  232 | ⭕️   16_865_472 | 🕗     21.5891342 | 📌 HamCycle |
| 🇳  233 | ⭕️   17_083_560 | 🕗     20.3988705 | 📌 HamCycle |
| 🇳  234 | ⭕️   17_303_520 | 🕗     21.7813530 | 📌 HamCycle |
| 🇳  235 | ⭕️   17_525_360 | 🕗     21.0040817 | 📌 HamCycle |
| 🇳  236 | ⭕️   17_749_088 | 🕗     21.1004257 | 📌 HamCycle |
| 🇳  237 | ⭕️   17_974_712 | 🕗     21.8259106 | 📌 HamCycle |
| 🇳  238 | ⭕️   18_202_240 | 🕗     21.2663593 | 📌 HamCycle |
| 🇳  239 | ⭕️   18_431_680 | 🕗     23.2225170 | 📌 HamCycle |
| 🇳  240 | ⭕️   18_663_040 | 🕗     22.7874584 | 📌 HamCycle |
| 🇳  241 | ⭕️   18_896_328 | 🕗     22.8182411 | 📌 HamCycle |
| 🇳  242 | ⭕️   19_131_552 | 🕗     25.7061996 | 📌 HamCycle |
| 🇳  243 | ⭕️   19_368_720 | 🕗     23.7010460 | 📌 HamCycle |
| 🇳  244 | ⭕️   19_607_840 | 🕗     24.9740658 | 📌 HamCycle |
| 🇳  245 | ⭕️   19_848_920 | 🕗     25.3079643 | 📌 HamCycle |
| 🇳  246 | ⭕️   20_091_968 | 🕗     27.0324688 | 📌 HamCycle |
| 🇳  247 | ⭕️   20_336_992 | 🕗     27.4217663 | 📌 HamCycle |
| 🇳  248 | ⭕️   20_584_000 | 🕗     26.6019325 | 📌 HamCycle |
| 🇳  249 | ⭕️   20_833_000 | 🕗     27.6203270 | 📌 HamCycle |
| 🇳  250 | ⭕️   21_084_000 | 🕗     28.2027264 | 📌 HamCycle |
| 🇳  251 | ⭕️   21_337_008 | 🕗     27.0395622 | 📌 HamCycle |
| 🇳  252 | ⭕️   21_592_032 | 🕗     28.6985397 | 📌 HamCycle |
| 🇳  253 | ⭕️   21_849_080 | 🕗     29.4975128 | 📌 HamCycle |
| 🇳  254 | ⭕️   22_108_160 | 🕗     29.6304131 | 📌 HamCycle |
| 🇳  255 | ⭕️   22_369_280 | 🕗     30.6687088 | 📌 HamCycle |
| 🇳  256 | ⭕️   22_632_448 | 🕗     30.4113617 | 📌 HamCycle |
| 🇳  257 | ⭕️   22_897_672 | 🕗     29.9068413 | 📌 HamCycle |
| 🇳  258 | ⭕️   23_164_960 | 🕗     31.8059349 | 📌 HamCycle |
| 🇳  259 | ⭕️   23_434_320 | 🕗     32.5649033 | 📌 HamCycle |
| 🇳  260 | ⭕️   23_705_760 | 🕗     43.0857239 | 📌 HamCycle |
| 🇳  261 | ⭕️   23_979_288 | 🕗     35.0142784 | 📌 HamCycle |
| 🇳  262 | ⭕️   24_254_912 | 🕗     34.9660568 | 📌 HamCycle |
| 🇳  263 | ⭕️   24_532_640 | 🕗     37.7527428 | 📌 HamCycle |
| 🇳  264 | ⭕️   24_812_480 | 🕗     35.2763977 | 📌 HamCycle |
| 🇳  265 | ⭕️   25_094_440 | 🕗     35.9335899 | 📌 HamCycle |
| 🇳  266 | ⭕️   25_378_528 | 🕗     35.7696381 | 📌 HamCycle |
| 🇳  267 | ⭕️   25_664_752 | 🕗     36.4937744 | 📌 HamCycle |
| 🇳  268 | ⭕️   25_953_120 | 🕗     37.5988083 | 📌 HamCycle |
| 🇳  269 | ⭕️   26_243_640 | 🕗     38.6533852 | 📌 HamCycle |
| 🇳  270 | ⭕️   26_536_320 | 🕗     37.9381294 | 📌 HamCycle |
| 🇳  271 | ⭕️   26_831_168 | 🕗     38.3561020 | 📌 HamCycle |
| 🇳  272 | ⭕️   27_128_192 | 🕗     40.8908157 | 📌 HamCycle |
| 🇳  273 | ⭕️   27_427_400 | 🕗     40.7272644 | 📌 HamCycle |
| 🇳  274 | ⭕️   27_728_800 | 🕗     42.1620789 | 📌 HamCycle |
| 🇳  275 | ⭕️   28_032_400 | 🕗     43.5559692 | 📌 HamCycle |
| 🇳  276 | ⭕️   28_338_208 | 🕗     43.0557137 | 📌 HamCycle |
| 🇳  277 | ⭕️   28_646_232 | 🕗     43.3311501 | 📌 HamCycle |
| 🇳  278 | ⭕️   28_956_480 | 🕗     42.8031197 | 📌 HamCycle |
| 🇳  279 | ⭕️   29_268_960 | 🕗     42.8381500 | 📌 HamCycle |
| 🇳  280 | ⭕️   29_583_680 | 🕗     44.9772606 | 📌 HamCycle |
| 🇳  281 | ⭕️   29_900_648 | 🕗     43.9280205 | 📌 HamCycle |
| 🇳  282 | ⭕️   30_219_872 | 🕗     46.0957870 | 📌 HamCycle |
| 🇳  283 | ⭕️   30_541_360 | 🕗     45.5541229 | 📌 HamCycle |
| 🇳  284 | ⭕️   30_865_120 | 🕗     47.6350288 | 📌 HamCycle |
| 🇳  285 | ⭕️   31_191_160 | 🕗     45.8156738 | 📌 HamCycle |
| 🇳  286 | ⭕️   31_519_488 | 🕗     49.6814232 | 📌 HamCycle |
| 🇳  287 | ⭕️   31_850_112 | 🕗     50.4522057 | 📌 HamCycle |
| 🇳  288 | ⭕️   32_183_040 | 🕗     52.4631805 | 📌 HamCycle |
| 🇳  289 | ⭕️   32_518_280 | 🕗     51.8870506 | 📌 HamCycle |
| 🇳  290 | ⭕️   32_855_840 | 🕗     50.3747063 | 📌 HamCycle |
| 🇳  291 | ⭕️   33_195_728 | 🕗     52.7208900 | 📌 HamCycle |
| 🇳  292 | ⭕️   33_537_952 | 🕗     52.0394096 | 📌 HamCycle |
| 🇳  293 | ⭕️   33_882_520 | 🕗     55.0489998 | 📌 HamCycle |
| 🇳  294 | ⭕️   34_229_440 | 🕗     53.3723679 | 📌 HamCycle |
| 🇳  295 | ⭕️   34_578_720 | 🕗     55.3163872 | 📌 HamCycle |
| 🇳  296 | ⭕️   34_930_368 | 🕗     54.2411690 | 📌 HamCycle |
| 🇳  297 | ⭕️   35_284_392 | 🕗     51.9341431 | 📌 HamCycle |
| 🇳  298 | ⭕️   35_640_800 | 🕗     51.9760246 | 📌 HamCycle |
| 🇳  299 | ⭕️   35_999_600 | 🕗     55.4841614 | 📌 HamCycle |
| 🇳  300 | ⭕️   36_360_800 | 🕗     54.6801987 | 📌 HamCycle |
| 🇳  301 | ⭕️   36_724_408 | 🕗     54.8235283 | 📌 HamCycle |
| 🇳  302 | ⭕️   37_090_432 | 🕗     54.9316254 | 📌 HamCycle |
| 🇳  303 | ⭕️   37_458_880 | 🕗     56.6585999 | 📌 HamCycle |
| 🇳  304 | ⭕️   37_829_760 | 🕗     56.6689377 | 📌 HamCycle |
| 🇳  305 | ⭕️   38_203_080 | 🕗     58.0978699 | 📌 HamCycle |
| 🇳  306 | ⭕️   38_578_848 | 🕗     60.0594254 | 📌 HamCycle |
| 🇳  307 | ⭕️   38_957_072 | 🕗     58.4241867 | 📌 HamCycle |
| 🇳  308 | ⭕️   39_337_760 | 🕗     60.0390778 | 📌 HamCycle |
| 🇳  309 | ⭕️   39_720_920 | 🕗     58.0487709 | 📌 HamCycle |
| 🇳  310 | ⭕️   40_106_560 | 🕗     63.4117470 | 📌 HamCycle |
| 🇳  311 | ⭕️   40_494_688 | 🕗     64.7791367 | 📌 HamCycle |
| 🇳  312 | ⭕️   40_885_312 | 🕗     63.1891747 | 📌 HamCycle |
| 🇳  313 | ⭕️   41_278_440 | 🕗     62.3610802 | 📌 HamCycle |
| 🇳  314 | ⭕️   41_674_080 | 🕗     62.7601547 | 📌 HamCycle |
| 🇳  315 | ⭕️   42_072_240 | 🕗     70.5453110 | 📌 HamCycle |
| 🇳  316 | ⭕️   42_472_928 | 🕗     64.9709167 | 📌 HamCycle |
| 🇳  317 | ⭕️   42_876_152 | 🕗     67.3512802 | 📌 HamCycle |
| 🇳  318 | ⭕️   43_281_920 | 🕗     66.5858994 | 📌 HamCycle |
| 🇳  319 | ⭕️   43_690_240 | 🕗     63.9276123 | 📌 HamCycle |
| 🇳  320 | ⭕️   44_101_120 | 🕗     69.6732635 | 📌 HamCycle |
| 🇳  321 | ⭕️   44_514_568 | 🕗     69.3464203 | 📌 HamCycle |
| 🇳  322 | ⭕️   44_930_592 | 🕗     70.3431854 | 📌 HamCycle |
| 🇳  323 | ⭕️   45_349_200 | 🕗     73.5013657 | 📌 HamCycle |
| 🇳  324 | ⭕️   45_770_400 | 🕗     79.6885605 | 📌 HamCycle |
| 🇳  325 | ⭕️   46_194_200 | 🕗     74.0430298 | 📌 HamCycle |
| 🇳  326 | ⭕️   46_620_608 | 🕗     75.2163620 | 📌 HamCycle |
| 🇳  327 | ⭕️   47_049_632 | 🕗     78.0254517 | 📌 HamCycle |
| 🇳  328 | ⭕️   47_481_280 | 🕗     80.5947723 | 📌 HamCycle |
| 🇳  329 | ⭕️   47_915_560 | 🕗     79.0147934 | 📌 HamCycle |
| 🇳  330 | ⭕️   48_352_480 | 🕗     84.0448608 | 📌 HamCycle |
| 🇳  331 | ⭕️   48_792_048 | 🕗     75.8492889 | 📌 HamCycle |
| 🇳  332 | ⭕️   49_234_272 | 🕗     81.1769180 | 📌 HamCycle |
| 🇳  333 | ⭕️   49_679_160 | 🕗     81.2884064 | 📌 HamCycle |
| 🇳  334 | ⭕️   50_126_720 | 🕗     82.8648071 | 📌 HamCycle |
| 🇳  335 | ⭕️   50_576_960 | 🕗     86.6697006 | 📌 HamCycle |
| 🇳  336 | ⭕️   51_029_888 | 🕗     80.7290878 | 📌 HamCycle |
| 🇳  337 | ⭕️   51_485_512 | 🕗     87.1696930 | 📌 HamCycle |
| 🇳  338 | ⭕️   51_943_840 | 🕗     88.1169586 | 📌 HamCycle |
| 🇳  339 | ⭕️   52_404_880 | 🕗     89.6203918 | 📌 HamCycle |
| 🇳  340 | ⭕️   52_868_640 | 🕗     88.3642960 | 📌 HamCycle |
| 🇳  341 | ⭕️   53_335_128 | 🕗     91.5185699 | 📌 HamCycle |
| 🇳  342 | ⭕️   53_804_352 | 🕗     90.6791458 | 📌 HamCycle |
| 🇳  343 | ⭕️   54_276_320 | 🕗     93.8216400 | 📌 HamCycle |
| 🇳  344 | ⭕️   54_751_040 | 🕗     97.2271423 | 📌 HamCycle |
| 🇳  345 | ⭕️   55_228_520 | 🕗     98.5113297 | 📌 HamCycle |
| 🇳  346 | ⭕️   55_708_768 | 🕗     98.7739105 | 📌 HamCycle |
| 🇳  347 | ⭕️   56_191_792 | 🕗    100.9957047 | 📌 HamCycle |
| 🇳  348 | ⭕️   56_677_600 | 🕗    101.0744858 | 📌 HamCycle |
| 🇳  349 | ⭕️   57_166_200 | 🕗    102.4343033 | 📌 HamCycle |
| 🇳  350 | ⭕️   57_657_600 | 🕗     99.1940536 | 📌 HamCycle |
| 🇳  351 | ⭕️   58_151_808 | 🕗     99.3149796 | 📌 HamCycle |
| 🇳  352 | ⭕️   58_648_832 | 🕗    106.0990753 | 📌 HamCycle |
| 🇳  353 | ⭕️   59_148_680 | 🕗    105.4814377 | 📌 HamCycle |
| 🇳  354 | ⭕️   59_651_360 | 🕗    105.9914627 | 📌 HamCycle |
| 🇳  355 | ⭕️   60_156_880 | 🕗    107.3770676 | 📌 HamCycle |
| 🇳  356 | ⭕️   60_665_248 | 🕗    112.0929108 | 📌 HamCycle |
| 🇳  357 | ⭕️   61_176_472 | 🕗    113.0250702 | 📌 HamCycle |
| 🇳  358 | ⭕️   61_690_560 | 🕗    112.2821579 | 📌 HamCycle |
| 🇳  359 | ⭕️   62_207_520 | 🕗    113.3290863 | 📌 HamCycle |
| 🇳  360 | ⭕️   62_727_360 | 🕗    118.3366318 | 📌 HamCycle |
| 🇳  361 | ⭕️   63_250_088 | 🕗    114.1650162 | 📌 HamCycle |
| 🇳  362 | ⭕️   63_775_712 | 🕗    119.7422180 | 📌 HamCycle |
| 🇳  363 | ⭕️   64_304_240 | 🕗    118.4231873 | 📌 HamCycle |
| 🇳  364 | ⭕️   64_835_680 | 🕗    119.1417236 | 📌 HamCycle |
| 🇳  365 | ⭕️   65_370_040 | 🕗    122.5782623 | 📌 HamCycle |
| 🇳  366 | ⭕️   65_907_328 | 🕗    122.3472290 | 📌 HamCycle |
| 🇳  367 | ⭕️   66_447_552 | 🕗    121.7662506 | 📌 HamCycle |
| 🇳  368 | ⭕️   66_990_720 | 🕗    128.8301849 | 📌 HamCycle |
| 🇳  369 | ⭕️   67_536_840 | 🕗    127.1080475 | 📌 HamCycle |
| 🇳  370 | ⭕️   68_085_920 | 🕗    129.2564545 | 📌 HamCycle |
| 🇳  371 | ⭕️   68_637_968 | 🕗    142.4124908 | 📌 HamCycle |
| 🇳  372 | ⭕️   69_192_992 | 🕗    138.0613861 | 📌 HamCycle |
| 🇳  373 | ⭕️   69_751_000 | 🕗    138.7724304 | 📌 HamCycle |
| 🇳  374 | ⭕️   70_312_000 | 🕗    137.8485718 | 📌 HamCycle |
| 🇳  375 | ⭕️   70_876_000 | 🕗    141.4355316 | 📌 HamCycle |
| 🇳  376 | ⭕️   71_443_008 | 🕗    142.3190918 | 📌 HamCycle |
| 🇳  377 | ⭕️   72_013_032 | 🕗    138.6949768 | 📌 HamCycle |
| 🇳  378 | ⭕️   72_586_080 | 🕗    139.6701355 | 📌 HamCycle |
| 🇳  379 | ⭕️   73_162_160 | 🕗    140.9620361 | 📌 HamCycle |
| 🇳  380 | ⭕️   73_741_280 | 🕗    147.0547333 | 📌 HamCycle |
| 🇳  381 | ⭕️   74_323_448 | 🕗    147.3148956 | 📌 HamCycle |
| 🇳  382 | ⭕️   74_908_672 | 🕗    154.1600494 | 📌 HamCycle |
| 🇳  383 | ⭕️   75_496_960 | 🕗    148.8446350 | 📌 HamCycle |
| 🇳  384 | ⭕️   76_088_320 | 🕗    159.7727356 | 📌 HamCycle |
| 🇳  385 | ⭕️   76_682_760 | 🕗    153.8689575 | 📌 HamCycle |
| 🇳  386 | ⭕️   77_280_288 | 🕗    153.9478607 | 📌 HamCycle |
| 🇳  387 | ⭕️   77_880_912 | 🕗    152.6566315 | 📌 HamCycle |
| 🇳  388 | ⭕️   78_484_640 | 🕗    154.2044067 | 📌 HamCycle |
| 🇳  389 | ⭕️   79_091_480 | 🕗    151.8361816 | 📌 HamCycle |
| 🇳  390 | ⭕️   79_701_440 | 🕗    159.2859039 | 📌 HamCycle |
| 🇳  391 | ⭕️   80_314_528 | 🕗    157.1283112 | 📌 HamCycle |
| 🇳  392 | ⭕️   80_930_752 | 🕗    163.1609955 | 📌 HamCycle |
| 🇳  393 | ⭕️   81_550_120 | 🕗    168.0905151 | 📌 HamCycle |
| 🇳  394 | ⭕️   82_172_640 | 🕗    174.3224030 | 📌 HamCycle |
| 🇳  395 | ⭕️   82_798_320 | 🕗    173.4094696 | 📌 HamCycle |
| 🇳  396 | ⭕️   83_427_168 | 🕗    166.3798370 | 📌 HamCycle |
| 🇳  397 | ⭕️   84_059_192 | 🕗    174.8748474 | 📌 HamCycle |
| 🇳  398 | ⭕️   84_694_400 | 🕗    171.5663300 | 📌 HamCycle |
| 🇳  399 | ⭕️   85_332_800 | 🕗    176.2673798 | 📌 HamCycle |
| 🇳  400 | ⭕️   85_974_400 | 🕗    181.5958099 | 📌 HamCycle |
| 🇳  401 | ⭕️   86_619_208 | 🕗    180.0063477 | 📌 HamCycle |
| 🇳  402 | ⭕️   87_267_232 | 🕗    179.0526123 | 📌 HamCycle |
| 🇳  403 | ⭕️   87_918_480 | 🕗    182.1016083 | 📌 HamCycle |
| 🇳  404 | ⭕️   88_572_960 | 🕗    180.8249359 | 📌 HamCycle |
| 🇳  405 | ⭕️   89_230_680 | 🕗    180.9492340 | 📌 HamCycle |
| 🇳  406 | ⭕️   89_891_648 | 🕗    190.3913879 | 📌 HamCycle |
| 🇳  407 | ⭕️   90_555_872 | 🕗    192.4388580 | 📌 HamCycle |
| 🇳  408 | ⭕️   91_223_360 | 🕗    191.1726532 | 📌 HamCycle |
| 🇳  409 | ⭕️   91_894_120 | 🕗    201.1885529 | 📌 HamCycle |
| 🇳  410 | ⭕️   92_568_160 | 🕗    198.3737030 | 📌 HamCycle |
| 🇳  411 | ⭕️   93_245_488 | 🕗    194.6151276 | 📌 HamCycle |
| 🇳  412 | ⭕️   93_926_112 | 🕗    205.8743744 | 📌 HamCycle |
| 🇳  413 | ⭕️   94_610_040 | 🕗    205.3444519 | 📌 HamCycle |
| 🇳  414 | ⭕️   95_297_280 | 🕗    204.2312012 | 📌 HamCycle |
| 🇳  415 | ⭕️   95_987_840 | 🕗    214.5655060 | 📌 HamCycle |
| 🇳  416 | ⭕️   96_681_728 | 🕗    204.0137482 | 📌 HamCycle |
| 🇳  417 | ⭕️   97_378_952 | 🕗    212.5135193 | 📌 HamCycle |
| 🇳  418 | ⭕️   98_079_520 | 🕗    225.6294098 | 📌 HamCycle |
| 🇳  419 | ⭕️   98_783_440 | 🕗    209.5414124 | 📌 HamCycle |
| 🇳  420 | ⭕️   99_490_720 | 🕗    205.7293549 | 📌 HamCycle |
| 🇳  421 | ⭕️  100_201_368 | 🕗    228.3509674 | 📌 HamCycle |
| 🇳  422 | ⭕️  100_915_392 | 🕗    229.7209167 | 📌 HamCycle |
| 🇳  423 | ⭕️  101_632_800 | 🕗    233.1378174 | 📌 HamCycle |
| 🇳  424 | ⭕️  102_353_600 | 🕗    222.1818085 | 📌 HamCycle |
| 🇳  425 | ⭕️  103_077_800 | 🕗    232.3297272 | 📌 HamCycle |
| 🇳  426 | ⭕️  103_805_408 | 🕗    229.1684723 | 📌 HamCycle |
| 🇳  427 | ⭕️  104_536_432 | 🕗    243.6679382 | 📌 HamCycle |
| 🇳  428 | ⭕️  105_270_880 | 🕗    236.8065186 | 📌 HamCycle |
| 🇳  425 | ⭕️  103_077_800 | 🕗    232.3297272 | 📌 HamCycle |
| 🇳  426 | ⭕️  103_805_408 | 🕗    229.1684723 | 📌 HamCycle |
| 🇳  427 | ⭕️  104_536_432 | 🕗    243.6679382 | 📌 HamCycle |
| 🇳  428 | ⭕️  105_270_880 | 🕗    236.8065186 | 📌 HamCycle |
| 🇳  425 | ⭕️  103_077_800 | 🕗    232.3297272 | 📌 HamCycle |
| 🇳  426 | ⭕️  103_805_408 | 🕗    229.1684723 | 📌 HamCycle |
| 🇳  427 | ⭕️  104_536_432 | 🕗    243.6679382 | 📌 HamCycle |
| 🇳  428 | ⭕️  105_270_880 | 🕗    236.8065186 | 📌 HamCycle |
| 🇳  429 | ⭕️  106_008_760 | 🕗    238.1333313 | 📌 HamCycle |
| 🇳  430 | ⭕️  106_750_080 | 🕗    246.2862091 | 📌 HamCycle |
| 🇳  431 | ⭕️  107_494_848 | 🕗    246.6469116 | 📌 HamCycle |
| 🇳  432 | ⭕️  108_243_072 | 🕗    262.9234924 | 📌 HamCycle |
| 🇳  433 | ⭕️  108_994_760 | 🕗    249.9560699 | 📌 HamCycle |
| 🇳  434 | ⭕️  109_749_920 | 🕗    251.8942566 | 📌 HamCycle |
| 🇳  440 | ⭕️  114_354_240 | 🕗    260.3761292 | 📌 HamCycle |
| 🇳  441 | ⭕️  115_133_928 | 🕗    263.5421143 | 📌 HamCycle |
| 🇳  442 | ⭕️  115_917_152 | 🕗    259.1378784 | 📌 HamCycle |
| 🇳  443 | ⭕️  116_703_920 | 🕗    270.7347717 | 📌 HamCycle |
| 🇳  444 | ⭕️  117_494_240 | 🕗    288.3459778 | 📌 HamCycle |
| 🇳  445 | ⭕️  118_288_120 | 🕗    278.2333679 | 📌 HamCycle |
| 🇳  446 | ⭕️  119_085_568 | 🕗    280.1857910 | 📌 HamCycle |
| 🇳  447 | ⭕️  119_886_592 | 🕗    299.4025269 | 📌 HamCycle |
| 🇳  448 | ⭕️  120_691_200 | 🕗    303.8379517 | 📌 HamCycle |
| 🇳  449 | ⭕️  121_499_400 | 🕗    315.9012451 | 📌 HamCycle |
| 🇳  450 | ⭕️  122_311_200 | 🕗    314.5906372 | 📌 HamCycle |
| 🇳  451 | ⭕️  123_126_608 | 🕗    326.6910400 | 📌 HamCycle |
| 🇳  452 | ⭕️  123_945_632 | 🕗    334.6388855 | 📌 HamCycle |
| 🇳  453 | ⭕️  124_768_280 | 🕗    340.3795471 | 📌 HamCycle |
| 🇳  454 | ⭕️  125_594_560 | 🕗    345.4236450 | 📌 HamCycle |
| 🇳  455 | ⭕️  126_424_480 | 🕗    332.5567932 | 📌 HamCycle |
| 🇳  456 | ⭕️  127_258_048 | 🕗    346.8245544 | 📌 HamCycle |
| 🇳  457 | ⭕️  128_095_272 | 🕗    355.9805908 | 📌 HamCycle |
| 🇳  458 | ⭕️  128_936_160 | 🕗    359.0083313 | 📌 HamCycle |
| 🇳  459 | ⭕️  129_780_720 | 🕗    375.8215637 | 📌 HamCycle |
| 🇳  460 | ⭕️  130_628_960 | 🕗    376.1432495 | 📌 HamCycle |
| 🇳  461 | ⭕️  131_480_888 | 🕗    376.5814209 | 📌 HamCycle |
| 🇳  462 | ⭕️  132_336_512 | 🕗    397.5814209 | 📌 HamCycle |
| 🇳  463 | ⭕️  133_195_840 | 🕗    377.2076111 | 📌 HamCycle |
| 🇳  464 | ⭕️  134_058_880 | 🕗    375.8837280 | 📌 HamCycle |
| 🇳  465 | ⭕️  134_925_640 | 🕗    388.6386108 | 📌 HamCycle |
| 🇳  466 | ⭕️  135_796_128 | 🕗    395.4193420 | 📌 HamCycle |
| 🇳  467 | ⭕️  136_670_352 | 🕗    388.5943604 | 📌 HamCycle |
| 🇳  468 | ⭕️  137_548_320 | 🕗    379.1246338 | 📌 HamCycle |
| 🇳  469 | ⭕️  138_430_040 | 🕗    392.6550903 | 📌 HamCycle |
| 🇳  470 | ⭕️  139_315_520 | 🕗    388.9270325 | 📌 HamCycle |
| 🇳  471 | ⭕️  140_204_768 | 🕗    318.1142578 | 📌 HamCycle |
| 🇳  472 | ⭕️  141_097_792 | 🕗    355.8233643 | 📌 HamCycle |
| 🇳  473 | ⭕️  141_994_600 | 🕗    344.4631348 | 📌 HamCycle |
| 🇳  474 | ⭕️  142_895_200 | 🕗    345.8013916 | 📌 HamCycle |
| 🇳  475 | ⭕️  143_799_600 | 🕗    352.4673462 | 📌 HamCycle |
| 🇳  476 | ⭕️  144_707_808 | 🕗    343.4831543 | 📌 HamCycle |
| 🇳  477 | ⭕️  145_619_832 | 🕗    365.9343567 | 📌 HamCycle |
| 🇳  478 | ⭕️  146_535_680 | 🕗    377.1398010 | 📌 HamCycle |
| 🇳  479 | ⭕️  147_455_360 | 🕗    365.9949951 | 📌 HamCycle |
| 🇳  480 | ⭕️  148_378_880 | 🕗    386.4993591 | 📌 HamCycle |
| 🇳  481 | ⭕️  149_306_248 | 🕗    373.3592224 | 📌 HamCycle |
| 🇳  482 | ⭕️  150_237_472 | 🕗    370.6579895 | 📌 HamCycle |
| 🇳  483 | ⭕️  151_172_560 | 🕗    372.9201965 | 📌 HamCycle |
| 🇳  484 | ⭕️  152_111_520 | 🕗    392.2236938 | 📌 HamCycle |
| 🇳  485 | ⭕️  153_054_360 | 🕗    391.6574402 | 📌 HamCycle |
| 🇳  486 | ⭕️  154_001_088 | 🕗    376.3860474 | 📌 HamCycle |
| 🇳  487 | ⭕️  154_951_712 | 🕗    394.4570312 | 📌 HamCycle |
| 🇳  488 | ⭕️  155_906_240 | 🕗    390.2160034 | 📌 HamCycle |
| 🇳  489 | ⭕️  156_864_680 | 🕗    388.6850586 | 📌 HamCycle |
| 🇳  490 | ⭕️  157_827_040 | 🕗    403.0396729 | 📌 HamCycle |
| 🇳  491 | ⭕️  158_793_328 | 🕗    407.4985352 | 📌 HamCycle |
| 🇳  492 | ⭕️  159_763_552 | 🕗    409.3179932 | 📌 HamCycle |
| 🇳  493 | ⭕️  160_737_720 | 🕗    412.5714417 | 📌 HamCycle |
| 🇳  494 | ⭕️  161_715_840 | 🕗    410.0222778 | 📌 HamCycle |
| 🇳  495 | ⭕️  162_697_920 | 🕗    418.6289673 | 📌 HamCycle |
| 🇳  496 | ⭕️  163_683_968 | 🕗    418.1947937 | 📌 HamCycle |
| 🇳  497 | ⭕️  164_673_992 | 🕗    439.4223633 | 📌 HamCycle |
| 🇳  498 | ⭕️  165_668_000 | 🕗    438.3364868 | 📌 HamCycle |
| 🇳  499 | ⭕️  166_666_000 | 🕗    492.7932739 | 📌 HamCycle |
| 🇳  500 | ⭕️  167_668_000 | 🕗    516.4312134 | 📌 HamCycle |
| 🇳  501 | ⭕️  168_674_008 | 🕗    524.0866699 | 📌 HamCycle |
| 🇳  502 | ⭕️  169_684_032 | 🕗    511.3805542 | 📌 HamCycle |
| 🇳  503 | ⭕️  170_698_080 | 🕗    514.9252930 | 📌 HamCycle |
| 🇳  504 | ⭕️  171_716_160 | 🕗    522.7913818 | 📌 HamCycle |
| 🇳  505 | ⭕️  172_738_280 | 🕗    505.0050659 | 📌 HamCycle |
| 🇳  506 | ⭕️  173_764_448 | 🕗    521.3956299 | 📌 HamCycle |
| 🇳  507 | ⭕️  174_794_672 | 🕗    534.2333374 | 📌 HamCycle |
| 🇳  508 | ⭕️  175_828_960 | 🕗    533.0936279 | 📌 HamCycle |
| 🇳  509 | ⭕️  176_867_320 | 🕗    530.5546265 | 📌 HamCycle |
| 🇳  510 | ⭕️  177_909_760 | 🕗    537.9629517 | 📌 HamCycle |
| 🇳  511 | ⭕️  178_956_288 | 🕗    554.1561279 | 📌 HamCycle |
| 🇳  512 | ⭕️  180_006_912 | 🕗    562.2781372 | 📌 HamCycle |
| 🇳  513 | ⭕️  181_061_640 | 🕗    536.6725464 | 📌 HamCycle |
| 🇳  514 | ⭕️  182_120_480 | 🕗    558.1255493 | 📌 HamCycle |
| 🇳  515 | ⭕️  183_183_440 | 🕗    559.7949829 | 📌 HamCycle |
| 🇳  516 | ⭕️  184_250_528 | 🕗    553.7504272 | 📌 HamCycle |
| 🇳  517 | ⭕️  185_321_752 | 🕗    568.6388550 | 📌 HamCycle |
| 🇳  518 | ⭕️  186_397_120 | 🕗    566.1765747 | 📌 HamCycle |
| 🇳  519 | ⭕️  187_476_640 | 🕗    554.7770996 | 📌 HamCycle |
| 🇳  520 | ⭕️  188_560_320 | 🕗    567.4897461 | 📌 HamCycle |
| 🇳  521 | ⭕️  189_648_168 | 🕗    582.4830933 | 📌 HamCycle |
| 🇳  512 | ⭕️  180_006_912 | 🕗    562.2781372 | 📌 HamCycle |
| 🇳  513 | ⭕️  181_061_640 | 🕗    536.6725464 | 📌 HamCycle |
| 🇳  514 | ⭕️  182_120_480 | 🕗    558.1255493 | 📌 HamCycle |
| 🇳  515 | ⭕️  183_183_440 | 🕗    559.7949829 | 📌 HamCycle |
| 🇳  516 | ⭕️  184_250_528 | 🕗    553.7504272 | 📌 HamCycle |
| 🇳  517 | ⭕️  185_321_752 | 🕗    568.6388550 | 📌 HamCycle |
| 🇳  518 | ⭕️  186_397_120 | 🕗    566.1765747 | 📌 HamCycle |
| 🇳  519 | ⭕️  187_476_640 | 🕗    554.7770996 | 📌 HamCycle |
| 🇳  520 | ⭕️  188_560_320 | 🕗    567.4897461 | 📌 HamCycle |
| 🇳  521 | ⭕️  189_648_168 | 🕗    582.4830933 | 📌 HamCycle |
| 🇳  522 | ⭕️  190_740_192 | 🕗    569.0377197 | 📌 HamCycle |
| 🇳  523 | ⭕️  191_836_400 | 🕗    597.1223755 | 📌 HamCycle |
| 🇳  524 | ⭕️  192_936_800 | 🕗    601.2348633 | 📌 HamCycle |
| 🇳  525 | ⭕️  194_041_400 | 🕗    580.0333862 | 📌 HamCycle |
| 🇳  526 | ⭕️  195_150_208 | 🕗    507.3114014 | 📌 HamCycle |
| 🇳  527 | ⭕️  196_263_232 | 🕗    502.4771118 | 📌 HamCycle |
| 🇳  528 | ⭕️  197_380_480 | 🕗    511.4998169 | 📌 HamCycle |
| 🇳  529 | ⭕️  198_501_960 | 🕗    509.7370605 | 📌 HamCycle |
| 🇳  530 | ⭕️  199_627_680 | 🕗    520.2709961 | 📌 HamCycle |
| 🇳  531 | ⭕️  200_757_648 | 🕗    518.3181152 | 📌 HamCycle |
| 🇳  532 | ⭕️  201_891_872 | 🕗    504.7783813 | 📌 HamCycle |
| 🇳  533 | ⭕️  203_030_360 | 🕗    535.0303345 | 📌 HamCycle |
| 🇳  534 | ⭕️  204_173_120 | 🕗    544.2910156 | 📌 HamCycle | 51.5 GB
| 🇳  535 | ⭕️  205_320_160 | 🕗    531.3876953 | 📌 HamCycle | 53.3 GB
| 🇳  536 | ⭕️  206_471_488 | 🕗    566.9558716 | 📌 HamCycle | 54.6 GB
| 🇳  537 | ⭕️  207_627_112 | 🕗    573.6485596 | 📌 HamCycle |
| 🇳  538 | ⭕️  208_787_040 | 🕗    555.6110229 | 📌 HamCycle |
| 🇳  539 | ⭕️  209_951_280 | 🕗    570.1338501 | 📌 HamCycle |
| 🇳  540 | ⭕️  211_119_840 | 🕗    565.0805664 | 📌 HamCycle |
| 🇳  541 | ⭕️  212_292_728 | 🕗    563.1724243 | 📌 HamCycle |
| 🇳  542 | ⭕️  213_469_952 | 🕗    572.6234131 | 📌 HamCycle |
| 🇳  543 | ⭕️  214_651_520 | 🕗    595.1621094 | 📌 HamCycle |
| 🇳  544 | ⭕️  215_837_440 | 🕗    595.5155029 | 📌 HamCycle | 55.8 GB
| 🇳  545 | ⭕️  217_027_720 | 🕗    614.3336182 | 📌 HamCycle | 56.8 GB
| 🇳  546 | ⭕️  218_222_368 | 🕗    676.3571167 | 📌 HamCycle |
| 🇳  547 | ⭕️  219_421_392 | 🕗    706.4018555 | 📌 HamCycle |
| 🇳  548 | ⭕️  220_624_800 | 🕗    707.0447388 | 📌 HamCycle |
| 🇳  549 | ⭕️  221_832_600 | 🕗    690.8361206 | 📌 HamCycle |
| 🇳  550 | ⭕️  223_044_800 | 🕗    601.1645508 | 📌 HamCycle | 51.6 GB
| 🇳  551 | ⭕️  224_261_408 | 🕗    666.2920532 | 📌 HamCycle | 53.5 GB
| 🇳  552 | ⭕️  225_482_432 | 🕗    707.9029541 | 📌 HamCycle |
| 🇳  553 | ⭕️  226_707_880 | 🕗    748.0688477 | 📌 HamCycle |
| 🇳  554 | ⭕️  227_937_760 | 🕗    676.2214966 | 📌 HamCycle |
| 🇳  555 | ⭕️  229_172_080 | 🕗    712.5463867 | 📌 HamCycle |
| 🇳  556 | ⭕️  230_410_848 | 🕗    714.9957275 | 📌 HamCycle | 56.1 GB Memory
| 🇳  557 | ⭕️  231_654_072 | 🕗    714.2865601 | 📌 HamCycle | 53.5 GB Memory
| 🇳  558 | ⭕️  232_901_760 | 🕗    738.4812622 | 📌 HamCycle | 53.1 GB Memory
| 🇳  559 | ⭕️  234_153_920 | 🕗    678.1728516 | 📌 HamCycle | 61.1 GB Memory
__________________________________________________________________________________________________
| 🇳  560 | ⭕️  235_410_560 | 🕗    **** killed | 📌 ???????? | 74.9 GB Memory | Too much memory |
‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾
| 🇳  560 | ⭕️  235_410_560 | 🕗    741.5216064 | 📌 HamCycle | 58.2 GB Memory | i32 to i16
| 🇳  561 | ⭕️  235_410_560 | 🕗    739.4226074 | 📌 HamCycle | 60.3 GB Memory
| 🇳  562 | ⭕️  237_937_312 | 🕗    751.3547363 | 📌 HamCycle | 60.3 GB Memory
| 🇳  563 | ⭕️  239_207_440 | 🕗    751.3547363 | 📌 HamCycle | 58.6 GB Memory | 71.4 Virtual
| 🇳  563 | ⭕️  239_207_440 | 🕗    807.9031982 | 📌 HamCycle |
| 🇳  564 | ⭕️  240_482_080 | 🕗    800.5877686 | 📌 HamCycle |
| 🇳  565 | ⭕️  241_761_240 | 🕗    803.6581421 | 📌 HamCycle |
| 🇳  566 | ⭕️  243_044_928 | 🕗    814.1091919 | 📌 HamCycle |
| 🇳  567 | ⭕️  244_333_152 | 🕗    862.6309204 | 📌 HamCycle |
| 🇳  568 | ⭕️  245_625_920 | 🕗    827.5814819 | 📌 HamCycle | 63.7 GB
| 🇳  569 | ⭕️  246_923_240 | 🕗    795.9773560 | 📌 HamCycle | 
__________________________________________________________________________________________________
| 🇳  570 | ⭕️  248_225_120 | 🕗    **** killed | 📌 ???????? | 56.2 GB Memory | 73.9 Virtual    |
‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾
| 🇳  570 | ⭕️  248_225_120 | 🕗    830.3286133 | 📌 HamCycle | 60.2 GB Memory | 73.2 Virtual
__________________________________________________________________________________________________
| 🇳  571 | ⭕️  249_531_568 | 🕗    **** killed | 📌 ???????? | 60.7 GB Memory | 69.8 Virtual    |
‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾
| 🇳  571 | ⭕️  249_531_568 | 🕗    904.9118652 | 📌 HamCycle | 61.7 GB / 72.1 GB Memory/Virtual    
__________________________________________________________________________________________________
| 🇳  572 | ⭕️  250_842_592 | 🕗    **** killed | 📌 ???????? | 64.1 GB Memory | 72.8 Virtual    |
‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾
| 🇳  572 | ⭕️  250_842_592 | 🕗    842.4531250 | 📌 HamCycle | 60.5 GB Memory | 73.2 Virtual 
| 🇳  573 | ⭕️  252_158_200 | 🕗    832.7617188 | 📌 HamCycle | 61.5 GB Memory | 85.4 Virtual 
| 🇳  574 | ⭕️  253_478_400 | 🕗    923.9602661 | 📌 HamCycle | 
| 🇳  575 | ⭕️  254_803_200 | 🕗    766.1255493 | 📌 HamCycle |
| 🇳  576 | ⭕️  256_132_608 | 🕗    774.5940552 | 📌 HamCycle |
| 🇳  577 | ⭕️  257_466_632 | 🕗    791.0694580 | 📌 HamCycle |
| 🇳  578 | ⭕️  258_805_280 | 🕗    807.0501709 | 📌 HamCycle | 60.4 GB Memory | 70.2 Virtual
| 🇳  579 | ⭕️  260_148_560 | 🕗    824.6404419 | 📌 HamCycle | I don't have to check the validity of eadjs at all only if it intersects with edges.
| 🇳  580 | 61.8 111.1 GB Virtual
UPDATE 1: TO BE CONTINUALLY OPTIMIZED TO SOLVE UP TO: | 🇳  1000 | ⭕️  1_337_336_000 BILLION VERTICES
UPDATE 2: ACTUALLY NOT, THE AMOUNT OF MEMORY LISTED BELOW TO ACCOMPLISH THE TASK IS MORE THAN THE 64 GB I HAVE 😢.
UPDATE 3: JOIN VERTICES + VI INTO A SINGLE STRUCTURE: AN INDEXABLE HASHMAP. LIKE AN ORDERED DICT IN PYTHON.
UPDATE 4: TRIED #3 WHICH RESULTED IN 3X SLOWER RESULTS: SEE BELOW:
| 🇳    1 | ⭕️          8 | 🕗      0.0000084 | 📌 HamCycle |
| 🇳    2 | ⭕️         32 | 🕗      0.0000092 | 📌 HamCycle |
| 🇳    3 | ⭕️         80 | 🕗      0.0000454 | 📌 HamCycle |
| 🇳    4 | ⭕️        160 | 🕗      0.0000397 | 📌 HamCycle |
| 🇳    5 | ⭕️        280 | 🕗      0.0000515 | 📌 HamCycle |
| 🇳    6 | ⭕️        448 | 🕗      0.0000699 | 📌 HamCycle |
| 🇳    7 | ⭕️        672 | 🕗      0.0000965 | 📌 HamCycle |
| 🇳    8 | ⭕️        960 | 🕗      0.0001237 | 📌 HamCycle |
| 🇳    9 | ⭕️       1320 | 🕗      0.0001662 | 📌 HamCycle |
| 🇳   10 | ⭕️       1760 | 🕗      0.0003106 | 📌 HamCycle |
| 🇳   11 | ⭕️       2288 | 🕗      0.0003596 | 📌 HamCycle |
| 🇳   12 | ⭕️       2912 | 🕗      0.0006962 | 📌 HamCycle |
| 🇳   13 | ⭕️       3640 | 🕗      0.0009002 | 📌 HamCycle |
| 🇳   14 | ⭕️       4480 | 🕗      0.0008008 | 📌 HamCycle |
| 🇳   15 | ⭕️       5440 | 🕗      0.0006189 | 📌 HamCycle |
| 🇳   16 | ⭕️       6528 | 🕗      0.0007720 | 📌 HamCycle |
| 🇳   17 | ⭕️       7752 | 🕗      0.0009733 | 📌 HamCycle |
| 🇳   18 | ⭕️       9120 | 🕗      0.0011333 | 📌 HamCycle |
| 🇳   19 | ⭕️      10640 | 🕗      0.0019485 | 📌 HamCycle |
| 🇳   20 | ⭕️      12320 | 🕗      0.0018215 | 📌 HamCycle |
| 🇳   21 | ⭕️      14168 | 🕗      0.0021890 | 📌 HamCycle |
| 🇳   22 | ⭕️      16192 | 🕗      0.0043887 | 📌 HamCycle |
| 🇳   23 | ⭕️      18400 | 🕗      0.0024746 | 📌 HamCycle |
| 🇳   24 | ⭕️      20800 | 🕗      0.0031840 | 📌 HamCycle |
| 🇳   25 | ⭕️      23400 | 🕗      0.0046989 | 📌 HamCycle |
| 🇳   26 | ⭕️      26208 | 🕗      0.0057269 | 📌 HamCycle |
| 🇳   27 | ⭕️      29232 | 🕗      0.0052942 | 📌 HamCycle |
| 🇳   28 | ⭕️      32480 | 🕗      0.0085430 | 📌 HamCycle |
| 🇳   29 | ⭕️      35960 | 🕗      0.0086081 | 📌 HamCycle |
| 🇳   30 | ⭕️      39680 | 🕗      0.0086503 | 📌 HamCycle |
| 🇳   31 | ⭕️      43648 | 🕗      0.0110642 | 📌 HamCycle |
| 🇳   32 | ⭕️      47872 | 🕗      0.0107873 | 📌 HamCycle |
| 🇳   33 | ⭕️      52360 | 🕗      0.0130128 | 📌 HamCycle |
| 🇳   34 | ⭕️      57120 | 🕗      0.0132657 | 📌 HamCycle |
| 🇳   35 | ⭕️      62160 | 🕗      0.0148311 | 📌 HamCycle |
| 🇳   36 | ⭕️      67488 | 🕗      0.0157533 | 📌 HamCycle |
| 🇳   37 | ⭕️      73112 | 🕗      0.0174286 | 📌 HamCycle |
| 🇳   38 | ⭕️      79040 | 🕗      0.0181613 | 📌 HamCycle |
| 🇳   39 | ⭕️      85280 | 🕗      0.0209774 | 📌 HamCycle |
| 🇳   40 | ⭕️      91840 | 🕗      0.0221653 | 📌 HamCycle |
| 🇳   41 | ⭕️      98728 | 🕗      0.0244488 | 📌 HamCycle |
| 🇳   42 | ⭕️     105952 | 🕗      0.0252573 | 📌 HamCycle |
| 🇳   43 | ⭕️     113520 | 🕗      0.0287851 | 📌 HamCycle |
| 🇳   44 | ⭕️     121440 | 🕗      0.0327196 | 📌 HamCycle |
| 🇳   45 | ⭕️     129720 | 🕗      0.0357147 | 📌 HamCycle |
| 🇳   46 | ⭕️     138368 | 🕗      0.0387448 | 📌 HamCycle |
| 🇳   47 | ⭕️     147392 | 🕗      0.0432163 | 📌 HamCycle |
| 🇳   48 | ⭕️     156800 | 🕗      0.0471831 | 📌 HamCycle |
| 🇳   49 | ⭕️     166600 | 🕗      0.0468241 | 📌 HamCycle |
| 🇳   50 | ⭕️     176800 | 🕗      0.0520043 | 📌 HamCycle |
| 🇳   51 | ⭕️     187408 | 🕗      0.0577361 | 📌 HamCycle |
| 🇳   52 | ⭕️     198432 | 🕗      0.0727845 | 📌 HamCycle |
| 🇳   53 | ⭕️     209880 | 🕗      0.0671161 | 📌 HamCycle |
| 🇳   54 | ⭕️     221760 | 🕗      0.0813031 | 📌 HamCycle |
| 🇳   55 | ⭕️     234080 | 🕗      0.0862697 | 📌 HamCycle |
| 🇳   56 | ⭕️     246848 | 🕗      0.0977101 | 📌 HamCycle |
| 🇳   57 | ⭕️     260072 | 🕗      0.1017856 | 📌 HamCycle |
| 🇳   58 | ⭕️     273760 | 🕗      0.1056793 | 📌 HamCycle |
| 🇳   59 | ⭕️     287920 | 🕗      0.1051013 | 📌 HamCycle |
| 🇳   60 | ⭕️     302560 | 🕗      0.1356453 | 📌 HamCycle |
| 🇳   61 | ⭕️     317688 | 🕗      0.1464440 | 📌 HamCycle |
| 🇳   62 | ⭕️     333312 | 🕗      0.1422239 | 📌 HamCycle |
| 🇳   63 | ⭕️     349440 | 🕗      0.1766653 | 📌 HamCycle |
| 🇳   64 | ⭕️     366080 | 🕗      0.1739050 | 📌 HamCycle |
| 🇳   65 | ⭕️     383240 | 🕗      0.1783360 | 📌 HamCycle |
| 🇳   66 | ⭕️     400928 | 🕗      0.2064866 | 📌 HamCycle |
| 🇳   67 | ⭕️     419152 | 🕗      0.2505056 | 📌 HamCycle |
| 🇳   68 | ⭕️     437920 | 🕗      0.2384429 | 📌 HamCycle |
| 🇳   69 | ⭕️     457240 | 🕗      0.2432004 | 📌 HamCycle |
| 🇳   70 | ⭕️     477120 | 🕗      0.3078538 | 📌 HamCycle |
| 🇳   71 | ⭕️     497568 | 🕗      0.2706421 | 📌 HamCycle |
| 🇳   72 | ⭕️     518592 | 🕗      0.3341260 | 📌 HamCycle |
| 🇳   73 | ⭕️     540200 | 🕗      0.3754710 | 📌 HamCycle |
| 🇳   74 | ⭕️     562400 | 🕗      0.3784696 | 📌 HamCycle |
| 🇳   75 | ⭕️     585200 | 🕗      0.3891708 | 📌 HamCycle |
| 🇳   76 | ⭕️     608608 | 🕗      0.4071339 | 📌 HamCycle |
| 🇳   77 | ⭕️     632632 | 🕗      0.4418812 | 📌 HamCycle |
| 🇳   78 | ⭕️     657280 | 🕗      0.4390116 | 📌 HamCycle |
| 🇳   79 | ⭕️     682560 | 🕗      0.4957499 | 📌 HamCycle |
| 🇳   80 | ⭕️     708480 | 🕗      0.5129622 | 📌 HamCycle |
| 🇳   81 | ⭕️     735048 | 🕗      0.5297590 | 📌 HamCycle |
| 🇳   82 | ⭕️     762272 | 🕗      0.5312024 | 📌 HamCycle |
| 🇳   83 | ⭕️     790160 | 🕗      0.6232494 | 📌 HamCycle |
| 🇳   84 | ⭕️     818720 | 🕗      0.6278361 | 📌 HamCycle |
| 🇳   85 | ⭕️     847960 | 🕗      0.6911164 | 📌 HamCycle |
| 🇳   86 | ⭕️     877888 | 🕗      0.6974847 | 📌 HamCycle |
| 🇳   87 | ⭕️     908512 | 🕗      0.6883463 | 📌 HamCycle |
| 🇳   88 | ⭕️     939840 | 🕗      0.9471568 | 📌 HamCycle |
| 🇳   89 | ⭕️     971880 | 🕗      0.8314820 | 📌 HamCycle |
| 🇳   90 | ⭕️    1004640 | 🕗      0.8707603 | 📌 HamCycle |
| 🇳   91 | ⭕️    1038128 | 🕗      0.9356786 | 📌 HamCycle |
| 🇳   92 | ⭕️    1072352 | 🕗      1.1933094 | 📌 HamCycle |
| 🇳   93 | ⭕️    1107320 | 🕗      1.1849506 | 📌 HamCycle |
| 🇳   94 | ⭕️    1143040 | 🕗      1.1610720 | 📌 HamCycle |
| 🇳   95 | ⭕️    1179520 | 🕗      1.1411682 | 📌 HamCycle |
| 🇳   96 | ⭕️    1216768 | 🕗      1.2048258 | 📌 HamCycle |
| 🇳   97 | ⭕️    1254792 | 🕗      1.2460364 | 📌 HamCycle |
| 🇳   98 | ⭕️    1293600 | 🕗      1.3229424 | 📌 HamCycle |
| 🇳   99 | ⭕️    1333200 | 🕗      1.3375677 | 📌 HamCycle |
| 🇳  100 | ⭕️    1373600 | 🕗      1.3939092 | 📌 HamCycle |

```


#### EACH GRAPH STRUCTURE'S SIZE REQUIREMENTS:


| ORDER  |   VERTS    |   TOUR   |    VI_MAP |   ADJ     |   TOTAL     |
| ------:| ----------:| --------:|----------:| ---------:| -----------:|
| 200M   |    7.6 GB  |  0.8 GB  |    8.4 GB |   2.4 GB  |   18.2 GB   |
| 250M   |   11.7 GB  |  1.3 GB  |   14.0 GB |   3.8 GB  |   31.8 GB   |
| 300M   |   16.7 GB  |  1.8 GB  |   21.0 GB |   5.9 GB  |   45.4 GB   |
| 350M   |   22.6 GB  |  2.3 GB  |   30.0 GB |   8.4 GB  |   63.3 GB   |
| 400M   |   29.4 GB  |  3.1 GB  |   42.0 GB |  12.0 GB  |   86.5 GB   |
| 450M   |   37.2 GB  |  3.8 GB  |   58.0 GB |  16.0 GB  |  115.0 GB   |
| 500M   |   46.0 GB  |  4.6 GB  |   80.0 GB |  22.0 GB  |  153.6 GB   |
| 550M   |   55.8 GB  |  5.5 GB  |  108.0 GB |  29.0 GB  |  198.3 GB   |
| 600M   |   66.6 GB  |  6.6 GB  |  144.0 GB |  37.0 GB  |  254.6 GB   |
| 650M   |   78.4 GB  |  7.8 GB  |  188.0 GB |  46.0 GB  |  320.2 GB   |
| 700M   |   91.2 GB  |  9.1 GB  |  240.0 GB |  56.0 GB  |  417.3 GB   |
| 750M   |  105.0 GB  | 10.5 GB  |  302.0 GB |  67.0 GB  |  484.5 GB   |
| 800M   |  119.8 GB  | 12.0 GB  |  376.0 GB |  79.0 GB  |  586.8 GB   |
| 850M   |  135.6 GB  | 13.6 GB  |  462.0 GB |  92.0 GB  |  703.2 GB   |
| 900M   |  152.4 GB  | 15.2 GB  |  562.0 GB | 106.0 GB  |  836.6 GB   |
| 950M   |  170.2 GB  | 17.0 GB  |  678.0 GB | 121.0 GB  |  1.005 TB   |
| 1B     |  189.0 GB  | 18.9 GB  |  810.0 GB | 137.0 GB  |  1.154 TB   |


## Licensing:

This package is licensed under the MIT license.

Thanks for making it this far!
