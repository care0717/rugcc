require 'json'

def parce_like_json_string(str, i)
  res = {}
  ident = ""
  key = ""
  array = []
  h = {}
  regax = /^[[:alpha:][0-9]\(\)\+\-=*\/_]$/
  while i < str.size do
      c = str[i]
      if c == "{"
        if key != "" 
          res.merge!(h)
          ident, i = parce_like_json_string(str, i)
        end
        i+=1
      elsif c.match(regax) 
        ident = ""
        while str[i].match(regax) do
          ident+=str[i]
          i+=1
        end
      elsif c == ":" 
        key = ident
        ident = ""
        i+=1
      elsif c == "["
        i+=1
        while str[i] != "]"
          ident, i = parce_like_json_string(str, i)
          array.push(ident)
          i+=1
        end
        ident = array
        array = []
        i+=1
      elsif c == "," 
        if key == ""
          i+=1
          next
        elsif ident == "None" || ident == [] || ident == ""
          i+=1
          key = ""
          ident = ""
          next
        end 
        h[key] = ident
        i+=1
      elsif c == "}" 
        if ident != "None" 
          h[key] = ident
        end
        res.merge!(h)
        return res, i
      else 
        p "error: undefined char => #{str[i]}"
        exit
      end
  end
end

def puts_with_ln(lines)
  lines.each_with_index{|l, i|
    puts "#{i+1}:".ljust(4)+ l
  }
end

lines = STDIN.read.gsub(" ","").gsub('"',"").gsub('\'', "").gsub("Some(Node", "").gsub("Node", "").gsub("})", "}").split(/\n/)

left = lines.select{|line| line.include?("left:`")}[0]
right = lines.select{|line| line.include?("right:`")}[0]

node_start = left.index("`")
node_end = left.rindex("`")
left = left[node_start+1..node_end-1]

res, _  = parce_like_json_string(left, 0)
left = JSON.pretty_generate(res).split(/\n/)

node_start = right.index("`")
node_end = right.rindex("`")
right = right[node_start+1..node_end-1].gsub("Some(Node", "").gsub("Node", "")

res, _  = parce_like_json_string(right, 0)
right =JSON.pretty_generate(res).split(/\n/)

puts_with_ln(left)
puts_with_ln(right)

puts ""
[left.size, right.size].min.times { |i|
  if left[i] != right[i] 
    puts "diff: line #{i+1}"
    puts "result: #{left[i]}"
    puts "expect: #{right[i]}"
    puts ""
  end
}

