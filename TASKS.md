# Tasks

# Sling Shot Mechanic

Right now the bird always shoots with the same diections and velocity. In this task we want the player to customize the `LinearVelocity` (`Vec2`) of the bird on spawn.

Acceptance Criteria:
- The bird should be able to be launched with a custom velocity.
- The velocity should be customizable by the player.
- The velocity should be applied to the bird when it is launched.

# Load level from file

Right now the level is hardcoded. We want to load the level from a file.
For this ideally we define a data structure that is serde compatible to be loaded from an asset file.
Step two is then to use `bevy_custom_asset` to load this from a `ron` file.

# Preview Trajectory Path

Right now its hard for the player to estimate where the bird will be flying. Using the sling Resource we can simulate the trajectory path of the bird. We can then sample a few points on its path and draw them using dots.

Acceptance Criteria:
- Simulate physical flight path of bird either by hand or using `bevy_ballistic`
- Visualize Flight Path using dots (either via a custom sprite or using Gizmos)
- Only show while the player is holding the sling.

# Different Bird Types

For now we only support one simple bird type. Lets support also the one that allows the player to tap another time while its flying to make it spawn 4 other smaller birds instead with a spread out trajectory.

Acceptance Criteria:
- Implement two buttons to switch between bird types.
- The new bird type acts like a cluster bird that disappears and spawns 4 smaller birds instead.
- Triggering this requires the user to tap again while this bird is flying.
- Ensure the smaller birds have a spread out trajectory.

# Camera auto scaling

On various window sizes the game will not scale properly. This is mostly a problem in the web build.
Ideally we scale the camera out just as much as needed to keep all entites in view.
We can iterate all colliders after the level is spawned and calculate the levels extens. then we place the camera in the middle of it and use simple math to calculate a zoom level that will keep all entites in view.

Acceptance Criteria:
- The camera is scaled automatically to fit all entities in view.
- The camera will not keep updating as entites move around after the loading phase.

# Integrate EGUI

We want to support opening [bevy_egui_inspector](https://github.com/jakobhellermann/bevy-inspector-egui/) using a magic key to facilitate debugging our scene hierarchy of entities, components and be able to manipulate them on the fly.

Acceptance Criteria:
- The EGUI inspector is integrated into the game.
- It only opens on a special key press.
- It is only enabled and compiled into the project when a `dbg` cargo feature is enabled. (stretch goal)

# Ground Collider Visuals

Right now we only see the Ground visually because the Avian Physics Debug Rendering is enabled. Ideally we show a ground image either stretched or tiled to cover the entire surface of the Collider.

Acceptance Criteria:
- The ground collider is visually represented by a sprite.

# Limit Shots

The player currently has unlimited shots. This change will make it limited and trigger a gameover if all birds are used and not all piggies destroyed.

Acceptance Criteria:
- The player has a limited number of shots.
- The game triggers a gameover if all birds are used and not all piggies destroyed.
- Store available amount of birds in a resource so it can be shown in the UI (followup)

# Gameover Screen

Right now the game just switches to the Menu on Gameover. Lets Show an overlay UI if this happens for the user to confirm.

Acceptance Criteria:
- The game shows a Gameover screen with a button to go back to menu.
- This screen is triggered when all piggies died (or if we added other lose criteria yet, see above)

# Sound Effects

Lets play some sound effects if the player shoots the bird, hits a piggy or for other effects. Lets build a flexible `Event` we can trigger from anywhere to start playing a sound.

Acceptance Criteria:
- Convenient way to trigger a sound effect anywhere in the codebase.
- Use bevy_audio to play the different sound effects.

# Explosion Effect

It would be nice if we could spawn a Sprite animation effect when a piggy is hit.

Acceptance Criteria:
- Find a sprite sheet animation (e.g. on `open game art`)
- Build an explosion effect plugin that accepts an `Event` with a position `Vec2`
- The plugin then spawns the animation at that position and iterates over the different sprite sheet / texture atlas indices over time till done
- When the animation is done we despawn the enitity to cleanup

# Smaller goals

* More level elements
* A bomb in the level that when explodes adds an impuls to all colliders in a radius
* CI pipeline to prevent breakage
