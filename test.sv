module my_module #(
	BitWidth = 8
) (
	input logic[BitWidth-1:0] my_input,
	output logic[BitWidth-1:0] my_output);
 	assign my_output = my_input;
endmodule;

