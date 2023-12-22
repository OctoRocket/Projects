function rng-audio-to-color
    set x (random 100 2000)
    set y (random 100 2000)
    set r (random 0 255)
    set g (random 0 255)
    set b (random 0 255)
    set buh "x"

    set colorc (random choice white black red orange yellow green blue indigo purple)

    ffmpeg -f lavfi -i color=c=$colorc:s=$x$buh$y -i $argv[1] -shortest -fflags +shortest $argv[2]
end
