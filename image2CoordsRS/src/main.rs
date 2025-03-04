use std::env;
use std::fs;
use std::i32::MAX;
use std::process::Command;
use image::io::Reader as ImageReader;
use image::GenericImageView;
use csv;

static TEMPFOLDER:&str = "imageTemp";
static FORMAT:&str = "bmp"; // BMP has no compression and is lossless
static FPS:u8 = 24; // Usually used by movies as it's the minimum for human eye to perceive motion
static RADIUS:i32 = 2; // Affects the amount of space around a pixel that is also searched
static WIDTH:i32 = 480; // Width of the video
static HEIGHT:i32 = 480; // Height of the video

struct PointObj {
    path: Vec<(i32, i32)>,
}

trait Point {
    fn new() -> Self;
    fn add(&mut self, x: i32, y: i32);
    fn get(&self, i: usize) -> (i32, i32);
    fn get_full(&self) -> Vec<(i32, i32)>;
    fn dist(&self, i:i32, x: i32, y: i32) -> i32;
}

impl Point for PointObj {
    fn new() -> PointObj {
        PointObj {
            path: Vec::new(),
        }
    }

    fn add(&mut self, x: i32, y: i32) {
        self.path.push((x, y));
    }

    fn get(&self, i: usize) -> (i32, i32) {
        self.path[i]
    }

    fn get_full(&self) -> Vec<(i32, i32)> {
        self.path.clone()
    }

    fn dist(&self, i: i32, x: i32, y: i32) -> i32 {
        let (px, py) = self.get(i as usize);
        return f32::sqrt(((px - x).pow(2) + (py - y).pow(2)) as f32) as i32 // Pythagorean theorem
    }
}

#[derive(PartialEq, PartialOrd)]
struct ColorObj {
    r: u8,
    g: u8,
    b: u8,
}

impl ColorObj {
    fn new(r: u8, g: u8, b: u8) -> ColorObj {
        ColorObj {
            r: r,
            g: g,
            b: b,
        }
    }

    fn copy(&self) -> ColorObj {
        ColorObj {
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }
}

struct ImageObject {
    name: String,
    width: i32,
    height: i32,
    pixels: Vec<u8>,
}

trait Image {
    fn new(filename: String) -> Self;
    fn to_string(&self) -> String;
    fn get_pixel(&self, x: i32, y: i32) -> ColorObj;
    fn set_pixel(&mut self, x: i32, y: i32, color: ColorObj);
    fn tolerance_image(&self, color: ColorObj, tol: u8) -> Self;
    fn cluster_identification(self) -> Self;
    fn to_file(&self, filename: String);
    fn clone(&self) -> Self;
}

impl Image for ImageObject {
    fn new(filename: String) -> ImageObject {
        let img = ImageReader::open(filename.clone()).unwrap().decode().unwrap();
        let (width, height) = img.dimensions();
        let pixels = img.to_rgb8().into_raw();
        ImageObject {
            name: filename,
            width: width as i32,
            height: height as i32,
            pixels: pixels,
        }
    }

    fn to_string(&self) -> String {
        format!("[File: {} Image width: {}, height: {}]", self.name, self.width, self.height)
    }

    fn get_pixel(&self, x: i32, y: i32) -> ColorObj {
        let i = (y * self.width + x) as usize;
        let r = self.pixels[i * 3];
        let g = self.pixels[i * 3 + 1];
        let b = self.pixels[i * 3 + 2];
        ColorObj::new(r, g, b)
    }

    fn set_pixel(&mut self, x: i32, y: i32, color: ColorObj) {
        let i = (y * self.width + x) as usize;
        self.pixels[i * 3] = color.r;
        self.pixels[i * 3 + 1] = color.g;
        self.pixels[i * 3 + 2] = color.b;
    }

    fn tolerance_image(&self, color: ColorObj, tol: u8) -> ImageObject {
        fn tolerance(color: &ColorObj, refcolor: &ColorObj, tol: u8) -> bool {
            let ur = if 255 - tol < refcolor.r { 255 } else { refcolor.r + tol };
            let br = if tol > refcolor.r { 0 } else { refcolor.r - tol };
            let ug = if 255 - tol < refcolor.g { 255 } else { refcolor.g + tol };
            let bg = if tol > refcolor.g { 0 } else { refcolor.g - tol };
            let ub = if 255 - tol < refcolor.b { 255 } else { refcolor.b + tol };
            let bb = if tol > refcolor.b { 0 } else { refcolor.b - tol };
            color.r >= br && color.r <= ur && color.g >= bg && color.g <= ug && color.b >= bb && color.b <= ub
        }
        let mut new_image = ImageObject::clone(&self);
        for x in 0..self.width {
            for y in 0..self.height {
                let pixel = self.get_pixel(x, y);
                if tolerance(&pixel, &color, tol) {
                    let new_color = ColorObj::new(255, 255, 255);
                    new_image.set_pixel(x, y, new_color);
                } else {
                    let new_color = ColorObj::new(0, 0, 0);
                    new_image.set_pixel(x, y, new_color);
                
                }
            }
        }
        new_image
    }

    fn cluster_identification(self) -> ImageObject {
        // Perform connected component labelling
        let mut traversed = vec![vec![false; self.height as usize]; self.width as usize];
        let mut clusters = Vec::new();
        for x in 0..self.width {
            for y in 0..self.height {
                if traversed[x as usize][y as usize] {
                    continue;
                }
                // iterate until all pixels are visited
                // if pixel is white, then perform BFS
                // if pixel is black, then skip
                if self.get_pixel(x, y) == ColorObj::new(255, 255, 255) {
                    // perform BFS
                    let mut cluster = Vec::new();
                    cluster.push((x, y));
                    let mut queue = Vec::new();
                    queue.push((x, y));
                    while !queue.is_empty() {
                        // check all 8 directions
                        let (lx, ly) = queue.pop().unwrap();
                        // add all 8 directions to queue
                        // NOT FULLY OPTIMIZED
                        for i in -RADIUS..RADIUS {
                            for j in -RADIUS..RADIUS {
                                let nx = lx + i;
                                let ny = ly + j;
                                if nx >= 0 && nx < self.width && ny >= 0 && ny < self.height {
                                    if !traversed[nx as usize][ny as usize] {
                                        traversed[nx as usize][ny as usize] = true;
                                        if self.get_pixel(nx, ny) == ColorObj::new(255, 255, 255) {
                                            queue.push((nx, ny));
                                            cluster.push((nx, ny));
                                        }
                                    }
                                }
                            }
                        }
                    }
                    clusters.push(cluster);
                } else {
                    traversed[x as usize][y as usize] = true;
                }
            }
        }
        let mut new_image = ImageObject::clone(&self);
        // Compute the centroid of each cluster
        for cluster in &clusters {
            let mut sumx = 0;
            let mut sumy = 0;
            for (x, y) in cluster {
                sumx += *x;
                sumy += *y;
            }
            let centroid_x = sumx / cluster.len() as i32;
            let centroid_y = sumy / cluster.len() as i32;
            new_image.set_pixel(centroid_x, centroid_y, ColorObj::new(0, 255, 0));
            // // Shade in surrounding pixels -- DEBUG ONLY
            // for i in -RADIUS..RADIUS {
            //     for j in -RADIUS..RADIUS {
            //         let nx = centroid_x + i;
            //         let ny = centroid_y + j;
            //         if nx >= 0 && nx < self.width && ny >= 0 && ny < self.height {
            //             new_image.set_pixel(nx, ny, ColorObj::new(0, 255, 0));
            //         }
            //     }
            // }
        }
        new_image
    }

    fn to_file(&self, filename: String) {
        let img = image::RgbImage::from_raw(self.width as u32, self.height as u32, self.pixels.clone()).unwrap();
        img.save(filename).unwrap();
    }

    fn clone(&self) -> Self {
        ImageObject {
            name: self.name.clone(),
            width: self.width,
            height: self.height,
            pixels: self.pixels.clone(),
        }
    }
}

struct VideoObject {
    images: Vec<ImageObject>,
}

trait Video {
    fn new(filename: String) -> Self;
    fn to_string(&self) -> String;
    fn apply_tolerances(self, color: ColorObj, tol: u8) -> Self;
    fn cluster_identification(self) -> Self;
    fn render(&self, vformat: String) -> ();
    fn csv(&self, output: String) -> ();
}

impl Video for VideoObject {
    fn new(filename: String) -> VideoObject {
        // Create Result directory if it doesn't exist
        fs::create_dir_all(TEMPFOLDER).expect("Failed to create Result directory");
        // Delete all files in the tempfolder directory
        for entry in fs::read_dir(TEMPFOLDER).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            fs::remove_file(path.clone()).expect(&format!("Failed to delete file: {:?}", path));
        }
        // Execute ffmpeg command to convert video to images
        println!("Executing Video to Image conversion");
        let output = Command::new("ffmpeg")
            .args(&["-i", &filename, "-s", &format!("{}x{}", WIDTH, HEIGHT), "-vf" ,&format!("fps={}", FPS), &format!("./{}/image%d.{}", TEMPFOLDER, FORMAT)])
            .output()
            .expect("Failed to execute command");
        // println!("Output: {:?}", output);
        if output.status.success() {
            println!("Video to Image conversion successful");
        } else {
            println!("Video to Image conversion failed: {}", String::from_utf8_lossy(&output.stderr));
        }
        // Get all images in the tempfolder directory
        let mut images = Vec::new();
        // Read files in tempfolder in order by name
        let mut count = 1;
        loop {
            let filename = format!("{}/image{}.{}", TEMPFOLDER, count, FORMAT);
            if fs::metadata(&filename).is_ok() {
                images.push(ImageObject::new(filename));
                count += 1;
            } else {
                break;
            }
        }
        VideoObject {
            images: images,
        }
    }

    fn to_string(&self) -> String {
        format!("Frames: {}", self.images.len())
    }

    fn apply_tolerances(self, color: ColorObj, tol: u8) -> VideoObject {
        let mut new_images = Vec::new();
        for image in self.images {
            new_images.push(image.tolerance_image(color.copy(), tol));
        }
        VideoObject {
            images: new_images,
        }
    }

    fn cluster_identification(self) -> VideoObject {
        let mut new_images = Vec::new();
        for image in self.images {
            new_images.push(image.cluster_identification());
        }
        VideoObject {
            images: new_images,
        }
    }

    fn render(&self, vformat: String) -> () {
        // Delete all files in the tempfolder directory
        for entry in fs::read_dir(TEMPFOLDER).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            fs::remove_file(path.clone()).expect(&format!("Failed to delete file: {:?}", path));
        }
        // Delete vformat file if it exists otherwise skip
        if fs::metadata(&vformat).is_ok() {
            fs::remove_file(&vformat).expect(&format!("Failed to delete file: {:?}", vformat));
        }
        // convert all into files for ffmpeg
        let mut count = 1;
        for image in &self.images {
            image.to_file(format!("{}/image{}.png", TEMPFOLDER, count));
            count += 1;
        }
        // Check which encoder is available on system
        let codecs_output = Command::new("ffmpeg")
            .args(&["-codecs"])
            .output()
            .expect("Failed to execute command");
        let mut h264_encoder = "ERROR";
        for line in String::from_utf8_lossy(&codecs_output.stdout).lines() {
                if line.contains("h264") {
                    if line.contains("libopenh264") {
                        h264_encoder = "libopenh264";
                    } else if line.contains("libx264") {
                        h264_encoder = "libx264";
                    }
                }
        }
        println!("Converting Images to Video");
        let conversion_output = Command::new("ffmpeg")
            .args(&["-framerate", &format!("{}", FPS), "-i", &format!("{}/image%d.png", TEMPFOLDER), "-c:v", h264_encoder, &vformat])
            .output()
            .expect("Failed to execute command");
        if conversion_output.status.success() {
            println!("Image to Video conversion successful");
        } else {
            println!("Image to Video conversion failed: {}", String::from_utf8_lossy(&conversion_output.stderr));
        }
    }
    fn csv(&self, output: String) -> () {
        // In first image locate all green pixels and generate points
        let mut points = Vec::new();
        let first_image = &self.images[0];
        for x in 0..first_image.width {
            for y in 0..first_image.height {
                if first_image.get_pixel(x, y) == ColorObj::new(0, 255, 0) {
                    let mut point = PointObj::new();
                    point.add(x, y);
                    points.push(point);
                }
            }
        }
        // Then for each subsequent image, green points are matched to the nearest green point in the previous frame by using the points
        for i in 1..self.images.len() {
            let image = &self.images[i];
            for x in 0..image.width {
                for y in 0..image.height {
                    if image.get_pixel(x, y) == ColorObj::new(0, 255, 0) {
                        let mut min_dist = MAX;
                        let mut min_index = 0;
                        for (j, point) in points.iter().enumerate() {
                            let dist = point.dist(i as i32 - 1, x, y);
                            if dist < min_dist {
                                min_dist = dist;
                                min_index = j;
                            }
                        }
                        points[min_index].add(x, y);
                    }
                }
            }
        }
        // Write to CSV with each point getting it's own column
        let mut wtr = csv::Writer::from_path(output).unwrap();
        let mut column_declare:String = String::new();
        for i in 0..points.len() {
            column_declare.push_str(&format!("pointx_{}, pointy_{},", i, i));
        }
        wtr.write_record(column_declare.split(",").collect::<Vec<&str>>()).unwrap();
        for i in 0..self.images.len() {
            let mut line = String::new();
            for point in &points {
                let (x, y) = point.get(i);
                line.push_str(&format!("{},{},", x, y));
            }
            wtr.write_record(line.split(",").collect::<Vec<&str>>()).unwrap();
        }
        wtr.flush().unwrap();
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <input.mp4>", args[0]);
        return;
    }
    let video = VideoObject::new(args[1].clone());
    println!("Imported Video: {}", video.to_string());
    println!("Applying Tolerances");
    let tol_video = video.apply_tolerances(ColorObj::new(255, 0, 0), 50);
    println!("Identifying Clusters");
    let cluster_video = tol_video.cluster_identification();
    println!("Preparing to output video");
    cluster_video.render("output.mp4".to_string());
    println!("Preparing to output CSV");
    cluster_video.csv("output.csv".to_string());
}
