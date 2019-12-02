line_num=0
data = File.open('sample.txt').read()
data.gsub!(/\r\n?/, "\n")
data_array = []
data.each_line do |line|
  print "#{line_num += 1} #{line}"
  data_array.push(line)
end

data_array.each() do |sad|
  puts(sad)
end

