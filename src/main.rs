use num_format::{Locale, ToFormattedString};

fn get_chunk_size_from_user() -> u64 {
    loop {
        let mut number = String::new();
        println!("Please enter a chunk size (divisible by 2, over 16):");
        std::io::stdin()
            .read_line(&mut number)
            .expect("Failed to read input");

        match number.trim().parse::<u64>() {
            Ok(val) => {
                if val % 2 != 0 || val < 16 {
                    eprintln!("Please enter a valid number divisible by 2 and over 16!");
                    continue;
                }
                return val;
            }
            Err(_) => {
                eprintln!("Please enter a valid number divisible by 2!");
            }
        };
    }
}

fn main() {
    // ask the user how many subuids per uid

    let chunk_size: u64 = get_chunk_size_from_user();
    println!("Using chunk size {}", chunk_size);

    let sub_uid_max: u64 = (u32::MAX - 1) as u64;
    let uid_min: u64 = 5000;
    let uid_max: u64 = sub_uid_max / (chunk_size * 2);
    let sub_uid_min: u64 = uid_max + 1;

    let num_uids = uid_max - uid_min;

    println!("sub_uid_min: {}", sub_uid_min);
    println!("sub_uid_max: {}", sub_uid_max);
    println!("uid_min: {}", uid_min);
    println!("uid_max: {}", uid_max);

    let mut max_allocated_subuid = 0;

    for uid in uid_min..=uid_max {
        // start at the bottom of the subuid range, allocating in blocks of CHUNK_SIZE ids.
        let min_subuid = sub_uid_min + ((uid - uid_min) * (chunk_size));
        let max_subuid = min_subuid + chunk_size - 1;

        if max_subuid > sub_uid_max {
            panic!(
                "Out of subuid range uid={uid} subuid={max_subuid} > {sub_uid_max} by {}, we were {} from the max uid",
                (max_subuid - sub_uid_max),
                (uid_max - uid)
            );
        }
        // catch that first time
        if max_allocated_subuid == 0 {
            max_allocated_subuid = min_subuid;
        } else if min_subuid <= max_allocated_subuid {
            panic!("Overlap uid={uid} min_subuid={min_subuid} <= {max_allocated_subuid}");
        } else {
            max_allocated_subuid = max_subuid;
        }
    }
    println!(
        "All good! You get {} users with {} subuids each",
        (num_uids).to_formatted_string(&Locale::en),
        chunk_size
    );
}
