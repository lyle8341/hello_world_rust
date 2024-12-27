use std::thread;
use std::time::Duration;

struct Cache<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T, //闭包
    value: Option<u32>,//可以使用HashMap
}

impl<T> Cache<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cache<T> {
        Cache {
            calculation,
            value: None,
        }
    }

    /// 会修改结构体的内容
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

//定义一个函数，函数遇到小括号才执行
fn generate_workout(intensity: u32, random_num: u32) {
    let mut cache = Cache::new(|num| {
        println!("calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", cache.value(intensity));
        println!("Next, do {} situps!", cache.value(intensity));
    } else {
        if random_num == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cache.value(intensity));
        }
    }
}
