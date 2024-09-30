square1 = circle(4):scale({ 2, 2 })
square2 = square1:translate({ 1, 0 })

-- all = square1:concat(square2)

all = square1:union(square2)

app:output(all:extrude_linear(0.1))
