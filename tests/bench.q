/
* @file bench.q
* @some benchmarking between rusty-api and api versions of certain workloads
\

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                           Inital Settings    			                  //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

// Fetch shared objects from build directory
system "cp ../target/release/libapi_examples.so .";
system "cp ../target/release/librusty_api_examples.so .";

// functions to load shared libaries.
LIBAPI_: `libapi_examples 2:
LIBRUSTY_: `librusty_api_examples 2:

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          	Load Libraries     			                  //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

// merging lists
.rusty.concat_list2: LIBRUSTY_ (`concat_list2; 2);
.api.concat_list2: LIBAPI_ (`concat_list2; 2);

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          	  Benches  	        		                  //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

/ as_mb: { ms:`long$`time$x; 1000f*BYTES_MB%`float$ms } / convert timings to MB/s
sizes: {ceiling 2 xexp x} {[step;start;length] start+step*til length}[1;4;16]

show "Making data for list using sizes..."
show sizes

data: { (::) , x ? 10000 } each sizes

/ show data

show "Done"

show "creating timing functions..."
time_rusty_join: {[x;y] start:.z.p; .rusty.concat_list2[x;y]; end:.z.p; ellapsed: end - start; ellapsed }
time_api_join:   {[x;y] start:.z.p; .api.concat_list2[x;y];   end:.z.p; ellapsed: end - start; ellapsed } 
show "Done"

show "create benchmark functions"
bench_rusty_join: {[x] time_rusty_join[x; reverse x] } each data
bench_api_join:   {[x] time_api_join  [x; reverse x] } each data
show "Done"

show "Running benchmarks..."
results_join: flip `SIZE`API_JOIN`RUSTY_API_JOIN!(sizes;bench_api_join;bench_rusty_join)
show "JOIN: "
show results_join

exit 0
