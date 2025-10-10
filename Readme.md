# Copar - A COmmand PARser in rust
Copar (COmmand PArser) allows to generate structured command lists in different languages from command logs complying with the COPAR language specification.  

## Typical use case
You have a driver sending a complex series of command to a device for initializing it. Your goal is to reproduce the driver initialization. However you don't want to dive into understanding the complex drive code or maybe it is closed source and you can't access it.

You can insert COPAR-style records at key locations in the low-level access layer to log every operation going in and out of the driver. Give those line of logs to the parser to synthetize an array of commands in the language of your choice. It's up to you to write the program that will read and execute this array of operations.

## How to use it

## COPAR language specification
