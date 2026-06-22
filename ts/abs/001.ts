import * as fs from 'fs';

function main() {

    const input = fs.readFileSync(0, 'utf8').trim().split(/\s+/);
    
    const a = parseInt(input[0], 10);
    const b = parseInt(input[1], 10);
    const c = parseInt(input[2], 10);
    const s = input[3];
    
    console.log(`${a + b + c} ${s}`);
}

main();