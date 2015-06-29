extern crate symbolic_polynomials;

use symbolic_polynomials::SymPolynomial;

#[test]
/// In this test we will work with 2 symbolic variables. 
/// In the comments I will refer to them as a and b
/// in same order, e.g. a-0,b-1
pub fn example_test(){
	let a = &SymPolynomial::get_first_order(0,2);
	let b = &SymPolynomial::get_first_order(1,2);
	let a_plus_b = &(a + b);
	let a_square = &(a*a);
	let b_square = &(b*b);
	let a_b = &(a*b);
	let a_plus_b_square = &(&(a_square + b_square) + &(2*a_b));
	let a_plus_b_2 = a_plus_b_square / a_plus_b;
	assert!(a_plus_b_2.is_some());
	assert_eq!(a_plus_b_2.unwrap(), *a_plus_b);
	// a^3  - 1
	let a_3 = &(&(a * a) * a) - 1;
	let a_minus_1 = a - 1;
	let a_res = &a_3 / &a_minus_1;
	assert!(a_res.is_some());
	assert_eq!(format!("{}", a_res.unwrap()), "X₀²+X₀+1");	
	let a_3 = &a_3 + 2;
	let fail = &a_3 / &a_minus_1;
	assert!(fail.is_none());
}
