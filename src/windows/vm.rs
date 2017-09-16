/*!
Virtual memory interaction with a process.
*/

use std::{ptr, mem};
use std::ops::Range;

use kernel32::{ReadProcessMemory, WriteProcessMemory, VirtualAllocEx, VirtualFreeEx, VirtualQueryEx};
use winapi::{FALSE, BOOL, DWORD, SIZE_T, LPVOID, LPCVOID, HANDLE, MEMORY_BASIC_INFORMATION, ERROR_PARTIAL_COPY};
use winapi::{PAGE_EXECUTE, PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE, PAGE_EXECUTE_WRITECOPY, PAGE_NOACCESS,
	PAGE_READONLY, PAGE_READWRITE, PAGE_WRITECOPY, PAGE_GUARD};
use winapi::{MEM_COMMIT, MEM_RESERVE, MEM_RESET, MEM_RESET_UNDO, MEM_DECOMMIT, MEM_RELEASE};

use process::Process;
use error::ErrorCode;
use ptr::{Pod, RawPtr, TypePtr};
use {Result, AsInner, IntoInner, FromInner};

//----------------------------------------------------------------

extern "system" {
	// Broken definition in kernel32 crate.
	fn VirtualProtectEx(hProcess: HANDLE, lpAddress: LPVOID, dwSize: SIZE_T, flNewProtect: DWORD, lpflOldProtect: &mut DWORD) -> BOOL;
}

//----------------------------------------------------------------

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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FreeType(u32);
impl_inner!(FreeType: u32);
impl FreeType {
	pub const DECOMMIT: FreeType = FreeType(MEM_DECOMMIT);
	pub const RELEASE: FreeType = FreeType(MEM_RELEASE);
}

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

//----------------------------------------------------------------

/// Virtual memory API.
pub trait VirtualMemory {
	/// Reads bytes.
	fn vm_read_bytes<'a>(&self, ptr: RawPtr, bytes: &'a mut [u8]) -> Result<&'a mut [u8]>;
	/// Reads as many bytes as are available.
	fn vm_read_partial<'a>(&self, ptr: RawPtr, dest: &'a mut [u8]) -> Result<&'a mut [u8]>;
	/// Writes bytes.
	fn vm_write_bytes(&self, ptr: RawPtr, bytes: &[u8]) -> Result<()>;
	/// Writes as many bytes as it can.
	fn vm_write_partial<'a>(&self, ptr: RawPtr, bytes: &'a [u8]) -> Result<&'a [u8]>;
	fn vm_alloc(&self, ptr: RawPtr, len: usize, alloc_type: AllocType, protect: Protect) -> Result<RawPtr>;
	fn vm_free(&self, ptr: RawPtr, len: usize, free_type: FreeType) -> Result<()>;
	fn vm_protect(&self, ptr: RawPtr, len: usize, protect: Protect) -> Result<Protect>;
	fn vm_query(&self, ptr: RawPtr) -> Result<MemoryInformation>;

	#[inline]
	fn vm_read<T: Pod>(&self, ptr: TypePtr<T>) -> Result<T> where Self: Sized {
		let mut dest: T = unsafe { mem::uninitialized() };
		match self.vm_read_bytes(ptr.into(), dest.as_bytes_mut()) {
			Ok(_) => Ok(dest),
			Err(err) => Err(err),
		}
	}
	#[inline]
	fn vm_read_slice<'a, T: Pod>(&self, ptr: TypePtr<[T]>, dest: &'a mut [T]) -> Result<&'a mut [T]> where Self: Sized {
		match self.vm_read_bytes(ptr.into(), dest.as_bytes_mut()) {
			Ok(_) => Ok(dest),
			Err(err) => Err(err),
		}
	}
	#[inline]
	fn vm_read_vec<'a, T: Pod>(&self, ptr: TypePtr<[T]>, dest: &'a mut Vec<T>, len: usize) -> Result<&'a mut [T]> where Self: Sized {
		let old_len = dest.len();
		let new_len = old_len + len;
		if dest.capacity() < new_len {
			let additional = new_len - dest.capacity();
			dest.reserve(additional);
		}
		let dest_slice = unsafe {
			// This is unfortunate, it should only `set_len` when memory was successfully read...
			// Because this function returns a mutable slice to the original vector, it's not possible to `set_len` afterwards
			// As that would mean aliasing mutable memory.
			dest.set_len(new_len);
			dest.get_unchecked_mut(old_len..new_len)
		};
		self.vm_read_slice(ptr, dest_slice)
	}
	#[inline]
	fn vm_write<T: ?Sized + Pod>(&self, ptr: TypePtr<T>, val: &T) -> Result<()> where Self: Sized {
		self.vm_write_bytes(ptr.into(), val.as_bytes())
	}
	#[inline]
	fn vm_write_range<T: Pod>(&self, ptr: TypePtr<T>, val: &T, range: Range<usize>) -> Result<()> where Self: Sized {
		let ptr: RawPtr = ptr.into();
		let ptr = ptr + range.start;
		let val = &val.as_bytes()[range];
		self.vm_write_bytes(ptr, val)
	}
	#[inline]
	fn vm_commit(&self, ptr: RawPtr, len: usize, protect: Protect) -> Result<RawPtr> where Self: Sized {
		self.vm_alloc(ptr, len, AllocType::COMMIT, protect)
	}
	#[inline]
	fn vm_reserve(&self, ptr: RawPtr, len: usize, protect: Protect) -> Result<RawPtr> where Self: Sized {
		self.vm_alloc(ptr, len, AllocType::RESERVE, protect)
	}
	#[inline]
	fn vm_decommit(&self, ptr: RawPtr, len: usize) -> Result<()> where Self: Sized {
		self.vm_free(ptr, len, FreeType::DECOMMIT)
	}
	#[inline]
	fn vm_release(&self, ptr: RawPtr) -> Result<()> where Self: Sized {
		self.vm_free(ptr, 0, FreeType::RELEASE)
	}
}

impl VirtualMemory for Process {
	#[inline]
	fn vm_read_bytes<'a>(&self, ptr: RawPtr, bytes: &'a mut [u8]) -> Result<&'a mut [u8]> {
		let ptr: usize = ptr.into();
		let num_bytes = mem::size_of_val(bytes);
		let success = unsafe {
			ReadProcessMemory(
				*self.as_inner(),
				ptr as LPCVOID,
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
	#[inline]
	fn vm_read_partial<'a>(&self, ptr: RawPtr, dest: &'a mut [u8]) -> Result<&'a mut [u8]> {
		let ptr: usize = ptr.into();
		let mut bytes_read = 0;
		let num_bytes = mem::size_of_val(dest);
		let success = unsafe {
			ReadProcessMemory(
				*self.as_inner(),
				ptr as LPCVOID,
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
	#[inline]
	fn vm_write_bytes(&self, ptr: RawPtr, bytes: &[u8]) -> Result<()> {
		let ptr: usize = ptr.into();
		let mut bytes_written = 0;
		let num_bytes = mem::size_of_val(bytes);
		let success = unsafe {
			WriteProcessMemory(
				*self.as_inner(),
				ptr as LPVOID,
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
	#[inline]
	fn vm_write_partial<'a>(&self, ptr: RawPtr, bytes: &'a [u8]) -> Result<&'a [u8]> {
		let ptr: usize = ptr.into();
		let mut bytes_written = 0;
		let num_bytes = mem::size_of_val(bytes);
		let success = unsafe {
			WriteProcessMemory(
				*self.as_inner(),
				ptr as LPVOID,
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
	#[inline]
	fn vm_alloc(&self, ptr: RawPtr, len: usize, alloc_type: AllocType, protect: Protect) -> Result<RawPtr> {
		let ptr: usize = ptr.into();
		let result = unsafe {
			VirtualAllocEx(
				*self.as_inner(),
				ptr as LPVOID,
				len as SIZE_T,
				alloc_type.into_inner(),
				protect.into_inner(),
			)
		};
		if !result.is_null() {
			Ok(RawPtr::from(result as usize))
		}
		else {
			Err(ErrorCode::last())
		}
	}
	#[inline]
	fn vm_free(&self, ptr: RawPtr, len: usize, free_type: FreeType) -> Result<()> {
		let ptr: usize = ptr.into();
		let success = unsafe {
			VirtualFreeEx(
				*self.as_inner(),
				ptr as LPVOID,
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
	#[inline]
	fn vm_protect(&self, ptr: RawPtr, len: usize, protect: Protect) -> Result<Protect> {
		let ptr: usize = ptr.into();
		let mut old = 0;
		let success = unsafe {
			VirtualProtectEx(
				*self.as_inner(),
				ptr as LPVOID,
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
	#[inline]
	fn vm_query(&self, ptr: RawPtr) -> Result<MemoryInformation> {
		let ptr: usize = ptr.into();
		let size = mem::size_of::<MEMORY_BASIC_INFORMATION>() as SIZE_T;
		unsafe {
			let mut mem_basic_info: MemoryInformation = mem::uninitialized();
			if VirtualQueryEx(*self.as_inner(), ptr as LPCVOID, &mut mem_basic_info.0, size) != size {
				Ok(mem_basic_info)
			}
			else {
				Err(ErrorCode::last())
			}
		}
	}
}
