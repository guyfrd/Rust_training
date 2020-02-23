

fn main(){

    let mut my_vec = vec!['G','G','G','R','G','R','R'];
    let _len = my_vec.len(); 
    let mut i_from_end = _len -1; 
    let mut i_from_start = 0; 
    println!("{:?} ", _len);
    println!("{:?} ",my_vec);

    while i_from_start < i_from_end{
        if my_vec[i_from_start] == 'G'{
            while my_vec[i_from_end] != 'R'{
                i_from_end = i_from_end -1;
            }
            if i_from_end <= i_from_start {
                break;
            } else {
                my_vec.swap(i_from_start, i_from_end); 
            };
        }
        println!("{:?} ",my_vec);
        i_from_start = i_from_start+1;
    }

    println!("{:?} ",my_vec);
    // sort_balls(vec: &mut [String])
}

// fn sort_balls(vec: &mut [String]){


// }