#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut a = 3;
        // Push first reference to a to borrowed stack
        let ref1 = &mut a;
        // Push second reference to a to borrowed stack
        // Because Rust doesn't allow to explicitly reference to a more than once at a time
        // So we need to reference a through deference ref1
        // This way we also can keep track of stack, this just like push a element to a stack
        let ref2 = &mut *ref1;

        // Use raw pointer to reborrow
        let ptr: *mut _ = &mut *ref2;

        // Borrowed stack: ref2 -> ref1
        // Can't use ref1 before ref2
        // Rust will pop stack until found a reference that a line of code using
        // In this case, ref1 is used, so ref2 is popped
        // Mean try to use ref2 after ref1 will cause panic
        // *ref1 += 1;

        // *ref2 += 1;

        unsafe {
            *ptr += 1;
        }

        *ref2 += 1;
        println!("{}", *ref1);
        assert_eq!(*ref1, 5);

        *ref1 += 1;
    }

    #[test]
    fn test2() {
        unsafe {
            let mut a = 3;
            let ref1 = &mut a;
            // ptr1 is a raw pointer, not a reference
            let ptr1 = ref1 as *mut _;
            let ref2 = &mut *ptr1;
            // ptr2 is a raw pointer, not a reference
            let ptr2 = ref2 as *mut _;

            // In miri perspective Borrow Stack should be like below order:
            // ptr2 -> ref2 -> ptr1 -> ref1
            // *mut -> &mut -> *mut -> &mut

            // Even we access in violated order of Borrow Stack
            // Rust still build program if we put these inside unsafe block
            // *ref1 += 1;
            // *ref2 += 1;
            // *ptr1 += 1;
            // *ptr2 += 1;
            //

            // So if we access in right order of Borrow Stack, miri test will be passed
            // ptr2 -> ref2 -> ptr1 -> ref1
            *ptr2 += 1;
            // ref2 -> ptr1 -> ref1
            *ref2 += 1;
            // ptr1 -> ref1
            *ptr1 += 1;
            // ref1
            *ref1 += 1;

            assert_eq!(a, 7)
        }
    }

    #[test]
    fn test3() {
        unsafe {
            let mut data = [0; 10];
            let ref1_at_0 = &mut data[0];
            let ptr1_at_0 = ref1_at_0 as *mut i32;
            let ref2_at_0 = &mut *ptr1_at_0;
            let ptr2_at_0 = ref2_at_0 as *mut i32;
            // raw_ptr.add() or sub() is offset a size of type that this pointer point to (i32 in this case)
            // offset zero size so it still pointer at the same address
            let ptr3_at_0 = ptr2_at_0.add(0);
            // add 1 i32 size and sub 1 i32 -> no offset -> same as add(0)
            let ptr4_at_0 = ptr3_at_0.add(1).sub(1);

            // Borrow Stack:
            // ptr4_at_0 -> ptr3_at_0 -> ptr2_at_0 -> ref2_at_0 -> ptr1_at_0 -> ref1_at_0
            // Below is valid Borrow Stack access order
            *ptr2_at_0 += 4;
            *ptr3_at_0 += 4;
            *ptr4_at_0 += 4;
            *ref2_at_0 += 4;
            *ptr1_at_0 += 1;
            *ref1_at_0 += 3;
            assert_eq!(data[0], 20);
        }
    }

    #[test]
    fn test4() {
        unsafe {
            let mut data = [0; 10];
            let (slice1, slice2) = data.split_at_mut(1);
            let ref1_at_0 = &mut slice1[0];
            let ref1_at_1 = &mut slice2[0];
            let ptr1_at_0 = ref1_at_0 as *mut i32;
            let ptr1_at_1 = ref1_at_1 as *mut i32;

            *ptr1_at_0 = 10;
            *ref1_at_0 += 10;

            *ptr1_at_1 = 20;
            *ref1_at_1 += 20;

            assert_eq!(data[0], 20);
            assert_eq!(data[1], 40);
        }
    }

    #[test]
    fn test5() {
        unsafe {
            let mut data = [0; 10];
            let slice_all = &mut data[..];
            let ptr_all = slice_all.as_mut_ptr();
            let ptr1_at_0 = ptr_all.add(0);
            let ptr1_at_1 = ptr_all.add(1);
            let ref2_at_0 = &mut *ptr1_at_0;

            *ref2_at_0 += 10;

            for i in 0..slice_all.len() {
                *ptr1_at_0.add(i) += 30;
            }

            *ptr1_at_1 = 20;

            assert_eq!(data[0], 40);
        }
    }

    #[test]
    fn test6() {
        unsafe {
            let mut data = 0;
            // ptr1 is a raw pointer point to reference to data
            // NOTE: this is just personal opinion
            // When explicitly reference to a data instance through raw pointer [not by a reference]
            // Rust create a an anonymous reference and point raw pointer to it, and push anonymous reference to Borrow Stack
            // let anonymous0_mut_ref_data = &mut data;
            // let ptr1 = &mut *anonymous0_mut_ref_data;
            // Borrow Stack: [anonymous0_mut_ref_data]
            let ptr1: *mut _ = &mut data;
            let ptr2 = ptr1;
            let ptr3 = ptr2.add(0);
            // Push ref1 to Borrow Stack
            // Borrow Stack: [anonymous0_mut_ref_data -> ref1]
            let ref1 = &mut *ptr3;

            // But when we explicitly reference to that instance again
            // Rust will empty stack an create new anonymous reference
            // let anonymous1_mut_ref_data = &mut data;
            // let ptr4 = &mut *anonymous1_mut_ref_data;
            // Borrow Stack: [anonymous1_mut_ref_data]
            // let ptr4: *mut _ = &mut data;
            // let ref2 = &mut *ptr4;
            // *ref2 += 10;
            // *ptr4 += 10;
            //

            // Ref1 is popped from Borrow Stack
            *ref1 += 10;
            // Point to anonymous0_mut_ref_data, which popped from Borrow Stack
            *ptr3 += 10;
            *ptr1 += 10;
            *ptr2 += 10;
            //

            assert_eq!(data, 40);
        }
    }

    #[test]
    fn test_shared_references() {
        fn opaque_read(val: &i32) {
            println!("{}", val);
        }

        unsafe {
            let mut data = 10;
            let mut_ref = &mut data;
            let ptr1 = mut_ref as *mut i32;
            // When we immutably reference (aka shared reference) to a mutable reference or raw_pointer
            // Rust push shared reference to Borrow Stack as Shared Read Only Reference
            let shared_ref1 = &*mut_ref;
            // Same as above even we cast to *mut as last, but we cast to *const first
            // So Rust assume that this is a immutable reference (aka shared reference)
            // And push shared reference to Borrow Stack
            let ptr2 = shared_ref1 as *const i32 as *mut i32;
            // Borrow Stack: [ptr2(point to shared reference) -> shared_ref1 -> ptr1 -> mut_ref]

            opaque_read(&*ptr2);
            opaque_read(shared_ref1);
            *ptr1 += 1;
            *mut_ref += 1;

            opaque_read(&data);
        }
    }
}
