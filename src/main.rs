// * Pseudocode Preface

// We need to figure out where a 3D point should appear on a flat screen, making it look like part of a 3D scene.

// 1. Factor Calculation
// We calculate a factor to adjust the size of the point based on how far away it is (viewer_distance) 
// and how wide our view is (fov). This makes distant points look smaller.

// 2. Adjust X Coordinate:
// We adjust the x position of the point using this factor and move it to the center of the screen (width / 2.0).

// 3. Adjust Y Coordinate:
// We adjust the y position similarly but flip it upside down (-point.y) because screen coordinates go downwards. 
// Then, we move it to the center vertically (height / 2.0).

// 4. Return Coordinates:
// Finally, we return the adjusted x and y coordinates as integers (since screen positions are whole numbers).

// SO TO PUT THIS INTO CODING TERMS
// func project_3D_to_2D {point, width, height, fov, viewer_distance} {
//     const factor = fov / (viewer_distance + point.z)
//     const x = point.x * factor + width / 2.0
//     const y = -point.y * factor + height / 2.0
//     return {x as i32, y as i32}
// }

// Below are matrices used to perform a rotation in euclidean space
// xMatrix(θ) =  [[1,0,0],[0,cos(θ),sin(θ)],[0, -sin(θ), cos(θ)]]
// yMatrix(θ) =  [[cos(θ), 0,sin(θ)],[0,1,0],[-sin(θ), 0, cos(θ)]]
// zMatrix(θ) =  [[cos(θ), -sin(θ), 0],[sin(θ), cos(θ),0],[0,0,1]]

// When we multiply the matrix by [x,y,z] (the point in space). We get the following three, one dimensional  matricies:
// [[1,0,0],[0,cos(θ),sin(θ)],[0, -sin(θ), cos(θ)]].[x,y,z]  = [x               , ycos(θ)+zsin(θ), -ysin(θ)+zcos(θ)] 
// [[cos(θ), 0,sin(θ)],[0,1,0],[0, -sin(θ), cos(θ)]].[x,y,z] = [xcos(θ)+zsin(θ) , y              , -xsin(θ)+zcos(θ)]
// [[cos(θ), -sin(θ), 0],[sin(θ), cos(θ),0],[0,0,1]].[x,y,z] = [xcos(θ)+-ysin(θ), xsin(θ)+ycos(θ), z               ]

// In coding terms:

//  roll = rotation angle about X
//  func rotatePointAboutX {x,y,z, roll}(
//     const rotatedY number const = y*cos(roll)+z*sin(roll)
//     const rotatedZ number const = -y*sin(roll)+z*cos(roll)]
//     return {x, rotatedY, rotatedZ}
//  )

//  pitch = rotation angle about Y
//  func rotatePointAboutY {x,y,z, pitch}( 
//     const rotatedX number const = x*cos(pitch)+z*sin(pitch)
//     const rotatedZ number const = -y*sin(pitch)+z*cos(pitch)]
//     return {rotatedX, y, rotatedZ}
//  )

//  yaw = rotation angle about Z
//  func rotatePointAboutZ {x,y,z, yaw}( 
//     const rotatedY number const = x*cos(yaw)-y*sin(yaw)
//     const rotatedZ number const = x*sin(yaw)+y*cos(yaw)]
//     return {rotatedX, rotatedY, z}
//  )


// * Rust Time
// First we do our imports


// Crossterm library to manage input and output into the console. I.e. to allow user to zoom in/out
use crossterm::event::{self, Event, KeyCode};
// Importing the standard library's floating point mathematical constants
use std::f64::consts::PI;
// The sleep function the std::thread module is used to pause the current thread for a specified amount of time. i.e. for animations.
use std::thread::sleep;
//The Duration struct from the std::time module represents a span of time. Used to use Sleep.^
use std::time::Duration;
//io::{self, Write} is used to clear the terminal screen and ensure the output is flushed correctly, so each frame of the animation appears in sequence without delay.
use std::io::{self, Write};

// Define a 3D point
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

// Rotate a point around the X axis
fn rotate_about_x(point: &Point3D, roll: f64) -> Point3D {
    let (sin, cos) = roll.sin_cos();
    Point3D {
        x: point.x,
        y: point.y * cos - point.z * sin,
        z: point.y * sin + point.z * cos,
    }
}

// Rotate a point around the Y axis
fn rotate_about_y(point: &Point3D, pitch: f64) -> Point3D {
    let (sin, cos) = pitch.sin_cos();
    Point3D {
        x: point.x * cos + point.z * sin,
        y: point.y,
        z: -point.x * sin + point.z * cos,
    }
}

// Rotate a point around the Z axis
fn rotate_about_z(point: &Point3D, yaw: f64) -> Point3D {
    let (sin, cos) = yaw.sin_cos();
    Point3D {
        x: point.x * cos - point.y * sin,
        y: point.x * sin + point.y * cos,
        z: point.z,
    }
}

// Project a 3D point to 2D
fn project(point: &Point3D, width: f64, height: f64, fov: f64, viewer_distance: f64) -> (i32, i32) {
    let factor = fov / (viewer_distance + point.z);
    let x = point.x * factor + width / 2.0;
    let y = -point.y * factor + height / 2.0;
    (x as i32, y as i32)
}

// Clear the terminal
fn clear() {
    //Rust junk that clears the terminal/resets the cursor
    print!("\x1B[2J\x1B[1;1H");
//     The stdout gets a function for the terminal, and its flush method forces any buffered data to be written to the terminal immediately. (Unwrap causes it to crash if it fails.)
    io::stdout().flush().unwrap();
}

fn main() {
    let width = 80.0;
    let height = 40.0;

    //Mutable so they get fucked up by the io stuff I've put in further down for zooming in/out
    let mut fov = 100.0;
    let mut viewer_distance = 6.0;

    let mut angle = PI/4.0;

    let vertices: [Point3D; 8] = [
        Point3D { x: -1.0, y: -1.0, z: -1.0 },
        Point3D { x:  1.0, y: -1.0, z: -1.0 },
        Point3D { x:  1.0, y:  1.0, z: -1.0 },
        Point3D { x: -1.0, y:  1.0, z: -1.0 },
        Point3D { x: -1.0, y: -1.0, z:  1.0 },
        Point3D { x:  1.0, y: -1.0, z:  1.0 },
        Point3D { x:  1.0, y:  1.0, z:  1.0 },
        Point3D { x: -1.0, y:  1.0, z:  1.0 },
    ];

    let edges: [(usize, usize); 12] = [
        (0, 1), (1, 2), (2, 3), (3, 0), // back face
        (4, 5), (5, 6), (6, 7), (7, 4), // front face
        (0, 4), (1, 5), (2, 6), (3, 7), // connecting edges
    ];

    loop {
        //Creates empty vector
        let mut projected_points:Vec<(i32, i32)>  = vec![];

        for vertex in &vertices {
            let rotated_x = rotate_about_x(vertex, angle);
            let rotated_xy = rotate_about_y(&rotated_x, angle);
            let rotated_xyz = rotate_about_z(&rotated_xy, angle);
            let projected = project(&rotated_xyz, width, height, fov, viewer_distance);
            projected_points.push(projected);
        }

        clear();

        //Creates a x by y vector that is width space character across
        let mut screen: Vec<Vec<char>> = vec![vec![' '; width as usize]; height as usize];


        for &(start, end) in &edges {
            
            let (x_start, y_start) = projected_points[start];
            let (x_end, y_end) = projected_points[end];
            //dx and dy are the differences in x and y coordinates between the start and end points.
            let x_length: i32 = x_end - x_start;
            let y_length: i32 = y_end - y_start;

            //Number of #s to draw the line
            let steps: i32 = x_length.abs().max(y_length.abs());
            //How much to move the # x and y with each step
            let x_movement: f64 = x_length as f64 / steps as f64;
            let y_movement: f64 = y_length as f64 / steps as f64;

            //Define x and y that are mapped through on each step
            let mut x: f64 = x_start as f64;
            let mut y: f64 = y_start as f64;

            for _ in 0..=steps {
                //If movement bigger
                if x >= 0.0 && x < width && y >= 0.0 && y < height {
                    screen[y as usize][x as usize] = '#';
                }
                x += x_movement;
                y += y_movement;
            }
        }

        for row in screen {
            println!("{}", row.iter().collect::<String>());
        }

        angle += 0.01;
        //Duration between each rotation
        sleep(Duration::from_millis(20));
        // Event poll(timeout) checks for an event, in this case a key input every 1 seconds
        if event::poll(Duration::from_millis(1)).unwrap() {
            // 
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Char('=') => {
                        fov += 10.0;
                        viewer_distance -= 1.0;
                    }
                    KeyCode::Char('-') => {
                        fov -= 10.0;
                        viewer_distance += 1.0;
                    }
                    KeyCode::Char('q') => break,
                    _ => (),
                }
            }
        }
    }
}