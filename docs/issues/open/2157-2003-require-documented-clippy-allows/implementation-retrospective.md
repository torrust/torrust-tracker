# Implementation Retrospective

## Material Finding

The prospective baseline is a merge-base diff rather than a checked-in inventory of legacy
attributes. It keeps #2157 reviewable and prevents the baseline from silently accepting a new or
modified undocumented `allow(clippy::...)`; #2158 remains responsible for the historical inventory.

The CI checkout explicitly fetches complete history because a shallow checkout cannot reliably
calculate that merge base.
