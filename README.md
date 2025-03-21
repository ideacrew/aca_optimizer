# Aca Optimizer

Optimized calculations and structures for ACA applications.

Useful when you need pure speed or greatly reduced memory usage - or both.

## Rate Cache

Provide far quicker rate calculations, with far less RAM usage, than the existing Enroll calculations.

Speed comparison (using ME rates, sampled with 20000k lookups):
```
       user     system      total        real
   0.022371   0.000746   0.023117 (  0.023150)
   0.035533   0.001344   0.036877 (  0.036888)
```

Memory usage (using ME rate catalog):
```
Ruby Cache Size:  334715872
Rust Cache Size:   76697256
Difference:       258018616 (77%)
```

This means the rate lookups are approximately **33%** faster, and use an average of **258M** less memory per Enroll process.

Speed comparison (using DC rates, sampled with 20000k lookups):
```
       user     system      total        real
   0.026077   0.001184   0.027261 (  0.027301)
   0.059429   0.001987   0.061416 (  0.062231)
```

Memory usage (using DC rate catalog):
```
Full Size: 822067601
Rust Cache Size: 105600696
Full Size after rust drop: 716467593
Full Size after all drop: 7092889
```
709374704
This means the rate lookups are approximately **56%** faster, and use an average of **600M** less memory per Enroll process.
