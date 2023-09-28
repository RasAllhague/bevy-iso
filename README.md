# bevy-iso

A plugin for the bevy game engine for handling isometric tilemaps and games. 
It does not optimize any rendering and is mearly handling placement of tiles and objects in the isometric space.
 
## Features:

 - [ ] Simple isometric tilespawning and tilemaps
 - [ ] Spawn tilemaps from files
 - [ ] Isometric camera rotation
 - [ ] Tile interactions
 - [ ] Multilayer tilemaps
 - [ ] Object movement in iso-space
 - [ ] Custom Tilemap editor
 - [ ] Animated tiles

### Simple isometric tilespawning

Create tiles directly via spawning them at the appropriate grid location. Or spawn entire tilemaps at once.

### Spawn tilemaps from files

Spawn tilemaps based on RON files. Each file can have multiple layers. 

### Isometric camera rotation

Enables an camera rotation in an isometric "way". 
In reality it just reorders and repositions the tiles to fit the new view.
You can choose between instant rotation, or small animations. You can also implement your own animations for rotation transitions.

### Tile interactions

Add logic for easy interactions with single tiles. Possible interactions that can be used are hovered and clicked. 
Also enables you to use some build in animations like color changing, sprite image changing or height changing of individual tiles.

### Multilayer tilemaps

Adds the ability to have height in tilemaps. Each layer will render on top of the next one.

### Object movement in iso-space

Includes build in features to easy or even fully take over control of movement for objects in the iso space.

### Animated tiles

Animated tiles.

### Custom Tilemap editor

A custom tool for creating tilemaps which then can later be loaded as assets of this plugin.