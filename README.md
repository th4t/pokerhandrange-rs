#pokerhandrange-rs

There is an example application which makes use of them to let two ranges compete against each other.
The crate defines a Range trait and contains a SimpleRange, which
can be used to create a range representation with *new_from_string("AA,AK+")*, where the string can differ.
The resulting SimpleRange can be used to check if any two cards are *contained* by it or to *draw* sample cards from it.

The crate is called `pokerhandrange` and you can depend on it via cargo:

```ini
[dependencies.pokerhandrange]
git = "https://github.com/th4t/pokerhandrange-rs.git"
```

## Implemented string range descriptors
Possible "kinds" of range describing components:
* JJ-KK
* 99
* QQ+  (QQ,KK,AA)
* AJo
* KJo+ (only J rises up to K, but not including)
* AJs
* KJs+
* AJ (AJs and AJo)
* AJ+ (AJs+ and AJo+)

Potentionally useful, but missing:
* Axs = A2s+
* Ax = A2+

##About the example application
This particular application is not accurate for several reasons, some obvious (one of them is described below) and some more sneaky. The approach will not scale to three or more hands. It works in the following way: two hand ranges are specified through strings at the beginning and then cards are repeatedly drawn from them, community cards are dealt and the strength of each hand is evaluated. Thousands of times. Stats are printed out at the end.

As an example of a statistical error that is able to skew the results: one narrower handrange is capable of *dominating* the other if they overlap on each occasion when the narrower one is assembled first and we insist on non-overlapping hole cards. Thanks to the "hole cards must be different, always" check. This approach will work even worse for 3+ competing ranges.

##TODOs

* More complex range types (weighted manually (90% AA, 10% 22-TT))
* More Comments
* More tests
* Place utility functions elsewhere
* Exclude example application from being built on *cargo test* target
* OTHER: Pokerevaluator trait in the pokereval crate, so lookup and eval are exchangeable?
