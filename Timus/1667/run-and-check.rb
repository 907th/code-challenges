def sh(cmd)
  puts cmd
  `#{cmd}`
  status = $?.exitstatus
  raise "bad exit status: #{status}" if status != 0
end

sh 'docker-compose run --rm godev go build -o ./1667/main.o ./1667/main.go'
sh 'docker-compose run --rm godev sh -c "./1667/main.o <./1667/in.txt >./1667/out.txt"'

inputs_count, *inputs = File.readlines('in.txt').map(&:strip)
*results = File.readlines('out.txt').map(&:strip)

inputs_count = inputs_count.to_i
raise 'inputs_count != 400' if inputs_count != 400

def square?(num)
  a = Math.sqrt(num).to_i
  a += 1 if a * a < num
  a * a == num
end

def check(r, c, ary)
  rs = [0] * r
  cs = [0] * c
  ary.each_with_index do |row_nums, i|
    row_nums.each_with_index do |num, j|
      rs[i] += num
      cs[j] += num

      return false unless square?(num)
    end
  end
  return false unless rs.all? { |s| square?(s) }
  return false unless cs.all? { |s| square?(s) }

  true
end

def ary_to_ans(ary)
  ary.map { |row_nums| row_nums.join(' ') + "\n" }.join
end

inputs_count.times do |i|
  r, c = inputs[i].split(' ').map(&:to_i)

  if i > 0
    raise 'no empty line between results' if results[0] != ''

    results = results[1, results.size - 1]
  end

  raise 'not enough lines in results' if results.size < r

  result = results[0, r]
  results = results[r, results.size - r]

  ary = result.map do |str|
    nums = str.split(' ').map(&:to_i)
    raise 'not enough columns' if nums.size != c

    nums
  end

  raise "bad answer for r = #{r}, c = #{c}:\n#{ary_to_ans(ary)}" unless check(r, c, ary)
end

raise 'too many lines in results' if results.size != 0
