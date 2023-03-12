use std::fs;

#[derive(Clone, PartialEq)]
struct Voxel {
    x: i8,
    y: i8,
    z: i8,
}

impl Voxel {
    fn from(line: &str) -> Self {
        let components = line.split(',').collect::<Vec<&str>>();
        if components.len() != 3 {
            panic!("Unable to parse line {} into a Voxel.", line);
        }
        Self {
            x: components[0].parse().unwrap(),
            y: components[1].parse().unwrap(),
            z: components[2].parse().unwrap(),
        }
    }
}

fn find_total_surface_area(voxels: &Vec<Voxel>) -> usize {
    let mut total_surface = 0;
    for voxel in voxels {
        total_surface += 6 - voxels.iter()
                            .filter(|v| 
                                    v.x == voxel.x && v.y == voxel.y && (v.z - voxel.z).abs() == 1 ||
                                    v.x == voxel.x && (v.y - voxel.y).abs() == 1 && v.z == voxel.z ||
                                    (v.x - voxel.x).abs() == 1 && v.y == voxel.y && v.z == voxel.z
                                )
                            .count()
        
    }
    total_surface
}

fn find_area_reachable_from_origin(voxels: &Vec<Voxel>) -> usize {
    let max_x = voxels.iter().map(|v| v.x).max().unwrap() + 1;
    let max_y = voxels.iter().map(|v| v.y).max().unwrap() + 1;
    let max_z = voxels.iter().map(|v| v.z).max().unwrap() + 1;
    
    let mut water = vec![vec![Voxel { x: 0, y: 0, z: 0 }]];
    loop {
        let mut water_this_step = Vec::new();
        for water_from_last_step in &water[water.len()-1] {
            let existing_water = water.iter().flatten().cloned().collect::<Vec<Voxel>>();
            for x_shift in [-1,1] {
                let this_dropplet = Voxel { 
                                x: water_from_last_step.x + x_shift,
                                y: water_from_last_step.y,
                                z: water_from_last_step.z,
                            };
                if !existing_water.contains(&this_dropplet) && 
                            !water_this_step.contains(&this_dropplet) && 
                            !voxels.contains(&this_dropplet) && 
                            this_dropplet.x <= max_x && this_dropplet.x >= -1 {
                                water_this_step.push(this_dropplet);
                            }
            }
            for y_shift in [-1,1] {
                let this_dropplet = Voxel { 
                                x: water_from_last_step.x,
                                y: water_from_last_step.y + y_shift,
                                z: water_from_last_step.z,
                            };
                if !existing_water.contains(&this_dropplet) && 
                            !water_this_step.contains(&this_dropplet) && 
                            !voxels.contains(&this_dropplet) && 
                            this_dropplet.y <= max_y && this_dropplet.y >= -1 {
                                water_this_step.push(this_dropplet);
                            }
            }
            for z_shift in [-1,1] {
                let this_dropplet = Voxel { 
                                x: water_from_last_step.x,
                                y: water_from_last_step.y,
                                z: water_from_last_step.z + z_shift,
                            };
                if !existing_water.contains(&this_dropplet) && 
                            !water_this_step.contains(&this_dropplet) && 
                            !voxels.contains(&this_dropplet) && 
                            this_dropplet.z <= max_z && this_dropplet.z >= -1 {
                                water_this_step.push(this_dropplet);
                            }
            }
        }
        if water_this_step.is_empty() {
            break;
        }
        water.push(water_this_step);
    }

    let mut total_surface = 0;
    for voxel in voxels {
        total_surface += water.iter()
                            .flatten()
                            .filter(|v| 
                                    v.x == voxel.x && v.y == voxel.y && (v.z - voxel.z).abs() == 1 ||
                                    v.x == voxel.x && (v.y - voxel.y).abs() == 1 && v.z == voxel.z ||
                                    (v.x - voxel.x).abs() == 1 && v.y == voxel.y && v.z == voxel.z
                                )
                            .count()
        
    }
    total_surface
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path)
        .expect("File not found")
}

fn main() {
    let scan = read_file("input");
    let voxels = scan.lines().map(Voxel::from).collect::<Vec<_>>();
    
    println!("The total surface Area including air pockets is {}.", find_total_surface_area(&voxels));
    println!("The outside surface Area is {}.", find_area_reachable_from_origin(&voxels));
}

#[test]
fn sample_input() {
    let scan = read_file("tests/sample_input");
    let voxels = scan.lines().map(Voxel::from).collect::<Vec<_>>();

    assert_eq!(find_total_surface_area(&voxels), 64);
    assert_eq!(find_area_reachable_from_origin(&voxels), 58);
}

#[test]
fn challenge_input() {
    let scan = read_file("tests/input");
    let voxels = scan.lines().map(Voxel::from).collect::<Vec<_>>();

    assert_eq!(find_total_surface_area(&voxels), 4320);
    assert_eq!(find_area_reachable_from_origin(&voxels), 2456);
}
