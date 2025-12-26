tests = []
20.times do |r|
  20.times do |c|
    tests << [r + 1, c + 1]
  end
end

File.open('in.txt', 'w') do |f|
  f.puts tests.size
  tests.each do |r, c|
    f.puts "#{r} #{c}"
  end
end
