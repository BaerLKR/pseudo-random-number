A very small script for pseudo random number generation in a custom range.

It is implemented by creating a high frequency sine wave and checking for the value of
the current unix time. Then that sine wave is modified to fit the parameters and the
values are transformed into i32 again.
