"""Simulation behind the G3 row of the #2244 review table.

Replays the exact arithmetic of `Metrics::recalculate_udp_avg_processing_time_ns`
before the fix, where the previous average was truncated to `u64`, for 200,000
random 20-60 microsecond requests, and prints it next to the true mean.
"""
import math
import random

random.seed(1)
stored = 0.0
n = 0
true_sum = 0
for i in range(1, 200001):
    x = random.randint(20_000, 60_000)
    n += 1
    true_sum += x
    prev = math.trunc(stored)  # previous_avg as u64
    stored = prev + (x - prev) / n  # the code before the fix
    if i in (10, 100, 1000, 10000, 100000, 200000):
        print(f"n={i:>6} current={stored:>10.1f} true={true_sum / n:>10.1f}")
