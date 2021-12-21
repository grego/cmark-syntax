#!/bin/bash
for f in $(find . -maxdepth 2 -name '*.tif' -o -name '*.ppm'); do
    nf=${f%.*}.jpg
    wf=${f%.*}.webp

    convert $f -resize 2048x pnm:- | cjpeg -quality 80 -optimize > ${f%/*}/half/${nf##*/}
    echo ${f%/*}/half/${nf##*/} created

    convert $f -resize 1280x pnm:- | cjpeg -quality 80 -optimize > ${f%/*}/retina/${nf##*/}
    echo ${f%/*}/retina/${nf##*/} created

    convert $f -resize 768x pnm:- | cjpeg -quality 80 -optimize > ${f%/*}/thumb/${nf##*/}
    echo ${f%/*}/thumb/${nf##*/} created

    cwebp -resize 2048 0 -q 80 -m 6 -mt -hint photo -quiet $f -o ${f%/*}/half/${wf##*/}
    echo ${f%/*}/half/${wf##*/} created
    
    cwebp -resize 1280 0 -q 75 -m 6 -mt -hint photo -quiet $f -o ${f%/*}/retina/${wf##*/}
    echo ${f%/*}/retina/${wf##*/} created

    cwebp -resize 768 0 -q 85 -m 6 -mt -hint photo -quiet $f -o ${f%/*}/thumb/${wf##*/}
    echo ${f%/*}/thumb/${wf##*/} created

    ./rm $f
done
