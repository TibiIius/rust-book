use core::slice;

// extern code is always unsafe
extern "C" {
  fn abs(input: i32) -> i32;
}

unsafe fn danger_time(x: i32) {
  let r = &x as *const i32;

  println!("r holds {}", *r);
}

fn split_at_mut(values_vec: &mut [i32], split: usize) -> (&mut [i32], &mut [i32]) {
  let len = values_vec.len();
  let ptr = values_vec.as_mut_ptr();

  assert!(split <= len);

  // can't be called like this
  // we borrow from the `values_vec` slice twice, but you can normally only borrow from a slice once
  // this is ok, since both parts of the slice are different, but the borrow checker can't know that
  // (&mut values_vec[..split], &mut values_vec[split..])

  // unsafe implementation
  unsafe {
    (
      slice::from_raw_parts_mut(ptr, split),
      slice::from_raw_parts_mut(ptr.add(split), len - split),
    )
  }
}

fn main() {
  let mut num = 5;

  let ref_1 = &num as *const i32;
  let ref_2 = &mut num as *mut i32;

  // raw pointers are only deferencable in unsafe blocks
  unsafe {
    println!("ref_1 is: {}", *ref_1);
    println!("ref_2 is: {}", *ref_2);

    danger_time(num);

    println!("Absolute value of -3, with abs called from C: {}", abs(-3));
  }
}
