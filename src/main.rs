use image::io::Reader as ImageReader;
use image::{RgbImage, Rgb};
use async_std::{task};
use std::sync::Arc;
use futures::join;
use std::ops::Deref;
use rusttype::Point;
use std::mem;
use std::time::{Duration, Instant};

fn find_pixel( img : Arc<RgbImage>, p : Rgb::<u8> ) -> (bool, u32, u32)
{
    for y in 0..img.height()
    {
        for x in 0..img.width()
        {
            let p2 = *img.get_pixel(x,y);
          //  print!("{:?}", p2);
            if p == p2
            {
                return (true, x,y);
            }
        }
    }

    return (false, 0,0);
}


const WALL : i32 = i32::MAX - 0;
const UNSET : i32 = i32::MAX - 1;

fn convert_to_32bit_vector( img : &RgbImage ) -> Vec<i32>
{
    let mut v : Vec<i32> = Vec::new();
    v.resize( ( img.width() * img.height() ) as usize, 0);

    let mut i = 0;
    for p in img.pixels()
    {
        v[i] = if p[0] <= 100 {WALL} else {UNSET};
        i += 1;
    }

    return v;
}

const DIRECTIONS : [(i32, i32); 4] = [(0,1), (1,0), (-1, 0), (0,-1)];

fn flood_distance( v: &mut Vec<i32>, width : i32, height : i32, start : Point<i32>, end : Point<i32> )
{
    let mut next_points : Vec<Point<i32>> = Vec::new();
    let mut current_points : Vec<Point<i32>> = Vec::new();

    current_points.push(start);

    let mut distance : i32 = 1;

    {
        let ref mut val = v[(start.y*width + start.x) as usize];

        if *val == WALL
        {
            panic!("Start point is a wall");
        }

        *val = 0;
    }

    {
        let val = v[(end.y*width + end.x) as usize];

        if val == WALL
        {
            panic!("End point is a wall");
        }
    }

    loop
    {
        for point in &current_points
        {
            for d in DIRECTIONS
            {
                let x = point.x + d.0;
                let y = point.y + d.1;

                if x < 0 || y < 0 || x >= width || y >= height
                {
                    continue;
                }

                let ref mut val = v[(y*width + x) as usize];
                if *val != WALL && *val > distance
                {
                    *val = distance;
                    next_points.push(rusttype::point(x,y));

                    if *point == end
                    {
                        return;
                    }
                }
            }
        }

        distance += 1;
        mem::swap( &mut next_points, &mut current_points );
        next_points.clear();

        // println!("{}, {}", distance, current_points.len());

        if current_points.len() == 0
        {
            return;
        }
    }
}

fn draw_solution( v : &Vec<i32>, width : i32, height : i32, start : Point<i32>, end : Point<i32>, img : &mut RgbImage )
{
    let mut point = end;
    let mut distance = v[(end.y*width+end.x) as usize];

 //   println!("{}", distance);

    while distance > 0
    {
        for d in DIRECTIONS
        {
            let x = point.x + d.0;
            let y = point.y + d.1;

            if x < 0 || y < 0 || x >= width || y >= height
            {
                continue;
            }

            if v[(y*width+x) as usize] == distance - 1
            {
                point.x = x;
                point.y = y;
                distance -= 1;

                img.put_pixel(x as u32, y as u32, Rgb([255, 0, 0]));

                if point == start
                {
                    return;
                }

               //  println!("{}", distance);

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
        println!("   images_test input.png start_x,start_y end_x,end_y output.png");
        return;
    }

    let input_filename = &args[1];

    let start_split : Vec<&str> = args[2].split(",").collect();
    let start = rusttype::point(start_split[0].parse::<i32>().unwrap(), start_split[1].parse::<i32>().unwrap());

    let end_split : Vec<&str> = args[3].split(",").collect();
    let end = rusttype::point(end_split[0].parse::<i32>().unwrap(), end_split[1].parse::<i32>().unwrap());

    let output_filename = &args[4];

    let mut img = ImageReader::open(input_filename).unwrap().decode().unwrap();
    let mut rgb_buffer = Arc::new(img.to_rgb8());

    let mut vec = convert_to_32bit_vector( rgb_buffer.deref() );

    {
        let now = Instant::now();
        flood_distance( &mut vec, rgb_buffer.width().try_into().unwrap(), rgb_buffer.height().try_into().unwrap(), start, end);
        println!("flood: {}", now.elapsed().as_millis() as f32 / 1000.0);
    }

    {
        let now = Instant::now();
        draw_solution( &vec, rgb_buffer.width().try_into().unwrap(), rgb_buffer.height().try_into().unwrap(), start, end, Arc::get_mut(&mut rgb_buffer).unwrap() );
        println!("draw_solution: {}", now.elapsed().as_millis() as f32 / 1000.0);
    }

    {
        let now = Instant::now();
        rgb_buffer.save(output_filename).unwrap();
        println!("save as png: {}", now.elapsed().as_millis() as f32 / 1000.0);
    }

    /*
    let i1 = img.clone();
    let p1 = task::spawn(async{ find_pixel(i1, Rgb::<u8>([255, 0, 0]))});
    let i2 = img.clone();
    let p2 = task::spawn(async{ find_pixel(i2, Rgb::<u8>([0, 255, 0]))});

    let (start, end) = join!(p1, p2);


    let mut v = convert_to_32bit_vector(img.deref());

    println!("{}", v.len());

    println!("{}x{}", img.width(), img.height());

    if start.0
    {
        println!("{}x{}", start.1, start.2);
    }
    if end.0
    {
        println!("{}x{}", end.1, end.2);
    }*/
}

fn main() {
    task::block_on(run());
}
