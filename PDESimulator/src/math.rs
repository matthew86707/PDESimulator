pub mod math{

    use std::vec;
    use std::string::ToString;

    // pub struct PDE {
    //     coefficients : Vec<f32>,
    //     derivitives : Vec<f32>,
    //     variables : Vec<u32>,
    //     variable_symbols : Vec<char>
    // }

    // impl PDE {
    //     pub fn new(coef : Vec<f32>, deri : Vec<f32>, vars : Vec<char>) -> PDE {
    //         let mut var_symbols : Vec<char> = Vec::new();
    //         let mut var_nums : Vec<u32> = Vec::new();
    //         for var in &vars {
    //             let mut is_included = false;
    //             for symbol in &var_symbols {
    //                 if var == symbol {
    //                     is_included = true;
    //                 }
    //             }
    //             if !is_included{
    //                 var_symbols.push(*var);
    //             }
    //         }
    //         for var in &vars {
    //             for i in 0..var_symbols.len() {
    //                 if var == var_symbols.get(i).unwrap() {
    //                     var_nums.push(i as u32);
    //                 }
    //             }
    //         }
    //         return PDE{
    //             coefficients : coef,
    //             derivitives : deri,
    //             variables : var_nums,
    //             variable_symbols : var_symbols
    //         };
    //     }
    //     pub fn solve_first_derivitive(&mut self, var : char /*Not to be confused with SQL's VARCHAR...ha...ha......ha */){
    //         for i in 0..self.coefficients.len() {
    //             let mut is_other_variable = false;
    //             let variable_index = self.get_var_index_from_symbol(var);
    //             is_other_variable = self.variables[i] != variable_index;

    //             if is_other_variable {
    //                 //"Move" to other side of equation
    //                 self.coefficients[i] = -self.coefficients[i];
    //             }
    //         }
    //     }
    //     fn get_var_index_from_symbol(&self, var : char) -> u32 {
    //         for i in 0..self.variable_symbols.len() {
    //             if self.variable_symbols[i] == var {
    //                 return i as u32;
    //             }
    //         }
    //         return 0;
    //     }
    // }

    // impl ToString for PDE {
    //     fn to_string(&self) -> String {

    //         return String::from("This is a PDE!");
    //     }
    // }

    // pub struct DiscretePDE {
    //     PDE : PDE,
    //     axis : Vec<DiscreteBoundedAxis>
    // }

    // impl DiscretePDE {
    //     pub fn compute_step_central_diff(&self) -> Vec<f32> {
    //         for axis in self.axis {
                
    //         }
    //         return Vec::new();
    //     }
    //     pub fn compute_step_forward_diff(&self) -> Vec<f32> {
    //         return Vec::new();
    //     }
    //     pub fn compute_step_backward_diff(&self) -> Vec<f32> {
    //         return Vec::new();
    //     }
    //     pub fn compute_step_RK4(&self) -> Vec<f32> {
    //         return Vec::new();
    //     }
    // }

    // pub struct DiscreteBoundedAxis {
    //     lower_bound : BoundaryCondition,
    //     upper_bound : BoundaryCondition,
    //     axis_span : f32,
    //     symbol : char, 
    //     interval : f32,
    //     values : Vec<f32>
    // }

    // impl DiscreteBoundedAxis {
    //     pub fn new(sym : char, mut lower : BoundaryCondition, mut upper : BoundaryCondition, inter : f32) -> DiscreteBoundedAxis{
    //         if upper.location < lower.location {
    //             //Basically whoever is using this is an idiot and got the order mixed up. Switch them, because we're nice.
    //             let temp = lower;
    //             lower = upper;
    //             upper = temp;
    //         }
    //         //TODO : Check this calculation again...
    //         let span : f32 = upper.location - lower.location;
    //         let values_length : usize = (((upper.location - lower.location) / inter) + 1.0) as usize;
    //         DiscreteBoundedAxis{
    //             lower_bound : lower,
    //             upper_bound : upper,
    //             symbol : sym,
    //             interval : inter,
    //             values : vec![0.0; values_length],
    //             axis_span : span
    //         }
    //     }
    //     pub fn get_value(&self, location : f32) -> f32 {
    //         if location > self.upper_bound.location {
    //             return self.upper_bound.value;
    //         } else if location < self.lower_bound.location {
    //             return self.lower_bound.value;
    //         }
    //         let index : usize = ((location - self.lower_bound.location) / self.axis_span) as usize;
    //         return self.values[index];
    //     }
    //     pub fn compute_central_derivitive(&self, location : f32, order : u32) -> f32 {
    //         let coef = get_pascal_triangle_row(order);
    //         let mut derivitive : f32 = 0.0;
    //         for i in 0..coef.len() {
    //             let c = coef[i];
    //             derivitive += f32::powi(-1.0, order as i32) * c * get_value(location + (self.interval * ((order as f32 / 2.0) - i)));
    //         }
    //         //derivitive /= f32::powi(self.interval, order as i32);
    //         return derivitive;
    //     }
    // }

    // pub struct BoundaryCondition {
    //     location : f32,
    //     value : f32
    // }

    // pub fn get_pascal_triangle_row(row_num : u32) -> Vec<u32> {
    //     //Thanks https://stackoverflow.com/questions/15580291/how-to-efficiently-calculate-a-row-in-pascals-triangle !
    //     let mut row : Vec<u32> = vec![1];
    //     for k in 0..row_num {
    //         row.push(row[k as usize] * (row_num-k) / (k+1));
    //     }
    //     return row;
    // }
}