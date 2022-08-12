### Running Benchmarks

#### HTTP(S) Announce Peer + Torrent
For this benchmark we use the tool [wrk](https://github.com/wg/wrk).

To run the benchmark using wrk, execute the following example script (change the url to your own tracker url):

    wrk -c200 -t1 -d10s -s ./wrk_benchmark_announce.lua --latency http://tracker.dutchbits.nl

