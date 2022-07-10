
#[repr(C)]
pub struct ExecutionContext {
	/// Where to write the final return address.
	pub ret_addr_slot: u32,
	pub rax: u64,
	pub rcx: u64,
	pub rdx: u64,
	pub rbx: u64,
	// pub rsp: u64,
	pub rbp: u64,
	pub rsi: u64,
	pub rdi: u64,
	pub r8: u64,
	pub r9: u64,
	pub r10: u64,
	pub r11: u64,
	pub r12: u64,
	pub r13: u64,
	pub r14: u64,
	pub r15: u64,

	pub xmm0: [f32; 4],
	pub xmm1: [f32; 4],
	pub xmm2: [f32; 4],
	pub xmm3: [f32; 4],
	pub xmm4: [f32; 4],
	pub xmm5: [f32; 4],
	pub xmm6: [f32; 4],
	pub xmm7: [f32; 4],
	pub xmm8: [f32; 4],
	pub xmm9: [f32; 4],
	pub xmm10: [f32; 4],
	pub xmm11: [f32; 4],
	pub xmm12: [f32; 4],
	pub xmm13: [f32; 4],
	pub xmm14: [f32; 4],
	pub xmm15: [f32; 4],
}

pub unsafe fn execute(address: u64, ctx: &mut ExecutionContext, stack: &mut [u64]) {

}
