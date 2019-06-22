(1..250).each do |frame|
  time = 10.0 * frame / 250
  `./target/release/raygrass --samples 50 --resolution 1280x960 --time #{time} frames/#{"%03d" % frame}.png`
end

`ffmpeg -r 25 -pattern_type glob -i 'frames/*.png' -c:v libx264 out.mp4`
`ffmpeg -f concat -safe 0 -i <(for i in {1..4}; do printf "file '%s'\n" "${PWD}/out.mp4"; done) -c copy looped.mp4`
`ffmpeg -i looped.mp4 -vcodec libx264 -pix_fmt yuv420p -strict -2 -acodec aac twitter.mp4`
