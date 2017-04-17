#!/usr/bin/gnuplot

#set terminal pngcairo size 1920,1080 enhanced font 'Verdana,9'
set terminal pngcairo size 800,600 enhanced font 'Verdana,12'
set output 'convergence.png'

#set terminal epslatex  font 'Verdana,9'
#set output 'mip_results.tex'

set title "Iteration versus Fitness"

set style line 11 lc rgb '#808080' lt 1
set border 3 back ls 11

set tics nomirror

set style line 12 lc rgb '#808080' lt 0 lw 1

set key top right

set grid back ls 12

set style line 1 lc rgb '#8b1a0e' pt 1 ps 1 lt 1 lw 2 # --- red
set style line 2 lc rgb '#5e9c36' pt 2 ps 1 lt 1 lw 2 # --- green
#set style line 3 lc rgb '#65393d' pt 3 ps 1 lt 1 lw 2 # --- brown
#set style line 4 lc rgb '#3db7c2' pt 4 ps 1 lt 1 lw 2 # --- blue
#set style line 5 lc rgb '#f9c386' pt 5 ps 1 lt 1 lw 2 # --- blue
#set style line 6 lc rgb '#98cdc5' pt 6 ps 1 lt 1 lw 2 # --- grey-cyan-thing

set ylabel 'Fitness'
set xlabel 'Iteration'

set xtics rotate out

plot 'best.log'  using 1:2 title 'Best'      with lines,\
     'average.log' using 1:2 title 'Average' with lines
