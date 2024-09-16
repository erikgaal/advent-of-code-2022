let fs = require('fs');

let file = process.argv[2];

if (!file) {
    console.warn('Please provide a file to read from');
    process.exit();
}

console.log(`Reading from ${file}`);

let data = fs.readFileSync(file, 'utf8');

// Helper functions
String.prototype.lines = function () {
    return this.split(/\n+/g);
}

Array.prototype.chunk = function (size) {
      if (!this.length || size < 1) {
        return [];
      }

      var index = 0,
          resIndex = 0,
          result = Array(Math.ceil(this.length / size));

      while (index < this.length) {
        result[resIndex++] = this.slice(index, (index += size));
      }

      return result;
}

function part_one(data) {
    let x = 1;
    let cycle = 1;
    let total = 0;

    data.lines().forEach(function (line) {
        console.debug(`CYCLE ${cycle}`);
        
        if (cycle % 40 == 20) {
            total += cycle * x;
        }

        let [op, arg] = line.split(' ');

        cycle += 1;

        if (op === 'addx') {
            console.debug(`CYCLE ${cycle}`);

            if (cycle % 40 == 20) {
                total += cycle * x;
            }

            cycle += 1;
            x += parseInt(arg);
        }

        console.debug(`X ${x}, T ${total}`);
    });

    return total;
}

console.log(`Solution to part one is ${part_one(data)}`);

function part_two(data) {
    const COLS = 40;
    const ROWS = 6;

    let x = 1;
    let cycle = 1;
    let screen = [];

    function render_screen(screen) {
        return screen.chunk(COLS).map((row) => row.join('')).join("\n");
    }

    function is_lit(cycle, x) {
        let col = (cycle - 1) % COLS;

        return col >= (x - 1) && col <= (x + 1);
    }

    data.lines().forEach(function (line) {
        screen[cycle - 1] = is_lit(cycle, x) ? '#' : '.'

        // process.stdout.write(is_lit(cycle, x) ? '#' : '.');

        let [op, arg] = line.split(' ');

        cycle += 1;

        if (op === 'addx') {
            screen[cycle - 1] = is_lit(cycle, x) ? '#' : '.';

            // process.stdout.write(is_lit(cycle, x) ? '#' : '.');

            cycle += 1;
            x += parseInt(arg);
        }
    });

    console.log(render_screen(screen));
}

part_two(data);