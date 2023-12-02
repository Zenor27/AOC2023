#!/bin/sh

# Ask the user for the day number
echo "📆 Enter the day number: "
read day

echo "Creating file structure 📁"
# Create the directory if not exists in src/aoc2023/day<day>
mkdir -p src/aoc2023/day"$day"

# Create the input files
touch src/aoc2023/day"$day"/input1.txt
touch src/aoc2023/day"$day"/input2.txt

# Copy the day.template.rs file to the day<day>.rs file in the src/aoc2023/day<day> directory
cp ./day.template.rs src/aoc2023/day"$day"/day"$day".rs

# Copy the mod.template.rs file to the mod.rs file in the src/aoc2023/day<day> directory
touch src/aoc2023/day"$day"/mod.rs

echo  "Registering the modules 📝"
# Replace every occurrence of "{{day}}" by the day number in the mod.rs file
sed -E "s/{{day}}/$day/g" ./mod.template.rs > src/aoc2023/day"$day"/mod.rs

# Add pub mod day<day> to the mod.rs file in the src/aoc2023 directory
echo "pub mod day$day;" >> src/aoc2023/mod.rs

# In day_to_solve_functions.rs, add '"day<day>" => [day<day>::solve1, day<day>::solve2],' next to '"day1" => [day1::solve1, day1::solve2],'
sed -E "s/\"day$((day-1))\" => \[aoc2023::day$((day-1))::solve1, aoc2023::day$((day-1))::solve2\],/\"day$((day-1))\" => [aoc2023::day$((day-1))::solve1, aoc2023::day$((day-1))::solve2],\n\t\"day$day\" => [aoc2023::day$day::solve1, aoc2023::day$day::solve2],/g" ./src/aoc2023/day_to_solve_functions.rs > src/aoc2023/day_to_solve_functions.rs.tmp && mv src/aoc2023/day_to_solve_functions.rs.tmp src/aoc2023/day_to_solve_functions.rs

echo "Formatting the code 🧹"
# format the code
cargo fmt

echo "Done 🎉"