c3 = circle(3)
c3rot = c3:rotate(45):translate({ -2, 1 })
c12 = circle(12):translate({ 2, 0.5 })
c12scaled = c12:scale({ 0.25, 0.75 }):translate({ 0, -2 })

all = c3:concat(c3rot):concat(c12):concat(c12scaled)

app:output(all:extrude_linear(0.1))
