set boxwidth 0.5
set terminal pngcairo enhanced font "Sans,25.0" size 1920,1080
set style fill solid
set key off
set ylabel "Relative runtime (lower is better)"

set output "out/rust-only.png"
set yrange [0:2]
plot "graphs/rust-only.dat" using 1:3:xtic(2) with boxes

set output "out/all.png"
set yrange [0:22]
plot "graphs/all.dat" using 1:3:xtic(2) with boxes
