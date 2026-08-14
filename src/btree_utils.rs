
use core::mem::size_of;

const B: usize = 6;
const CAPACITY: usize = 2 * B - 1;
const MIN_LEN_AFTER_SPLIT: usize = B - 1;

pub fn mem_size_of_btree<T>(len: u32) -> usize { panic!("STUB: not implemented") }

#[cfg(test)]
#[cfg(not(miri))]
#[rustversion::nightly]
mod test {
	use super::*;
	use crate::alloc::{
		collections::{BTreeMap, BTreeSet},
		sync::{Arc, Mutex},
	};
	use core::{
		alloc::{AllocError, Allocator, Layout},
		ptr::NonNull,
	};

	#[derive(Clone)]
	struct MockAllocator {
		total: Arc<Mutex<usize>>,
	}

	unsafe impl Allocator for MockAllocator {
		fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
			let ptr = std::alloc::System.allocate(layout);
			if ptr.is_ok() {
				*self.total.lock().unwrap() += layout.size();
			}
			ptr
		}

		unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
			*self.total.lock().unwrap() -= layout.size();
			std::alloc::System.deallocate(ptr, layout)
		}
	}

	fn check_btree_size(expected_size: usize, actual_size: Arc<Mutex<usize>>) {
		
		assert!(*actual_size.lock().unwrap() as f64 * 0.75 <= expected_size as f64);
		assert!(*actual_size.lock().unwrap() as f64 * 1.25 >= expected_size as f64);
	}

	#[test]
	fn mem_size_of_btree_works() {
		let map_allocator = MockAllocator { total: Arc::new(Mutex::new(0)) };
		let map_actual_size = map_allocator.total.clone();
		let mut map = BTreeMap::<u32, u32, MockAllocator>::new_in(map_allocator);

		let set_allocator = MockAllocator { total: Arc::new(Mutex::new(0)) };
		let set_actual_size = set_allocator.total.clone();
		let mut set = BTreeSet::<u128, MockAllocator>::new_in(set_allocator);

		for i in 0..1000000 {
			map.insert(i, 0);
			set.insert(i as u128);

			if i > 100 {
				let map_expected_size = mem_size_of_btree::<(u32, u32)>(map.len() as u32);
				check_btree_size(map_expected_size, map_actual_size.clone());

				let set_expected_size = mem_size_of_btree::<u128>(set.len() as u32);
				check_btree_size(set_expected_size, set_actual_size.clone());
			}
		}
	}
}
