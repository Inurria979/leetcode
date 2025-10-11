/*
A magician has various spells.

You are given an array power, where each element represents the damage of a spell. Multiple spells can have the same damage value.

It is a known fact that if a magician decides to cast a spell with a damage of power[i], they cannot cast any spell with a damage of power[i] - 2, power[i] - 1, power[i] + 1, or power[i] + 2.

Each spell can be cast only once.

Return the maximum possible total damage that a magician can cast.

 

Example 1:

Input: power = [1,1,3,4]

Output: 6

Explanation:

The maximum possible damage of 6 is produced by casting spells 0, 1, 3 with damage 1, 1, 4.

Example 2:

Input: power = [7,1,6,6]

Output: 13

Explanation:

The maximum possible damage of 13 is produced by casting spells 1, 2, 3 with damage 1, 6, 6.

 

Constraints:

1 <= power.length <= 105
1 <= power[i] <= 109
*/

use std::collections::HashMap;

fn main(){
    let power1 = Vec::<i32>::from([1,1,3,4]);
    let power2 = Vec::<i32>::from([7,1,6,6]);

    print!("\nThe max val is: {:?}\n", maximum_total_damage(power1));
    print!("\nThe max val is: {:?}\n", maximum_total_damage(power2));
}
pub fn maximum_total_damage(power: Vec<i32>) -> i64 {

    let val_ocu = create_pair_power_occurences(power.clone());
    let mut set_of_keys = Vec::<i32>::new();
    //while some key exist it is possible to find an usable power
    let mut mut_val_ocu = val_ocu.clone();
    while mut_val_ocu.len() > 0{
        let key = get_best_option(mut_val_ocu.clone());
        if set_of_keys.len() == 0{
            set_of_keys.push(key);
        }else if (set_of_keys.get(set_of_keys.len()-1).unwrap() - key).abs() > 2 {
            set_of_keys.push(key)
        }
        //Remove this key
        mut_val_ocu.remove(&key);
    }
    let mut sum: i64 = 0;
    for key in &set_of_keys{
        sum += i64::from(*val_ocu.get(key).unwrap()) * i64::from(*key);
        //print!("\nKey: {:?} VAL: {:?}", key, *val_ocu.get(key).unwrap());
    }
    
    return sum;
}
fn create_pair_power_occurences(v: Vec<i32>)-> HashMap<i32, i32>{
    let mut pair = HashMap::<i32, i32>::new();
    for i in &v{
        if  !pair.contains_key(i) {
            pair.insert(*i, 1);
        }else {
            pair.insert(*i, pair.get(i).unwrap()+1);
        }
    }
    return pair;
}
fn get_best_option(val_ocu: HashMap<i32, i32>) -> i32{
    
    let mut best_key: i32 = 0;
    let mut best_mult: i32 = 0;
    for (key, val) in val_ocu{
        if key*val > best_mult {
            best_mult = key*val;
            best_key = key;
        }
    }
    return best_key;
}