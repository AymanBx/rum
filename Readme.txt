Ayman Sandouk & Omar Martinez

-Two main structures were used where the field structure handles
returning the value of whatever is found in a register and
the machine structure which is utilized to initialize
the registers, memory, instruction pointer, and a tracker
for unmapped segments. The machine structure has been
implemented correctly as we have executed all binary files
properly. When it goes to handle the operations once the
Opcode has been matched, every operation is properly handling
edge cases. For our Add and multiply we are assuring there is
no overflow error by utilizing the wrapping call. For our map
seg we are checking to see if there is a key available in our
unmapKey array to grab from, if not we will insert a value in
the first position then create a key to utilize. This works as
we can grab and remove the keys and values associated with the
register. The outputVal call is properly outputting the value
that is located in the certain register. This was proved by 
using cat.um as every input we passed into std::In was printed
as it is supposed to do. Not only that, but it also proved that
std::in was fully functional as it took our input and properly
handled it to get passed onto the output call. Our rumLoad is
also properly working as we are able to disssamble the instructions
that is found within the binary file that will be passed into our
main to be processed. The way we can prove this works is because
we are able to input a file and get some sort of output. We can
see that instructions are being passed and operations are being
utilized. 


- For the modules that were created for the UM implementation
we have rumOps, rumLoad, main, and lib. What rum ops takes care
of is it deals with the operations neccessary once the opcode has
been called. For exmaple in rumOps we have mapSeg, load, input,
jump and Cmov. They got their own module because it required more
work, so it was neater to conceal it and just call on the function
neccessary. All main does is intialize the memory/registers and
pass in the file to retrive the instructions needed to execute.
The lib module initializes our structure so we can keep those hidden
while still being able to utilizie them everywhere else. They all
relate to one another as the main takes in the file and if an
instruction is called by the Opcode rumOps will do the operation
neccessary and return its work. The main is also calling on the lib
to bring in the structure and intialize the registers to the required
size and such. RumLoad is also called from main in order to take in the
binary file to disect the instructions from it.

- The time it takes our machine to execute 50 million instructions is


-We have spent 3 hours analyizing the design document to see what
must be done in order to create a functional Universal Machine

-We have spent 2 hours preparing the design from how we are going
to represent the registers/memory with the idea of building an
effiecent machine.

-For the final assignment and the construction of the universal
machine it took about 15 hours.