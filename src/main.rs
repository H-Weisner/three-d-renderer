//Below are matrices used to perform a rotation in euclidean space
// xMatrix(θ) =  [[1,0,0],[0,cos(θ),sin(θ)],[0, -sin(θ), cos(θ)]]
// yMatrix(θ) =  [[cos(θ), 0,sin(θ)],[0,1,0],[-sin(θ), 0, cos(θ)]]
// zMatrix(θ) =  [[cos(θ), -sin(θ), 0],[sin(θ), cos(θ),0],[0,0,1]]

//When we multiply the matrix by [x,y,z] (the point in space). We get the following three, one dimensional  matricies:
// [[1,0,0],[0,cos(θ),sin(θ)],[0, -sin(θ), cos(θ)]].[x,y,z]  = [x               , ycos(θ)+zsin(θ), -ysin(θ)+zcos(θ)] 
// [[cos(θ), 0,sin(θ)],[0,1,0],[0, -sin(θ), cos(θ)]].[x,y,z] = [xcos(θ)+zsin(θ) , y              , -xsin(θ)+zcos(θ)]
// [[cos(θ), -sin(θ), 0],[sin(θ), cos(θ),0],[0,0,1]].[x,y,z] = [xcos(θ)+-ysin(θ), xsin(θ)+ycos(θ), z               ]

// So to put this in coding terms
//  yaw = rotation angle about Z
//  func rotatePointAboutZ {x,y,z, yaw}( 
//     const rotatedY number const = x*cos(yaw)-y*sin(yaw)
//     const rotatedZ number const = x*sin(yaw)+y*cos(yaw)]
//     return {rotatedX, rotatedY, z}
//  )

//  pitch = rotation angle about Y
//  func rotatePointAboutY {x,y,z, pitch}( 
//     const rotatedX number const = x*cos(pitch)+z*sin(pitch)
//     const rotatedZ number const = -y*sin(pitch)+z*cos(pitch)]
//     return {rotatedX, y, rotatedZ}
//  )

//  roll = rotation angle about X
//  func rotatePointAboutX {x,y,z, roll}(
//     const rotatedY number const = y*cos(roll)+z*sin(roll)
//     const rotatedZ number const = -y*sin(roll)+z*cos(roll)]
//     return {x, rotatedY, rotatedZ}
//  )


// And in RUST 

// Importing the standard library's floating point mathematical constants
use std::f64::consts::PI;

// Define a function to rotate a point around the Z axis
fn rotate_point_about_z(x: f64, y: f64, z: f64, yaw: f64) -> (f64, f64, f64) {
    // Calculate the new x-coordinate after rotation using trigonometric functions
    let rotated_x = x * yaw.cos() - y * yaw.sin();
    // Calculate the new y-coordinate after rotation
    let rotated_y = x * yaw.sin() + y * yaw.cos();
    // Return the new coordinates as a tuple
    (rotated_x, rotated_y, z)
}

// Define a function to rotate a point around the Y axis
fn rotate_point_about_y(x: f64, y: f64, z: f64, pitch: f64) -> (f64, f64, f64) {
    // Calculate the new x-coordinate after rotation
    let rotated_x = x * pitch.cos() + z * pitch.sin();
    // Calculate the new z-coordinate after rotation
    let rotated_z = -x * pitch.sin() + z * pitch.cos();
    // Return the new coordinates as a tuple
    (rotated_x, y, rotated_z)
}

// Define a function to rotate a point around the X axis
fn rotate_point_about_x(x: f64, y: f64, z: f64, roll: f64) -> (f64, f64, f64) {
    // Calculate the new y-coordinate after rotation
    let rotated_y = y * roll.cos() - z * roll.sin();
    // Calculate the new z-coordinate after rotation
    let rotated_z = y * roll.sin() + z * roll.cos();
    // Return the new coordinates as a tuple
    (x, rotated_y, rotated_z)
}

// The main function is the entry point of the program
fn main() {
    // Define initial coordinates and angles
    let (x, y, z) = (1.0, 2.0, 3.0); // Immutable by default
    let yaw = PI / 4.0;   // 45 degrees in radians
    let pitch = PI / 4.0; // 45 degrees in radians
    let roll = PI / 4.0;  // 45 degrees in radians

    // Rotate the point around the Z axis and print the result
    let rotated_about_z = rotate_point_about_z(x, y, z, yaw);
    // `{:?}` is used for debugging output of the tuple
    println!("Rotated about Z: {:?}", rotated_about_z);

    // Rotate the point around the Y axis and print the result
    let rotated_about_y = rotate_point_about_y(x, y, z, pitch);
    println!("Rotated about Y: {:?}", rotated_about_y);

    // Rotate the point around the X axis and print the result
    let rotated_about_x = rotate_point_about_x(x, y, z, roll);
    println!("Rotated about X: {:?}", rotated_about_x);
}