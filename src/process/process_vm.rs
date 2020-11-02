use std::{ops, ptr, mem};
use dataview::Pod;
use intptr::IntPtr;
use crate::winapi::*;
use crate::process::Process;
use crate::error::ErrorCode;
use crate::{Result, AsInner, IntoInner, FromInner};

use crate::memory::*;

/// Virtual memory API.
impl Process {
	#[inline]
	unsafe fn vm_read_raw<T: Pod + ?Sized>(&self, ptr: IntPtr<T>, dest: *mut T) -> Result<()> {
		let num_bytes = mem::size_of_val(&*dest);
		let success = ReadProcessMemory(
			*self.as_inner(),
			ptr.into_usize() as LPCVOID,
			dest as LPVOID,
			num_bytes as SIZE_T,
			ptr::null_mut()
		) != FALSE;
		if success {
			Ok(())
		}
		else {
			Err(ErrorCode::last())
		}
	}
	/// Reads as many bytes as are available.
	#[inline]
	pub fn vm_read_partial<'a>(&self, ptr: IntPtr<[u8]>, dest: &'a mut [u8]) -> Result<&'a mut [u8]> {
		let mut bytes_read = 0;
		let num_bytes = mem::size_of_val(dest);
		let success = unsafe {
			ReadProcessMemory(
				*self.as_inner(),
				ptr.into_usize() as LPCVOID,
				dest.as_mut_ptr() as LPVOID,
				num_bytes as SIZE_T,
				&mut bytes_read,
			) != FALSE
		};
		if success {
			Ok(dest)
		}
		else {
			let err = ErrorCode::last();
			if err.into_inner() != ERROR_PARTIAL_COPY {
				Err(err)
			}
			else {
				Ok(unsafe { dest.get_unchecked_mut(..bytes_read as usize) })
			}
		}
	}
	/// Reads a Pod `T` from the process.
	#[inline]
	pub fn vm_read<T: Pod>(&self, ptr: IntPtr<T>) -> Result<T> {
		unsafe {
			let mut dest = mem::MaybeUninit::<T>::uninit();
			self.vm_read_raw(ptr, dest.as_mut_ptr())?;
			Ok(dest.assume_init())
		}
	}
	/// Reads a slice of Pod `T` from the process.
	#[inline]
	pub fn vm_read_into<'a, T: Pod + ?Sized>(&self, ptr: IntPtr<T>, dest: &'a mut T) -> Result<&'a mut T> {
		match unsafe { self.vm_read_raw(ptr, dest) } {
			Ok(_) => Ok(dest),
			Err(err) => Err(err),
		}
	}
	/// Reads a number of Pod `T` and appends the read elements to the given Vec.
	#[inline]
	pub fn vm_read_append<'a, T: Pod>(&self, ptr: IntPtr<[T]>, dest: &'a mut Vec<T>, len: usize) -> Result<&'a mut [T]> {
		let old_len = dest.len();
		let new_len = usize::checked_add(old_len, len).expect("overflow");
		if dest.capacity() < new_len {
			let additional = new_len - dest.capacity();
			dest.reserve(additional);
		}
		// This is unfortunate, it should only `set_len` when memory was successfully read...
		// Because this function returns a mutable slice to the original vector, it's not possible to `set_len` afterwards
		// As that would mean aliasing mutable memory.
		// Bypass all of this by going through a mut pointer.
		unsafe {
			let dest = dest as *mut Vec<T>;
			let dest_slice = (*dest).get_unchecked_mut(old_len..new_len);
			self.vm_read_into(ptr, dest_slice).map(|dest_slice| {
				(*dest).set_len(new_len);
				dest_slice
			})
		}
	}
	/// Writes bytes.
	#[inline]
	pub fn vm_write_bytes(&self, address: IntPtr, bytes: &[u8]) -> Result<()> {
		let mut bytes_written = 0;
		let num_bytes = mem::size_of_val(bytes);
		let success = unsafe {
			WriteProcessMemory(
				*self.as_inner(),
				address.into_usize() as LPVOID,
				bytes.as_ptr() as LPCVOID,
				num_bytes as SIZE_T,
				&mut bytes_written,
			) != FALSE
		};
		if success {
			Ok(())
		}
		else {
			Err(ErrorCode::last())
		}
	}
	/// Writes as many bytes as it can.
	#[inline]
	pub fn vm_write_partial<'a>(&self, address: usize, bytes: &'a [u8]) -> Result<&'a [u8]> {
		let mut bytes_written = 0;
		let num_bytes = mem::size_of_val(bytes);
		let success = unsafe {
			WriteProcessMemory(
				*self.as_inner(),
				address as LPVOID,
				bytes.as_ptr() as LPCVOID,
				num_bytes as SIZE_T,
				&mut bytes_written,
			) != FALSE
		};
		if success {
			Ok(bytes)
		}
		else {
			let err = ErrorCode::last();
			if *err.as_inner() != ERROR_PARTIAL_COPY {
				Err(err)
			}
			else {
				Ok(unsafe { bytes.get_unchecked(..bytes_written as usize) })
			}
		}
	}
	/// Writes the Pod `T` to the process.
	#[inline]
	pub fn vm_write<T: ?Sized + Pod>(&self, ptr: IntPtr<T>, val: &T) -> Result<()> {
		self.vm_write_bytes(ptr.cast(), val.as_bytes())
	}
	/// Writes a sub range of the Pod `T` to the process.
	/// Panics if the range falls outside the bytes of the given value.
	#[inline]
	pub fn vm_write_range<T: Pod>(&self, ptr: IntPtr<T>, val: &T, range: ops::Range<usize>) -> Result<()> {
		let address = IntPtr::from_usize(ptr.into_usize() + range.start);
		let val = &val.as_bytes()[range];
		self.vm_write_bytes(address, val)
	}
	/// Allocates memomry in the process.
	#[inline]
	pub fn vm_alloc(&self, address: IntPtr, len: usize, alloc_type: AllocType, protect: Protect) -> Result<IntPtr> {
		let result = unsafe {
			VirtualAllocEx(
				*self.as_inner(),
				address.into_usize() as LPVOID,
				len as SIZE_T,
				alloc_type.into_inner(),
				protect.into_inner(),
			)
		};
		if !result.is_null() {
			Ok(IntPtr::from_usize(result as usize))
		}
		else {
			Err(ErrorCode::last())
		}
	}
	/// Commits memory in the process.
	#[inline]
	pub fn vm_commit(&self, address: IntPtr, len: usize, protect: Protect) -> Result<IntPtr> {
		self.vm_alloc(address, len, AllocType::COMMIT, protect)
	}
	/// Reserves memory in the process.
	#[inline]
	pub fn vm_reserve(&self, address: IntPtr, len: usize, protect: Protect) -> Result<IntPtr> {
		self.vm_alloc(address, len, AllocType::RESERVE, protect)
	}
	/// Frees memory in the process.
	#[inline]
	pub fn vm_free(&self, address: IntPtr, len: usize, free_type: FreeType) -> Result<()> {
		let success = unsafe {
			VirtualFreeEx(
				*self.as_inner(),
				address.into_usize() as LPVOID,
				len as SIZE_T,
				free_type.into_inner(),
			) != FALSE
		};
		if success {
			Ok(())
		}
		else {
			Err(ErrorCode::last())
		}
	}
	/// Decommits memory in the process.
	#[inline]
	pub fn vm_decommit(&self, address: IntPtr, len: usize) -> Result<()> {
		self.vm_free(address, len, FreeType::DECOMMIT)
	}
	/// Releases memory in the process.
	#[inline]
	pub fn vm_release(&self, address: IntPtr) -> Result<()> {
		self.vm_free(address, 0, FreeType::RELEASE)
	}
	/// Changes memory protection in the process.
	#[inline]
	pub fn vm_protect(&self, address: IntPtr, len: usize, protect: Protect) -> Result<Protect> {
		let mut old = 0;
		let success = unsafe {
			VirtualProtectEx(
				*self.as_inner(),
				address.into_usize() as LPVOID,
				len as SIZE_T,
				protect.into_inner(),
				&mut old,
			) != FALSE
		};
		if success {
			Ok(unsafe { Protect::from_inner(old) })
		}
		else {
			Err(ErrorCode::last())
		}
	}
	/// Queries the state of virtual memory in the process.
	#[inline]
	pub fn vm_query(&self, address: IntPtr) -> Result<MemoryInformation> {
		let size = mem::size_of::<MEMORY_BASIC_INFORMATION>() as SIZE_T;
		unsafe {
			let mut mem_basic_info = mem::MaybeUninit::<MemoryInformation>::uninit();
			if VirtualQueryEx(*self.as_inner(), address.into_usize() as LPCVOID, mem_basic_info.as_mut_ptr() as *mut MEMORY_BASIC_INFORMATION, size) == size {
				Ok(mem_basic_info.assume_init())
			}
			else {
				Err(ErrorCode::last())
			}
		}
	}
	/// Queries the working set ex of virtual memory in the process.
	#[inline]
	pub fn vm_query_ws_ex(&self, address: IntPtr) -> Result<WorkingSetExBlock> {
		let size = mem::size_of::<PSAPI_WORKING_SET_EX_INFORMATION>() as DWORD;
		unsafe {
			let mut buffer: PSAPI_WORKING_SET_EX_INFORMATION = mem::zeroed();
			buffer.VirtualAddress = address.into_usize() as PVOID;
			if K32QueryWorkingSetEx(*self.as_inner(), &mut buffer as *mut _ as PVOID, size) != 0 {
				Ok(WorkingSetExBlock::from(buffer.VirtualAttributes))
			}
			else {
				Err(ErrorCode::last())
			}
		}
	}
	/// Iterator over vm_query starting from the given base address.
	#[inline]
	pub fn vm_regions(&self, start_address: IntPtr) -> impl '_ + Clone + Iterator<Item = MemoryInformation> {
		let mut start_address = start_address;
		std::iter::from_fn(move || {
			let mi = self.vm_query(start_address).ok()?;
			start_address = IntPtr::from_usize((mi.BaseAddress as usize).wrapping_add(mi.RegionSize));
			Some(mi)
		})
	}
	/// Iterator returning all unique allocation bases from vm_query.
	pub fn vm_allocations(&self) -> impl '_ + Clone + Iterator<Item = (IntPtr, Protect, MemoryType)> {
		let mut allocation_base = ptr::null_mut();
		let mut address = IntPtr::NULL;
		std::iter::from_fn(move || {
			let allocation_protect;
			let memory_type;
			loop {
				let mi = self.vm_query(address).ok()?;
				if mi.AllocationBase != ptr::null_mut() && mi.AllocationBase != allocation_base {
					allocation_base = mi.AllocationBase;
					allocation_protect = unsafe { Protect::from_inner(mi.AllocationProtect) };
					memory_type = unsafe { MemoryType::from_inner(mi.Type) };
					break;
				}
				address = IntPtr::from_usize((mi.BaseAddress as usize).wrapping_add(mi.RegionSize));
			}
			Some((IntPtr::from_usize(allocation_base as usize), allocation_protect, memory_type))
		})
	}
}
