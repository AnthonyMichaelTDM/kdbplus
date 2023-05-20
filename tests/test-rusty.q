/
* @file test.q
* @overview Tests of C API examples. The artefact of `rusty_api_examples` is loaded
* and functions are called from q side.
\

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                           Inital Setting     			            //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

// Fetch shared object from build directory.
//system "cp ../target/debug/libapi_examples.so .";

// Load test helper functions.
\l test_helper_function.q

// Function to load shared library.
LIBPATH_: `librusty_api_examples 2:

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          	Load Libraries     			            //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//


// These function list can be checked against `nm -D librusty_api_examples.so | awk '$2 ~/T/ {print $3}'`.

/ // decrement_reference_count
/ .api.agriculture: LIBPATH_ (`agriculture; 1);
/ // ee
/ .api.catchy: LIBPATH_ (`catchy; 2);
// qnull_base::C
.api.char_border: LIBPATH_ (`char_border; 1);
/ // jv
/ .api.concat_list: LIBPATH_ (`concat_list; 2);
/ // b9
/ .api.conceal: LIBPATH_ (`conceal; 1);
// append
.api.concat_list2: LIBPATH_ (`concat_list2; 2);
/ // kb
/ .api.create_bool: LIBPATH_ (`create_bool; 1);
/ // kb
/ .api.create_bool2: LIBPATH_ (`create_bool2; 1);
/ // kg
/ .api.create_byte: LIBPATH_ (`create_byte; 1);
/ // ku
/ .api.create_guid: LIBPATH_ (`create_guid; 1);
/ // ki
/ .api.create_int: LIBPATH_ (`create_int; 1);
/ // kj
/ .api.create_long: LIBPATH_ (`create_long; 1);
/ // kc
/ .api.create_char: LIBPATH_ (`create_char; 1);
/ // new_char
/ .api.create_char2: LIBPATH_ (`create_char2; 1);
/ // jk
/ .api.create_compound_list: LIBPATH_ (`create_compound_list; 1);
// push
.api.create_compound_list2: LIBPATH_ (`create_compound_list2; 1);
// kd
/ .api.create_date: LIBPATH_ (`create_date; 1);
/ // kz
/ .api.create_datetime: LIBPATH_ (`create_datetime; 1);
/ // xD
/ .api.create_dictionary: LIBPATH_ (`create_dictionary; 1);
/ // new_enum
/ .api.create_enum: LIBPATH_ (`create_enum; 2);
/ // kf
/ .api.create_float: LIBPATH_ (`create_float; 1);
/ // knt
/ .api.create_keyed_table: LIBPATH_ (`create_keyed_table; 1);
/ // enkey
/ .api.create_keyed_table2: LIBPATH_ (`create_keyed_table2; 1);
/ // new_minute
/ .api.create_minute: LIBPATH_ (`create_minute; 1);
/ // new_month
/ .api.create_month: LIBPATH_ (`create_month; 1);
/ // ke
/ .api.create_real: LIBPATH_ (`create_real; 1);
/ // kh
/ .api.create_short: LIBPATH_ (`create_short; 1);
/ // new_second
/ .api.create_second: LIBPATH_ (`create_second; 1);
/ // ja
/ .api.create_simple_list: LIBPATH_ (`create_simple_list; 1);
/ // push_raw
/ .api.create_simple_list2: LIBPATH_ (`create_simple_list2; 1);
/ // kp
/ .api.create_string: LIBPATH_ (`create_string; 1);
/ // kpn
/ .api.create_string2: LIBPATH_ (`create_string2; 1);
/ // ks
/ .api.create_symbol: LIBPATH_ (`create_symbol; 1);
/ // new_symbol
/ .api.create_symbol2: LIBPATH_ (`create_symbol2; 1);
/ // js
/ .api.create_symbol_list: LIBPATH_ (`create_symbol_list; 1);
// push_symbol
.api.create_symbol_list2: LIBPATH_ (`create_symbol_list2; 1);
/ // xT
/ .api.create_table: LIBPATH_ (`create_table; 1);
/ // flip
/ .api.create_table2: LIBPATH_ (`create_table2; 1);
/ // kt
/ .api.create_time: LIBPATH_ (`create_time; 1);
/ // new_time
/ .api.create_time2: LIBPATH_ (`create_time2; 1);
/ // ktj
/ .api.create_timespan: LIBPATH_ (`create_timespan; 1);
/ // new_timespan
/ .api.create_timespan2: LIBPATH_ (`create_timespan2; 1);
/ // ktj
/ .api.create_timestamp: LIBPATH_ (`create_timestamp; 1);
/ // new_timestamp
/ .api.create_timestamp2: LIBPATH_ (`create_timestamp2; 1);
/ // dj
/ .api.days_to_date: LIBPATH_ (`days_to_date; 1);
/ // q_ipc_decode
/ .api.decrypt: LIBPATH_ (`decrypt; 1);
/ // k
/ .api.dictionary_list_to_table: LIBPATH_ (`dictionary_list_to_table; 1);
// simple_to_compound
.api.drift: LIBPATH_ (`drift; 1);
// simple_to_compound
.api.drift2: LIBPATH_ (`drift2; 1);
/ // set_qtype
/ .api.eden: LIBPATH_ (`eden; 1);
/ // q_ipc_encode
/ .api.encrypt: LIBPATH_ (`encrypt; 1);
// qnull_base::F
.api.float_borders: LIBPATH_ (`float_borders; 1);
// qnull_base::U
.api.guid_border: LIBPATH_ (`guid_border; 1);
/ // get_dictionary
/ .api.hidden_key: LIBPATH_ (`hidden_key; 1);
/ // r0
/ .api.idle_man: LIBPATH_ (`idle_man; 1);
// qnull_base::I
.api.int_borders: LIBPATH_ (`int_borders; 1);
/ // new_error
/ .api.keep_out: LIBPATH_ (`keep_out; 1);
/ // ktd
/ .api.keyed_to_simple_table: LIBPATH_ (`keyed_to_simple_table; 1);
/ // unkey
/ .api.keyed_to_simple_table2: LIBPATH_ (`keyed_to_simple_table2; 1);
/ // set_attribute
/ .api.labeling: LIBPATH_ (`labeling; 1);
// qnull_base::J
.api.long_borders: LIBPATH_ (`long_borders; 1);
// as_mut_slice
.api.modify_long_list_a_bit: LIBPATH_ (`modify_long_list_a_bit; 1);
/ // get_attribute
/ .api.murmur: LIBPATH_ (`murmur; 1);
// str_to_const_S
.api.must_be_int: LIBPATH_ (`must_be_int; 1);
/ // len
/ .api.numbers: LIBPATH_ (`numbers; 1);
/ // error_to_string
/ .api.no_panick: LIBPATH_ (`no_panick; 2);
/ // new_null
/ .api.nullify: LIBPATH_ (`nullify; 1);
/ // setm
/ .api.parallel_sym_change: LIBPATH_ (`parallel_sym_change; 1);
/ // r1
/ .api.pass_through_cave: LIBPATH_ (`pass_through_cave; 1);
/ // get_row
/ .api.pick_row: LIBPATH_ (`pick_row; 2);
// str_to_S
.api.pingpong: LIBPATH_ (`pingpong; 1);
/ // null_terminated_str_to_S
/ .api.pingpong2: LIBPATH_ (`pingpong2; 1);
// register_callback
.api.plumber: LIBPATH_ (`plumber; 1);
// get_bool
.api.print_bool: LIBPATH_ (`print_bool; 1);
// get_byte
.api.print_byte: LIBPATH_ (`print_byte; 1);
// get_char
.api.print_char: LIBPATH_ (`print_char; 1);
// get_float
.api.print_float: LIBPATH_ (`print_float; 1);
// get_guid
.api.print_guid: LIBPATH_ (`print_guid; 1);
// get_int
.api.print_int: LIBPATH_ (`print_int; 1);
// get_long
.api.print_long: LIBPATH_ (`print_long; 1);
// get_real
.api.print_real: LIBPATH_ (`print_real; 1);
// get_short
.api.print_short: LIBPATH_ (`print_short; 1);
// get_str
.api.print_string: LIBPATH_ (`print_string; 1);
// get_string
.api.print_string2: LIBPATH_ (`print_string2; 1);
/ // S_to_str
/ .api.print_symbol: LIBPATH_ (`print_symbol; 1);
// get_symbol
.api.print_symbol2: LIBPATH_ (`print_symbol2; 1);
/ // load_as_q_function
/ .api.probe: LIBPATH_ (`probe; 1);
/ // error_to_string
/ .api.propagate: LIBPATH_ (`propagate; 1);
// qnull_base::E
.api.real_borders: LIBPATH_ (`real_borders; 1);
/ // d9
/ .api.reveal: LIBPATH_ (`reveal; 1);
/ // dot
/ .api.rust_parse: LIBPATH_ (`rust_parse; 2);
/ // increment_reference_count
/ .api.satisfy_5000_men: LIBPATH_ (`satisfy_5000_men; 1);
// qnull_base::H
.api.short_borders: LIBPATH_ (`short_borders; 1);
// qnull_base::S
.api.string_borders: LIBPATH_ (`string_borders; 1);
/ // krr
/ .api.thai_kick: LIBPATH_ (`thai_kick; 1);
// KNULL
.api.vanity: LIBPATH_ (`vanity; 1);
/ // ymd
/ .api.ymd_to_days: LIBPATH_ (`ymd_to_days; 1);

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          	  Tests    	        		                  //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% Global Variable %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// KNULL
.test.ASSERT_EQ["KNULL"; .api.vanity[]; (::)]
// qnull_base::U
.test.ASSERT_EQ["qnull_base::U"; .api.guid_border[]; 0Ng]
// qnull_base::H
.test.ASSERT_EQ["qnull_base::H"; .api.short_borders[]; (0Nh; 0Wh; -0Wh)]
// qnull_base::I
.test.ASSERT_EQ["qnull_base::I"; .api.int_borders[]; (0Ni; 0Wi; -0Wi)]
// qnull_base::J
.test.ASSERT_EQ["qnull_base::J"; .api.long_borders[]; (0Np; 0Wp; -0Wp)]
// qnull_base::E
.test.ASSERT_EQ["qnull_base::E"; .api.real_borders[]; (0Ne; 0We; -0We)]
// qnull_base::F
.test.ASSERT_EQ["qnull_base::F"; .api.float_borders[]; (0Nz; 0Wz; -0Wz)]
// qnull_base::C
.test.ASSERT_EQ["qnull_base::C"; .api.char_border[]; " "]
// qnull_base::S
.test.ASSERT_EQ["qnull_base::S"; .api.string_borders[]; (`; "")]

//%% Macros %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// str_to_S
ping:{[int] `$string[int], "_pong!!"};
.test.ASSERT_EQ["str_to_S"; .api.pingpong[]; `$"77_pong!!"]

//%% KVal as Utilities %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// as_mut_slice
// Assign to a variable to keep the result.
.test.ASSERT_EQ["as_mut_slice - success"; .api.modify_long_list_a_bit[list:1 2 3]; 1 30000 3]
// as_mut_slice (return error)
.test.ASSERT_ERROR["as_mut_slice - failure"; .api.modify_long_list_a_bit; enlist enlist 1; "this list is not long enough"]

// get_bool
.test.ASSERT_EQ["get_bool - true"; .api.print_bool[1b]; (::)]
// get_bool
.test.ASSERT_EQ["get_bool - false"; .api.print_bool[0b]; (::)]
// get_bool - failure
.test.ASSERT_ERROR["get_bool - failure"; .api.print_bool; enlist 100; "not a bool"]

// get_byte
.test.ASSERT_EQ["get_byte"; .api.print_byte[0xc4]; (::)]
// get_byte - failure
.test.ASSERT_ERROR["get_byte - failure"; .api.print_byte; enlist "c"; "not a byte"]

// get_guid
guid: first 1?0Ng;
.test.ASSERT_EQ["get_guid"; .api.print_guid[guid]; (::)]
// get_guid - failure
.test.ASSERT_ERROR["get_guid - failure"; .api.print_guid; enlist 0x7a; "not a GUID"]

// get_short
.test.ASSERT_EQ["get_short"; .api.print_short[10h]; (::)]
// get_short - failure
.test.ASSERT_ERROR["get_short - failure"; .api.print_short; enlist 10; "not a short"]

// get_int
.test.ASSERT_EQ["get_int"; .api.print_int[42i]; (::)]
// get_int - month
.test.ASSERT_EQ["get_int - month"; .api.print_int[2010.03m]; (::)]
// get_int - date
.test.ASSERT_EQ["get_int - date"; .api.print_int[2020.02.01]; (::)]
// get_int - minute
.test.ASSERT_EQ["get_int - minute"; .api.print_int[12:03]; (::)]
// get_int - second
.test.ASSERT_EQ["get_int - second"; .api.print_int[03:57:20]; (::)]
// get_int - time
.test.ASSERT_EQ["get_int - time"; .api.print_int[00:34:16.636]; (::)]
// get_int - error
.test.ASSERT_ERROR["get_int - failure1"; .api.print_int; enlist `error; "not an int"]
// get_int - error
.test.ASSERT_ERROR["get_int - failure2"; .api.print_int; enlist 10000; "not an int"]

// get_long
.test.ASSERT_EQ["get_long"; .api.print_long[-109210]; (::)]
// get_long - timestamp
.test.ASSERT_EQ["get_long - timestamp"; .api.print_long[2000.01.01D12:00:00.123456789]; (::)]
// get_long - timespan
.test.ASSERT_EQ["get_long - timespan"; .api.print_long[-3D18:23:09.000000021]; (::)]
// get_long - enum
enum: `a`b;
.test.ASSERT_EQ["get_long - enum"; .api.print_long[`enum$`a]; (::)]
// get_long - error
.test.ASSERT_ERROR["get_long - failure"; .api.print_long; enlist 1b; "not a long"]

// get_real
.test.ASSERT_EQ["get_real"; .api.print_real[193810.32e]; (::)]
// get_real - error
.test.ASSERT_ERROR["get_real - failure"; .api.print_real; enlist 100f; "not a real"]

// get_float
.test.ASSERT_EQ["get_float"; .api.print_float[-37017.0933]; (::)]
// get_float - datetime
.test.ASSERT_EQ["get_float - datetime"; .api.print_float[2002.01.12T10:03:45.332]; (::)]
// get_float - error
.test.ASSERT_ERROR["get_float - failure"; .api.print_float; enlist .z.p; "not a float"]

// get_char
.test.ASSERT_EQ["get_char"; .api.print_char["k"]; (::)]
// get_char - error
.test.ASSERT_ERROR["get_char - failure1"; .api.print_char; enlist "devour"; "not a char"]
// get_char - error
.test.ASSERT_ERROR["get_char - failure2"; .api.print_char; enlist 1b; "not a char"]

// get_symbol
.test.ASSERT_EQ["get_symbol"; .api.print_symbol2[`locust]; (::)]
// get_symool - error
.test.ASSERT_ERROR["get_symbol - failure"; .api.print_symbol2; enlist "attack!"; "not a symbol"]

// get_str
.test.ASSERT_EQ["get_str"; .api.print_string["gnat"]; (::)]

// get_string
.test.ASSERT_EQ["get_string"; .api.print_string2["grasshopper"]; (::)]
// get_string - error
.test.ASSERT_ERROR["get_string - failure"; .api.print_string2; enlist (1 2; `a`b); "not a string"]

// get_dictionary
/ .test.ASSERT_EQ["get_string"; .api.hidden_key[([] t: `timestamp$.z.p+1e9*til 9; chr:"ljppkgfgs"; is: 7 8 12 14 21 316 400 1000 6000i)]; -8!`t`chr`is]
// get_dictionary - error
/ .test.ASSERT_ERROR["get_dictionary - failure"; .api.hidden_key; enlist 777; "not a table"]

// get_row
/ dictionaries: ([] time: `timestamp$2022.01.30D12:00:54.125743896 + 1000000000 * 1 + til 3; sym: `Green`Yellow`Red; go: "oxx"; miscellaneous: ("cow"; `lion; "eagle"));
/ .test.ASSERT_EQ["get_row - 2nd"; .api.pick_row[dictionaries; 1]; `time`sym`go`miscellaneous!(2022.01.30D12:00:56.125743896;`Yellow;"x";`lion)]
/ .test.ASSERT_EQ["get_row - 3rd"; .api.pick_row[dictionaries; 2]; `time`sym`go`miscellaneous!(2022.01.30D12:00:57.125743896;`Red;"x";"eagle")]
/ .test.ASSERT_ERROR["get_row - 4th"; .api.pick_row; (dictionaries; 3); "index out of bounds"]

// get_attribute - sorted
/ .test.ASSERT_EQ["get_attribute - sorted"; .api.murmur[`s#1 2 3]; "Clean"]
// get_attribute - unique
/ .test.ASSERT_EQ["get_attribute - unique"; .api.murmur[`u#1 2 3]; `Alone]
// get_attribute - parted
/ .test.ASSERT_EQ["get_attribute - parted"; .api.murmur[`p#1 2 3]; (::)]

// append
.test.ASSERT_EQ["append - compound"; .api.concat_list2[(::; `metals; `fire); ("clay"; 316)]; (::; `metals; `fire; "clay"; 316)]
.test.ASSERT_EQ["append - long"; .api.concat_list2[1 2 3; 4 5]; 1 2 3 4 5]
.test.ASSERT_EQ["append - symbol"; .api.concat_list2[`a`b`c; `d`e]; `a`b`c`d`e]
/ // append - error
.test.ASSERT_ERROR["append - failure"; .api.concat_list2; (1 2 3; "45"); "not a list or types do not match"]

/ // push
/ .test.ASSERT_EQ["push"; .api.create_compound_list2[5i]; (til 5), 5i]

/ // push_raw
/ .test.ASSERT_EQ["push_raw"; .api.create_simple_list2[]; 2000.01.01+til 5]

/ // push_symbol
/ .test.ASSERT_EQ["push_symbol"; .api.create_symbol_list2[]; `Abraham`Isaac`Jacob`Joseph]

//%% KVal as Constructors %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

//%% IPC Functions %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

//%% Reference count %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

//%% Miscellaneous %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

//%% Utility Functions %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

//%% Re-Export %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          	  Result   	        		                  //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

// Show result.
/ .test.DISPLAY_RESULT[]