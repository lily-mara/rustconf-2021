set boxwidth 0.5
set terminal pngcairo enhanced font "Sans,25.0" size 1920,1080 background rgb '#0e0a29'

set border lw 3 lc rgb "#f0ece8"

set ylabel "Y" textcolor rgb "#f0ece8" font ",40"
set key font ",40"
set xtics font ",40"

set style fill solid
set key off
set ylabel "Relative runtime (lower is better)"

set output "out/rust-only.png"
set yrange [0:2]
plot "graphs/rust-only.dat" using 1:3:xtic(2) with boxes lt rgb "#fabf87"

set output "out/all.png"
set yrange [0:22]
plot "graphs/all.dat" using 1:3:xtic(2) with boxes lt rgb "#fabf87"
