//---------------------- Sequential ----------------------------------------------
pub fn average_exclude_min_max_seq(salary: &[i32]) -> f64 {
    let res = average_exclude_min_max_seq_inner(salary);
    let sum = res.0 - (res.1 + res.2) as i64;
    return (sum as f64)/((salary.len()-2) as f64); 
}
pub fn average_exclude_min_max_seq_inner(salary: &[i32]) -> (i64,i32,i32) {
    let mut min = salary[0];
    let mut max = salary[0];
    let mut sum: i64 = salary[0] as i64;
    for current in &salary[1..]  {
        if *current < min
        {
            min = *current;
        }
        if *current > max
        {
            max = *current;
        }
        sum += *current as i64;
    }
    (sum,min,max)
}
//---------------------- Parallel ----------------------------------------------
pub fn average_exclude_min_max_parallel(salary: &[i32]) -> f64 {
    let res = average_exclude_min_max_parallel_inner(salary);
    let sum = res.0 - (res.1 + res.2) as i64;
    return (sum as f64)/((salary.len()-2) as f64); 
}


pub fn average_exclude_min_max_parallel_inner(salary: &[i32]) -> (i64,i32,i32) {
    if salary.len() == 0 {
        (0, std::i32::MAX, std::i32::MIN)
    }else if salary.len()<=100_000{
        average_exclude_min_max_seq_inner(salary)
    } else {
        let mid = salary.len() / 2;
        let (left_input, right_input) = salary.split_at(mid);
        let (left_res, right_res) = diam::join(|| average_exclude_min_max_parallel_inner(left_input),
        || average_exclude_min_max_parallel_inner(right_input));
        (left_res.0 + right_res.0, std::cmp::min(left_res.1,right_res.1), std::cmp::max(left_res.2,right_res.2))
    }
}

const ARRAY_SIZE: usize = 1_000_000;
const THREADS_COUNT: usize = 8;

fn main() {
    let v: Vec<i32> = std::iter::repeat_with(rand::random).take(ARRAY_SIZE).collect();
    rayon::ThreadPoolBuilder::new().num_threads(THREADS_COUNT).build_global().unwrap();
    main_performance(v)
}
fn main_performance(v: Vec<i32>) {

    let start = std::time::Instant::now();
    let res_seq = average_exclude_min_max_seq(&v);
    println!("The sequential value is {:?} took {:?}",res_seq, start.elapsed());

    let start = std::time::Instant::now();
    let res_parallel = average_exclude_min_max_parallel(&v);
    println!("The parallel value is {:?} took {:?}",res_parallel, start.elapsed());
}

fn main_svg(v: Vec<i32>) {

    let res_seq = average_exclude_min_max_seq(&v);

    let _ = diam::svg("Run_Tree.svg" , || {
        let res_parallel = average_exclude_min_max_parallel(&v);
        assert_eq!(res_seq , res_parallel);
    })
    .expect("failed generating svg file");

    
}
