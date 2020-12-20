### Toy project that checks whether a string matches a given regex or not.

It is mostly based on [an article by Russ Cox](https://swtch.com/~rsc/regexp/regexp1.html) 
in an attempt to build a minimal regex engine inspired by 
[Thompson's construction algorithm](https://en.wikipedia.org/wiki/Thompson%27s_construction).
The main purpose of this exercise is to play with NFA, DFA and regex.
It is by no means a finished regex library and it deliberately omits support 
for some of the regex features like lookaheads etc.