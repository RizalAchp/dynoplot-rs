# LIBRARY FOR GENERATING PLOT IMAGE FROM DATA IN MY DYNOTEST APPS C++
using rust, because in c++ there is no `simple` plot image generator. so im using rust for that.
using `plotters` library from rust that simple enough to to the job.

## Design
1. pointer data (from c++)
2. wrap it with unsafe block (in rust)
3. then, process the given data from c++ and save it to file.
4. return status code (in c++)
