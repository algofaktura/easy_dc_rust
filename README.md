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

I hope to tell the story of the discocube, introduce an undefined graph class *Cubic Accretion Graphs*, some of its properties, and the share insights I've gained by solving this problem having taken an approach similar to that of sculpting the human body...After thousands of studies, drawings, a little math: this is a tiny glimpse into how moving towards a specific aethetic goal yields results. When a graph becomes an artist's muse, how does the artist go about rendering their vision as a painter paints a portrait?

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


## Running times
![Running times from 8 to 68,085,920 vertices](imgs/8_to_68085920.png?raw=true "Runtimes up to 68 million")
8_to_68085920.png

### PYTHON VS. RUST:

### solve python profile 5,061,680 vertices:
![Profile of solve_np](imgs/profile_solve_np5.png?raw=true "Profile of solve_np")
### solve rust speed 5,061,680 vertices:
![Profile of solve_np](imgs/rust_speed_graph_5061680_verts.png?raw=true "Profile of solve_np")


### digital discocubes
As each solution is as unique as a fingerprint, or a diamond it allows one to have their own digital version of a discocube, which is also an instruction for building your own.

![Discocube 3640 view](imgs/icy_cube.png?raw=true "icy cube") 
![Discocube 3640 view](imgs/icy_cube5.png?raw=true "icy cube")
![Discocube 3640 view](imgs/icy_cube4.png?raw=true "icy cube")
![Discocube 3640 view](imgs/icy_cube3.png?raw=true "icy cube another view")
![Discocube 3640 view](imgs/icy_cube2.png?raw=true "icy cube another view")
![Always Turning Discocube 9120 view](imgs/always_turning_9120.png?raw=true "Always Turning Discocube 9120 view")
*Discocubes as glb, using different mirrored texture yields personalized results and unique reflections meaning each discocube has its own reflection/shadow fingerprint! With millions of combinations available (glass texture/image/color, mirror texture/image/color, edge texture/image/color), the possibilities are endless!*


### coming soon...
![Discocube 3640 view](imgs/comingsoom.png?raw=true "Drawing of the algorithmic process") 
*I'm currently in the process of illustrating the process of the algorithm. But that's its own process.*

### ps...
Please note: the hamiltonian cycle produced by this algorithm is the base form, without a high mutation rate. The polished versions available have no nonturns and all their edges are distributed evenly across the three axes.
The other algorithms I spoke of earlier accomplish this task.

## Command line usage
To use the package via the command line, navigate to the root directory of the project in your terminal and run the following command:
```
cargo run --release [N] [N_UPPER INCLUSIVE] [REPEATS]
```
Where [N] is extrusion level for which the graph is made and solved. Note n=100 is already a graph with 1,373,600 vertices.
```
cargo run --release 1 100 10
```
Means order 32 to 1,373,600 where each order is run 10 times.

## Licensing:

This package is licensed under the MIT license.




Happy reading!
