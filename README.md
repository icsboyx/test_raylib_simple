# Rust Simple Game Raylib

This is a simple game built in Rust using the Raylib game development library. The game involves an entity (red circle) that moves around a canvas and interacts with a target (blue circle).

## Code Overview

### Entity Struct

The `Entity` struct represents a circle with properties like position (x, y), radius, color, and the size of the canvas it's on. It has methods to get and set its position, and to draw itself on the canvas.

```rust
struct Entity {
    x: f32,
    y: f32,
    radius: i32,
    color: Color,
    canvas: (i32, i32),
}
```

### Entity Methods

The `Entity` struct has several methods:

- `new`: This method is used to create a new `Entity` instance.
- `get_x` & `get_y`: These methods are used to get the x and y coordinates of the entity.
- `set_x` & `set_y`: These methods are used to set the x and y coordinates of the entity. They also handle the logic for wrapping the entity around the canvas.
- `draw`: This method is used to draw the entity on the canvas.

### Main Function

The `main` function initializes the game, creates the entity and target, and enters the main game loop. In each iteration of the loop, it checks if any arrow keys are pressed and moves the entity accordingly. It also checks if the entity has collided with the target. If a collision is detected, the target is moved to a new random position.

### ArrowKey Enum

The `ArrowKey` enum and its associated methods are used to handle the arrow key inputs from the user.
With a transcode to raylib keys.

```rust
enum ArrowKey {
    Up,
    Down,
    Left,
    Right,
}
```

## Running the Game

To run the game, you need to have Rust installed on your machine. You can then use the `cargo run` command in the terminal from the root directory of the project.

## Contributing

Contributions are welcome. Please feel free to submit a pull request or open an issue.

## Special Thanks

This project was inspired by the Twitch channel of:\
![Prof. Andrea Pollini](https://static-cdn.jtvnw.net/jtv_user_pictures/b4199595-d595-4788-9f04-f4aa370e902a-profile_image-70x70.png)[Prof. Andrea Pollini](https://www.twitch.tv/profandreapollini)\
and  the supportive Twitch community. Thanks to their encouragement and feedback!



## License

This project is licensed under the MIT License - see the [LICENSE](https://www.mit.edu/~amini/LICENSE.md) for details.