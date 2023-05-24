/
* @file rusty.q
* @overview playground for me to test out the Rust API.
\



//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                           Inital Setting     			            //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

// Fetch shared object from build directory.
system "cp ../target/debug/librusty_api_examples.so .";

// Load test helper functions.
\l test_helper_function.q

// Function to load shared library.
LIBPATH_: `librusty_api_examples 2:

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          	Load Libraries     			            //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//


// These function list can be checked against `nm -D librusty_api_examples.so | awk '$2 ~/T/ {print $3}'`.

// get_str
.api.print_string: LIBPATH_ (`print_string; 1);
// get_string
.api.print_string2: LIBPATH_ (`print_string2; 1);
// get_char
.api.print_char: LIBPATH_ (`print_char; 1);
// as_mut_slice
.api.modify_long_list_a_bit: LIBPATH_ (`modify_long_list_a_bit; 1);
// push_symbol
.api.create_symbol_list2: LIBPATH_ (`create_symbol_list2; 1);
// push
.api.create_compound_list2: LIBPATH_ (`create_compound_list2; 1);
// append
.api.concat_list2: LIBPATH_ (`concat_list2; 2);
// get_symbol
.api.print_symbol2: LIBPATH_ (`print_symbol2; 1);
// print
.api.print: LIBPATH_ (`print; 1);
// simple_to_compound
.api.drift: LIBPATH_ (`drift; 1);
// simple_to_compound
.api.drift2: LIBPATH_ (`drift2; 1);

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          	  Tests    	        		                  //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//


// get_char
.test.ASSERT_EQ["get_char"; .api.print_char["k"]; (::)]
// get_char - error
.test.ASSERT_ERROR["get_char - failure1"; .api.print_char; enlist "devour"; "not a char"]
// get_char - error
.test.ASSERT_ERROR["get_char - failure2"; .api.print_char; enlist 1b; "not a char"]
// get_str
.test.ASSERT_EQ["get_str"; .api.print_string["gnat"]; (::)]
// get_string
.test.ASSERT_EQ["get_string"; .api.print_string2["grasshopper"]; (::)]
// get_string - error
.test.ASSERT_ERROR["get_string - failure"; .api.print_string2; enlist (1 2; `a`b); "not a string"]
// as_mut_slice
// Assign to a variable to keep the result.
.test.ASSERT_EQ["as_mut_slice - success"; .api.modify_long_list_a_bit[list:1 2 3]; 1 30000 3]
// as_mut_slice (return error)
.test.ASSERT_ERROR["as_mut_slice - failure"; .api.modify_long_list_a_bit; enlist enlist 1; "this list is not long enough"]

// get_symbol
.test.ASSERT_EQ["get_symbol"; .api.print_symbol2[`locust]; (::)]
// get_symool - error
.test.ASSERT_ERROR["get_symbol - failure"; .api.print_symbol2; enlist "attack!"; "not a symbol"]

//// can we make simple lists w/o seg faulting
// push_symbol
.test.ASSERT_EQ["push_symbol"; .api.create_symbol_list2[]; `Abraham`Isaac`Jacob`Joseph]
//// can we make compound lists w/o seg faulting
// push
.test.ASSERT_EQ["push"; .api.create_compound_list2[5i]; (til 5), 5i]
//// can we merge lists w/o seg faulting
// append
.test.ASSERT_EQ["append - compound"; .api.concat_list2[(::; `metals; `fire); ("clay"; 316)]; (::; `metals; `fire; "clay"; 316)]
.test.ASSERT_EQ["append - long"; .api.concat_list2[1 2 3; 4 5]; 1 2 3 4 5]
.test.ASSERT_EQ["append - symbol"; .api.concat_list2[`a`b`c; `d`e]; `a`b`c`d`e]
// append - error
.test.ASSERT_ERROR["append - failure"; .api.concat_list2; (1 2 3; "45"); "not a list or types do not match"]


// simple_to_compound
.test.ASSERT_EQ["simple_to_compound"; .api.drift[]; (12i; 34i; `vague; -3000i)]
enum: `mashroom`broccoli`cucumber;
enum2: `mackerel`swordfish`tuna;
.test.ASSERT_EQ["simple_to_compound"; .api.drift2[]; (`enum$`mashroom; `enum$`broccoli; `enum2$`tuna; 2000.04m)]



//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          	  Result   	        		                  //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

// Show result.
.test.DISPLAY_RESULT[]
exit 0