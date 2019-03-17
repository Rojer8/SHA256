use std::env;
use std::mem;
use std::vec;
use std::result;
use std::ops;
use std::io;
use std::borrow;

mod GLOBAL_CONST;

#[derive(Debug)]
struct Block<'a>{
	byte : &'a [u8]
}

/*
fn input_args<T>(arg : &[T]) -> u8
   where T: Sized
{
	const size_of_block : u8 = 16;
	let l_var = std::mem::transmute::<usize, u8>(mem::size_of_val(&arg));
	let init_size  =  Some(l_var);

	let init_block = match init_size {
		Some(expr) => if expr > size_of_block || expr < size_of_block {
			let local_expr = size_of_block - expr % size_of_block;
			while local_expr < size_of_block {
				arg[local_expr] = 0;
			}

			Block {
		        byte : Some(arg)
		    }

		},
		None => Block {
			byte : None
		}
	};	

}
*/

/*
fn input_args<'a>(arg : &'a str) -> Vec<Block<'a>>
{
	let mut input = String::new();
	let mut new_arg : &str;

	let mut ret_vec : Vec<Block> = Vec::new();
	let mut length = arg.len();

	while arg.is_empty() {
		println!("You hadn't wrote everything. /r Write again.");
		std::io::stdin().read_line(&mut input);

		new_arg = &input[..];
		length = new_arg.len();

	}

	let mut start_index = 1;

	while length > GLOBAL_CONST::size_of_block  {

		if GLOBAL_CONST::size_of_block + start_index > length {
			//let len : usize = GLOBAL_CONST::size_of_block - (length - start_index);
			//let adds_null = ['0'; &len];

			let block = Block {
			byte : &arg[start_index..length]
		};

		ret_vec.push(block);
	}

		let block = Block {
			byte : &arg[start_index..GLOBAL_CONST::size_of_block + start_index]
		};

		ret_vec.push(block);

		start_index += GLOBAL_CONST::size_of_block;
	}

	ret_vec
}
*/

fn work_with_blocks(_arg : &[u8]) -> Vec<Block>
{
	let _len = _arg.len();
	let mut index = 1; 
	let mut ret_arr : Vec<Block>= Vec::new();

	while index + GLOBAL_CONST::size_of_block < _len {
		let block = Block{
			byte: &_arg[index..index + GLOBAL_CONST::size_of_block]
		};
		ret_arr.push(block);
		index += GLOBAL_CONST::size_of_block;

		if index + GLOBAL_CONST::size_of_block > _len {
		let block = Block {
			byte: &_arg[index.._len]
		};

 		ret_arr.push(block);
		break;
	}
}

	ret_arr
}

fn ret_null_arr<'a>(_arg_null : &usize, _arg_byte : &usize) -> Block<'a>
{

	let ret_block = Block{};
	ret_block
}

fn check_blocks<'a>(_arg : &'a Vec<Block>) -> Vec<Block<'a>>
{
	let mut ret_arr : Vec<Block>= Vec::new();
	for block in _arg {
		if block.byte.len() != GLOBAL_CONST::size_of_block{
			let ret_block = ret_null_arr(GLOBAL_CONST::size_of_block - (block.byte.len() % GLOBAL_CONST::size_of_block), block.byte.len());
			std::mem::swap
		}
	}
	ret_arr
}


fn main()
{
	let mut s_input = String::new();
	println!("Start SHA256 executing");
	println!("Write string");
	std::io::stdin().read_line(&mut s_input);

	while s_input.is_empty() {
		println!("You hadn't wrote everything. /r Write again.");
		std::io::stdin().read_line(&mut s_input);
	}

	let vec_of_u8 = s_input.as_bytes();
	work_with_blocks(&vec_of_u8);


}