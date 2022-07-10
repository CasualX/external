use external::prelude::*;

#[test]
fn test_query_working_set_ex() {
	let process = Process::current();
	let page = process.vm_alloc(IntPtr::NULL, 1, AllocType::COMMIT, Protect::READWRITE).unwrap();

	let _ = dbg!(process.vm_query_ws_ex(IntPtr::from_usize(test_query_working_set_ex as usize)));
	let _ = dbg!(process.vm_query_ws_ex(IntPtr::from_usize(&process as *const _ as usize)));

	let _ = dbg!(process.vm_query_ws_ex(page));
	unsafe { *(page.into_usize() as *mut u8) = 1; }
	let _ = dbg!(process.vm_query_ws_ex(page));
}

#[test]
fn test_vm_allocations() {
	let process = Process::current();
	let mut buffer = [0; 256];
	println!();
	for (address, _, ty) in process.vm_allocations() {
		if ty == MemoryType::IMAGE {
			let s = match process.get_mapped_file_name_wide(address, &mut buffer) {
				Ok(s) => String::from_utf16_lossy(s),
				Err(err) => err.to_string(),
			};
			println!("{:#x} {}", address, s);
		}
	}
}
