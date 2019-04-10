/*!
Virtual memory interaction with a process.
!*/

use std::{ops, ptr, mem};
use crate::winapi::*;
use crate::process::Process;
use crate::error::ErrorCode;
use crate::ptr::{Pod, Ptr};
use crate::{Result, AsInner, IntoInner, FromInner};

//----------------------------------------------------------------

/// Memory protection type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Protect(u32);
impl_inner!(Protect: u32);
impl Protect {
	pub const EXECUTE: Protect = Protect(PAGE_EXECUTE);
	pub const EXECUTE_READ: Protect = Protect(PAGE_EXECUTE_READ);
	pub const EXECUTE_READ_WRITE: Protect = Protect(PAGE_EXECUTE_READWRITE);
	pub const NO_ACCESS: Protect = Protect(PAGE_NOACCESS);
	pub const READ_ONLY: Protect = Protect(PAGE_READONLY);
	pub const READ_WRITE: Protect = Protect(PAGE_READWRITE);
	pub fn is_executable(self) -> bool {
		self.0 & (PAGE_EXECUTE | PAGE_EXECUTE_READ | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY) != 0
	}
	pub fn is_readable(self) -> bool {
		self.0 & (PAGE_EXECUTE_READ | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY | PAGE_READONLY | PAGE_READWRITE | PAGE_WRITECOPY) != 0
	}
	pub fn is_writable(self) -> bool {
		self.0 & (PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY | PAGE_READWRITE | PAGE_WRITECOPY) != 0
	}
	pub fn has_guard(self) -> bool {
		self.0 & (PAGE_GUARD) != 0
	}
	pub fn set_guard(self, value: bool) -> Protect {
		if value {
			Protect(self.0 | PAGE_GUARD)
		}
		else {
			Protect(self.0 & !PAGE_GUARD)
		}
	}
}

//----------------------------------------------------------------

/// Free type for virtual memory.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FreeType(u32);
impl_inner!(FreeType: u32);
impl FreeType {
	pub const DECOMMIT: FreeType = FreeType(MEM_DECOMMIT);
	pub const RELEASE: FreeType = FreeType(MEM_RELEASE);
}

/// Allocation type for virtual memory.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AllocType(u32);
impl_inner!(AllocType: u32);
impl AllocType {
	pub const COMMIT: AllocType = AllocType(MEM_COMMIT);
	pub const RESERVE: AllocType = AllocType(MEM_RESERVE);
	pub const RESET: AllocType = AllocType(MEM_RESET);
	pub const RESET_UNDO: AllocType = AllocType(MEM_RESET_UNDO);
}

pub struct MemoryInformation(MEMORY_BASIC_INFORMATION);
impl_inner!(MemoryInformation: MEMORY_BASIC_INFORMATION);
impl ops::Deref for MemoryInformation {
	type Target = MEMORY_BASIC_INFORMATION;
	fn deref(&self) -> &MEMORY_BASIC_INFORMATION {
		&self.0
	}
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct ForEachAllocation {
	pub allocation_base: usize,
	pub allocation_size: usize,
	pub allocation_protect: u32,
	pub regions_state: u32,
	pub regions_protect: u32,
	pub regions_type: u32,
}

//----------------------------------------------------------------

/// Virtual memory API.
impl Process {
	/// Reads bytes into the destination buffer.
	#[inline]
	pub fn vm_read_bytes<'a>(&self, address: usize, bytes: &'a mut [u8]) -> Result<&'a mut [u8]> {
		let num_bytes = mem::size_of_val(bytes);
		let success = unsafe {
			ReadProcessMemory(
				*self.as_inner(),
				address as LPCVOID,
				bytes.as_mut_ptr() as LPVOID,
				num_bytes as SIZE_T,
				ptr::null_mut(),
			) != FALSE
		};
		if success {
			Ok(bytes)
		}
		else {
			Err(ErrorCode::last())
		}
	}
	/// Reads as many bytes as are available.
	#[inline]
	pub fn vm_read_partial<'a>(&self, address: usize, dest: &'a mut [u8]) -> Result<&'a mut [u8]> {
		let mut bytes_read = 0;
		let num_bytes = mem::size_of_val(dest);
		let success = unsafe {
			ReadProcessMemory(
				*self.as_inner(),
				address as LPCVOID,
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
	pub fn vm_read<T: Pod>(&self, ptr: Ptr<T>) -> Result<T> {
		let address = ptr.into_raw() as usize;
		let mut dest: T = unsafe { mem::uninitialized() };
		match self.vm_read_bytes(address, dest.as_bytes_mut()) {
			Ok(_) => Ok(dest),
			Err(err) => {
				mem::forget(dest);
				Err(err)
			},
		}
	}
	/// Reads a slice of Pod `T` from the process.
	#[inline]
	pub fn vm_read_into<'a, T: Pod + ?Sized>(&self, ptr: Ptr<T>, dest: &'a mut T) -> Result<&'a mut T> {
		let address = ptr.into_raw() as usize;
		match self.vm_read_bytes(address, dest.as_bytes_mut()) {
			Ok(_) => Ok(dest),
			Err(err) => Err(err),
		}
	}
	/// Reads a number of Pod `T` and appends the read elements to the given Vec.
	#[inline]
	pub fn vm_read_append<'a, T: Pod>(&self, ptr: Ptr<[T]>, dest: &'a mut Vec<T>, len: usize) -> Result<&'a mut [T]> {
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
	pub fn vm_write_bytes(&self, address: usize, bytes: &[u8]) -> Result<()> {
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
	pub fn vm_write<T: ?Sized + Pod>(&self, ptr: Ptr<T>, val: &T) -> Result<()> {
		let address = ptr.into_raw() as usize;
		self.vm_write_bytes(address, val.as_bytes())
	}
	/// Writes a sub range of the Pod `T` to the process.
	/// Panics if the range falls outside the bytes of the given value.
	#[inline]
	pub fn vm_write_range<T: Pod>(&self, ptr: Ptr<T>, val: &T, range: ops::Range<usize>) -> Result<()> {
		let address = ptr.into_raw() as usize + range.start;
		let val = &val.as_bytes()[range];
		self.vm_write_bytes(address, val)
	}
	/// Allocates memomry in the process.
	#[inline]
	pub fn vm_alloc(&self, address: usize, len: usize, alloc_type: AllocType, protect: Protect) -> Result<usize> {
		let result = unsafe {
			VirtualAllocEx(
				*self.as_inner(),
				address as LPVOID,
				len as SIZE_T,
				alloc_type.into_inner(),
				protect.into_inner(),
			)
		};
		if !result.is_null() {
			Ok(result as usize)
		}
		else {
			Err(ErrorCode::last())
		}
	}
	/// Commits memory in the process.
	#[inline]
	pub fn vm_commit(&self, address: usize, len: usize, protect: Protect) -> Result<usize> {
		self.vm_alloc(address, len, AllocType::COMMIT, protect)
	}
	/// Reserves memory in the process.
	#[inline]
	pub fn vm_reserve(&self, address: usize, len: usize, protect: Protect) -> Result<usize> {
		self.vm_alloc(address, len, AllocType::RESERVE, protect)
	}
	/// Frees memory in the process.
	#[inline]
	pub fn vm_free(&self, address: usize, len: usize, free_type: FreeType) -> Result<()> {
		let success = unsafe {
			VirtualFreeEx(
				*self.as_inner(),
				address as LPVOID,
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
	pub fn vm_decommit(&self, address: usize, len: usize) -> Result<()> {
		self.vm_free(address, len, FreeType::DECOMMIT)
	}
	/// Releases memory in the process.
	#[inline]
	pub fn vm_release(&self, address: usize) -> Result<()> {
		self.vm_free(address, 0, FreeType::RELEASE)
	}
	/// Changes memory protection in the process.
	#[inline]
	pub fn vm_protect(&self, address: usize, len: usize, protect: Protect) -> Result<Protect> {
		let mut old = 0;
		let success = unsafe {
			VirtualProtectEx(
				*self.as_inner(),
				address as LPVOID,
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
	pub fn vm_query(&self, address: usize) -> Result<MemoryInformation> {
		let size = mem::size_of::<MEMORY_BASIC_INFORMATION>() as SIZE_T;
		unsafe {
			let mut mem_basic_info: MemoryInformation = mem::uninitialized();
			if VirtualQueryEx(*self.as_inner(), address as LPCVOID, &mut mem_basic_info.0, size) == size {
				Ok(mem_basic_info)
			}
			else {
				Err(ErrorCode::last())
			}
		}
	}
	/// Queries the working set ex of virtual memory in the process.
	#[inline]
	pub fn vm_query_ws_ex(&self, address: usize) -> Result<PSAPI_WORKING_SET_EX_BLOCK> {
		let size = mem::size_of::<PSAPI_WORKING_SET_EX_INFORMATION>() as DWORD;
		unsafe {
			let mut buffer: PSAPI_WORKING_SET_EX_INFORMATION = mem::zeroed();
			buffer.VirtualAddress = address as PVOID;
			if K32QueryWorkingSetEx(*self.as_inner(), &mut buffer as *mut _ as PVOID, size) != 0 {
				Ok(buffer.VirtualAttributes)
			}
			else {
				Err(ErrorCode::last())
			}
		}
	}
	/// Foreach virtual memory region in the specified address range.
	#[inline]
	pub fn vm_regions<F: FnMut(&MemoryInformation)>(&self, mut base_address: usize, size: usize, mut f: F) -> Result<()> {
		let end_address = base_address + size;
		while base_address < end_address {
			let mi = self.vm_query(base_address)?;
			f(&mi);
			base_address += mi.RegionSize;
		}
		Ok(())
	}
	/// Foreach virtual memory allocation and associated mapped filename.
	#[inline]
	pub fn vm_allocations<F: FnMut(&ForEachAllocation, Option<&[u16]>)>(&self, mut f: F) -> Result<()> {
		let mut base_address = 0;
		let mut allocation_base = 0;
		let mut allocation_size = 0;
		let mut allocation_protect = 0;
		let mut regions_state = 0;
		let mut regions_protect = 0;
		let mut regions_type = 0;
		let mut mapped_file_name = vec![0u16; 0x200];
		loop {
			let mi = self.vm_query(base_address)?;
			if mi.AllocationBase as usize != allocation_base {
				let path = self.get_mapped_file_name_wide(allocation_base, &mut mapped_file_name)
					.ok().map(|path| &*path);
				f(&ForEachAllocation {
					allocation_base,
					allocation_size,
					allocation_protect,
					regions_state,
					regions_protect,
					regions_type,
				}, path);
				allocation_base = mi.AllocationBase as usize;
				allocation_size = 0;
				allocation_protect = mi.AllocationProtect;
				regions_state = 0;
				regions_protect = 0;
				regions_type = 0;
			}
			allocation_size += mi.RegionSize;
			regions_state |= mi.State;
			regions_protect |= mi.Protect;
			regions_type |= mi.Type;
			base_address += mi.RegionSize;
		}
	}
}
