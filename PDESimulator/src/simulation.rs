use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::{thread, time};

pub fn simulation_loop(display_values_mutex : Arc<Mutex<[f32; 15000]>>, GRID_SIZE_X : usize, GRID_SIZE_Y : usize){
        let TIME_SAMPLES_PER_PRINTOUT : u32 = 100;
        let mut time_samples : u32 = 0;
        let mut time_acc : u32 = 0;
        let mut tick_number : u32 = 0;

        loop {
            let start = Instant::now();

            {
                let mut old_values = display_values_mutex.lock().unwrap();
                let new_values = heat_simulation_loop(block_array(*old_values), tick_number);
                tick_number += 1;
                *old_values = serialize_array(new_values);
            }

            time_acc += Instant::now().duration_since(start).subsec_millis();
            if (time_samples == TIME_SAMPLES_PER_PRINTOUT){
                println!("Delay : {}ms", time_acc as f32 / TIME_SAMPLES_PER_PRINTOUT as f32);
                time_samples = 0;
                time_acc = 0;
            }
            time_samples += 1;
            
            let ten_millis = time::Duration::from_millis(5);
            thread::sleep(ten_millis);

        }
    }


fn heat_simulation_loop(data : [[f32; 100];150], tick_number : u32) -> [[f32; 100];150] {
    let mut next_data : [[f32; 100];150] = data;
    //Actual simulation code
    {
        /*
        Heat equation : du/dt - (du/dx)^2, where u = temperature, t = time, and x = our spacial domain
        which, solved for du/dt : du/dt = (du/dx)^2
        expanding spacial domain and therefor it's derivitive into two dimensions : du/dt = (du/dx)^2 + (du/dy)^2

        From this PDE we can use the central difference method to approximate the 2nd spacial derivitives...
        
        From the difference quotient of the first derivitive...
            f(x - h) - f(x + h) / 2h
        We then derive the difference quotient of the 2nd derivitive...
            f(x - h) - 2 * f(x) + f(x + h) / h^2
        In code, this recurrence equation implies...
            next_data[i][j] += ((data[i  + 1][j] - (2.0 * data[i][j]) + data[i - 1][j]) + (data[i][j + 1] - (2.0 * data[i][j]) + data[i][j - 1])) / h ^ 2; 
        Which is seen implemented below.

        A note on boundry conditions : 
            First derivitive boundry conditions set du/dx on all walls to be 0.5

        */
        for i in 0..150 - 1{
            for j in 0..100 - 1{
                if i == 0 || i == 150 || j == 0 || j == 100{
                //next_data[i][j] = 0.5;
                 }else{
                next_data[i][j] += ((data[i  + 1][j] - (2.0 * data[i][j]) + data[i - 1][j]) + (data[i][j + 1] - (2.0 * data[i][j]) + data[i][j - 1])) / 4.00; 
                 }
            }
         }
        if(tick_number < 200){
            next_data[75][20] = 1.0;
            next_data[75][40] = 1.0;
            next_data[75][60] = 1.0;
            next_data[75][80] = 1.0;

            next_data[95][20] = 1.0;
            next_data[95][40] = 1.0;
            next_data[95][60] = 1.0;
            next_data[95][80] = 1.0;
        }
    }
    return next_data;
}


fn serialize_array(array : [[f32; 100];150]) -> [f32; 15000]{
    let mut to_return : [f32; 15000] = [0.0; 15000];
    for i in 0..to_return.len() {
        to_return[i] = array[i % 150][i / 150];
    }
    return to_return;
}

fn block_array(array : [f32; 15000]) -> [[f32; 100];150]{
    let mut to_return : [[f32; 100];150] = [[0.0; 100]; 150];
    for i in 0..150{
        for j in 0..100{
        to_return[i][j] = array[j * 150 + i];
        }
    }
    return to_return;
}