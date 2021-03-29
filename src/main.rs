use std::io;
use std::char;

fn main() {
    let input = input_taker(); // in tuple code word and message
    let answer = encoder(input); 
    println!("{}", answer);
}

fn input_taker() -> (String, String) {
	println!("Input message");
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    let message = buffer.trim().to_string();
    println!("Input code");
    buffer = String::new();
    stdin = io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    let code = buffer.trim().to_string();
    return(message,code);
}

fn encoder_constructor(code: Vec<char>, length: usize) -> Vec<char> {
	let mut encode: Vec<char> = Vec::new();
    if code.len() < length {
	    let mut position = 0;
	    let mut end = 0;
		loop {
			encode.push(code[position]);
			position += 1;
			end +=1;
			if position == code.len(){
				position = 0
			}
		    if end == length {
				break
			}
		}
	    return(encode);
	
	}else {
		return(code);
	}
}

fn encoder(a: (String, String)) -> String {
	let message: Vec<char> = filter((a.0).chars().collect());
	let code: Vec<char> = filter((a.1).chars().collect());
	let length = message.len();
	let encode = encoder_constructor(code,length);
	let mut encode_vec: Vec<char> = Vec::new();
	let mut position = 0; 
    for charecter in message {
		let mut encode_char = 0;
		let mut code_value = 0;
		if (encode[position]).is_ascii_uppercase() {
			code_value = encode[position] as u32 - 'A' as u32;
		}else if (encode[position]).is_ascii_whitespace(){
			code_value = 0
		}else{
			code_value = encode[position] as u32 - 'a' as u32;
		}
		if charecter.is_ascii_uppercase() {
		    if charecter as u32 + code_value > 'Z' as u32 {
				encode_char = charecter as u32 + code_value -( 'Z' as u32 + 1) + 'A' as u32;
			}else{ 
				encode_char = charecter as u32 + code_value;
			}
		}else if charecter.is_ascii_whitespace() {
			encode_char = ' ' as u32;
	    }else{
		    if charecter as u32 + code_value > 'z' as u32 {
				encode_char = charecter as u32 + code_value -( 'z' as u32 + 1) + 'a' as u32;
			}else{ 
				encode_char = charecter as u32 + code_value;
			}
		}
			 
		unsafe {encode_vec.push(char::from_u32_unchecked(encode_char));}
		position += 1;
	}
	
	let encode_message = encode_vec.into_iter().collect();
	
	return(encode_message);
}

fn filter(list:Vec<char>) -> Vec<char> {
	let mut returning_list = Vec::new();
	for charecter in list {
		if charecter.is_ascii_alphabetic() || charecter.is_ascii_whitespace() {  
			returning_list.push(charecter)
		}
	}
	return(returning_list);
}
