# readtime

Readtime is a domain specific regular language, along with an associated set of algorithms, for specifying and computing with human representations of time. It's designed such that oridinary representations of relative time– for instance, "89 hours after this time tomorrow"– can be computed in an unambiguous way.

## Concepts

readtime primarily manages time as timestamps– that is, a collection consisting of integer quantities of the year, month, day, hour, minute, second, and nanoseconds. It is purely a system for manipulating time in this representation; it does NOT attempt to handle absolute moments in time, UNIX time, or other more fundamental measures of time. Instead, we assume that your programming language's readtime implemention is capable of converting to and from these representations in the way that is most appropriate to your language.

readtime has two primary types: a point in time, called a `Moment`, and spans of time, called a `Duration`. These types are not representative of fixed points in time, or fixed durations. Instead, they are representative of relative units of time in various semi-independent units. For example, "1 month" and "1000 seconds" are both valid readtime durations. The specific rules for how duration calculations are applied are described further down, but as a simple example, 1 month after June 30 is July 30, and 1 month after January 31 is March 3 (March 2 in a leap year).

### Non-concepts

In an effort to be as simple as possible, readtime explicitly does not account for certain real-world aspects of time. While this may change in future versions, currently we do not support:

- Time zones
- Daylight savings time
- Other representations of time, such as UNIX time
- Relativity

A special note about daylight savings: while we'd very much like to support it, there's really no way to do it without also bringing in time zones, and it makes it substantially harder to provide unambiguous representation of moments in time. For now, we ask that you manually add or remove an hour for moments in time that cross DST boundaries.

## Language

The readtime language is an expression-based language which is capable of expressing `Moment`s and `Duration`s. It provides a simple set of arithmetic operations to interlink them, allowing for expressions of arbitrariy complexity.

### Durations

A readtime expression can be a `Duration`. The simplest durations consist of a quantity and a unit: `4 days`, `3 minutes`, `10 months`, `1 hour`.

Durations can be summed together by simply being next to each other, optionally separated by a comma or the word `and`: `4 days, 3 hours`, `1 year and 1 day`

#### Table of Units

This table shows the units of duration recognized by readtime. Many of them are simply a fixed quantity of a different unit; when this is the case it is indicated.

Unit       | Equivelent to
===========+==============
Nanosecond  |
Microsecond | 1000 Nanoseconds
Millisecond | 1000 Microseconds
Second      |
Minute      |
Hour        |
Day         |
Week        | 7 days
Tenday      | 10 days
Fortnight   | 2 weeks
Month       |
Season      | 3 months
Quarter     | 3 months
Year        |
Decade      | 10 years
Score       | 20 years
Century     | 10 decades
Millennium  | 10 centuries

### Moments

A readtime expression can be a `Moment`, which is a representation of a point in time. The simplest moments consist of specific, fully written out timestamps, which must include a date and time. In turn, a date must include a year, month, and day, and a time must include an hour, minute, second, and nanoseconds. All of the time fields default to 0 if not given (corresponding to exactly midnight).

#### Relative moments

readtime provides a `now` moment, which is (by default) whatever the current time is in the local time zone. Your readtime implementation should provide a more exact definition of how `now` is calculated.

readtime provides separate sub-moments for the current date and current time, represented by "this time" and "today". It also provides relative moments which are based on the current moment, such as "tomorrow",

In addition, readtime provides a few kinds of relative moments. All of these are relative to `now`, and can fulfill different parts of a moment:

- TBD

#### Moment arithmetic

Finally, moments can be combined with durations to compute new moments. Some example syntax:

```
60 minutes from now

2 weeks from now

1 month after January 31, 2018

2 years before this tuesday
```

### Grammar

```
Expression -> MomentExpression | DurationExpression

DurationExpression -> CompoundDuration
CompoundDuration -> Duration (DurationJoin Duration)*
DurationJoin -> () | ',' | 'and' | ',' 'and'
Duration -> DurationQuantity DurationUnit
DurationQuantity -> integer | 'a'
DurationUnit -> see table

MomentExpression -> (DurationExpression DurationMomentJoin)* Moment
Moment -> AbsYearMoment | RelativeYearMoment
AbsYearMoment ->

```
