use super::structs::vectors::{Vector2, Vector3, Vector4};

pub fn world_to_screen(position: Vector3, screen: &mut Vector2, view_matrix: [f32; 16], window_width: i32, window_height: i32) -> bool {

  let clip_coords = Vector4 {
    x: position.x * view_matrix[0]
      + position.y * view_matrix[4]
      + position.z * view_matrix[8]
      + view_matrix[12],
    y: position.x * view_matrix[1]
      + position.y * view_matrix[5]
      + position.z * view_matrix[9]
      + view_matrix[13],
    z: position.x * view_matrix[2]
      + position.y * view_matrix[6]
      + position.z * view_matrix[10]
      + view_matrix[14],
    w: position.x * view_matrix[3]
      + position.y * view_matrix[7]
      + position.z * view_matrix[11]
      + view_matrix[15],
  };

  if clip_coords.w < 0.1 {
    return false;
  }

  let normalized_device_coordinates = Vector3 {
    x: clip_coords.x / clip_coords.w,
    y: clip_coords.y / clip_coords.w,
    z: clip_coords.z / clip_coords.w,
  };

  screen.x = ((window_width / 2) as f32 * normalized_device_coordinates.x)
    + (normalized_device_coordinates.x + (window_width / 2) as f32);
  screen.y = -((window_height / 2) as f32 * normalized_device_coordinates.y)
    + (normalized_device_coordinates.y + (window_height / 2) as f32);

  true
}