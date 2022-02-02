use image::io::Reader as ImageReader;
use image::{RgbaImage, Rgba};
use async_std::{task};
use std::sync::Arc;
use std::ops::Deref;
use rusttype::Point;
use std::mem;
use std::time::{Instant};

const WALL_THRESHOLD : u8 = 240;
const WALL : i32 = i32::MAX - 0;
const UNSET : i32 = i32::MAX - 1;
const DIRECTIONS : [(i32, i32); 4] = [(0,1), (1,0), (-1, 0), (0,-1)];
const SOLUTION_COLOR : Rgba<u8> = Rgba([255, 0, 0, 255]);

fn convert_to_32bit_vector( img : &RgbaImage ) -> Vec<i32>
{
    let mut v : Vec<i32> = Vec::new();
    v.resize( ( img.width() * img.height() ) as usize, 0);

    let mut i = 0;
    for p in img.pixels()
    {
        v[i] = if p[0] <= WALL_THRESHOLD && p[3] >= 255 - WALL_THRESHOLD {WALL} else {UNSET};
        i += 1;
    }

    return v;
}

fn move_point( point: &mut Point<u32>, d : (i32,i32), width: u32, height: u32 ) -> bool
{
    // would move negative
    if d.0 < 0 && -d.0 as u32 > point.x || d.1 < 0 && -d.1 as u32 > point.y 
    {
        return false;
    }

    // would move beyond width, height
    if d.0 > 0 && d.0 as u32 + point.x >= width || d.1 > 0 && d.1 as u32 + point.y >= height
    {
        return false;
    }

    point.x = (point.x as i32 + d.0) as u32;
    point.y = (point.y as i32 + d.1) as u32;

    return true;
}

fn flood_distance( v: &mut Vec<i32>, width : u32, height : u32, start : Point<u32>, end : Point<u32> )
{
    {
        let ref mut val = v[(start.y*width + start.x) as usize];

        if *val == WALL
        {
            panic!("Start point is a wall");
        }

        // start has distance of 0 from itself
        *val = 0;
    }

    {
        let val = v[(end.y*width + end.x) as usize];

        if val == WALL
        {
            panic!("End point is a wall");
        }
    }

    let mut next_points : Vec<Point<u32>> = Vec::new();
    let mut current_points : Vec<Point<u32>> = Vec::new();

    current_points.push(start);
    let mut distance : i32 = 1;

    loop
    {
        for point in &current_points
        {
            for d in DIRECTIONS
            {
                let mut moved_point = point.clone();

                if !move_point( &mut moved_point, d, width, height )
                {
                    continue;
                }

                let ref mut val = v[(moved_point.y*width + moved_point.x) as usize];
       
                // find neighbors that have not been visited or are not a wall
                if *val != WALL && *val > distance
                {
                    // set the current distance for this pixel
                    *val = distance;

                    // save the pixel to enumerate neighbors next
                    next_points.push(moved_point);

                    if end == moved_point
                    {
                        return;
                    }
                }
            }
        }

        distance += 1;
        mem::swap( &mut next_points, &mut current_points );
        next_points.clear();

        if current_points.len() == 0
        {
            panic!("No solution found");
        }
    }
}

fn draw_solution( v : &Vec<i32>, width : u32, height : u32, start : Point<u32>, end : Point<u32>, img : &mut RgbaImage )
{
    let mut point = end;
    let mut distance = v[(end.y*width+end.x) as usize];

    img.put_pixel(end.x, end.y, SOLUTION_COLOR);

    while distance > 0
    {
        for d in DIRECTIONS
        {
            let mut moved_point = point.clone();
            if !move_point( &mut moved_point, d, width, height )
            {
                continue;
            }

            if v[(moved_point.y*width+moved_point.x) as usize] == distance - 1
            {
                point = moved_point.clone();
                distance -= 1;

                img.put_pixel(point.x, point.y, SOLUTION_COLOR);

                if point == start
                {
                    return;
                }

                break;
            }
        }
    }
}

async fn run()
{
    let args : Vec<String> = std::env::args().collect();

    if args.len() != 5
    {
        println!("Usage: ");
        println!("   maze_solver input.png start_x,start_y end_x,end_y output.png");
        return;
    }

    let input_filename = &args[1];

    let start_split : Vec<&str> = args[2].split(",").collect();
    let start = rusttype::point(start_split[0].parse::<u32>().unwrap(), start_split[1].parse::<u32>().unwrap());

    let end_split : Vec<&str> = args[3].split(",").collect();
    let end = rusttype::point(end_split[0].parse::<u32>().unwrap(), end_split[1].parse::<u32>().unwrap());

    let output_filename = &args[4];

    let img;
    {
        let now = Instant::now();
        img = ImageReader::open(input_filename).unwrap().decode().unwrap();
        println!("loading image: {}", now.elapsed().as_millis() as f32 / 1000.0);
    }

    let mut rgb_buffer = Arc::new(img.to_rgba8());

    if start.x >= rgb_buffer.width() || start.y >= rgb_buffer.height()
    {
        panic!("Start is outside image");
    }

    if end.x >= rgb_buffer.width() || end.y >= rgb_buffer.height()
    {
        panic!("End is outside image");
    }

    let mut vec = convert_to_32bit_vector( rgb_buffer.deref() );

    {
        let now = Instant::now();
        flood_distance( &mut vec, rgb_buffer.width(), rgb_buffer.height(), start, end);
        println!("flood: {}", now.elapsed().as_millis() as f32 / 1000.0);
    }

    {
        let now = Instant::now();
        draw_solution( &vec, rgb_buffer.width(), rgb_buffer.height(), start, end, Arc::get_mut(&mut rgb_buffer).unwrap() );
        println!("draw_solution: {}", now.elapsed().as_millis() as f32 / 1000.0);
    }

    {
        let now = Instant::now();
        rgb_buffer.save(output_filename).unwrap();
        println!("save as png: {}", now.elapsed().as_millis() as f32 / 1000.0);
    }
}

fn main() {
    task::block_on(run());
}
