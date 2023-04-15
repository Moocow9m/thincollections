# Benchmark Setup

The benchmarks can be found in the `benches` folder. The following results were executed on
an Intel i7-7700K with 16GB RAM, under Windows 10 with Rust nightly (around rustc 1.30). Unfortunately,
the rust benchmark feature is still unstable, so nightly is the only choice. No special compiler flags
were used (to be fair to the `std` impl).

We've measured insert and get performance for both `ThinMap` and `HashMap`.

Running the benchmarks takes a long time, so it you're going
to run them yourself, either run them overnight or run just one or two of them.

# Benchmark Results
Raw data is not included due to the sheer amount of file churn criterion makes. Simply run cargo bench on this repo to get the data.
Charts are in average time rather than throughput, so the lower, the better.

## 32 bit x 32 bit Insert `(K,V) = (i32, u32)`

The maps are initialized using `::new()`, so using all the defaults.

We generate a random vector and insert N values into the map. N is the X axis in the graph.
The Y axis is million operations per second. Note the logarithmic X axis.

The zig-zag in the performance is typical of all map implementation and happens because of
the amortized resizing algorithm.

![32 bit x 32 bit Insert](./32x32-rnd-insert.png)

## 32 bit x 32 bit get

The maps are initialized using `::new()`, so using all the defaults. We then insert N random keys
into the map. We then shuffle the keys and call get. Obviously, the order of the inserts and get are
different. Only the get call time is measured.

![32 bit x 32 bit get](./32x32-rnd-get.png)

## 64 bit x 64 bit Insert `(K,V) = (i64, u64)`

Same as above, but with 64 bit K,V.

![64 bit x 64 bit Insert](./64x64-rnd-insert.png)

## 64 bit x 64 bit get

Same as above, but with 64 bit K,V.

![64 bit x 64 bit get](./64x64-rnd-get.png)

## Trivial Hasher
It's interesting to tease out the difference in performance caused by the `Hasher` implementation
versus the structural differences in the two map implementation. Here, we use the same trivial (pass-through)
hasher for both maps. `ThinMap` maintains a smaller, but significant lead.

## 32 bit x 32 bit insert, trivial hasher

The maps are initialized using `::with_hasher(TrivialOneFieldHasherBuilder::new())`
We generate a random vector and insert N values into the map.

![32 bit x 32 bit Insert with Trivial hasher](./trivial-rnd-insert.png)

## 32 bit x 32 bit get, trivial hasher

The maps are initialized using `::with_hasher(TrivialOneFieldHasherBuilder::new())`. We then insert N random keys
into the map. We then shuffle the keys and call get. Obviously, the order of the inserts and get are
different. Only the get call time is measured.

![32 bit x 32 bit get with Trivial hasher](./trivial-rnd-get.png)

## 64 bit x 64 bit insert, Attack pattern

Could not find which benchmark generated this, so I excluded it from this fork for now.
