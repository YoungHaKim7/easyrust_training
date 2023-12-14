# Result

```

angle [0.0, 30.0, 45.0, 60.0, 90.0], shape=[5], strides=[1], layout=CFcf (0xf), const ndim=1
sine(a) [0, 0.49999999999999994, 0.7071067811865475, 0.8660254037844386, 1]
b: [[0.0, 1.0, 2.0],
 [3.0, 4.0, 5.0],
 [6.0, 7.0, 8.0]], shape=[3, 3], strides=[3, 1], layout=Cc (0x5), const ndim=2
a * 2 [0, 60, 90, 120, 180]
c + d [60.0, 110.0, 110.0], shape=[3], strides=[1], layout=CFcf (0xf), const ndim=1
c * d [800, 1000, 1000]
average(a) 45
mean(b) = Some(4.0)
mean(c) = Some(50.0)
```
