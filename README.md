# Rusty Camel
Ruby snake to camel in rust

### Example
``` ruby
require 'rusty_camel'

"some_snake_case_string".camelcase
"someCamelCaseString".snakecase

```

### Benchmark
Converting 50k rails accounts in json format 
```
To camel case
       user     system      total        real
Rust  0.970000   0.080000   1.050000 (  1.056655)
Ruby  9.940000   0.360000  10.300000 ( 10.363646)

To snake case
       user     system      total        real
Rust  1.100000   0.100000   1.200000 (  1.208300)
Ruby 10.960000   0.430000  11.390000 ( 11.404156)
```
