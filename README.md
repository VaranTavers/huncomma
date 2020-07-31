# huncomma

This library aims to be able to detect missing commas in text files that are written in Hungarian. 

To be able to correctly identify missing commas all the time, complex sentence analysis would be necessary. This project
doesn't aim to provide a library that is correct 100% of the time, and it doesn't use the aforementioned methods.

This is a library, if you need an executable check out these repositories:
* [huncomma_terminal][https://github.com/VaranTavers/huncomma_terminal]

## Methods that are used:
No methods presented are correct all the time, so each method also returns a floating-point number, which represents 
the probability of needing a comma, if that method detects it. The end result is the combination these probabilities. 

These are the methods that are used to determine possible missing commas:

### Words that are usually preceded by a comma

There are certain words which are usually preceded by commas:

`hogy`, `ami`, `aki`, etc.

If there are two or more of these words right after each other, only the first should be preceded by a comma.

### Word-pairs which imply a dependent clause

There are certain word pairs which when present point towards the existence of a dependent clause 
which should be separated by a comma.

Example: `ha ... akkor`: Ha mész vásárolni, akkor ne felejts el tejet hozni! (If you go shopping don't forget to buy milk!)

### Words that are usually followed by a comma

There are certain words which are usually followed by commas:

`szia`, `na`, etc.

Example: If a sentence begins with `Na` that words must be followed by a comma.

`Na, mondd már, hogy sikerült!` (the second comma is there because there is an implicit `azt, hogy`)


