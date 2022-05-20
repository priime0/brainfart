[
    This demonstrates a brainfuck swapping algorithm that uses a temporary
    variable.
]
                                        #
+++++>                                  Cell0 = 5
+++++++>                                Cell1 = 7
++++<<                                  Cell2 = 4
                                        Begin swap:
>>[-]<<                                 Clear Cell2
[->>+<<]>                               Move Cell0 to Cell2
[-<+>]>                                 Move Cell1 to Cell0
[-<+>]<<                                Move Cell2 to Cell1
.>.>.                                   Print the cell contents
